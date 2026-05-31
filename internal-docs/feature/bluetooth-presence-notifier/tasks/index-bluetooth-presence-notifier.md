---
description: Task index for Bluetooth Presence Notifier (btnotify)
prd_name: bluetooth-presence-notifier
prd_file: docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

# Bluetooth Presence Notifier — Implementation Task Index

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules | Status |
| -------- | ----------- | ------ | ------------ | ------------- | ------- | ------ |
| 01-001 | Config & Device Mapping Loading | feature/current/bluetooth-presence-notifier/story-01-001-config-device-mapping-loading | None | Parallel-safe: true | src/config/ |
| 01-002 | BLE Scanning Loop | feature/current/bluetooth-presence-notifier/story-01-002-ble-scanning-loop | None | Parallel-safe: true | src/bluetooth/ | [~] In-Progress |
| 01-003 | Device State Tracking Structures | feature/current/bluetooth-presence-notifier/story-01-003-device-state-tracking-structures | None | Parallel-safe: true | src/state/ |
| 02-001 | Enter/Exit Detection with Debounce | feature/current/bluetooth-presence-notifier/story-02-001-enter-exit-detection-with-debounce | 01-001, 01-002, 01-003 | Parallel-safe: true | src/detection/ |
| 03-001 | Pluggable Notifier Trait & Discord Webhook | feature/current/bluetooth-presence-notifier/story-03-001-pluggable-notifier-trait-and-discord-webhook | 02-001 | Parallel-safe: true | src/notifier/ |
| 03-002 | Docker Multi-Arch Packaging | feature/current/bluetooth-presence-notifier/story-03-002-docker-multi-arch-packaging | 01-001, 01-002, 01-003 | Parallel-safe: true | Dockerfile, docker-compose.yml |
| 04-001 | Observability, Error Handling & Retry Logic | feature/current/bluetooth-presence-notifier/story-04-001-observability-error-handling-and-retry-logic | 03-001 | Parallel-safe: true | src/ (logging, retry, healthcheck) |
| 04-002 | Documentation & Repo Polish | feature/current/bluetooth-presence-notifier/story-04-002-documentation-and-repo-polish | 03-002 | Parallel-safe: true | README.md, docs/, examples/ |

## Phase Overview

- **Phase 01** — Foundation: Config parsing, BLE scanning, and core data structures. All three stories are independent.
- **Phase 02** — Presence Detection: Combines scan results with config to determine enter/exit events.
- **Phase 03** — Notifications & Packaging: Discord webhook notifier + hardened Docker image.
- **Phase 04** — Resilience & Polish: Retry logic, health checks, structured logging, and documentation.
