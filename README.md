# proximityd — Generic Presence Detection Service

A containerized Bluetooth Low Energy (BLE) presence detection service that discovers nearby Bluetooth devices, maps them to labeled identities, and sends configurable enter/exit notifications via Discord (and other pluggable channels).

## Features

- **BLE Presence Detection** — Continuously scans for nearby Bluetooth devices and tracks their proximity.
- **Device Labeling** — Map MAC addresses to human-readable names via a simple TOML file.
- **Enter/Exit Detection** — Configurable debounce thresholds prevent false notifications.
- **Discord Notifications** — Send alerts to a Discord channel via webhook or bot token.
- **Pluggable Notifiers** — Trait-based architecture for adding Slack, webhooks, or OS-native notifications.
- **Docker Ready** — Multi-arch (`amd64`/`arm64`) container with minimal privileges and health checks.
- **Structured Logging** — JSON or human-readable logs via `tracing` with configurable verbosity.

## Quick Start

Get your first notification in under 10 minutes:

```bash
# 1. Clone the repository
git clone https://github.com/levonk/proximityd.git
cd proximityd

# 2. Copy example configs
cp config.example.toml ~/.config/proximityd/config.toml
cp devices.example.toml ~/.config/proximityd/devices.toml

# 3. Edit device mapping
# ~/.config/proximityd/devices.toml
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Your Phone"

# 4. Set your Discord webhook URL
export PROXIMITYD_DISCORD_WEBHOOK="https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
# Or edit the webhook URL directly in ~/.config/proximityd/config.toml

# 5. Run the daemon (Linux with BlueZ required)
cargo run -- --daemon
```

### Docker Quick Start

```bash
# Build and run with Docker Compose
cp config.example.toml ./config.toml
cp devices.example.toml ./devices.toml
# Edit config.toml and devices.toml with your settings

docker compose up --build
```

## Configuration

### `config.toml`

Place at `~/.config/proximityd/config.toml` (Linux/macOS) or set `PROXIMITYD_CONFIG_DIR`.

| Option | Default | Description |
|--------|---------|-------------|
| `scan_interval_seconds` | `30` | Seconds between BLE scan cycles |
| `enter_rssi_threshold_dbm` | `-70` | Minimum RSSI (signal strength) to count as present |
| `enter_duration_seconds` | `5` | Seconds a device must be seen before triggering "enter" |
| `exit_timeout_seconds` | `60` | Seconds since last detection before triggering "exit" |
| `track_unknown` | `false` | Whether to log unmapped MAC addresses |
| `log_level` | `"INFO"` | Log level: `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE` |

### Notifiers

```toml
[[notifiers]]
kind = "discord"
target = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
include_timestamp = true
```

### `devices.toml`

```toml
[devices."AA:BB:CC:DD:EE:FF"]
mac = "AA:BB:CC:DD:EE:FF"
name = "Alice's Phone"
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `PROXIMITYD_CONFIG_DIR` | Directory containing `config.toml` and `devices.toml` |
| `PROXIMITYD_DISCORD_WEBHOOK` | Discord webhook URL (overrides config file) |
| `PROXIMITYD_LOG_LEVEL` | Override log level (`DEBUG`, `TRACE`, etc.) |
| `NO_COLOR` | Disable colored output |

## CLI Usage

```bash
# Run daemon mode (Linux only)
proximityd --daemon

# Run with verbose logging
proximityd --daemon -vv

# Run health check (returns 0 if healthy)
proximityd --health-check

# Override config path
proximityd --daemon --config /path/to/config.toml --devices /path/to/devices.toml
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
│  BLE Scan   │────▶│  Detection  │────▶│  Notifier   │
│   (BlueZ)   │     │   Engine    │     │  (Discord)  │
└─────────────┘     └─────────────┘     └─────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ devices.toml│     │ config.toml │     │   Webhook   │
└─────────────┘     └─────────────┘     └─────────────┘
```

## License

Dual-licensed under MIT or Apache-2.0.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
