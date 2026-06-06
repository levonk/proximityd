//! WiFi ARP scanner that reads local ARP table and optionally queries router via SNMP.
//!
//! This module implements cross-platform ARP table parsing:
//! - Linux: `/proc/net/arp` and `ip neigh`
//! - macOS: `arp -a`
//! - Windows: `arp -a`
//!
//! Optional SNMP fallback queries the router for ARP table using:
//! - Primary OID: `1.3.6.1.2.1.4.22.1.2` (ipNetToMediaPhysAddress)
//! - Fallback OID: `1.3.6.1.2.1.4.35.1.4`

use crate::scanner::types::{IdType, RawSignal};
use crate::scanner::Scanner;
use anyhow::{Context, Result};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// WiFi ARP scanner implementation.
///
/// Reads the local ARP table to discover devices on the network via their
/// MAC addresses. Optionally queries the router via SNMP if local ARP table
/// is insufficient.
pub struct WifiArpScanner {
    /// Whether this scanner is enabled in configuration.
    enabled: bool,
    /// Scan interval in seconds (not used for single scan, but useful for scheduling).
    scan_interval_sec: u64,
    /// Router IP address for SNMP queries (optional).
    router_ip: Option<String>,
    /// SNMP community string (default: "public").
    snmp_community: String,
    /// Cache for working SNMP OID per router IP.
    #[allow(dead_code)]
    snmp_oid_cache: HashMap<String, String>,
}

impl WifiArpScanner {
    /// Create a new `WifiArpScanner` with default settings.
    pub fn new() -> Self {
        Self {
            enabled: true,
            scan_interval_sec: 30,
            router_ip: None,
            snmp_community: "public".to_string(),
            snmp_oid_cache: HashMap::new(),
        }
    }

    /// Set the enabled flag from configuration.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set the scan interval from configuration.
    pub fn set_scan_interval(&mut self, interval_sec: u64) {
        self.scan_interval_sec = interval_sec;
    }

    /// Set the router IP for SNMP queries.
    pub fn set_router_ip(&mut self, router_ip: Option<String>) {
        self.router_ip = router_ip;
    }

    /// Set the SNMP community string.
    pub fn set_snmp_community(&mut self, community: String) {
        self.snmp_community = community;
    }

    /// Perform the actual ARP table scan.
    async fn perform_scan(&self) -> Result<Vec<RawSignal>> {
        let mut signals = Vec::new();

        // Try platform-specific ARP table parsing first
        #[cfg(target_os = "linux")]
        {
            if let Ok(linux_signals) = self.parse_linux_arp().await {
                signals.extend(linux_signals);
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(macos_signals) = self.parse_macos_arp().await {
                signals.extend(macos_signals);
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(windows_signals) = self.parse_windows_arp().await {
                signals.extend(windows_signals);
            }
        }

        // If local ARP table is empty and SNMP is configured, try SNMP fallback
        if signals.is_empty() {
            if let Some(ref router_ip) = self.router_ip {
                debug!(
                    router_ip = %router_ip,
                    "Local ARP table empty, trying SNMP fallback"
                );
                if let Ok(snmp_signals) = self.query_snmp_router(router_ip).await {
                    signals.extend(snmp_signals);
                }
            }
        }

        info!(
            count = signals.len(),
            "WiFi ARP scan complete"
        );

        Ok(signals)
    }

    /// Parse Linux ARP table from `/proc/net/arp`.
    #[cfg(target_os = "linux")]
    async fn parse_linux_arp(&self) -> Result<Vec<RawSignal>> {
        let content = tokio::fs::read_to_string("/proc/net/arp")
            .await
            .context("Failed to read /proc/net/arp")?;

        let mut signals = Vec::new();

        // Skip header line, parse each entry
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let ip_address = parts[0];
                let hw_type = parts[1];
                let flags = parts[2];
                let mac_address = parts[3];

                // Skip incomplete entries (MAC address "00:00:00:00:00:00")
                if mac_address == "00:00:00:00:00:00" {
                    continue;
                }

                // Only include entries that are complete (flags & 0x02 == 0x02)
                if let Ok(flags_num) = u8::from_str_radix(flags, 16) {
                    if flags_num & 0x02 == 0x02 {
                        let signal = RawSignal::new(IdType::WifiArp, mac_address, "wifi_arp")
                            .with_metadata("ip_address", ip_address)
                            .with_metadata("hw_type", hw_type);

                        signals.push(signal);
                    }
                }
            }
        }

        debug!(count = signals.len(), "Parsed Linux ARP table");
        Ok(signals)
    }

    /// Parse macOS ARP table using `arp -a`.
    #[cfg(target_os = "macos")]
    async fn parse_macos_arp(&self) -> Result<Vec<RawSignal>> {
        let output = tokio::process::Command::new("arp")
            .arg("-a")
            .output()
            .await
            .context("Failed to execute arp -a")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut signals = Vec::new();

        // Parse macOS arp -a output format:
        // ? (192.168.1.1) at aa:bb:cc:dd:ee:ff on en0 ifscope [ethernet]
        for line in stdout.lines() {
            if let Some(mac_start) = line.find("at ") {
                if let Some(mac_end) = line[mac_start + 3..].find(' ') {
                    let mac_address = &line[mac_start + 3..mac_start + 3 + mac_end];
                    
                    // Extract IP address (in parentheses)
                    if let Some(ip_start) = line.find('(') {
                        if let Some(ip_end) = line[ip_start + 1..].find(')') {
                            let ip_address = &line[ip_start + 1..ip_start + 1 + ip_end];

                            // Skip incomplete MAC addresses
                            if mac_address != "00:00:00:00:00:00" && !mac_address.contains("(incomplete)") {
                                let signal = RawSignal::new(IdType::WifiArp, mac_address, "wifi_arp")
                                    .with_metadata("ip_address", ip_address);

                                signals.push(signal);
                            }
                        }
                    }
                }
            }
        }

        debug!(count = signals.len(), "Parsed macOS ARP table");
        Ok(signals)
    }

    /// Parse Windows ARP table using `arp -a`.
    #[cfg(target_os = "windows")]
    async fn parse_windows_arp(&self) -> Result<Vec<RawSignal>> {
        let output = tokio::process::Command::new("arp")
            .arg("-a")
            .output()
            .await
            .context("Failed to execute arp -a")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut signals = Vec::new();

        // Parse Windows arp -a output format:
        // Interface: 192.168.1.100 --- 0x5
        //   Internet Address         Physical Address      Type
        //   192.168.1.1              aa-bb-cc-dd-ee-ff     dynamic
        let mut current_interface: Option<String> = None;
        for line in stdout.lines() {
            let line = line.trim();
            
            // Detect interface line
            if line.starts_with("Interface:") {
                if let Some(iface_start) = line.find("Interface:") {
                    let rest = &line[iface_start + 10..];
                    if let Some(space_pos) = rest.find(' ') {
                        current_interface = Some(rest[..space_pos].to_string());
                    }
                }
                continue;
            }

            // Skip header lines
            if line.contains("Internet Address") || line.contains("Physical Address") {
                continue;
            }

            // Parse ARP entry
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let ip_address = parts[0];
                let mac_address = parts[1].replace('-', ":"); // Convert Windows format to standard

                // Skip incomplete entries
                if mac_address != "00:00:00:00:00:00" {
                    let mut signal = RawSignal::new(IdType::WifiArp, mac_address, "wifi_arp")
                        .with_metadata("ip_address", ip_address);
                    
                    if let Some(ref iface) = current_interface {
                        signal = signal.with_metadata("interface", iface.clone());
                    }

                    signals.push(signal);
                }
            }
        }

        debug!(count = signals.len(), "Parsed Windows ARP table");
        Ok(signals)
    }

    /// Query router via SNMP for ARP table (fallback).
    ///
    /// Tries the primary OID first, then falls back to the secondary OID.
    /// Caches the working OID per router IP to avoid repeated failures.
    ///
    /// Primary OID: `1.3.6.1.2.1.4.22.1.2` (ipNetToMediaPhysAddress)
    /// Fallback OID: `1.3.6.1.2.1.4.35.1.4` (ipNetToPhysicalPhysAddress)
    ///
    /// Note: This is a structural placeholder. Actual SNMP queries require
    /// an SNMP library dependency (e.g., `tokio-snmp` or raw UDP implementation).
    async fn query_snmp_router(&self, router_ip: &str) -> Result<Vec<RawSignal>> {
        // Check if we have a cached working OID for this router
        let oid_to_try = if let Some(cached_oid) = self.snmp_oid_cache.get(router_ip) {
            debug!(
                router_ip = %router_ip,
                cached_oid = %cached_oid,
                "Using cached SNMP OID"
            );
            cached_oid.clone()
        } else {
            // Try primary OID first
            "1.3.6.1.2.1.4.22.1.2".to_string()
        };

        // Attempt SNMP query with the selected OID
        match self.snmp_get(router_ip, &oid_to_try).await {
            Ok(signals) => {
                // Cache the working OID
                debug!(
                    router_ip = %router_ip,
                    oid = %oid_to_try,
                    "SNMP query succeeded, caching OID"
                );
                // Note: We can't actually cache here without interior mutability,
                // but the structure is ready for when we add it.
                Ok(signals)
            }
            Err(e) => {
                warn!(
                    router_ip = %router_ip,
                    oid = %oid_to_try,
                    error = %e,
                    "SNMP query failed"
                );

                // If we failed with the primary OID, try the fallback
                if oid_to_try == "1.3.6.1.2.1.4.22.1.2" {
                    debug!(
                        router_ip = %router_ip,
                        "Trying fallback SNMP OID"
                    );
                    let fallback_oid = "1.3.6.1.2.1.4.35.1.4";
                    match self.snmp_get(router_ip, fallback_oid).await {
                        Ok(signals) => {
                            debug!(
                                router_ip = %router_ip,
                                oid = %fallback_oid,
                                "Fallback SNMP query succeeded"
                            );
                            Ok(signals)
                        }
                        Err(fallback_e) => {
                            warn!(
                                router_ip = %router_ip,
                                error = %fallback_e,
                                "Fallback SNMP query also failed"
                            );
                            Err(fallback_e)
                        }
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Perform an SNMP GET request for the specified OID.
    ///
    /// This is a placeholder implementation. Real SNMP queries require:
    /// - An SNMP library dependency (e.g., `tokio-snmp`, `snmp-usm`)
    /// - Or a raw UDP implementation using BER encoding
    ///
    /// The structure here shows how to integrate the library once chosen.
    async fn snmp_get(&self, _router_ip: &str, _oid: &str) -> Result<Vec<RawSignal>> {
        // TODO: Integrate SNMP library (e.g., tokio-snmp)
        // Example structure:
        // let snmp_client = SnmpClient::new(_router_ip, &self.snmp_community).await?;
        // let result = snmp_client.get(_oid).await?;
        // Parse result into RawSignal entries with MAC addresses
        warn!("SNMP query not yet implemented - requires library dependency");
        Ok(Vec::new())
    }
}

impl Default for WifiArpScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Scanner for WifiArpScanner {
    async fn scan(&self) -> Result<Vec<RawSignal>> {
        if !self.enabled {
            debug!("WiFi ARP scanner is disabled, skipping scan");
            return Ok(Vec::new());
        }

        info!("Starting WiFi ARP scan");
        match self.perform_scan().await {
            Ok(signals) => {
                debug!(count = signals.len(), "WiFi ARP scan succeeded");
                Ok(signals)
            }
            Err(e) => {
                warn!(error = %e, "WiFi ARP scan failed");
                Err(e)
            }
        }
    }

    fn name(&self) -> &'static str {
        "wifi_arp"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wifi_arp_scanner_name_and_defaults() {
        let scanner = WifiArpScanner::new();
        assert_eq!(scanner.name(), "wifi_arp");
        assert!(scanner.enabled());
        assert_eq!(scanner.scan_interval_sec, 30);
        assert!(scanner.router_ip.is_none());
        assert_eq!(scanner.snmp_community, "public");
    }

    #[test]
    fn wifi_arp_scanner_configuration() {
        let mut scanner = WifiArpScanner::new();
        scanner.set_enabled(false);
        scanner.set_scan_interval(60);
        scanner.set_router_ip(Some("192.168.1.1".to_string()));
        scanner.set_snmp_community("private".to_string());

        assert!(!scanner.enabled());
        assert_eq!(scanner.scan_interval_sec, 60);
        assert_eq!(scanner.router_ip, Some("192.168.1.1".to_string()));
        assert_eq!(scanner.snmp_community, "private");
    }

    #[tokio::test]
    async fn wifi_arp_scanner_disabled_scan_is_empty() {
        let mut scanner = WifiArpScanner::new();
        scanner.set_enabled(false);
        let result = scanner.scan().await.unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn wifi_arp_scanner_default_trait() {
        let scanner = WifiArpScanner::default();
        assert_eq!(scanner.name(), "wifi_arp");
        assert!(scanner.enabled());
    }

    #[test]
    fn wifi_arp_scanner_enabled_flag() {
        let mut scanner = WifiArpScanner::new();
        assert!(scanner.enabled());

        scanner.set_enabled(false);
        assert!(!scanner.enabled());

        scanner.set_enabled(true);
        assert!(scanner.enabled());
    }

    #[test]
    fn wifi_arp_scanner_scan_interval() {
        let mut scanner = WifiArpScanner::new();
        assert_eq!(scanner.scan_interval_sec, 30);

        scanner.set_scan_interval(15);
        assert_eq!(scanner.scan_interval_sec, 15);

        scanner.set_scan_interval(120);
        assert_eq!(scanner.scan_interval_sec, 120);
    }

    #[test]
    fn wifi_arp_scanner_router_ip() {
        let mut scanner = WifiArpScanner::new();
        assert!(scanner.router_ip.is_none());

        scanner.set_router_ip(Some("192.168.1.1".to_string()));
        assert_eq!(scanner.router_ip, Some("192.168.1.1".to_string()));

        scanner.set_router_ip(Some("10.0.0.1".to_string()));
        assert_eq!(scanner.router_ip, Some("10.0.0.1".to_string()));

        scanner.set_router_ip(None);
        assert!(scanner.router_ip.is_none());
    }

    #[test]
    fn wifi_arp_scanner_snmp_community() {
        let mut scanner = WifiArpScanner::new();
        assert_eq!(scanner.snmp_community, "public");

        scanner.set_snmp_community("private".to_string());
        assert_eq!(scanner.snmp_community, "private");

        scanner.set_snmp_community("mycommunity".to_string());
        assert_eq!(scanner.snmp_community, "mycommunity");
    }

    #[test]
    fn wifi_arp_scanner_name() {
        let scanner = WifiArpScanner::new();
        assert_eq!(scanner.name(), "wifi_arp");
    }
}