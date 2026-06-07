# Index: Generic Presence Notify Tasks

Generated from PRD: `internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md`

## Relevant Files

- `src/signals/` — SQLite schema, signal logger, query engine
- `src/scanner/` — Scanner trait + BLE, WiFi ARP, ping, mDNS implementations
- `src/config/` — PartyConfig, AppConfig, TOML loaders (refactored)
- `src/detection/` — DetectionEngine, debounce (refactored for multi-signal)
- `src/state/` — PresenceStateTable, PresenceEvent (with location + source)
- `src/notifier/` — Trait + Discord, Slack, Webhook, MQTT implementations
- `src/location/` — Model, mapper, GPS, IP geo
- `src/discovery/` — Correlator, report generator, CLI handler
- `src/main.rs` — clap CLI, daemon entry point, signal handlers
- `Cargo.toml` — Dependencies (btleplug, rusqlite, rumqttc, etc.)
- `tests/` — Integration and CLI tests

## Parallel Development Sets

### Phase 01 — Foundation
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 01-001 | Signal Log SQLite Schema and Logger | [x] Done | feature/current/generic-presence-notify/story-01-001-signal-log | None | true | src/signals/ |
| 01-002 | Rename btnotify to proximityd + Deprecation Wrapper | [x] Done | feature/current/generic-presence-notify/story-01-002-rename | None | true | root, Cargo.toml |
| 01-003 | Scanner Trait and BLE btleplug Migration | [x] Done | feature/current/generic-presence-notify/story-01-003-scanner-trait | None | true | src/scanner/ |
| 01-004 | Config Model and Legacy Migration | [x] Done | feature/current/generic-presence-notify/story-01-004-config-model | None | true | src/config/ |

### Phase 02 — Multi-Signal
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 02-001 | WiFi ARP Scanner | [x] Done | feature/current/generic-presence-notify/story-02-001-wifi-arp-scanner | 01-003 | true | src/scanner/ |
| 02-002 | Ping Sweep Scanner | [x] Done | feature/current/generic-presence-notify/story-02-002-ping-sweep-scanner | 01-003 | true | src/scanner/ |
| 02-003 | mDNS Scanner | [~] In-Progress | feature/current/generic-presence-notify/story-02-003-mdns-scanner | 01-003 | true | src/scanner/ |

### Phase 03 — Intelligence
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 03-001 | Discovery Engine + discover CLI | [ ] Todo | feature/current/generic-presence-notify/story-03-001-discovery-engine | 01-001, 01-004 | true | src/discovery/ |
| 03-002 | Suggestion Runtime Toggle | [ ] Todo | feature/current/generic-presence-notify/story-03-002-suggestion-runtime | 03-001 | true | src/discovery/ |

### Phase 04 — Location
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 04-001 | Hierarchical Location Model | [ ] Todo | feature/current/generic-presence-notify/story-04-001-hierarchical-location | 01-004 | true | src/location/ |
| 04-002 | GPS and IP Geolocation Logging | [ ] Todo | feature/current/generic-presence-notify/story-04-002-gps-ip-geo | 01-001, 04-001 | true | src/location/ |

### Phase 05 — Polish
| Story ID | Story Title | Status | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------ | ------------ | ------------- | ------- |
| 05-001 | New Notifiers (Slack, Webhook, MQTT) | [ ] Todo | feature/current/generic-presence-notify/story-05-001-new-notifiers | 01-004 | true | src/notifier/ |
| 05-002 | status/export CLI and Documentation | [ ] Todo | feature/current/generic-presence-notify/story-05-002-status-export-cli | 01-001, 01-004 | true | src/main.rs, docs/ |
