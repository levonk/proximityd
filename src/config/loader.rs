use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::env;
use std::path::PathBuf;
use tracing::{info, warn};

use super::{AppConfig, DevicesConfig};

const APP_QUALIFIER: &str = "com";
const APP_ORG: &str = "myorg";
const APP_NAME: &str = "btnotify";
const CONFIG_FILE: &str = "config.toml";
const DEVICES_FILE: &str = "devices.toml";

/// Resolve the config directory using XDG conventions with optional override.
fn resolve_config_dir() -> PathBuf {
    if let Ok(dir) = env::var("BTNOTIFY_CONFIG_DIR") {
        PathBuf::from(dir)
    } else {
        ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
            .map(|pd| pd.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    }
}

/// Load `AppConfig` from disk.
///
/// If `path` is `Some`, that file is loaded directly.
/// Otherwise, the function searches `BTNOTIFY_CONFIG_DIR/config.toml`
/// or falls back to the XDG config directory.
///
/// A missing file is allowed and returns the default `AppConfig`.
pub fn load_config(path: Option<PathBuf>) -> Result<AppConfig> {
    let file_path = path.unwrap_or_else(|| resolve_config_dir().join(CONFIG_FILE));

    if !file_path.exists() {
        warn!("Config not found at {file_path:?}; using defaults");
        return Ok(AppConfig::default());
    }

    let contents = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read config from {file_path:?}"))?;

    let config: AppConfig = toml::from_str(&contents)
        .with_context(|| format!("Malformed TOML in config file {file_path:?}"))?;

    info!("Loaded config from {file_path:?}");
    Ok(config)
}

/// Load `DevicesConfig` from disk.
///
/// If `path` is `Some`, that file is loaded directly.
/// Otherwise, the function searches `BTNOTIFY_CONFIG_DIR/devices.toml`
/// or falls back to the XDG config directory.
///
/// A missing file is allowed and returns an empty `DevicesConfig`.
pub fn load_devices(path: Option<PathBuf>) -> Result<DevicesConfig> {
    let file_path = path.unwrap_or_else(|| resolve_config_dir().join(DEVICES_FILE));

    if !file_path.exists() {
        warn!("Devices config not found at {file_path:?}; using empty mapping");
        return Ok(DevicesConfig::default());
    }

    let contents = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read devices config from {file_path:?}"))?;

    let devices: DevicesConfig = toml::from_str(&contents)
        .with_context(|| format!("Malformed TOML in devices file {file_path:?}"))?;

    info!("Loaded devices config from {file_path:?}");
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_config(dir: &tempfile::TempDir, name: &str, contents: &str) -> PathBuf {
        let path = dir.path().join(name);
        let mut file = std::fs::File::create(&path).expect("create temp file");
        file.write_all(contents.as_bytes()).expect("write temp file");
        path
    }

    #[test]
    fn load_config_reads_valid_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(&dir, "config.toml", r#"
scan_interval_seconds = 15
exit_timeout_seconds = 120
"#);

        let config = load_config(Some(path)).expect("load config");
        assert_eq!(config.scan_interval_seconds, 15);
        assert_eq!(config.exit_timeout_seconds, 120);
        assert_eq!(config.enter_rssi_threshold_dbm, -70); // default
    }

    #[test]
    fn load_config_missing_file_returns_defaults() {
        let missing = PathBuf::from("/nonexistent/config.toml");
        let config = load_config(Some(missing)).expect("missing file should return defaults");
        assert_eq!(config.scan_interval_seconds, 30); // default
        assert_eq!(config.enter_rssi_threshold_dbm, -70); // default
    }

    #[test]
    fn load_config_malformed_toml_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(&dir, "config.toml", r#"
scan_interval_seconds = "not a number"
"#);

        let result = load_config(Some(path));
        assert!(result.is_err(), "malformed TOML should error");
    }

    #[test]
    fn load_devices_reads_valid_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_temp_config(&dir, "devices.toml", r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Test Device"
"#);

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
        let path = write_temp_config(&dir, "devices.toml", r#"
[devices."AA:BB:CC:DD:EE:FF"]
mac = 123
"#);

        let result = load_devices(Some(path));
        assert!(result.is_err(), "malformed TOML should error");
    }
}
