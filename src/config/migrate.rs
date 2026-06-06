use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

use super::{Device, DevicesConfig, Identifier, IdentifierType, PresenceConfig};

/// Migrate legacy `devices.toml` to new `presence.toml` format.
///
/// Reads the legacy devices config, converts it to a single default party
/// with all devices as BLE MAC identifiers, writes the new presence.toml,
/// and renames the old file to devices.toml.bak.
pub fn migrate_devices_to_presence(devices_path: PathBuf, presence_path: PathBuf) -> Result<()> {
    // Read legacy devices config
    let devices_contents = std::fs::read_to_string(&devices_path)
        .with_context(|| format!("Failed to read legacy devices config from {devices_path:?}"))?;

    let legacy_devices: DevicesConfig = toml::from_str(&devices_contents)
        .with_context(|| format!("Malformed TOML in legacy devices file {devices_path:?}"))?;

    if legacy_devices.devices.is_empty() {
        warn!("Legacy devices config is empty; skipping migration");
        return Ok(());
    }

    // Convert to new presence format
    let mut presence = PresenceConfig::default();
    let mut party = crate::config::Party {
        name: "Unknown".to_string(),
        location: None,
        devices: Vec::new(),
    };

    for (mac, device_config) in &legacy_devices.devices {
        let device = Device {
            name: device_config.name.clone(),
            location: None,
            identifiers: vec![Identifier {
                name: format!("BLE MAC ({})", device_config.name),
                id_type: IdentifierType::BleMac,
                value: Identifier::normalize_value(mac.clone()),
            }],
        };
        party.devices.push(device);
    }

    presence.parties.push(party);

    // Write new presence.toml
    let presence_toml = toml::to_string_pretty(&presence)
        .context("Failed to serialize presence config to TOML")?;

    std::fs::write(&presence_path, presence_toml)
        .with_context(|| format!("Failed to write presence config to {presence_path:?}"))?;

    // Rename old file to .bak
    let backup_path = devices_path.with_extension("toml.bak");
    std::fs::rename(&devices_path, &backup_path)
        .with_context(|| format!("Failed to rename {devices_path:?} to {backup_path:?}"))?;

    info!(
        "Migrated {} devices from {devices_path:?} to {presence_path:?}",
        legacy_devices.devices.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn migrate_simple_devices() {
        let dir = tempfile::tempdir().expect("tempdir");
        let devices_path = dir.path().join("devices.toml");
        let presence_path = dir.path().join("presence.toml");

        let legacy_toml = r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Alice's Phone"

[devices."11:22:33:44:55:66"]
mac = "11:22:33:44:55:66"
name = "Bob's Laptop"
"#;

        std::fs::write(&devices_path, legacy_toml).expect("write legacy config");

        migrate_devices_to_presence(devices_path.clone(), presence_path.clone())
            .expect("migration should succeed");

        // Verify presence.toml was created
        assert!(presence_path.exists());
        let presence_contents = std::fs::read_to_string(&presence_path).expect("read presence");
        let presence: PresenceConfig = toml::from_str(&presence_contents).expect("parse presence");

        assert_eq!(presence.parties.len(), 1);
        assert_eq!(presence.parties[0].name, "Unknown");
        assert_eq!(presence.parties[0].devices.len(), 2);
        assert_eq!(presence.parties[0].devices[0].name, "Alice's Phone");
        assert_eq!(presence.parties[0].devices[1].name, "Bob's Laptop");

        // Verify old file was renamed to .bak
        assert!(!devices_path.exists());
        let backup_path = devices_path.with_extension("toml.bak");
        assert!(backup_path.exists());
    }

    #[test]
    fn migrate_empty_devices_skips() {
        let dir = tempfile::tempdir().expect("tempdir");
        let devices_path = dir.path().join("devices.toml");
        let presence_path = dir.path().join("presence.toml");

        std::fs::write(&devices_path, "").expect("write empty config");

        migrate_devices_to_presence(devices_path.clone(), presence_path.clone())
            .expect("migration should succeed");

        // Presence file should not be created for empty devices
        assert!(!presence_path.exists());
        // Original file should remain unchanged
        assert!(devices_path.exists());
    }

    #[test]
    fn identifier_normalization_in_migration() {
        let dir = tempfile::tempdir().expect("tempdir");
        let devices_path = dir.path().join("devices.toml");
        let presence_path = dir.path().join("presence.toml");

        let legacy_toml = r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "  AA:BB:CC:DD:EE:FF  "
name = "Test Device"
"#;

        std::fs::write(&devices_path, legacy_toml).expect("write legacy config");

        migrate_devices_to_presence(devices_path.clone(), presence_path.clone())
            .expect("migration should succeed");

        let presence_contents = std::fs::read_to_string(&presence_path).expect("read presence");
        let presence: PresenceConfig = toml::from_str(&presence_contents).expect("parse presence");

        // MAC should be normalized to lowercase and trimmed
        assert_eq!(presence.parties[0].devices[0].identifiers[0].value, "aa:bb:cc:dd:ee:ff");
    }
}
