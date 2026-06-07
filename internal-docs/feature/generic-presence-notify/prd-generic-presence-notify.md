# Product Requirements Document (PRD)

## Introduction / Overview

- **Feature name:** Generic Presence Notify (`proximityd`)
- **Summary:** Evolve `btnotify` from a Bluetooth-only presence notifier into a multi-signal, identity-aware presence detection daemon. The new system supports BLE, WiFi ARP, network ping sweep, and mDNS scanning; correlates identifiers into human identities (`party` abstraction); discovers probable device-to-person mappings automatically; and tracks hierarchical location (building → floor → room → zone, plus GPS and IP geolocation hints).
- **Context:**
  - `btnotify` currently detects BLE MAC addresses and sends Discord notifications. Users want to track the *same person* across multiple signals (phone BLE, laptop WiFi, static IP) without configuring each MAC manually.
  - Existing tools (ESPresense, find3, monitor) solve hardware scanning or ML positioning well, but none offer a standalone CLI-first identity broker with built-in notifications. See [competitive-analysis-feature-matrix.md](competitive-analysis-feature-matrix.md).
  - This PRD covers the open-core rename, architecture, and delivery plan. All features described herein are available in the free, self-hosted version.

---

## Goals

1. **Multi-signal scanning:** Support BLE, WiFi ARP table, ICMP ping sweep, and mDNS/hostname discovery. Each scanner can be enabled/disabled independently in `config.toml` (default: all enabled).
2. **Identity abstraction:** Introduce a `party` → `device` → `identifier` hierarchy so one person can be tracked via multiple MACs, IPs, and hostnames.
3. **Signal audit log:** Persist every raw signal sighting to a local SQLite database for forensics, debugging, and offline analysis.
4. **Intelligent correlation engine:** An out-of-band process that analyzes the signal log and suggests probabilistic party/device/identifier mappings. Suggestions default to advisory only; an `auto_promote_threshold` config flag controls whether high-confidence suggestions are trusted at runtime.
5. **Hierarchical location model:** Support static building/floor/room/zone mappings in TOML, plus automatic logging of GPS coordinates and IP-based geolocation hints when available.
6. **Standalone notifications:** Expand notifiers beyond Discord to include Slack, generic webhooks, and MQTT — all without requiring Home Assistant.
7. **Rename:** Rebrand `btnotify` to `proximityd` to reflect the broader scope, keeping `btnotify` as a deprecated wrapper for one release cycle.

---

## User Stories

- **As a** homeowner running a local server, **I want** to know when my family members arrive and leave via any of their devices (phone, laptop, tablet) **so that** I can trigger automations without depending on Home Assistant.
- **As a** systems administrator, **I want** a signal audit log of everything detected on my network **so that** I can audit unknown devices and trace security incidents.
- **As a** new user, **I want** the daemon to *suggest* which devices probably belong to me after 24 hours of observation **so that** I don't have to manually look up MAC addresses.
- **As a** privacy-conscious user, **I want** all data stored locally in SQLite with configurable retention **so that** no cloud service sees my family's presence patterns.
- **As a** multi-building operator, **I want** each scanner node to report location as building/floor/room **so that** I know *where* someone is, not just *if* they are present.

---

## Functional Requirements

### Scanner Layer

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F1 | **BLE Scanner:** Uses `btleplug` as a cross-platform `Scanner` trait implementation. Configurable scan interval. Runs on Linux, macOS 10.15+, and Windows 10+. | P0 |
| F2 | **WiFi ARP Scanner:** Reads local ARP table (`/proc/net/arp`, `ip neigh`, or SNMP query to router). Configurable router IP and scan interval. | P0 |
| F3 | **Ping Sweep Scanner:** Uses `fping` or raw ICMP to discover active hosts on a configured subnet. Disabled by default (requires subnet config). | P1 |
| F4 | **mDNS Scanner:** Listens for multicast DNS announcements (`avahi-browse`, `dns-sd`) to discover hostnames and infer device types. | P1 |
| F5 | **Scanner Toggle:** `config.toml` must contain `[scanner.<name>]` sections with `enabled = true/false` for each scanner type. Disabled scanners do not spawn tasks. | P0 |

### Identity / Configuration Model

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F6 | **Party Abstraction:** `presence.toml` supports multiple `[[parties]]`, each with a `name`. Each party has `[[parties.devices]]` entries; each device has a `name`, optional `location` (overrides party-level), and `[[parties.devices.identifiers]]`. | P0 |
| F7 | **Identifier Types:** Support `ble_mac`, `wifi_mac`, `ip_v4`, `ip_v6`, `hostname`, `card_id`, `door_sensor`. Each identifier has a `name` (human note), `type`, and `value`. A device may have multiple identifiers of the same type (e.g., two WiFi MACs, dual SIM). All values normalized on load (lowercase, trimmed). | P0 |
| F8 | **Runtime Resolution:** The detection engine resolves any incoming signal to a `party` by matching the identifier. If multiple identifiers for the same party are seen, the most recent signal source wins for state updates. | P0 |
| F9 | **Backward Compatibility:** Legacy flat `devices.toml` (top-level `[devices."MAC"]`) is parsed and auto-migrated into a single default party on first run. Each device retains its original `name` and its MAC becomes a `ble_mac` identifier. File is renamed to `presence.toml` after migration. Devices can be reorganized into proper parties via the `discover` command. | P1 |

#### Terminology: Anonymous vs Unknown

| Term | Definition |
|------|------------|
| **Party** | A named person or entity tracked by the system (e.g., "Alice", "Bob"). A party has one or more devices. |
| **Unknown** | A device or identifier that has been detected but not yet assigned to a party. Unknown devices appear in the signal log and `status` output but do not trigger party-level enter/exit notifications. The `discover` command suggests Unknown → Party mappings. |
| **Anonymous** | A device or identifier that the user explicitly chooses **not** to track (e.g., a guest's phone, a neighbor's BLE beacon). Anonymous identifiers are ignored by the detection engine and never logged to `signal_log`. Configured via `anonymous = ["aa:bb:cc:dd:ee:ff"]` in `config.toml`. |

### Signal Audit Log

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F10 | **SQLite Schema:** Append-only `signal_log` table with columns: `ts`, `scanner`, `id_type`, `id_value`, `rssi`, `party_name` (nullable, resolved at insert time), `device_name` (nullable), `location_*`, `gps_lat`, `gps_lon`, `public_ip`, `metadata` (JSON). | P0 |
| F11 | **Auto-Prune:** On startup, delete rows older than `max_log_age_days` (default 7). Pruning is logged at `info` level. | P0 |
| F12 | **Insertion Point:** Every signal from every scanner is logged *before* the detection engine evaluates it. | P0 |

### Correlation / Discovery Engine

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F13 | **Jaccard Similarity:** The `discover` CLI command computes pairwise co-occurrence of identifiers within a sliding time window (default 5 minutes). Suggestions include both party-level mappings ("these identifiers probably belong to the same person") and device-level mappings ("these identifiers probably belong to the same device"). | P1 |
| F14 | **Suggestion Output:** Results are written as TOML to stdout or `~/.config/proximityd/suggestions.toml`, grouped by suggested party with confidence score and evidence. Each suggestion lists the proposed device name, identifier mappings, and rationale. | P1 |
| F15 | **Runtime Toggle:** `config.toml` `[discovery]` section has `use_suggestions = false` (default). When `true`, suggestions above `auto_promote_threshold` (default 0.95) are treated as runtime mappings with a logged warning. | P1 |

### Location Model

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F16 | **Static Hierarchy:** `presence.toml` supports optional `location = { building, floor, room, zone }` on both `party` and `device` levels. Device-level overrides party-level. Scanner-node location mappings are also stored in `presence.toml`. | P1 |
| F17 | **GPS Logging:** If a GPS source is available (`geoclue`, USB GPS dongle, or host OS location), `latitude` and `longitude` are stored in `signal_log` for every sighting. Never blocks scanning. | P2 |
| F18 | **IP Logging:** The scanner node's local IP and public IP (via STUN or `icanhazip.com`) are logged per sighting. Private subnet mappings can hint at building identity. | P2 |

### Notifications

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F19 | **Discord:** Existing webhook notifier retained. | P0 |
| F20 | **Slack Webhook:** New notifier type `slack`. | P1 |
| F21 | **Generic Webhook:** New notifier type `webhook` with configurable URL, method, and payload template. | P1 |
| F22 | **MQTT Publisher:** New notifier type `mqtt` that publishes JSON presence events to a configurable topic. Enables optional Home Assistant integration without dependency. | P1 |
| F23 | **Rich Payload:** All notifiers include party name, signal source, identifier type, and location context (if configured). | P1 |

### CLI Commands

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| F24 | **`proximityd --daemon`:** Runs all enabled scanners + detection loop + notifiers. | P0 |
| F25 | **`proximityd discover --hours N --min-confidence X`:** Offline correlation analysis. | P1 |
| F26 | **`proximityd status`:** Prints currently present parties, last-seen signals, and locations. | P1 |
| F27 | **`proximityd export --format jsonl --since YYYY-MM-DD`:** Exports signal log for external analysis. | P2 |

---

## Non-Functional Requirements

| ID | Requirement | Priority |
|:--:|-------------|:--------:|
| NF1 | **Cross-Platform Builds:** Daemon mode runs on Linux, macOS 10.15+, and Windows 10+ thanks to `btleplug`. CLI commands (`discover`, `status`, `export`) compile and run on all three platforms. | P1 |
| NF2 | **Binary Size:** Final release binary < 15 MB (single static binary via `cargo build --release`). | P2 |
| NF3 | **Memory Footprint:** RSS < 64 MB during normal daemon operation (excluding SQLite cache). | P2 |
| NF4 | **Log Retention:** Signal log default 7 days. Configurable 1–90 days. | P0 |
| NF5 | **Privacy:** Zero network egress except to configured notifiers and optional IP geolocation services (disabled by default). No telemetry, no crash reporting. | P0 |
| NF6 | **Config Reload:** `SIGHUP` reloads `config.toml` and `presence.toml` without restart. | P2 |

---

## Technical Considerations

### Crate Layout

```
src/
  scanner/          -- Trait + BLE, WiFi ARP, Ping, mDNS implementations
  signals/          -- SQLite schema, logger, query engine
  discovery/        -- Correlator, report generator, CLI handler
  config/           -- PartyConfig, AppConfig, TOML loaders (serde)
  detection/        -- DetectionEngine, debounce (refactored for multi-signal)
  state/            -- PresenceStateTable, PresenceEvent (with location + source)
  notifier/         -- Trait + Discord, Slack, Webhook, MQTT implementations
  location/         -- Model, mapper, GPS, IP geo
  main.rs           -- clap CLI, daemon entry point, signal handlers
```

### Dependencies to Add

| Crate | Purpose | Notes |
|-------|---------|-------|
| `btleplug` | Cross-platform BLE scanning | Replaces `bluez-async`. BSD-3-Clause license, compatible with AGPL-3.0. |
| `geoclue-zbus` | D-Bus GPS source on Linux | LGPL-2.1+, compatible with AGPL-3.0. Linux only; macOS/Windows use native OS APIs. |
| `rusqlite` | SQLite signal log | |
| `chrono` | Timestamp handling | |
| `serde_json` | Metadata JSON in signal log | |
| `rumqttc` | MQTT notifier & future federation | Optional feature flag. Reused for Phase 5 multi-building federation. |
| `reqwest` | Webhook/Slack notifier | Already present for Discord. |
| `snmp` or `tokio-snmp` | SNMP router ARP queries | Evaluate crates; fallback to raw UDP if none are suitable. |
| `subprocess` or `tokio::process` | `fping`, `avahi-browse`, `ip neigh` wrappers | |

### Configuration Schemas

#### `config.toml` (Application Settings)

```toml
# proximityd application configuration
# Located at: ~/.config/proximityd/config.toml

[general]
log_level = "info"                    # trace, debug, info, warn, error
max_log_age_days = 7                  # Signal log retention (1-90)
config_reload = true                  # Enable SIGHUP config reload

[privacy]
privacy_mode = false                  # If true, disables ARP/ping/mDNS; BLE only
anonymous = [                         # Identifiers to ignore entirely
  "de:ad:be:ef:00:01",
  "192.168.1.200",
]

[scanner.ble]
enabled = true
scan_interval_sec = 10

[scanner.wifi_arp]
enabled = true
scan_interval_sec = 30
router_ip = "192.168.1.1"             # SNMP target or local ARP fallback
snmp_community = "public"

[scanner.ping_sweep]
enabled = false                       # Disabled by default
subnet = "192.168.1.0/24"
scan_interval_sec = 60

[scanner.mdns]
enabled = true
scan_interval_sec = 30

[detection]
enter_debounce_sec = 30               # Debounce before party enter notification
exit_debounce_sec = 120               # Debounce before party exit notification

[discovery]
use_suggestions = false
auto_promote_threshold = 0.95

# Notifiers: at least one required for notifications to fire
[[notifier]]
type = "discord"
webhook_url = "https://discord.com/api/webhooks/..."

[[notifier]]
type = "slack"
webhook_url = "https://hooks.slack.com/services/..."

[[notifier]]
type = "webhook"
url = "http://192.168.1.50:8123/api/webhook/presence"
method = "POST"
payload_template = '{"party":"{{party}}","event":"{{event}}","location":"{{location}}"}'

[[notifier]]
type = "mqtt"
broker = "192.168.1.10"
port = 1883
topic = "proximityd/presence"
```

---

#### `presence.toml` (Party / Device / Identifier Mapping)

```toml
# proximityd identity configuration
# Located at: ~/.config/proximityd/presence.toml
# Auto-migrated from legacy devices.toml on first run

[[parties]]
name = "Alice"
location = { building = "Home", floor = 1, room = "Living Room" }

  [[parties.devices]]
  name = "Alice's iPhone"
  # Device-level location overrides party-level (e.g., a laptop that moves)
  # location = { building = "Home", floor = 2, room = "Office" }

    [[parties.devices.identifiers]]
    name = "BLE MAC (main)"
    type = "ble_mac"
    value = "aa:bb:cc:dd:ee:f1"

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "11:22:33:44:55:66"

    [[parties.devices.identifiers]]
    name = "Hostname"
    type = "hostname"
    value = "alice-iphone"

  [[parties.devices]]
  name = "Alice's Laptop"
  location = { building = "Home", floor = 2, room = "Office" }

    [[parties.devices.identifiers]]
    name = "WiFi MAC"
    type = "wifi_mac"
    value = "11:22:33:44:55:77"

    [[parties.devices.identifiers]]
    name = "Ethernet IP"
    type = "ip_v4"
    value = "192.168.1.10"

[[parties]]
name = "Bob"

  [[parties.devices]]
  name = "Bob's Phone"

    [[parties.devices.identifiers]]
    name = "BLE MAC"
    type = "ble_mac"
    value = "aa:bb:cc:dd:ee:f2"

    # Multiple identifiers of the same type are supported
    [[parties.devices.identifiers]]
    name = "SIM 1"
    type = "ip_v4"
    value = "192.168.1.11"

    [[parties.devices.identifiers]]
    name = "SIM 2"
    type = "ip_v4"
    value = "192.168.1.12"
```

---

#### `suggestions.toml` (`discover` command output)

```toml
# proximityd auto-discovery suggestions
# Written by: proximityd discover --hours 24
# Located at: ~/.config/proximityd/suggestions.toml (or stdout)

[[suggestions]]
confidence = 0.97
rationale = "Co-occurrence within 5-minute window in 42 of 48 observations"

  [suggestions.party]
  name = "Suggested Party 1"

    [[suggestions.party.devices]]
    name = "Device A"

      [[suggestions.party.devices.identifiers]]
      type = "ble_mac"
      value = "aa:bb:cc:dd:ee:ff"

      [[suggestions.party.devices.identifiers]]
      type = "wifi_mac"
      value = "11:22:33:44:55:66"

    [[suggestions.party.devices]]
    name = "Device B"

      [[suggestions.party.devices.identifiers]]
      type = "ip_v4"
      value = "192.168.1.50"

[[suggestions]]
confidence = 0.82
rationale = "Same hostname pattern, different MAC, observed during overlapping hours"

  [suggestions.party]
  name = "Suggested Party 2"

    [[suggestions.party.devices]]
    name = "Work Laptop"

      [[suggestions.party.devices.identifiers]]
      type = "ble_mac"
      value = "aa:bb:cc:dd:ee:aa"

      [[suggestions.party.devices.identifiers]]
      type = "hostname"
      value = "CORP-LAPTOP-1234"
```

---

#### Legacy `devices.toml` (Auto-Migrated)

```toml
# DEPRECATED: legacy btnotify device format
# Auto-migrated to presence.toml on first run; file is renamed to devices.toml.bak

[devices."AA:BB:CC:DD:EE:FF"]
name = "Alice's Phone"

[devices."11:22:33:44:55:66"]
name = "Bob's Laptop"
```

Migration result in `presence.toml`:

```toml
# All legacy devices collapse into a single default party
[[parties]]
name = "Unknown"

  [[parties.devices]]
  name = "Alice's Phone"

    [[parties.devices.identifiers]]
    type = "ble_mac"
    value = "aa:bb:cc:dd:ee:ff"

  [[parties.devices]]
  name = "Bob's Laptop"

    [[parties.devices.identifiers]]
    type = "ble_mac"
    value = "11:22:33:44:55:66"
```

### Data Flow

```
Scanner(s) → RawSignal → SignalLogger::log() → SQLite
                              ↓
                        DetectionEngine::evaluate()
                              ↓
                        PresenceEvent → Notifier(s)
                              ↓
                        Discord/Slack/Webhook/MQTT
```

### SQLite File Location

- Linux: `$XDG_DATA_HOME/proximityd/signals.db` (default `~/.local/share/proximityd/signals.db`)
- macOS: `~/Library/Application Support/proximityd/signals.db`
- Windows: `%LOCALAPPDATA%\proximityd\signals.db`

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Zero BLE regressions | All BLE tests ported to `btleplug` and passing on Linux + macOS (DO NOT REDUCE TESTS)| `cargo test` on both platforms before merge |
| WiFi ARP scanner functional | Produces `signal_log` rows on real network | Manual test on home LAN |
| Suggestion accuracy | >80% top-1 correct after 24h of logging | Compare `suggestions.toml` to known ground truth |
| Binary size | <15 MB | `ls -lh target/release/proximityd` |
| Memory RSS | <64 MB | `ps -o rss= -p $(pgrep proximityd)` after 1 hour |
| Setup time | <5 minutes from `cargo install` to first notification | Stopwatch test on fresh VM |

---

## Open Questions

### 1. Final binary name

**Decision:** `proximityd` — proceed with this name. Rationale: `presenced` implies a daemon that *is* present; `presence-notify` is too long for a system binary; `proximityd` accurately describes proximity detection via multiple signals. Keep `btnotify` as a deprecated symlink/wrapper for one release cycle (see F9).

### 2. BLE cross-platform library: `btleplug`

**Research findings:**

- **Platforms:** Linux, Windows 10+, macOS 10.15+, iOS, Android, WASM/WebBluetooth (experimental).
- **Features:** Device enumeration, GATT services/characteristics, RSSI, manufacturer data, MAC address — all features needed for our BLE scanner.
- **License:** BSD-3-Clause (primary) with some MIT/Apache-2.0 components. BSD-3-Clause is compatible with AGPL-3.0 — permissive licenses can be linked/used in AGPL projects without license contamination. No issues.
- **Recommendation:** Replace `bluez-async` with `btleplug` to enable cross-platform BLE scanning. This removes the Linux-only restriction in NF1 and makes `proximityd --daemon` viable on macOS and Windows.

### 3. GPS source abstraction: `geoclue`

**Research findings:**

- **What it is:** D-Bus geolocation service on Linux that aggregates WiFi, cell tower, and GPS sources. Rust crate `geoclue-zbus` provides typed D-Bus bindings.
- **License:** LGPL-2.1+ / GPL-2+. LGPL is compatible with AGPL-3.0 (LGPL is designed for linking with GPL-family code). No license issues.
- **Approach:** On Linux, attempt D-Bus connection to `org.freedesktop.GeoClue2` at startup; if unavailable, GPS logging is silently disabled. On macOS/Windows, use host OS location APIs (Core Location, Windows.Devices.Geolocation) via conditional compilation. Always non-blocking — never wait for GPS fix before scanning.
- **Decision:** Integrate `geoclue` on Linux; defer macOS/Windows GPS to Phase 3.

### 4. SNMP router query for ARP table retrieval

**Research findings:**

The standard ARP table OID is `1.3.6.1.2.1.4.22.1.2` (`ipNetToMediaPhysAddress`). This is technically deprecated in favor of `1.3.6.1.2.1.4.35.1.4` (`ipNetToPhysicalPhysAddress`) in newer SNMP implementations, but most consumer routers still implement the older OID.

**Known-working router models:**

| Router / Firmware | SNMP OID | Notes |
|-------------------|----------|-------|
| **ASUS RT-AX86U** (ASUSWRT / Merlin) | `1.3.6.1.2.1.4.22.1.2` | Works on stock and Merlin. Enable SNMP under Administration > SNMP. |
| **TP-Link Archer AX50** (stock firmware) | `1.3.6.1.2.1.4.22.1.2` | Enable under System Tools > SNMP Settings. Returns wired + wireless ARP entries. |
| **OpenWrt / DD-WRT** (any supported hardware) | `1.3.6.1.2.1.4.22.1.2` | Install `snmpd` package. Most reliable; returns full ARP table including WiFi clients. |

**Implementation note:** The WiFi ARP scanner should try both OIDs (legacy first, then modern) and cache the working OID per router IP. If SNMP fails, fall back to local ARP table (`ip neigh` on Linux, `arp -a` on macOS/Windows).

### 5. Apple Private WiFi Addresses

**Research findings:**

- iOS 14+ and macOS 13+ randomize WiFi MAC addresses **per network** by default. The randomized MAC is *stable per SSID* — reconnecting to the same network yields the same MAC. This means for our home-network use case, the MAC is effectively static once a device has joined.
- Apple devices use locally administered MACs for private addresses (second LSB of first octet = 1). This prevents OUI-based manufacturer identification.
- The device hostname (e.g., "John's iPhone") is broadcast via mDNS and DHCP, which *can* be used for correlation.

**Decision:**

- The WiFi scanner will continue to use MAC as the primary identifier. On a single network, the private MAC is stable, so no additional correlation logic is needed for basic presence detection.
- Add `hostname` as a supported `identifier_type` (already planned in F7). The mDNS scanner (F4) will capture hostnames automatically.
- Document the limitation: "Apple Private WiFi Addresses generate a stable MAC per SSID, so presence detection works without configuration. However, the MAC cannot be pre-configured across multiple networks."
- Do not attempt DHCP fingerprinting — too complex for v1.0. mDNS hostname correlation is sufficient.

### 6. Federation protocol for multi-building support

**Research findings:**

| Protocol | Model | Pros | Cons |
|----------|-------|------|------|
| **MQTT** | Broker-based pub/sub | Mature, lightweight, excellent Rust support (`rumqttc`), works over TCP/SSL, easy to bridge buildings via a central broker | Requires a broker (single point of failure unless clustered) |
| **serf** | Gossip/membership | Decentralized, no broker, built-in failure detection, used by HashiCorp Consul | Overkill for presence events, heavier protocol, requires UDP multicast between nodes |
| **libp2p** | P2P mesh | Fully decentralized, NAT traversal, content routing | Extremely complex, massive dependency tree, overkill for our use case |

**Decision:** Use MQTT for post-v1.0 multi-building federation. Rationale:

- MQTT is the industry standard for IoT telemetry; every building already has network connectivity.
- A central MQTT broker (or bridged brokers per building) can aggregate presence events.
- `rumqttc` is already planned as a notifier dependency (F22), so we reuse the same crate.
- Federation is explicitly Phase 5 / post-v1.0 — no code needed now, just the architectural decision.

### 7. Additional open questions identified

- **Bluetooth MAC randomization:** Modern Android (10+) and iOS also randomize BLE MACs during scanning. Unlike WiFi, BLE random MACs rotate periodically (every ~15 minutes). How should the BLE scanner handle this? The `btleplug` adapter exposes a `PeripheralId` that may be more stable than the advertised MAC. Document: BLE presence detection should not rely on MAC alone; use a combination of MAC + manufacturer data + local name.
- **Device OUI tagging:** Should we add an `oui_hint` field to `presence.toml` so users can label devices by manufacturer (e.g., `oui_hint = "Apple Inc."`)? This would help with device classification but is not required for v1.0.
- **Multi-interface devices:** A laptop may be on both WiFi and Ethernet. Should the `presence.toml` model allow a single device to have both `wifi_mac` and `ip_v4` identifiers, and treat either signal as "present"? Yes — this is already covered by the `party` → `device` → `identifier` hierarchy (F6-F8), but clarify in docs.
- **Privacy toggle:** Should `config.toml` have a `[privacy]` section to disable specific scanners (e.g., disable WiFi scanning in a guest network)? Yes — scanner toggle (F5) already covers this, but add a `privacy_mode = false` flag that disables all network-level scanning (ARP, ping, mDNS) and runs only BLE.

---

## Dependencies

- **Internal:** Existing `btnotify` config loading, notifier trait, state table. BLE scanning is being replaced by `btleplug`.
- **External:** `btleplug`, `geoclue-zbus` (Linux GPS), `rusqlite`, `chrono`, `serde_json`, `rumqttc` (optional), SNMP library (TBD).
- **Research:** [competitive-analysis-feature-matrix.md](competitive-analysis-feature-matrix.md) must be reviewed by implementer before coding begins.
- **Tickets:** tkr ticket for Phase 0 should be created and assigned before any file renames.

---

## Timeline / Milestones

| Phase | Duration | Milestone | Deliverable |
|-------|:--------:|-----------|-------------|
| 0 | Weeks 1–2 | Foundation | Signal log, new config model, rename, zero regressions |
| 1 | Weeks 3–4 | Multi-signal | WiFi ARP, ping sweep, mDNS scanners integrated |
| 2 | Weeks 5–6 | Intelligence | `discover` command, suggestion TOML, runtime toggle |
| 3 | Weeks 7–8 | Location | Building/floor/room/zone, GPS, IP geo logging |
| 4 | Weeks 9–10 | Polish | New notifiers, `status` CLI, README, devbox/flake |
| 5 | Future | Advanced (open-core) | Arrival prediction, guest fingerprinting, multi-building federation via MQTT |

**Target v1.0 release:** End of Phase 4 (10 weeks from Phase 0 start).

---

## Licensing

`proximityd` is dual-licensed:

1. **AGPL-3.0** (or later) for the open-source community.
2. **Commercial license** available for organizations that cannot comply with AGPL terms.

### Why AGPL?

`proximityd` is a network-facing daemon. Standard GPL only triggers on distribution of binaries; it does not require sharing modifications when the software is run as a service (the "ASP loophole"). AGPL-3.0 closes this loophole: anyone who modifies `proximityd` and offers it as a service must publish their modifications under the same license.

This protects the project from proprietary forks that improve the code but never contribute back. It also creates a natural incentive for commercial users to purchase a proprietary license, which funds ongoing open-source development.

### Dual-License Mechanics

- **Community / self-hosted users:** Use AGPL-3.0. Free forever. Must share modifications if they distribute binaries or offer the software as a service.
- **Enterprise / SaaS vendors:** Purchase a commercial license that removes AGPL obligations. Pricing available on request.
- **Contributors:** All contributions to the main repository are accepted under AGPL-3.0. Contributors retain copyright to their work but grant the project the right to relicense under the commercial license.

---

*Generated from PRD template*
*Competitive analysis: [competitive-analysis-feature-matrix.md](competitive-analysis-feature-matrix.md)*
