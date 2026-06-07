# proximityd — Generic Presence Detection Service

A multi-signal presence detection service that discovers nearby devices via Bluetooth, WiFi, and other sensors, maps them to labeled identities, and sends configurable enter/exit notifications via Discord, Slack, webhooks, MQTT, and other pluggable channels.

## Features

- **Multi-Signal Detection** — BLE (btleplug), WiFi ARP table scanning, ping sweep, and mDNS hostname discovery
- **Hierarchical Identity Model** — Party → Device → Identifier structure for flexible device mapping
- **Signal Audit Log** — SQLite-based logging of all scanner detections with location metadata
- **Discovery Engine** — Offline correlation analysis using Jaccard similarity to suggest device groupings
- **Enter/Exit Detection** — Configurable debounce thresholds prevent false notifications
- **Multiple Notifiers** — Discord, Slack, webhooks, MQTT with rich payload support
- **Location Awareness** — Hierarchical location model (building/floor/room/zone) with GPS/IP geolocation
- **Docker Ready** — Multi-arch (`amd64`/`arm64`) container with minimal privileges and health checks
- **Structured Logging** — JSON or human-readable logs via `tracing` with configurable verbosity

## Quick Start

Get your first notification in under 10 minutes:

```bash
# 1. Clone the repository
git clone https://github.com/levonk/proximityd.git
cd proximityd

# 2. Copy example configs
cp config.example.toml ~/.config/proximityd/config.toml
cp presence.example.toml ~/.config/proximityd/presence.toml

# 3. Edit device mapping
# ~/.config/proximityd/presence.toml
[[parties]]
name = "Alice"
location = { building = "Home", floor = 1, room = "Living Room" }

  [[parties.devices]]
  name = "Alice's iPhone"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "AA:BB:CC:DD:EE:FF"

# 4. Set your Discord webhook URL
export PROXIMITYD_DISCORD_WEBHOOK="https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
# Or edit the webhook URL directly in ~/.config/proximityd/config.toml

# 5. Run the daemon
cargo run -- --daemon
```

### Docker Quick Start

```bash
# Build and run with Docker Compose
cp config.example.toml ./config.toml
cp presence.example.toml ./presence.toml
# Edit config.toml and presence.toml with your settings

docker compose up --build
```

## Configuration

### `config.toml`

Place at `~/.config/proximityd/config.toml` (Linux/macOS) or set `PROXIMITYD_CONFIG_DIR`.

```toml
[general]
log_level = "INFO"
log_format = "pretty"

[privacy]
anonymous = false

[scanner.ble]
enabled = true
scan_interval_sec = 30

[scanner.wifi_arp]
enabled = false
scan_interval_sec = 60
router_ip = "192.168.1.1"
snmp_community = "public"

[scanner.ping_sweep]
enabled = false
scan_interval_sec = 120
subnet = "192.168.1.0/24"

[scanner.mdns]
enabled = false
scan_interval_sec = 60

[detection]
enter_debounce_sec = 5
exit_debounce_sec = 60

[discovery]
use_suggestions = false
auto_promote_threshold = 0.95

[[notifiers]]
kind = "discord"
webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
include_timestamp = true
include_mac = true
```

### `presence.toml`

The hierarchical identity model with parties, devices, and identifiers:

```toml
[[parties]]
name = "Alice"
location = { building = "Home", floor = 1, room = "Living Room", zone = "Family" }

  [[parties.devices]]
  name = "Alice's iPhone"
  location = { building = "Home", floor = 2, room = "Bedroom" }

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "AA:BB:CC:DD:EE:FF"

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "11:22:33:44:55:66"
```

### Supported Identifier Types

- `ble_mac` — Bluetooth MAC address
- `wifi_mac` — WiFi MAC address
- `ip_v4` — IPv4 address
- `ip_v6` — IPv6 address
- `hostname` — mDNS hostname
- `card_id` — RFID card ID
- `door_sensor` — Door sensor ID

### Notifiers

#### Discord

```toml
[[notifiers]]
kind = "discord"
webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
include_timestamp = true
include_mac = true
```

#### Slack

```toml
[[notifiers]]
kind = "slack"
webhook_url = "https://hooks.slack.com/services/YOUR/WEBHOOK/URL"
include_timestamp = true
include_mac = true
```

#### Webhook

```toml
[[notifiers]]
kind = "webhook"
url = "https://your-webhook-endpoint.com/events"
method = "POST"
payload_template = '{"event": "{{event}}", "party": "{{party}}", "location": "{{location}}"}'
```

#### MQTT

```toml
[[notifiers]]
kind = "mqtt"
broker = "localhost"
port = 1883
topic = "proximityd/presence"
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `PROXIMITYD_CONFIG_DIR` | Directory containing `config.toml` and `presence.toml` |
| `PROXIMITYD_CONFIG` | Override config file path |
| `PROXIMITYD_DEVICES` | Override presence file path (legacy) |
| `PROXIMITYD_DISCORD_WEBHOOK` | Discord webhook URL (overrides config file) |
| `PROXIMITYD_LOG_LEVEL` | Override log level (`DEBUG`, `TRACE`, etc.) |
| `PROXIMITYD_LOG_FORMAT` | Log format: `json` or `pretty` |
| `NO_COLOR` | Disable colored output |

## CLI Usage

```bash
# Run daemon mode
proximityd --daemon

# Run with verbose logging
proximityd --daemon -vv

# Run health check (returns 0 if healthy)
proximityd --health-check

# Show current presence status
proximityd status
proximityd status --json

# Export signal log data
proximityd export --format jsonl --since 2024-01-01
proximityd export --format csv --output signals.csv

# Discover identifier correlations from signal log
proximityd discover --hours 24 --min-confidence 0.5
proximityd discover --hours 24 --output suggestions.toml

# Override config path
proximityd --daemon --config /path/to/config.toml
```

## Docker

### Build

```bash
docker build -t proximityd:latest .
```

### Run

```bash
docker run -d \
  --name proximityd \
  --network host \
  --cap-drop ALL \
  --cap-add NET_ADMIN \
  --cap-add NET_RAW \
  -v /var/run/dbus:/var/run/dbus:ro \
  -v ~/.config/proximityd:/home/proximityd/.config/proximityd:ro \
  -e PROXIMITYD_LOG_LEVEL=info \
  proximityd:latest
```

### Compose

See `docker-compose.yml` for a complete example with security options pre-configured.

## Troubleshooting

### Bluetooth adapter not found

- Ensure your host has a Bluetooth 4.0+ adapter and BlueZ 5.x is installed.
- On Docker, use `--network host` and mount `/var/run/dbus`.
- Some kernels require `--cap-add SYS_ADMIN` in addition to `NET_ADMIN`.

### Discord not delivering messages

- Verify your webhook URL is valid and the channel allows incoming webhooks.
- Check logs with `PROXIMITYD_LOG_LEVEL=debug` for HTTP error responses.
- Ensure the webhook URL is not expired (Discord webhooks can be revoked).

### Permission denied

- The daemon requires access to the D-Bus system bus for BlueZ.
- Ensure your user has permission to access Bluetooth: `sudo usermod -a -G bluetooth $USER`
- On Docker, the container drops to a non-root user (`proximityd`) automatically.

## Success Metrics

- **Notification latency:** Under 60 seconds from real-world enter/exit.
- **Zero false events:** Under normal home conditions over a 7-day period.
- **Uptime:** 30 days continuous without memory leaks or crashes.
- **Setup time:** Junior developer can clone, configure, and run within 10 minutes.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Scanners  │────▶│  Detection  │────▶│  Notifiers  │
│             │     │   Engine    │     │             │
│ • BLE       │     └─────────────┘     │ • Discord   │
│ • WiFi ARP  │            │            │ • Slack     │
│ • Ping      │            ▼            │ • Webhook   │
│ • mDNS      │     ┌─────────────┐     │ • MQTT      │
└─────────────┘     │Signal Logger │     └─────────────┘
        │            └─────────────┘
        ▼                   │
┌─────────────┐            ▼
│presence.toml│     ┌─────────────┐
│             │     │Discovery    │
│ Parties     │     │Engine       │
│ Devices     │     └─────────────┘
│ Identifiers │
└─────────────┘
```

## Discovery Engine

The discovery engine analyzes signal log data to suggest device groupings based on co-occurrence patterns:

```bash
# Run discovery analysis on the last 24 hours of signal log
proximityd discover --hours 24 --min-confidence 0.5

# Save suggestions to a file
proximityd discover --hours 24 --output suggestions.toml

# Use a higher confidence threshold for stricter matches
proximityd discover --hours 48 --min-confidence 0.8
```

The output is a TOML file with suggested party/device groupings and confidence scores. You can review and manually apply these suggestions to your `presence.toml` configuration.

## Signal Log Export

Export signal log data for analysis or integration with other tools:

```bash
# Export as JSONL (JSON Lines) format
proximityd export --format jsonl --since 2024-01-01

# Export as CSV
proximityd export --format csv --output signals.csv

# Export all data (no date filter)
proximityd export --format jsonl --output all-signals.jsonl
```

The signal log includes:
- Timestamp
- Scanner type (BLE, WiFi ARP, ping, mDNS)
- Identifier type and value
- RSSI (signal strength)
- Party and device names (if resolved)
- Location information
- GPS coordinates (if available)
- Public IP address (if available)

## Troubleshooting

### Bluetooth adapter not found

- Ensure your host has a Bluetooth 4.0+ adapter and BlueZ 5.x is installed.
- On Docker, use `--network host` and mount `/var/run/dbus`.
- Some kernels require `--cap-add SYS_ADMIN` in addition to `NET_ADMIN`.

### Discord not delivering messages

- Verify your webhook URL is valid and the channel allows incoming webhooks.
- Check logs with `PROXIMITYD_LOG_LEVEL=debug` for HTTP error responses.
- Ensure the webhook URL is not expired (Discord webhooks can be revoked).

### Permission denied

- The daemon requires access to the D-Bus system bus for BlueZ.
- Ensure your user has permission to access Bluetooth: `sudo usermod -a -G bluetooth $USER`
- On Docker, the container drops to a non-root user (`proximityd`) automatically.

## License

Dual-licensed under MIT or Apache-2.0.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
