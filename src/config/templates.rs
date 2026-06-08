/// Default configuration templates for automatic initialization.
///
/// These templates are used when config files don't exist on first run.
/// All settings are commented out with default values and explanations.

/// Default template for config.toml with all settings commented out.
pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"# proximityd application configuration
# Place this file at:
#   - $PROXIMITYD_CONFIG_DIR/config.toml  (if env var is set)
#   - $XDG_CONFIG_HOME/com.myorg.proximityd/config.toml  (default on Linux/macOS)
#   - %APPDATA%\com\myorg\proximityd\config\config.toml  (default on Windows)
#
# All fields are optional; sensible defaults are shown below in comments.
# Uncomment and modify settings as needed.

[general]
# Log level: "trace", "debug", "info", "warn", or "error"
# Default: "info"
# log_level = "info"

# Signal log retention in days (1-90)
# Default: 30
# max_log_age_days = 30

# Enable SIGHUP config reload (requires daemon restart to take effect)
# Default: false
# config_reload = false

[privacy]
# If true, disables ARP/ping/mDNS scanners; BLE only
# Default: false
# privacy_mode = false

# Identifiers to ignore entirely (e.g., guest devices, unknown sensors)
# Example: anonymous = ["AA:BB:CC:DD:EE:FF", "192.168.1.100"]
# Default: [] (empty list)
# anonymous = []

[scanner.ble]
# Bluetooth Low Energy scanner
# Default: enabled = true, scan_interval_sec = 30
# enabled = true
# scan_interval_sec = 30

[scanner.wifi_arp]
# WiFi ARP table scanner (reads local ARP table or queries router via SNMP)
# Default: enabled = false, scan_interval_sec = 60
# enabled = false
# scan_interval_sec = 60
# router_ip = "192.168.1.1"  # Optional: router IP for SNMP queries
# snmp_community = "public"  # SNMP community string (default: "public")

[scanner.ping_sweep]
# ICMP ping sweep scanner (requires fping or raw ICMP sockets)
# Default: enabled = false, scan_interval_sec = 300
# enabled = false
# scan_interval_sec = 300
# subnet = "192.168.1.0/24"  # Subnet to scan (e.g., "192.168.1.0/24")

[scanner.mdns]
# mDNS/Bonjour hostname discovery scanner (requires avahi-browse on Linux or dns-sd on macOS)
# Default: enabled = false, scan_interval_sec = 120
# enabled = false
# scan_interval_sec = 120

[detection]
# Debounce before party enter notification (seconds)
# Default: 30
# enter_debounce_sec = 30

# Debounce before party exit notification (seconds)
# Default: 60
# exit_debounce_sec = 60

[discovery]
# Whether to use auto-discovery suggestions at runtime
# Default: false
# use_suggestions = false

# Confidence threshold for auto-promoting suggestions (0.0-1.0)
# Default: 0.95
# auto_promote_threshold = 0.95

# Notifier configuration (add multiple [[notifiers]] sections for multiple targets)
# kind: "discord", "slack", "webhook", or "mqtt"
# Example for Discord webhook:
# [[notifiers]]
# kind = "discord"
# webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"

# Example for Slack webhook:
# [[notifiers]]
# kind = "slack"
# webhook_url = "https://hooks.slack.com/services/YOUR/WEBHOOK/URL"

# Example for generic webhook:
# [[notifiers]]
# kind = "webhook"
# url = "https://your-server.com/api/presence"
# method = "POST"  # Default: "POST"
# payload_template = '{"party": "{{party}}", "event": "{{event}}"}'  # Optional custom template

# Example for MQTT:
# [[notifiers]]
# kind = "mqtt"
# broker = "localhost"  # Default: "localhost"
# port = 1883  # Default: 1883
# topic = "proximityd/presence"  # Default: "proximityd/presence"
"#;

/// Default template for presence.toml with example party structure.
pub const DEFAULT_PRESENCE_TEMPLATE: &str = r#"# proximityd presence configuration
# This file maps identifiers (MAC addresses, IPs, hostnames) to parties (people/entities)
# Place this file at:
#   - $PROXIMITYD_CONFIG_DIR/presence.toml  (if env var is set)
#   - $XDG_CONFIG_HOME/com.myorg.proximityd/presence.toml  (default on Linux/macOS)
#   - %APPDATA%\com\myorg\proximityd\config\presence.toml  (default on Windows)
#
# Structure: Party → Device → Identifier
# - Party: A person or entity (e.g., "Alice", "Bob", "Office")
# - Device: A device owned by the party (e.g., "Alice's iPhone", "Bob's Laptop")
# - Identifier: A unique identifier for the device (MAC, IP, hostname, etc.)

# Example party with multiple devices and identifiers
[[parties]]
name = "Alice"

# Optional: Location for this party (can be overridden per-device)
# location = { building = "Home", floor = 1, room = "Living Room", zone = "Main" }

  [[parties.devices]]
  name = "Alice's iPhone"

  # Optional: Device-specific location override
  # location = { building = "Home", floor = 2, room = "Bedroom" }

    [[parties.devices.identifiers]]
    name = "BLE MAC (main)"
    type = "ble_mac"
    value = "aa:bb:cc:dd:ee:ff"

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "11:22:33:44:55:66"

    [[parties.devices.identifiers]]
    name = "Hostname"
    type = "hostname"
    value = "alice-iphone"

  [[parties.devices]]
  name = "Alice's Watch"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "cc:dd:ee:ff:00:11"

# Example party with single device and multiple identifier types
[[parties]]
name = "Bob"

  [[parties.devices]]
  name = "Bob's Laptop"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "dd:ee:ff:00:11:22"

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "ee:ff:00:11:22:33"

    [[parties.devices.identifiers]]
    name = "IPv4"
    type = "ip_v4"
    value = "192.168.1.10"

# Identifier types:
# - ble_mac: Bluetooth MAC address (e.g., "aa:bb:cc:dd:ee:ff")
# - wifi_mac: WiFi MAC address (e.g., "aa:bb:cc:dd:ee:ff")
# - ip_v4: IPv4 address (e.g., "192.168.1.10")
# - ip_v6: IPv6 address (e.g., "2001:db8::1")
# - hostname: Hostname (e.g., "alice-iphone")
# - card_id: RFID card ID
# - door_sensor: Door sensor ID

# Notes:
# - Identifier values are automatically normalized (lowercase + trimmed) on load
# - Multiple identifiers per device are supported (e.g., dual SIM, multiple WiFi adapters)
# - Location hierarchy: building → floor → room → zone (all optional)
# - Device-level location overrides party-level location
"#;
