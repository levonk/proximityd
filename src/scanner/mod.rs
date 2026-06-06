//! Generic presence scanner trait and implementations.
//!
//! The [`Scanner`] trait is the core abstraction for all signal sources:
//! BLE, WiFi ARP, ping sweep, mDNS, and future protocols.
//! Each implementation is toggleable via configuration.

pub mod ble;
pub mod scan_loop;
pub mod types;

use anyhow::Result;
use types::RawSignal;

/// Core trait for all presence detection scanners.
///
/// Implementors produce a batch of [`RawSignal`]s on each `scan()` call.
/// The trait is designed to be object-safe for use in registries and
/// dynamic dispatch scenarios.
#[async_trait::async_trait]
pub trait Scanner: Send + Sync {
    /// Perform a single scan cycle and return all discovered signals.
    ///
    /// The scan is expected to complete within a bounded time window
    /// (typically 5–15 seconds). An empty `Vec` means no devices were
    /// discovered during this cycle, not an error.
    async fn scan(&self) -> Result<Vec<RawSignal>>;

    /// Return the human-readable name of this scanner (e.g., `"ble"`, `"wifi_arp"`).
    fn name(&self) -> &'static str;

    /// Return whether this scanner is enabled in the current configuration.
    ///
    /// The default implementation returns `true`; override for config-driven toggles.
    fn enabled(&self) -> bool {
        true
    }
}

/// A registry that holds multiple [`Scanner`] implementations.
///
/// The registry filters out disabled scanners at insertion time so that
/// callers only iterate over active scanners.
pub struct ScannerRegistry {
    scanners: Vec<Box<dyn Scanner>>,
}

impl ScannerRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            scanners: Vec::new(),
        }
    }

    /// Register a scanner if it is enabled.
    pub fn register(&mut self, scanner: Box<dyn Scanner>) {
        if scanner.enabled() {
            tracing::debug!(scanner = %scanner.name(), "Registered scanner");
            self.scanners.push(scanner);
        } else {
            tracing::debug!(scanner = %scanner.name(), "Skipped disabled scanner");
        }
    }

    /// Iterate over all enabled scanners.
    pub fn iter(&self) -> impl Iterator<Item = &dyn Scanner> + '_ {
        self.scanners.iter().map(|s| s.as_ref())
    }

    /// Return the number of active scanners.
    pub fn len(&self) -> usize {
        self.scanners.len()
    }

    /// Return true if no scanners are registered.
    pub fn is_empty(&self) -> bool {
        self.scanners.is_empty()
    }
}

impl Default for ScannerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::types::IdType;

    struct MockScanner {
        name: &'static str,
        enabled: bool,
        results: Vec<RawSignal>,
    }

    #[async_trait::async_trait]
    impl Scanner for MockScanner {
        async fn scan(&self) -> Result<Vec<RawSignal>> {
            Ok(self.results.clone())
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn enabled(&self) -> bool {
            self.enabled
        }
    }

    #[tokio::test]
    async fn scanner_trait_mock_scan() {
        let scanner = MockScanner {
            name: "mock",
            enabled: true,
            results: vec![RawSignal::new(IdType::Generic, "test-1", "mock")],
        };

        let results = scanner.scan().await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id_value, "test-1");
        assert_eq!(scanner.name(), "mock");
    }

    #[test]
    fn registry_filters_disabled_scanners() {
        let mut reg = ScannerRegistry::new();
        reg.register(Box::new(MockScanner {
            name: "enabled_scanner",
            enabled: true,
            results: vec![],
        }));
        reg.register(Box::new(MockScanner {
            name: "disabled_scanner",
            enabled: false,
            results: vec![],
        }));

        assert_eq!(reg.len(), 1);
        assert_eq!(reg.iter().next().unwrap().name(), "enabled_scanner");
    }
}
