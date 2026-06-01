use serde::{Deserialize, Serialize};

/// Configuration for a single notifier target.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifierConfig {
    /// Notifier type (e.g., "discord", "webhook").
    pub kind: String,
    /// Target URL or identifier for the notifier.
    #[serde(default)]
    pub target: String,
    /// Optional bot token for Discord bot API access.
    #[serde(default)]
    pub token: Option<String>,
    /// Optional channel ID when using a bot token.
    #[serde(default)]
    pub channel_id: Option<String>,
    /// Include timestamp in notification messages.
    #[serde(default)]
    pub include_timestamp: bool,
    /// Include MAC address in notification messages.
    #[serde(default)]
    pub include_mac: bool,
}

/// Application-level behavior configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    /// Scan interval in seconds.
    #[serde(default = "default_scan_interval")]
    pub scan_interval_seconds: u64,

    /// RSSI threshold in dBm for considering a device "entered".
    #[serde(default = "default_enter_rssi")]
    pub enter_rssi_threshold_dbm: i16,

    /// Duration in seconds a device must be seen before triggering enter.
    #[serde(default = "default_enter_duration")]
    pub enter_duration_seconds: u64,

    /// Timeout in seconds before a device is considered "exited".
    #[serde(default = "default_exit_timeout")]
    pub exit_timeout_seconds: u64,

    /// List of configured notifiers.
    #[serde(default)]
    pub notifiers: Vec<NotifierConfig>,

    /// Whether to track devices not present in the device mapping.
    #[serde(default = "default_track_unknown")]
    pub track_unknown: bool,
}

fn default_scan_interval() -> u64 {
    30
}

fn default_enter_rssi() -> i16 {
    -70
}

fn default_enter_duration() -> u64 {
    5
}

fn default_exit_timeout() -> u64 {
    60
}

fn default_track_unknown() -> bool {
    false
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            scan_interval_seconds: default_scan_interval(),
            enter_rssi_threshold_dbm: default_enter_rssi(),
            enter_duration_seconds: default_enter_duration(),
            exit_timeout_seconds: default_exit_timeout(),
            notifiers: Vec::new(),
            track_unknown: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_toml_with_all_fields() {
        let toml = r#"
scan_interval_seconds = 10
enter_rssi_threshold_dbm = -65
enter_duration_seconds = 3
exit_timeout_seconds = 45

[[notifiers]]
kind = "discord"
target = "https://discord.com/api/webhooks/123"
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.scan_interval_seconds, 10);
        assert_eq!(config.enter_rssi_threshold_dbm, -65);
        assert_eq!(config.enter_duration_seconds, 3);
        assert_eq!(config.exit_timeout_seconds, 45);
        assert_eq!(config.notifiers.len(), 1);
        assert_eq!(config.notifiers[0].kind, "discord");
        assert_eq!(config.notifiers[0].target, "https://discord.com/api/webhooks/123");
    }

    #[test]
    fn parse_toml_with_missing_fields_uses_defaults() {
        let toml = r#"
scan_interval_seconds = 15
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.scan_interval_seconds, 15);
        assert_eq!(config.enter_rssi_threshold_dbm, -70);
        assert_eq!(config.enter_duration_seconds, 5);
        assert_eq!(config.exit_timeout_seconds, 60);
        assert!(config.notifiers.is_empty());
    }

    #[test]
    fn parse_empty_toml_uses_all_defaults() {
        let toml = "";
        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.scan_interval_seconds, 30);
        assert_eq!(config.enter_rssi_threshold_dbm, -70);
        assert_eq!(config.enter_duration_seconds, 5);
        assert_eq!(config.exit_timeout_seconds, 60);
        assert!(config.notifiers.is_empty());
    }

    #[test]
    fn malformed_toml_errors() {
        let toml = r#"
scan_interval_seconds = "not a number"
"#;

        let result: Result<AppConfig, _> = toml::from_str(toml);
        assert!(result.is_err(), "malformed TOML should fail to parse");
    }
}
