use serde::{Deserialize, Serialize};

/// Identifier type for device identity resolution.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IdentifierType {
    /// Bluetooth MAC address (e.g., "aa:bb:cc:dd:ee:ff")
    BleMac,
    /// WiFi MAC address (e.g., "aa:bb:cc:dd:ee:ff")
    WifiMac,
    /// IPv4 address (e.g., "192.168.1.10")
    IpV4,
    /// IPv6 address (e.g., "2001:db8::1")
    IpV6,
    /// Hostname (e.g., "alice-iphone")
    Hostname,
    /// RFID card ID
    CardId,
    /// Door sensor ID
    DoorSensor,
}

/// A single identifier for a device (e.g., a MAC address, IP, or hostname).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Identifier {
    /// Human-readable name for this identifier (e.g., "BLE MAC (main)")
    pub name: String,
    /// Type of identifier.
    #[serde(rename = "type")]
    pub id_type: IdentifierType,
    /// The identifier value (normalized to lowercase and trimmed on load).
    pub value: String,
}

/// A device with one or more identifiers.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Device {
    /// Human-readable device name (e.g., "Alice's iPhone").
    pub name: String,
    /// Optional location override for this device (overrides party-level location).
    #[serde(default)]
    pub location: Option<Location>,
    /// List of identifiers for this device.
    #[serde(default)]
    pub identifiers: Vec<Identifier>,
}

/// Hierarchical location model.
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Location {
    /// Building name.
    #[serde(default)]
    pub building: Option<String>,
    /// Floor number.
    #[serde(default)]
    pub floor: Option<u32>,
    /// Room name.
    #[serde(default)]
    pub room: Option<String>,
    /// Zone name.
    #[serde(default)]
    pub zone: Option<String>,
}

/// A party (person or entity) with one or more devices.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Party {
    /// Party name (e.g., "Alice", "Bob").
    pub name: String,
    /// Optional location for this party (can be overridden per-device).
    #[serde(default)]
    pub location: Option<Location>,
    /// List of devices belonging to this party.
    #[serde(default)]
    pub devices: Vec<Device>,
}

/// Top-level presence configuration containing all parties.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PresenceConfig {
    /// List of parties.
    #[serde(default)]
    pub parties: Vec<Party>,
}

impl Identifier {
    /// Normalize the identifier value (lowercase and trim).
    pub fn normalize_value(mut value: String) -> String {
        value = value.trim().to_lowercase();
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_presence_config() {
        let toml = r#"
[[parties]]
name = "Alice"
location = { building = "Home", floor = 1, room = "Living Room" }

  [[parties.devices]]
  name = "Alice's iPhone"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "AA:BB:CC:DD:EE:FF"

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "11:22:33:44:55:66"
"#;

        let config: PresenceConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.parties.len(), 1);
        assert_eq!(config.parties[0].name, "Alice");
        assert_eq!(config.parties[0].location.as_ref().unwrap().building, Some("Home".to_string()));
        assert_eq!(config.parties[0].devices.len(), 1);
        assert_eq!(config.parties[0].devices[0].name, "Alice's iPhone");
        assert_eq!(config.parties[0].devices[0].identifiers.len(), 2);
        assert_eq!(config.parties[0].devices[0].identifiers[0].id_type, IdentifierType::BleMac);
    }

    #[test]
    fn parse_empty_config_returns_empty() {
        let toml = "";
        let config: PresenceConfig = toml::from_str(toml).expect("valid TOML");
        assert!(config.parties.is_empty());
    }

    #[test]
    fn identifier_normalization() {
        assert_eq!(Identifier::normalize_value("  AA:BB:CC:DD:EE:FF  ".to_string()), "aa:bb:cc:dd:ee:ff");
        assert_eq!(Identifier::normalize_value("192.168.1.10".to_string()), "192.168.1.10");
        assert_eq!(Identifier::normalize_value("  My-Device  ".to_string()), "my-device");
    }

    #[test]
    fn all_identifier_types_parse() {
        let types = vec![
            ("ble_mac", IdentifierType::BleMac),
            ("wifi_mac", IdentifierType::WifiMac),
            ("ip_v4", IdentifierType::IpV4),
            ("ip_v6", IdentifierType::IpV6),
            ("hostname", IdentifierType::Hostname),
            ("card_id", IdentifierType::CardId),
            ("door_sensor", IdentifierType::DoorSensor),
        ];

        for (str_type, expected) in types {
            let toml = format!(
                r#"
name = "Test Identifier"
type = "{}"
value = "test-value"
"#,
                str_type
            );
            let id: Identifier = toml::from_str(&toml).expect(&format!("parse {}", str_type));
            assert_eq!(id.id_type, expected, "type {} should parse to {:?}", str_type, expected);
        }
    }
}
