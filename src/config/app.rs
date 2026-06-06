use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// General application settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    /// Log level (e.g., "trace", "debug", "info", "warn", "error").
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// Signal log retention in days (1-90).
    #[serde(default = "default_max_log_age_days")]
    pub max_log_age_days: u32,
    /// Enable SIGHUP config reload.
    #[serde(default = "default_config_reload")]
    pub config_reload: bool,
}

/// Privacy settings.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PrivacyConfig {
    /// If true, disables ARP/ping/mDNS; BLE only.
    #[serde(default)]
    pub privacy_mode: bool,
    /// Identifiers to ignore entirely.
    #[serde(default)]
    pub anonymous: Vec<String>,
}

/// Scanner configuration for a specific scanner type.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScannerConfig {
    /// Whether this scanner is enabled.
    #[serde(default = "default_scanner_enabled")]
    pub enabled: bool,
    /// Scan interval in seconds.
    #[serde(default = "default_scan_interval")]
    pub scan_interval_sec: u64,
    /// Router IP address for SNMP queries (WiFi ARP scanner only).
    #[serde(default)]
    pub router_ip: Option<String>,
    /// SNMP community string (WiFi ARP scanner only).
    #[serde(default = "default_snmp_community")]
    pub snmp_community: String,
    /// Subnet to scan for ping sweep (e.g., "192.168.1.0/24").
    #[serde(default)]
    pub subnet: Option<String>,
}

/// Detection engine settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetectionConfig {
    /// Debounce before party enter notification (seconds).
    #[serde(default = "default_enter_debounce")]
    pub enter_debounce_sec: u64,
    /// Debounce before party exit notification (seconds).
    #[serde(default = "default_exit_debounce")]
    pub exit_debounce_sec: u64,
}

/// Discovery engine settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscoveryConfig {
    /// Whether to use auto-discovery suggestions at runtime.
    #[serde(default)]
    pub use_suggestions: bool,
    /// Confidence threshold for auto-promoting suggestions (0.0-1.0).
    #[serde(default = "default_auto_promote_threshold")]
    pub auto_promote_threshold: f64,
}

/// Configuration for a single notifier target.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NotifierConfig {
    /// Notifier type (e.g., "discord", "slack", "webhook", "mqtt").
    #[serde(rename = "type")]
    pub kind: String,
    /// Webhook URL (for discord, slack, webhook types).
    #[serde(default)]
    pub webhook_url: String,
    /// Generic URL (for webhook type).
    #[serde(default)]
    pub url: String,
    /// HTTP method (for webhook type).
    #[serde(default)]
    pub method: String,
    /// Payload template (for webhook type).
    #[serde(default)]
    pub payload_template: String,
    /// MQTT broker address.
    #[serde(default)]
    pub broker: String,
    /// MQTT port.
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    /// MQTT topic.
    #[serde(default)]
    pub topic: String,
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
    /// General application settings.
    #[serde(default)]
    pub general: GeneralConfig,
    /// Privacy settings.
    #[serde(default)]
    pub privacy: PrivacyConfig,
    /// Scanner configurations keyed by scanner name.
    #[serde(default)]
    pub scanner: HashMap<String, ScannerConfig>,
    /// Detection engine settings.
    #[serde(default)]
    pub detection: DetectionConfig,
    /// Discovery engine settings.
    #[serde(default)]
    pub discovery: DiscoveryConfig,
    /// List of configured notifiers.
    #[serde(default)]
    pub notifiers: Vec<NotifierConfig>,

    // ========== BACKWARD COMPATIBILITY ==========
    // These fields are deprecated and will be removed after detection engine refactoring.
    // They exist to keep the existing code working during the migration.

    /// Whether to track devices not present in the device mapping.
    #[serde(default = "default_track_unknown")]
    #[deprecated(note = "Use privacy.anonymous instead")]
    #[allow(deprecated)]
    pub track_unknown: bool,

    /// RSSI threshold in dBm for considering a device "entered".
    #[serde(default = "default_enter_rssi")]
    #[deprecated(note = "Will be replaced by scanner-specific thresholds")]
    #[allow(deprecated)]
    pub enter_rssi_threshold_dbm: i16,

    /// Duration in seconds a device must be seen before triggering enter.
    #[serde(default = "default_enter_duration")]
    #[deprecated(note = "Use detection.enter_debounce_sec instead")]
    #[allow(deprecated)]
    pub enter_duration_seconds: u64,

    /// Timeout in seconds before a device is considered "exited".
    #[serde(default = "default_exit_timeout")]
    #[deprecated(note = "Use detection.exit_debounce_sec instead")]
    #[allow(deprecated)]
    pub exit_timeout_seconds: u64,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_log_age_days() -> u32 {
    7
}

fn default_config_reload() -> bool {
    true
}

fn default_scanner_enabled() -> bool {
    true
}

fn default_scan_interval() -> u64 {
    30
}

fn default_snmp_community() -> String {
    "public".to_string()
}

fn default_enter_debounce() -> u64 {
    30
}

fn default_exit_debounce() -> u64 {
    120
}

fn default_auto_promote_threshold() -> f64 {
    0.95
}

fn default_mqtt_port() -> u16 {
    1883
}

fn default_track_unknown() -> bool {
    false
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

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            max_log_age_days: default_max_log_age_days(),
            config_reload: default_config_reload(),
        }
    }
}


impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            enabled: default_scanner_enabled(),
            scan_interval_sec: default_scan_interval(),
            router_ip: None,
            snmp_community: default_snmp_community(),
            subnet: None,
        }
    }
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            enter_debounce_sec: default_enter_debounce(),
            exit_debounce_sec: default_exit_debounce(),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            use_suggestions: false,
            auto_promote_threshold: default_auto_promote_threshold(),
        }
    }
}

#[allow(deprecated)]
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            privacy: PrivacyConfig::default(),
            scanner: HashMap::new(),
            detection: DetectionConfig::default(),
            discovery: DiscoveryConfig::default(),
            notifiers: Vec::new(),
            track_unknown: default_track_unknown(),
            enter_rssi_threshold_dbm: default_enter_rssi(),
            enter_duration_seconds: default_enter_duration(),
            exit_timeout_seconds: default_exit_timeout(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_toml_with_all_sections() {
        let toml = r#"
[general]
log_level = "debug"
max_log_age_days = 14
config_reload = false

[privacy]
privacy_mode = true
anonymous = ["de:ad:be:ef:00:01", "192.168.1.200"]

[scanner.ble]
enabled = true
scan_interval_sec = 10

[scanner.wifi_arp]
enabled = true
scan_interval_sec = 30
router_ip = "192.168.1.1"
snmp_community = "public"

[detection]
enter_debounce_sec = 15
exit_debounce_sec = 90

[discovery]
use_suggestions = true
auto_promote_threshold = 0.98

[[notifiers]]
type = "discord"
webhook_url = "https://discord.com/api/webhooks/123"
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.general.log_level, "debug");
        assert_eq!(config.general.max_log_age_days, 14);
        assert_eq!(config.general.config_reload, false);
        assert_eq!(config.privacy.privacy_mode, true);
        assert_eq!(config.privacy.anonymous.len(), 2);
        assert_eq!(config.scanner.get("ble").unwrap().enabled, true);
        assert_eq!(config.scanner.get("ble").unwrap().scan_interval_sec, 10);
        assert_eq!(config.detection.enter_debounce_sec, 15);
        assert_eq!(config.detection.exit_debounce_sec, 90);
        assert_eq!(config.discovery.use_suggestions, true);
        assert_eq!(config.discovery.auto_promote_threshold, 0.98);
        assert_eq!(config.notifiers.len(), 1);
        assert_eq!(config.notifiers[0].kind, "discord");
    }

    #[test]
    fn parse_toml_with_missing_sections_uses_defaults() {
        let toml = r#"
[general]
log_level = "warn"
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.general.log_level, "warn");
        assert_eq!(config.general.max_log_age_days, 7); // default
        assert_eq!(config.privacy.privacy_mode, false); // default
        assert!(config.privacy.anonymous.is_empty());
        assert!(config.scanner.is_empty());
        assert_eq!(config.detection.enter_debounce_sec, 30); // default
        assert_eq!(config.discovery.use_suggestions, false); // default
        assert!(config.notifiers.is_empty());
    }

    #[test]
    fn parse_empty_toml_uses_all_defaults() {
        let toml = "";
        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.general.log_level, "info");
        assert_eq!(config.general.max_log_age_days, 7);
        assert_eq!(config.general.config_reload, true);
        assert_eq!(config.privacy.privacy_mode, false);
        assert!(config.scanner.is_empty());
        assert_eq!(config.detection.enter_debounce_sec, 30);
        assert_eq!(config.detection.exit_debounce_sec, 120);
        assert_eq!(config.discovery.auto_promote_threshold, 0.95);
        assert!(config.notifiers.is_empty());
    }

    #[test]
    fn malformed_toml_errors() {
        let toml = r#"
[general]
log_level = 123
"#;

        let result: Result<AppConfig, _> = toml::from_str(toml);
        assert!(result.is_err(), "malformed TOML should fail to parse");
    }

    #[test]
    fn notifier_with_mqtt_fields() {
        let toml = r#"
[[notifiers]]
type = "mqtt"
broker = "192.168.1.10"
port = 1883
topic = "proximityd/presence"
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.notifiers.len(), 1);
        assert_eq!(config.notifiers[0].kind, "mqtt");
        assert_eq!(config.notifiers[0].broker, "192.168.1.10");
        assert_eq!(config.notifiers[0].port, 1883);
        assert_eq!(config.notifiers[0].topic, "proximityd/presence");
    }

    #[test]
    fn notifier_with_webhook_fields() {
        let toml = r#"
[[notifiers]]
type = "webhook"
url = "http://192.168.1.50:8123/api/webhook/presence"
method = "POST"
payload_template = '{"party":"{{party}}","event":"{{event}}"}'
"#;

        let config: AppConfig = toml::from_str(toml).expect("valid TOML");
        assert_eq!(config.notifiers.len(), 1);
        assert_eq!(config.notifiers[0].kind, "webhook");
        assert_eq!(config.notifiers[0].url, "http://192.168.1.50:8123/api/webhook/presence");
        assert_eq!(config.notifiers[0].method, "POST");
    }
}
