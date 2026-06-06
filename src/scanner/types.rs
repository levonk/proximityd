use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of identifier for a discovered device/signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdType {
    /// Bluetooth Low Energy MAC address.
    #[serde(rename = "ble_mac")]
    BleMac,
    /// WiFi ARP table entry (MAC or IP).
    #[serde(rename = "wifi_arp")]
    WifiArp,
    /// Ping sweep response (IP address).
    #[serde(rename = "ping")]
    Ping,
    /// IPv4 address from ping sweep or other network discovery.
    #[serde(rename = "ip_v4")]
    IpV4,
    /// IPv6 address from network discovery.
    #[serde(rename = "ip_v6")]
    IpV6,
    /// mDNS broadcast identifier.
    #[serde(rename = "mdns")]
    Mdns,
    /// Generic or unknown identifier.
    #[serde(rename = "generic")]
    Generic,
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdType::BleMac => write!(f, "ble_mac"),
            IdType::WifiArp => write!(f, "wifi_arp"),
            IdType::Ping => write!(f, "ping"),
            IdType::IpV4 => write!(f, "ip_v4"),
            IdType::IpV6 => write!(f, "ip_v6"),
            IdType::Mdns => write!(f, "mdns"),
            IdType::Generic => write!(f, "generic"),
        }
    }
}

/// Predicate for `serde(skip_serializing_if)` on `HashMap` fields.
pub fn is_hashmap_empty(h: &HashMap<String, String>) -> bool {
    h.is_empty()
}

/// A raw signal discovered by any scanner implementation.
///
/// This is the universal data type produced by all `Scanner` implementations.
/// It captures the minimal set of fields needed for presence detection while
/// remaining extensible via the `metadata` map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawSignal {
    /// The kind of identifier (e.g., BLE MAC, WiFi ARP, ping IP, mDNS name).
    pub id_type: IdType,
    /// The actual identifier value (e.g., "AA:BB:CC:DD:EE:FF", "192.168.1.42").
    pub id_value: String,
    /// Received Signal Strength Indicator in dBm, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<i16>,
    /// Name of the scanner that produced this signal (e.g., "ble", "wifi_arp").
    pub scanner_name: String,
    /// Timestamp when the device was discovered.
    pub timestamp: DateTime<Utc>,
    /// Optional key-value metadata for scanner-specific extensions.
    #[serde(skip_serializing_if = "crate::scanner::types::is_hashmap_empty", default)]
    pub metadata: HashMap<String, String>,
}

impl RawSignal {
    /// Create a new `RawSignal` with the current timestamp.
    pub fn new(
        id_type: IdType,
        id_value: impl Into<String>,
        scanner_name: impl Into<String>,
    ) -> Self {
        Self {
            id_type,
            id_value: id_value.into(),
            rssi: None,
            scanner_name: scanner_name.into(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Set the RSSI value and return self for chaining.
    pub fn with_rssi(mut self, rssi: i16) -> Self {
        self.rssi = Some(rssi);
        self
    }

    /// Insert a metadata key-value pair and return self for chaining.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_signal_basic_construction() {
        let signal = RawSignal::new(IdType::BleMac, "AA:BB:CC:DD:EE:FF", "ble");
        assert_eq!(signal.id_type, IdType::BleMac);
        assert_eq!(signal.id_value, "AA:BB:CC:DD:EE:FF");
        assert_eq!(signal.scanner_name, "ble");
        assert!(signal.rssi.is_none());
        assert!(signal.metadata.is_empty());
    }

    #[test]
    fn raw_signal_with_rssi_and_metadata() {
        let signal = RawSignal::new(IdType::WifiArp, "192.168.1.1", "wifi_arp")
            .with_rssi(-45)
            .with_metadata("iface", "wlan0");

        assert_eq!(signal.rssi, Some(-45));
        assert_eq!(signal.metadata.get("iface"), Some(&"wlan0".to_string()));
    }

    #[test]
    fn id_type_display() {
        assert_eq!(IdType::BleMac.to_string(), "ble_mac");
        assert_eq!(IdType::WifiArp.to_string(), "wifi_arp");
        assert_eq!(IdType::Ping.to_string(), "ping");
        assert_eq!(IdType::IpV4.to_string(), "ip_v4");
        assert_eq!(IdType::IpV6.to_string(), "ip_v6");
        assert_eq!(IdType::Mdns.to_string(), "mdns");
        assert_eq!(IdType::Generic.to_string(), "generic");
    }
}
