use chrono::{DateTime, Utc};

/// A BLE device discovered during a scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedDevice {
    /// MAC address (or BLE address) as a normalized string.
    pub mac: String,
    /// Received Signal Strength Indicator in dBm.
    pub rssi: i16,
    /// Timestamp when the device was last seen.
    pub last_seen: DateTime<Utc>,
}

impl ScannedDevice {
    /// Create a new ScannedDevice with the current timestamp.
    pub fn new(mac: impl Into<String>, rssi: i16) -> Self {
        Self {
            mac: mac.into(),
            rssi,
            last_seen: Utc::now(),
        }
    }
}
