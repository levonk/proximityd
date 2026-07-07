use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::cli::format_file_reference_simple;
use super::{AppConfig, DevicesConfig, Identifier, PresenceConfig};
use super::templates::{DEFAULT_CONFIG_TEMPLATE, DEFAULT_PRESENCE_TEMPLATE};

const APP_QUALIFIER: &str = "com";
const APP_ORG: &str = "myorg";
const APP_NAME: &str = "proximityd";
const CONFIG_FILE: &str = "config.toml";
const DEVICES_FILE: &str = "devices.toml";
const PRESENCE_FILE: &str = "presence.toml";

/// Resolve the config directory using XDG conventions with optional override.
pub fn resolve_config_dir() -> PathBuf {
    if let Ok(dir) = env::var("PROXIMITYD_CONFIG_DIR") {
        PathBuf::from(dir)
    } else {
        ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
            .map(|pd| pd.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    }
}

/// Initialize config directory and default templates if they don't exist.
///
/// This function creates the config directory if missing and writes default
/// config templates for config.toml and presence.toml if they don't exist.
/// Existing files are never overwritten.
pub fn initialize_config(force: bool) -> Result<()> {
    let config_dir = resolve_config_dir();

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory at {}", format_file_reference_simple(&config_dir, None, None)))?;
        info!("Created config directory at {}", format_file_reference_simple(&config_dir, None, None));
    } else {
        debug!("Config directory already exists at {}", format_file_reference_simple(&config_dir, None, None));
    }

    let config_file = config_dir.join(CONFIG_FILE);
    let presence_file = config_dir.join(PRESENCE_FILE);

    // Write default config.toml if it doesn't exist (or if force is true)
    if !config_file.exists() || force {
        if force && config_file.exists() {
            warn!("Force re-initialization: overwriting {}", format_file_reference_simple(&config_file, None, None));
        }
        fs::write(&config_file, DEFAULT_CONFIG_TEMPLATE)
            .with_context(|| format!("Failed to write config template to {}", format_file_reference_simple(&config_file, None, None)))?;
        info!("Created default config at {}", format_file_reference_simple(&config_file, None, None));
    } else {
        debug!("Config file already exists at {}, skipping", format_file_reference_simple(&config_file, None, None));
    }

    // Write default presence.toml if it doesn't exist (or if force is true)
    if !presence_file.exists() || force {
        if force && presence_file.exists() {
            warn!("Force re-initialization: overwriting {}", format_file_reference_simple(&presence_file, None, None));
        }
        fs::write(&presence_file, DEFAULT_PRESENCE_TEMPLATE)
            .with_context(|| format!("Failed to write presence template to {}", format_file_reference_simple(&presence_file, None, None)))?;
        info!("Created default presence config at {}", format_file_reference_simple(&presence_file, None, None));
    } else {
        debug!("Presence config file already exists at {}, skipping", format_file_reference_simple(&presence_file, None, None));
    }

    Ok(())
}

/// Load `AppConfig` from disk.
///
/// If `path` is `Some`, that file is loaded directly.
/// Otherwise, the function searches `PROXIMITYD_CONFIG_DIR/config.toml`
/// or falls back to the XDG config directory.
///
/// A missing file returns default config without writing to disk. Use
/// `initialize_config` or `proximityd install` to create config files.
pub fn load_config(path: Option<PathBuf>) -> Result<AppConfig> {
    let file_path = path.unwrap_or_else(|| resolve_config_dir().join(CONFIG_FILE));

    if !file_path.exists() {
        info!("Config not found at {}; using defaults", format_file_reference_simple(&file_path, None, None));
        return Ok(AppConfig::default());
    }

    let contents = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read config from {}", format_file_reference_simple(&file_path, None, None)))?;

    let config: AppConfig = toml::from_str(&contents)
        .with_context(|| format!("Malformed TOML in config file {}", format_file_reference_simple(&file_path, None, None)))?;

    info!("Loaded config from {}", format_file_reference_simple(&file_path, None, None));
    Ok(config)
}

/// Load `DevicesConfig` from disk.
///
/// If `path` is `Some`, that file is loaded directly.
/// Otherwise, the function searches `PROXIMITYD_CONFIG_DIR/devices.toml`
/// or falls back to the XDG config directory.
///
/// A missing file is allowed and returns an empty `DevicesConfig`.
pub fn load_devices(path: Option<PathBuf>) -> Result<DevicesConfig> {
    let file_path = path.unwrap_or_else(|| resolve_config_dir().join(DEVICES_FILE));

    if !file_path.exists() {
        warn!("Devices config not found at {}; using empty mapping", format_file_reference_simple(&file_path, None, None));
        return Ok(DevicesConfig::default());
    }

    let contents = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read devices config from {}", format_file_reference_simple(&file_path, None, None)))?;

    let devices: DevicesConfig = toml::from_str(&contents)
        .with_context(|| format!("Malformed TOML in devices file {}", format_file_reference_simple(&file_path, None, None)))?;

    info!("Loaded devices config from {}", format_file_reference_simple(&file_path, None, None));
    Ok(devices)
}

/// Load `PresenceConfig` from disk.
///
/// If `path` is `Some`, that file is loaded directly.
/// Otherwise, the function searches `PROXIMITYD_CONFIG_DIR/presence.toml`
/// or falls back to the XDG config directory.
///
/// A missing file returns default config without writing to disk. Use
/// `initialize_config` or `proximityd install` to create config files.
///
/// If a legacy `devices.toml` file is detected, returns an error with
/// migration instructions.
///
/// Identifier values are automatically normalized (lowercase + trim) on load.
pub fn load_presence(path: Option<PathBuf>) -> Result<PresenceConfig> {
    let file_path = path.unwrap_or_else(|| resolve_config_dir().join(PRESENCE_FILE));
    let devices_path = file_path.parent().unwrap().join(DEVICES_FILE);

    // Check for legacy devices.toml and error if found
    if devices_path.exists() {
        return Err(anyhow::anyhow!(
            "Legacy config format detected: {} is no longer supported. \
            Please rename devices.toml to presence.toml and update the format according to the documentation. \
            See https://github.com/levonk/proximityd for migration instructions.",
            format_file_reference_simple(&devices_path, None, None)
        ));
    }

    if !file_path.exists() {
        info!("Presence config not found at {}; using defaults", format_file_reference_simple(&file_path, None, None));
        return Ok(PresenceConfig::default());
    }

    let contents = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read presence config from {}", format_file_reference_simple(&file_path, None, None)))?;

    let mut presence: PresenceConfig = toml::from_str(&contents)
        .with_context(|| format!("Malformed TOML in presence file {}", format_file_reference_simple(&file_path, None, None)))?;

    // Normalize all identifier values
    for party in &mut presence.parties {
        for device in &mut party.devices {
            for identifier in &mut device.identifiers {
                identifier.value = Identifier::normalize_value(identifier.value.clone());
            }
        }
    }

    info!("Loaded presence config from {}", format_file_reference_simple(&file_path, None, None));
    Ok(presence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_config(dir: &tempfile::TempDir, name: &str, contents: &str) -> PathBuf {
        let path = dir.path().join(name);
        let mut file = std::fs::File::create(&path).expect("create temp file");
        file.write_all(contents.as_bytes())
            .expect("write temp file");
        path
    }

    #[test]
    fn load_config_reads_valid_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "config.toml",
            r#"
[general]
log_level = "debug"

[privacy]
privacy_mode = true
"#,
        );

        let config = load_config(Some(path)).expect("load config");
        assert_eq!(config.general.log_level, "debug");
        assert_eq!(config.privacy.privacy_mode, true);
    }

    #[test]
    fn load_config_missing_file_returns_defaults() {
        let missing = PathBuf::from("/nonexistent/config.toml");
        let config = load_config(Some(missing)).expect("missing file should return defaults");
        assert_eq!(config.general.log_level, "info"); // default
    }

    #[test]
    fn load_config_malformed_toml_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "config.toml",
            r#"
[general]
log_level = 123
"#,
        );

        let result = load_config(Some(path));
        assert!(result.is_err(), "malformed TOML should error");
    }

    #[test]
    #[allow(deprecated)]
    fn load_config_uses_deprecated_fields_for_compatibility() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "config.toml",
            r#"
enter_rssi_threshold_dbm = -65
enter_duration_seconds = 3
exit_timeout_seconds = 45
track_unknown = true
"#,
        );

        let config = load_config(Some(path)).expect("load config");
        assert_eq!(config.enter_rssi_threshold_dbm, -65);
        assert_eq!(config.enter_duration_seconds, 3);
        assert_eq!(config.exit_timeout_seconds, 45);
        assert_eq!(config.track_unknown, true);
    }

    #[test]
    fn load_devices_reads_valid_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "devices.toml",
            r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Test Device"
"#,
        );

        let devices = load_devices(Some(path)).expect("load devices");
        assert_eq!(devices.devices.len(), 1);
        let dev = devices.get("AA:BB:CC:DD:EE:FF").expect("device found");
        assert_eq!(dev.name, "Test Device");
    }

    #[test]
    fn load_devices_missing_file_returns_empty() {
        let missing = PathBuf::from("/nonexistent/devices.toml");
        let devices = load_devices(Some(missing)).expect("load devices");
        assert!(devices.is_empty());
    }

    #[test]
    fn load_devices_malformed_toml_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "devices.toml",
            r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = 123
"#,
        );

        let result = load_devices(Some(path));
        assert!(result.is_err(), "malformed TOML should error");
    }

    #[test]
    fn load_presence_reads_valid_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "presence.toml",
            r#"
[[parties]]
name = "Alice"

  [[parties.devices]]
  name = "Alice's iPhone"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "aa:bb:cc:dd:ee:ff"
"#,
        );

        let presence = load_presence(Some(path)).expect("load presence");
        assert_eq!(presence.parties.len(), 1);
        assert_eq!(presence.parties[0].name, "Alice");
        assert_eq!(presence.parties[0].devices.len(), 1);
        assert_eq!(presence.parties[0].devices[0].identifiers.len(), 1);
    }

    #[test]
    fn load_presence_missing_file_returns_empty() {
        let missing = PathBuf::from("/nonexistent/presence.toml");
        let presence = load_presence(Some(missing)).expect("load presence");
        assert!(presence.parties.is_empty());
    }

    #[test]
    fn load_presence_errors_on_legacy_devices_toml() {
        let dir = tempfile::tempdir().expect("tempdir");
        let _devices_path = write_temp_config(
            &dir,
            "devices.toml",
            r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Test Device"
"#,
        );
        let presence_path = dir.path().join("presence.toml");

        let result = load_presence(Some(presence_path));
        assert!(result.is_err(), "should error when devices.toml exists");
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Legacy config format detected"), "error should mention legacy format");
        assert!(error_msg.contains("devices.toml"), "error should mention devices.toml");
    }

    #[test]
    fn load_presence_normalizes_identifiers() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(
            &dir,
            "presence.toml",
            r#"
[[parties]]
name = "Alice"

  [[parties.devices]]
  name = "Alice's iPhone"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "  AA:BB:CC:DD:EE:FF  "
"#,
        );

        let presence = load_presence(Some(path)).expect("load presence");
        assert_eq!(presence.parties[0].devices[0].identifiers[0].value, "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn initialize_config_creates_directory_and_templates() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Set environment variable to use temp directory
        let config_dir = dir.path().to_path_buf();
        std::env::set_var("PROXIMITYD_CONFIG_DIR", &config_dir);

        let result = initialize_config(false);
        assert!(result.is_ok(), "initialization should succeed");

        let config_file = config_dir.join(CONFIG_FILE);
        let presence_file = config_dir.join(PRESENCE_FILE);

        assert!(config_file.exists(), "config.toml should be created");
        assert!(presence_file.exists(), "presence.toml should be created");

        // Verify templates contain expected content
        let config_contents = std::fs::read_to_string(&config_file).expect("read config");
        assert!(config_contents.contains("# proximityd application configuration"), "should contain config header");
        assert!(config_contents.contains("[general]"), "should contain general section");

        let presence_contents = std::fs::read_to_string(&presence_file).expect("read presence");
        assert!(presence_contents.contains("# proximityd presence configuration"), "should contain presence header");
        assert!(presence_contents.contains("[[parties]]"), "should contain parties section");

        // Clean up environment variable
        std::env::remove_var("PROXIMITYD_CONFIG_DIR");
    }

    #[test]
    fn load_config_missing_file_returns_defaults_without_writing() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Set environment variable to use temp directory
        let config_dir = dir.path().to_path_buf();
        std::env::set_var("PROXIMITYD_CONFIG_DIR", &config_dir);

        let config_file = config_dir.join(CONFIG_FILE);
        assert!(!config_file.exists(), "config should not exist initially");

        // Load should return defaults without writing to disk
        let config = load_config(None).expect("load config should return defaults");
        assert_eq!(config.general.log_level, "info"); // default

        // Verify config file was NOT created (safe probes must not write)
        assert!(!config_file.exists(), "config should not be created by load_config");

        // Clean up environment variable
        std::env::remove_var("PROXIMITYD_CONFIG_DIR");
    }

    #[test]
    fn load_presence_missing_file_returns_defaults_without_writing() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Set environment variable to use temp directory
        let config_dir = dir.path().to_path_buf();
        std::env::set_var("PROXIMITYD_CONFIG_DIR", &config_dir);

        let presence_file = config_dir.join(PRESENCE_FILE);
        assert!(!presence_file.exists(), "presence should not exist initially");

        // Load should return defaults without writing to disk
        let presence = load_presence(None).expect("load presence should return defaults");
        assert!(presence.parties.is_empty()); // default

        // Verify presence file was NOT created (safe probes must not write)
        assert!(!presence_file.exists(), "presence should not be created by load_presence");

        // Clean up environment variable
        std::env::remove_var("PROXIMITYD_CONFIG_DIR");
    }
}
