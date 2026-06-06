//! Bluetooth Low Energy scanner using `btleplug`.
//!
//! `btleplug` is a cross-platform BLE library supporting Linux (BlueZ),
//! macOS (CoreBluetooth), and Windows (WinRT).
//!
//! This module replaces the legacy `bluez-async` Linux-only implementation.

use crate::scanner::types::{IdType, RawSignal};
use crate::scanner::Scanner;
use anyhow::{Context, Result};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use std::time::Duration;
use tracing::{debug, info, warn};

/// BLE scanner implementation via `btleplug`.
///
/// Discovers nearby BLE peripherals and converts them into [`RawSignal`]s.
/// The scan duration is fixed at 5 seconds per cycle (matching the legacy
/// scan window in `scan_loop.rs`).
pub struct BleScanner {
    /// Scan window duration in seconds.
    scan_duration_secs: u64,
    /// Whether this scanner is enabled in configuration.
    enabled: bool,
}

impl BleScanner {
    /// Create a new `BleScanner` with default settings.
    pub fn new() -> Self {
        Self {
            scan_duration_secs: 5,
            enabled: true,
        }
    }

    /// Create a new `BleScanner` with a custom scan duration.
    pub fn with_duration(seconds: u64) -> Self {
        Self {
            scan_duration_secs: seconds,
            enabled: true,
        }
    }

    /// Set the enabled flag from configuration.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Perform the actual BLE scan using `btleplug`.
    async fn perform_scan(&self) -> Result<Vec<RawSignal>> {
        let manager = Manager::new()
            .await
            .context("Failed to create btleplug Manager")?;

        let adapters = manager.adapters().await.context("Failed to list BLE adapters")?;

        if adapters.is_empty() {
            warn!("No BLE adapters found");
            return Ok(Vec::new());
        }

        let mut all_signals = Vec::new();

        for adapter in adapters {
            let adapter_info = adapter
                .adapter_info()
                .await
                .unwrap_or_else(|_| "unknown".to_string());
            debug!(adapter = %adapter_info, "Using BLE adapter");

            adapter
                .start_scan(ScanFilter::default())
                .await
                .context("Failed to start BLE scan")?;

            tokio::time::sleep(Duration::from_secs(self.scan_duration_secs)).await;

            adapter
                .stop_scan()
                .await
                .context("Failed to stop BLE scan")?;

            let peripherals = adapter
                .peripherals()
                .await
                .context("Failed to list peripherals")?;

            debug!(count = peripherals.len(), "Discovered BLE peripherals");

            for peripheral in peripherals {
                if let Some(props) = peripheral.properties().await.ok().flatten() {
                    let mac = props.address.to_string();
                    let mut signal = RawSignal::new(IdType::BleMac, mac, "ble");

                    if let Some(rssi_val) = props.rssi {
                        signal = signal.with_rssi(rssi_val);
                    }

                    if let Some(local_name) = props.local_name.as_ref() {
                        signal = signal.with_metadata("local_name", local_name.clone());
                    }

                    if let Some(manufacturer_data) = props.manufacturer_data.get(&0x004C) {
                        // Apple manufacturer ID — could be used for iBeacon detection
                        signal = signal.with_metadata(
                            "manufacturer",
                            format!("{:02X?}", manufacturer_data),
                        );
                    }

                    all_signals.push(signal);
                }
            }
        }

        info!(
            count = all_signals.len(),
            "BLE scan complete"
        );

        Ok(all_signals)
    }
}

impl Default for BleScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Scanner for BleScanner {
    async fn scan(&self) -> Result<Vec<RawSignal>> {
        if !self.enabled {
            debug!("BLE scanner is disabled, skipping scan");
            return Ok(Vec::new());
        }

        info!("Starting BLE scan via btleplug");
        match self.perform_scan().await {
            Ok(signals) => {
                debug!(count = signals.len(), "BLE scan succeeded");
                Ok(signals)
            }
            Err(e) => {
                warn!(error = %e, "BLE scan failed");
                Err(e)
            }
        }
    }

    fn name(&self) -> &'static str {
        "ble"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ble_scanner_name_and_defaults() {
        let scanner = BleScanner::new();
        assert_eq!(scanner.name(), "ble");
        assert!(scanner.enabled());
        assert_eq!(scanner.scan_duration_secs, 5);
    }

    #[test]
    fn ble_scanner_with_duration() {
        let scanner = BleScanner::with_duration(10);
        assert_eq!(scanner.scan_duration_secs, 10);
    }

    #[test]
    fn ble_scanner_disabled_returns_empty() {
        let mut scanner = BleScanner::new();
        scanner.set_enabled(false);
        assert!(!scanner.enabled());
    }

    #[tokio::test]
    async fn ble_scanner_disabled_scan_is_empty() {
        let mut scanner = BleScanner::new();
        scanner.set_enabled(false);
        let result = scanner.scan().await.unwrap();
        assert!(result.is_empty());
    }
}
