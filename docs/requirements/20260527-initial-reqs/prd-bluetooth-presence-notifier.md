---
# Product Requirements Document (PRD)

## Introduction / Overview

- **Feature name:** Bluetooth Presence Notifier (btnotify)
- **Summary:** A containerized Bluetooth Low Energy (BLE) presence detection service that discovers nearby Bluetooth devices, maps them to labeled identities, and sends configurable enter/exit notifications via Discord (and other pluggable channels). Users can label MAC addresses to identify people and receive alerts when they enter or leave the monitored area.
- **Context:**
  - This is a personal/homelab tool for tracking household member or guest presence via their carried Bluetooth devices (phones, watches, earbuds).
  - It runs primarily inside a Docker container for portability but requires host-level Bluetooth access.
  - Discord acts as the primary notification bus, abstracting OS-specific notification systems and allowing remote awareness without needing to be at the machine.

## Goals

- Detect Bluetooth devices entering and exiting a defined physical area within 30 seconds of the state change.
- Allow users to label discovered MAC addresses with human-readable names via a static configuration file.
- Send notifications to a configured Discord channel when a labeled device enters or exits.
- Support pluggable notification backends (Discord MVP, OS-native Linux/Windows/macOS, webhooks, and Slack for future expansion).
- Run reliably in a Docker container with minimal host privileges.
- Provide a simple TOML-based configuration that a junior developer can understand and modify.

## User Stories

- **As a** homeowner, **I want** to know when my family members arrive home or leave **so that** I can automate awareness without needing to check a dashboard.
- **As a** homelab operator, **I want** Discord notifications for BLE presence **so that** I can see who is home from any device, anywhere.
- **As a** privacy-conscious user, **I want** all device labeling and identity data stored locally in a config file **so that** no cloud service has access to my household's device mappings.
- **As a** developer, **I want** a pluggable notification system **so that** I can add Slack, webhook, or native OS notifications without rewriting core logic.

## Functional Requirements

- **FR-1: Bluetooth Scanning**
  - The app must continuously scan for discoverable BLE devices using the host system's Bluetooth adapter.
  - It must collect at minimum the MAC address and RSSI signal strength of each discovered device.

- **FR-2: Device Labeling**
  - The app must read a static device mapping from `~/.config/btnotify/devices.toml`.
  - Format: a TOML table of MAC-address-to-name mappings. Unknown MACs are ignored unless explicitly listed (configurable).
  - Example:
    ```toml
    [devices]
    "AA:BB:CC:DD:EE:FF" = "Alice"
    "11:22:33:44:55:66" = "Bob"
    ```

- **FR-3: Enter/Exit Detection**
  - The app must maintain a state table of which labeled devices are currently present.
  - A device is considered **entered** when it is consistently detected with sufficient signal strength and duration.
  - A device is considered **exited** when it has not been detected for a configurable timeout period.
  - De-bouncing must be configurable via `~/.config/btnotify/config.toml`:
    - `enter_rssi_threshold_dbm`: minimum RSSI to count as present (default: `-70`)
    - `enter_duration_seconds`: seconds of continuous detection before declaring entered (default: `30`)
    - `exit_timeout_seconds`: seconds since last detection before declaring exited (default: `180`)
    - `scan_interval_seconds`: seconds between scan cycles (default: `15`)

- **FR-4: Notifications**
  - On enter/exit state transitions, the app must send a notification via the active notifier(s).
  - The MVP notifier is **Discord** (via webhook or bot token).
  - Notification message format: `"{name} has entered the area"` or `"{name} has exited the area"`.
  - Timestamp and device MAC (optional, off by default) may be included for debugging.

- **FR-5: Pluggable Notifiers**
  - The notification system must be implemented behind a trait/interface so that new backends can be added.
  - MVP notifiers: `DiscordNotifier`.
  - Planned notifiers: `OsNativeNotifier` (Linux notify-rust, Windows winrt, macOS NSUserNotification), `WebhookNotifier`, `SlackNotifier`.
  - The active notifier(s) are selected in `config.toml` via a `notifiers` array.

- **FR-6: Discord Integration**
  - Support Discord incoming webhook URLs OR bot token + channel ID.
  - If configured as a bot, the app can register itself as a Discord app for potential slash-command extensions later (not MVP).
  - If a webhook URL is used, no bot registration is required.

- **FR-7: Configuration**
  - Two TOML config files:
    - `~/.config/btnotify/config.toml` — app behavior, scanning parameters, and notifier list.
    - `~/.config/btnotify/devices.toml` — MAC-to-name mapping table.
  - Environment variable overrides: `BTNOTIFY_CONFIG_DIR`, `BTNOTIFY_DISCORD_WEBHOOK`, `BTNOTIFY_LOG_LEVEL`.

## Non-Functional Requirements

- **NFR-1: Performance**
  - Scan cycle must complete within 5 seconds.
  - Notification delivery latency must be under 5 seconds from state transition detection.
  - Memory footprint should remain under 64 MB for the daemon.

- **NFR-2: Reliability**
  - If Bluetooth adapter becomes unavailable, the app must retry every 30 seconds and log the error.
  - If Discord delivery fails, retry up to 3 times with exponential backoff, then log and continue scanning.
  - App must not crash due to malformed TOML; print a clear error and exit with code `1` on startup.

- **NFR-3: Security**
  - Discord webhook URLs/bot tokens must be read from environment variables or secrets files, never hardcoded.
  - The Docker container must not run as root; use a dedicated `btnotify` user.
  - Bluetooth access must use minimal Linux capabilities (`NET_ADMIN`, `SYS_ADMIN` only if strictly required by BlueZ) rather than `--privileged`.
  - All MAC address data stays local; no cloud upload of scan results.

- **NFR-4: Portability**
  - Primary target: Linux (x86_64 and aarch64) with BlueZ.
  - Bluetooth code must be abstracted so that future Windows/macOS backends are possible.
  - Docker image must support both `amd64` and `arm64` architectures.

- **NFR-5: Observability**
  - Structured JSON logging via `tracing` or `slog`.
  - Log levels: `ERROR`, `WARN`, `INFO`, `DEBUG`.
  - A health check endpoint or signal for Docker `HEALTHCHECK`.

## Technical Considerations

- **Language & Runtime:** Rust (matches existing `Cargo.toml` in repo).
- **Bluetooth Stack:** `blurz` or `bluez-async` crate for D-Bus BlueZ integration on Linux.
- **Configuration:** `serde` + `toml` crates for config parsing.
- **Notifications:** `reqwest` for Discord webhooks; trait-based notifier architecture.
- **Docker:** Multi-stage build from `rust:1-slim`, final image based on `debian:bookworm-slim` or `distroless` with BlueZ client libraries. Mount `/var/run/dbus` and use host networking.
- **Data Model:**
  - `Device { mac: String, name: Option<String>, last_seen: DateTime<Utc>, rssi: i16 }`
  - `PresenceState { Entered, Exited, Pending }`
  - `NotifierConfig { discord_webhook_url: Option<String>, discord_bot_token: Option<String>, ... }`

## Success Metrics

- Notifications are delivered within 60 seconds of a person actually entering or leaving the monitored area.
- Zero false enter/exit notifications under normal home conditions over a 7-day period (tunable via debounce config).
- App runs continuously for 30 days without memory leaks or crashes (verified by container restart count).
- A junior developer can clone the repo, copy `config.example.toml`, fill in their Discord webhook, and see their first notification within 10 minutes.

## Open Questions

- Should the app support BLE advertising data parsing (iBeacon, Eddystone) for room-level accuracy, or is single-zone presence sufficient?
- Should there be a small web UI or TUI for discovering and labeling new devices interactively, or is manual TOML editing acceptable for MVP?
- For the Discord bot path: is an actual Discord app registration required for MVP, or is webhook-only acceptable for the first release?

## Dependencies

- **System:** Linux host with BlueZ 5.x and a Bluetooth 4.0+ adapter.
- **External:** Discord channel with incoming webhook enabled (or Discord bot token).
- **Rust crates:** `blurz`/`bluez-async`, `tokio`, `serde`, `toml`, `reqwest`, `tracing`, `chrono`.

## Timeline / Milestones

- **M1 (Week 1):** Core BLE scanning loop + device state tracking + TOML config loading.
- **M2 (Week 2):** Enter/exit detection logic with configurable debounce thresholds.
- **M3 (Week 3):** Discord webhook notifier + pluggable notifier trait + Docker packaging.
- **M4 (Week 4):** Error handling, retry logic, logging, documentation, and repo polish.

---
*Generated from PRD template*
