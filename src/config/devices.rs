use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single mapped device with its display name.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceConfig {
    /// Bluetooth MAC address (e.g., "AA:BB:CC:DD:EE:FF").
    pub mac: String,
    /// Human-readable display name for the device.
    pub name: String,
}

/// Collection of known device mappings keyed by MAC address.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DevicesConfig {
    /// Map of MAC address to device config.
    #[serde(default)]
    pub devices: HashMap<String, DeviceConfig>,
}

impl DevicesConfig {
    /// Look up a device by its MAC address.
    pub fn get(&self, mac: &str) -> Option<&DeviceConfig> {
        self.devices.get(mac)
    }

    /// Returns true if no devices are configured.
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_toml_with_devices() {
        let toml = r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Levon's Phone"

[devices."11:22:33:44:55:66"]
mac = "11:22:33:44:55:66"
name = "Levon's Watch"
"#;

        let config: DevicesConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.devices.len(), 2);
        let phone = config.get("AA:BB:CC:DD:EE:FF").expect("phone found");
        assert_eq!(phone.name, "Levon's Phone");
        let watch = config.get("11:22:33:44:55:66").expect("watch found");
        assert_eq!(watch.name, "Levon's Watch");
    }

    #[test]
    fn parse_empty_toml_returns_empty_config() {
        let toml = "";
        let config: DevicesConfig = toml::from_str(toml).expect("valid TOML");
        assert!(config.is_empty());
    }

    #[test]
    fn malformed_toml_errors() {
        let toml = r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = 123
"#;

        let result: Result<DevicesConfig, _> = toml::from_str(toml);
        assert!(result.is_err(), "malformed TOML should fail to parse");
    }
}
