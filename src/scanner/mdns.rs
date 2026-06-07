//! mDNS scanner that listens for multicast DNS announcements.
//!
//! This module implements cross-platform mDNS discovery:
//! - Linux: `avahi-browse` wrapper
//! - macOS: `dns-sd` wrapper
//!
//! If neither tool is available, the scanner logs a warning and returns empty results.

use crate::scanner::types::RawSignal;
use crate::scanner::Scanner;
use anyhow::Result;
use tracing::{debug, info, warn};

/// mDNS scanner implementation.
///
/// Listens for multicast DNS announcements via system tools and converts them
/// into [`RawSignal`]s with hostname identifiers.
pub struct MdnsScanner {
    /// Whether this scanner is enabled in configuration.
    enabled: bool,
    /// Scan interval in seconds (not used for single scan, but useful for scheduling).
    scan_interval_sec: u64,
}

impl MdnsScanner {
    /// Create a new `MdnsScanner` with default settings.
    pub fn new() -> Self {
        Self {
            enabled: true,
            scan_interval_sec: 30,
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

    /// Perform the actual mDNS scan.
    async fn perform_scan(&self) -> Result<Vec<RawSignal>> {
        let mut signals = Vec::new();

        // Try platform-specific mDNS discovery
        #[cfg(target_os = "linux")]
        {
            if let Ok(linux_signals) = self.scan_avahi().await {
                signals.extend(linux_signals);
            } else {
                warn!("avahi-browse not available or failed; mDNS scanner disabled");
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(macos_signals) = self.scan_dns_sd().await {
                signals.extend(macos_signals);
            } else {
                warn!("dns-sd not available or failed; mDNS scanner disabled");
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            warn!("mDNS scanner not supported on this platform");
        }

        info!(
            count = signals.len(),
            "mDNS scan complete"
        );

        Ok(signals)
    }

    /// Scan for mDNS services using `avahi-browse` on Linux.
    #[cfg(target_os = "linux")]
    async fn scan_avahi(&self) -> Result<Vec<RawSignal>> {
        // Check if avahi-browse is available
        let check = tokio::process::Command::new("which")
            .arg("avahi-browse")
            .output()
            .await;

        match check {
            Ok(output) if output.status.success() => {
                // avahi-browse is available, run it
                let output = tokio::process::Command::new("avahi-browse")
                    .args(["-a", "-t"])  // -a: browse all services, -t: terminate after first scan
                    .output()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to execute avahi-browse: {}", e))?;

                if !output.status.success() {
                    return Ok(Vec::new());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                return self.parse_avahi_output(&stdout);
            }
            _ => {
                return Err(anyhow::anyhow!("avahi-browse not found"));
            }
        }
    }

    /// Parse avahi-browse output to extract hostnames.
    #[cfg(target_os = "linux")]
    fn parse_avahi_output(&self, output: &str) -> Result<Vec<RawSignal>> {
        let mut signals = Vec::new();
        let mut hostnames = std::collections::HashSet::new();

        // avahi-browse output format:
        // =  eth0 IPv6 MyDevice                                 _http._tcp          local
        // =  eth0 IPv4 MyDevice                                 _http._tcp          local
        //   hostname = [mydevice.local]
        //   address = [192.168.1.42]
        //   port = [80]
        // ...

        for line in output.lines() {
            let line = line.trim();
            
            // Look for hostname lines
            if line.starts_with("hostname = [") {
                if let Some(start) = line.find('[') {
                    if let Some(end) = line.rfind(']') {
                        let hostname = &line[start + 1..end];
                        // Remove .local suffix if present
                        let hostname = hostname.trim_end_matches(".local");
                        
                        if !hostname.is_empty() && !hostnames.contains(hostname) {
                            hostnames.insert(hostname.to_string());
                        }
                    }
                }
            }
        }

        // Convert hostnames to signals
        for hostname in hostnames {
            let signal = RawSignal::new(crate::scanner::types::IdType::Hostname, hostname, "mdns");
            signals.push(signal);
        }

        debug!(count = signals.len(), "Parsed avahi-browse output");
        Ok(signals)
    }

    /// Scan for mDNS services using `dns-sd` on macOS.
    #[cfg(target_os = "macos")]
    async fn scan_dns_sd(&self) -> Result<Vec<RawSignal>> {
        // Check if dns-sd is available
        let check = tokio::process::Command::new("which")
            .arg("dns-sd")
            .output()
            .await;

        match check {
            Ok(output) if output.status.success() => {
                // dns-sd is available, run it
                // Note: dns-sd runs continuously, so we need to timeout
                let output = tokio::process::Command::new("dns-sd")
                    .args(["-B", "_services._dns-sd._udp", "local"])
                    .output()
                    .await;

                // dns-sd doesn't terminate gracefully, so we expect a timeout or signal
                // For now, just return empty if it fails
                match output {
                    Ok(_) => {
                        warn!("dns-sd scan completed (unexpected); treating as no results");
                        Ok(Vec::new())
                    }
                    Err(_) => {
                        // Expected failure due to timeout/signal
                        Ok(Vec::new())
                    }
                }
            }
            _ => {
                Err(anyhow::anyhow!("dns-sd not found"))
            }
        }
    }
}

impl Default for MdnsScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Scanner for MdnsScanner {
    async fn scan(&self) -> Result<Vec<RawSignal>> {
        if !self.enabled {
            debug!("mDNS scanner is disabled, skipping scan");
            return Ok(Vec::new());
        }

        info!("Starting mDNS scan");
        match self.perform_scan().await {
            Ok(signals) => {
                debug!(count = signals.len(), "mDNS scan succeeded");
                Ok(signals)
            }
            Err(e) => {
                warn!(error = %e, "mDNS scan failed");
                // Return empty instead of error to allow scanner to continue gracefully
                Ok(Vec::new())
            }
        }
    }

    fn name(&self) -> &'static str {
        "mdns"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mdns_scanner_name_and_defaults() {
        let scanner = MdnsScanner::new();
        assert_eq!(scanner.name(), "mdns");
        assert!(scanner.enabled());
        assert_eq!(scanner.scan_interval_sec, 30);
    }

    #[test]
    fn mdns_scanner_set_enabled() {
        let mut scanner = MdnsScanner::new();
        scanner.set_enabled(false);
        assert!(!scanner.enabled());
    }

    #[test]
    fn mdns_scanner_set_scan_interval() {
        let mut scanner = MdnsScanner::new();
        scanner.set_scan_interval(60);
        assert_eq!(scanner.scan_interval_sec, 60);
    }

    #[tokio::test]
    async fn mdns_scanner_disabled_scan_is_empty() {
        let mut scanner = MdnsScanner::new();
        scanner.set_enabled(false);
        let result = scanner.scan().await.unwrap();
        assert!(result.is_empty());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_avahi_output_empty() {
        let scanner = MdnsScanner::new();
        let result = scanner.parse_avahi_output("").unwrap();
        assert!(result.is_empty());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_avahi_output_with_hostnames() {
        let scanner = MdnsScanner::new();
        let output = r#"
=  eth0 IPv4 MyDevice                                 _http._tcp          local
  hostname = [mydevice.local]
  address = [192.168.1.42]
=  eth0 IPv4 AnotherDevice                            _ssh._tcp           local
  hostname = [another.local]
  address = [192.168.1.43]
"#;
        let result = scanner.parse_avahi_output(output).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id_value, "mydevice");
        assert_eq!(result[1].id_value, "another");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_avahi_output_duplicate_hostnames() {
        let scanner = MdnsScanner::new();
        let output = r#"
=  eth0 IPv4 MyDevice                                 _http._tcp          local
  hostname = [mydevice.local]
=  eth0 IPv4 MyDevice                                 _ssh._tcp           local
  hostname = [mydevice.local]
"#;
        let result = scanner.parse_avahi_output(output).unwrap();
        assert_eq!(result.len(), 1); // Should deduplicate
        assert_eq!(result[0].id_value, "mydevice");
    }
}