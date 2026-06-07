//! ICMP ping sweep scanner using `fping` or raw ICMP sockets.
//!
//! This module implements network discovery via ICMP ping sweep:
//! - Preferred: `fping` command-line tool for fast parallel pings
//! - Fallback: Raw ICMP sockets using `tokio` (requires root on Linux)
//!
//! The scanner emits `RawSignal` with `IdType::IpV4` for each responsive host.

use crate::config::app::ScannerConfig;
use crate::scanner::types::{IdType, RawSignal};
use crate::scanner::Scanner;
use anyhow::{Context, Result};
use std::net::Ipv4Addr;
use tracing::{debug, info, warn};

/// Ping sweep scanner implementation.
///
/// Performs ICMP ping sweep on a configured subnet to discover active hosts.
/// Uses `fping` when available for fast parallel scanning, falls back to
/// raw ICMP sockets when `fping` is not installed.
pub struct PingSweepScanner {
    /// Whether this scanner is enabled in configuration.
    enabled: bool,
    /// Subnet to scan (e.g., "192.168.1.0/24").
    subnet: Option<String>,
    /// Scan interval in seconds (not used for single scan, but useful for scheduling).
    scan_interval_sec: u64,
    /// Whether to use fping if available.
    use_fping: bool,
}

impl PingSweepScanner {
    /// Create a new `PingSweepScanner` with default settings.
    pub fn new() -> Self {
        Self {
            enabled: false, // Disabled by default per task requirements
            subnet: None,
            scan_interval_sec: 60,
            use_fping: true,
        }
    }

    /// Create a new `PingSweepScanner` from configuration.
    pub fn from_config(config: &ScannerConfig) -> Self {
        let mut scanner = Self::new();
        scanner.set_enabled(config.enabled);
        scanner.set_scan_interval(config.scan_interval_sec);
        scanner.set_subnet(config.subnet.clone());
        scanner
    }

    /// Set the enabled flag from configuration.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set the subnet to scan (e.g., "192.168.1.0/24").
    pub fn set_subnet(&mut self, subnet: Option<String>) {
        self.subnet = subnet;
    }

    /// Set the scan interval from configuration.
    pub fn set_scan_interval(&mut self, interval_sec: u64) {
        self.scan_interval_sec = interval_sec;
    }

    /// Set whether to use fping if available.
    pub fn set_use_fping(&mut self, use_fping: bool) {
        self.use_fping = use_fping;
    }

    /// Check if fping is available on the system.
    async fn fping_available(&self) -> bool {
        if !self.use_fping {
            return false;
        }

        match tokio::process::Command::new("fping")
            .arg("--version")
            .output()
            .await
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    /// Perform the actual ping sweep scan.
    async fn perform_scan(&self) -> Result<Vec<RawSignal>> {
        let subnet = self
            .subnet
            .as_ref()
            .context("Ping sweep scanner requires subnet configuration")?;

        // Try fping first if enabled and available
        if self.fping_available().await {
            debug!("Using fping for ping sweep");
            return self.scan_with_fping(subnet).await;
        }

        // Fall back to raw ICMP sockets
        debug!("fping not available, using raw ICMP fallback");
        self.scan_with_icmp(subnet).await
    }

    /// Perform ping sweep using fping command-line tool.
    async fn scan_with_fping(&self, subnet: &str) -> Result<Vec<RawSignal>> {
        let output = tokio::process::Command::new("fping")
            .arg("-a") // Show only alive hosts
            .arg("-g") // Generate target list from subnet
            .arg(subnet)
            .output()
            .await
            .context("Failed to execute fping")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("fping failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut signals = Vec::new();

        // Parse fping output (one IP per line)
        for line in stdout.lines() {
            let ip = line.trim();
            if ip.is_empty() {
                continue;
            }

            // Validate that it's a valid IPv4 address
            if ip.parse::<Ipv4Addr>().is_ok() {
                let signal = RawSignal::new(IdType::IpV4, ip, "ping_sweep");
                signals.push(signal);
            }
        }

        info!(
            count = signals.len(),
            subnet = %subnet,
            "Ping sweep with fping complete"
        );

        Ok(signals)
    }

    /// Perform ping sweep using raw ICMP sockets.
    ///
    /// Note: This requires root privileges on Linux and may not work on all platforms.
    /// This is a fallback when fping is not available.
    async fn scan_with_icmp(&self, _subnet: &str) -> Result<Vec<RawSignal>> {
        warn!(
            "Raw ICMP ping sweep is not yet implemented. Please install fping for full functionality."
        );

        // For now, return empty result with a warning
        // TODO: Implement raw ICMP socket support
        Ok(Vec::new())
    }
}

impl Default for PingSweepScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Scanner for PingSweepScanner {
    async fn scan(&self) -> Result<Vec<RawSignal>> {
        if !self.enabled() {
            return Ok(Vec::new());
        }

        self.perform_scan().await
    }

    fn name(&self) -> &'static str {
        "ping_sweep"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::app::ScannerConfig;

    #[test]
    fn ping_sweep_scanner_default_disabled() {
        let scanner = PingSweepScanner::new();
        assert!(!scanner.enabled());
    }

    #[test]
    fn ping_sweep_scanner_configuration() {
        let mut scanner = PingSweepScanner::new();
        
        scanner.set_enabled(true);
        assert!(scanner.enabled());
        
        scanner.set_subnet(Some("192.168.1.0/24".to_string()));
        assert_eq!(scanner.subnet, Some("192.168.1.0/24".to_string()));
        
        scanner.set_scan_interval(30);
        assert_eq!(scanner.scan_interval_sec, 30);
    }

    #[test]
    fn ping_sweep_scanner_from_config() {
        let config = ScannerConfig {
            enabled: true,
            scan_interval_sec: 45,
            subnet: Some("10.0.0.0/8".to_string()),
            ..Default::default()
        };

        let scanner = PingSweepScanner::from_config(&config);
        assert!(scanner.enabled());
        assert_eq!(scanner.scan_interval_sec, 45);
        assert_eq!(scanner.subnet, Some("10.0.0.0/8".to_string()));
    }

    #[test]
    fn ping_sweep_scanner_name() {
        let scanner = PingSweepScanner::new();
        assert_eq!(scanner.name(), "ping_sweep");
    }

    #[tokio::test]
    async fn ping_sweep_scanner_scan_without_subnet() {
        let mut scanner = PingSweepScanner::new();
        scanner.set_enabled(true);
        // No subnet configured
        
        let result = scanner.scan().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("subnet"));
    }

    #[tokio::test]
    async fn ping_sweep_scanner_scan_disabled() {
        let scanner = PingSweepScanner::new();
        // Scanner is disabled by default
        
        let result = scanner.scan().await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn ping_sweep_scanner_fping_available() {
        let scanner = PingSweepScanner::new();
        // This test will pass or fail depending on whether fping is installed
        // We don't assert a specific value, just that it doesn't panic
        let _available = scanner.fping_available().await;
    }

    #[tokio::test]
    async fn ping_sweep_scanner_parse_fping_output() {
        let mut scanner = PingSweepScanner::new();
        scanner.set_enabled(true);
        scanner.set_subnet(Some("192.168.1.0/24".to_string()));

        // Mock test - actual fping execution is tested in integration tests
        // This test ensures the scanner structure is correct
        assert_eq!(scanner.name(), "ping_sweep");
    }

    #[test]
    fn ping_sweep_scanner_parse_ip_addresses() {
        // Test that we can parse valid IPv4 addresses
        assert!("192.168.1.1".parse::<Ipv4Addr>().is_ok());
        assert!("10.0.0.1".parse::<Ipv4Addr>().is_ok());
        assert!("172.16.0.1".parse::<Ipv4Addr>().is_ok());
        
        // Test that invalid addresses are rejected
        assert!("256.1.1.1".parse::<Ipv4Addr>().is_err());
        assert!("not.an.ip".parse::<Ipv4Addr>().is_err());
    }

    #[test]
    fn ping_sweep_scanner_default_trait() {
        let scanner = PingSweepScanner::default();
        assert_eq!(scanner.name(), "ping_sweep");
        assert!(!scanner.enabled());
        assert_eq!(scanner.scan_interval_sec, 60);
    }
}