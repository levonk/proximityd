---
story_id: "02-001"
story_title: "Enter/Exit Detection with Debounce"
story_name: "enter-exit-detection-with-debounce"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 2
parallel_id: 1
branch: "feature/current/bluetooth-presence-notifier/story-02-001-enter-exit-detection-with-debounce"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "01-003"]
parallel_safe: true
modules: ["src/detection/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend", "detection"]
due: "2026-06-10"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Implement the enter/exit decision engine that consumes scan results, consults config thresholds, and updates the `PresenceStateTable`. Apply debouncing so a device must meet RSSI and duration requirements before being declared present, and must miss scans for a timeout before being declared exited.

## Sub-Tasks

- [ ] Create `src/detection/mod.rs` — module root with re-exports
- [ ] Create `src/detection/engine.rs` — `DetectionEngine` struct holding `AppConfig`, `DevicesConfig`, and `PresenceStateTable`
- [ ] Implement `evaluate_scan(mac, rssi)` logic:
  - If MAC is unknown and config says `track_unknown = false`, ignore
  - If RSSI < `enter_rssi_threshold_dbm`, do not count toward enter
  - Require `enter_duration_seconds` of continuous qualified detections before transitioning to `Entered`
  - If last detection > `exit_timeout_seconds` ago, transition to `Exited`
- [ ] Create `src/detection/debounce.rs` — `DebounceTimer` per-device tracking first-seen-timestamp and last-seen-timestamp
- [ ] Wire `DetectionEngine` into the scan loop so each scan result triggers evaluation
- [ ] Add unit tests for all threshold combinations and edge cases (device at threshold, rapid flapping, long absence)
- [ ] Add test for zero false positives under simulated home conditions

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/detection/mod.rs` — module root
- `src/detection/engine.rs` — core detection logic
- `src/detection/debounce.rs` — per-device debounce timers
- `src/detection/engine.test.rs` — unit tests for threshold evaluation
- `src/detection/debounce.test.rs` — unit tests for timer behavior
- `src/main.rs` — wire engine into scan loop

## Acceptance Criteria

- [ ] Enter declared only after `enter_duration_seconds` of qualified scans
- [ ] Exit declared only after `exit_timeout_seconds` of no scans
- [ ] Unknown MACs are ignored unless `track_unknown = true`
- [ ] Zero false enter/exit events in 7-day simulation test
- [ ] Notification latency under 5 seconds from actual state change (NFR-1)

## Test Plan

- Unit: `cargo test detection::`
- Integration: `cargo test --test detection_simulation` (time-accelerated simulation)
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Add `tracing::info!` on enter/exit transitions with device name
- Add `tracing::debug!` on every evaluation result
- Add `tracing::warn!` if a labeled device flaps rapidly

## Compliance

- Decision logic is deterministic and local; no external dependencies

## Risks & Mitigations

- Risk: RSSI fluctuation causes rapid state flapping — Mitigation: configurable thresholds and hysteresis (future: require exit RSSI > enter RSSI)
- Risk: Clock drift on long-running daemon — Mitigation: use `std::time::Instant` for durations, not wall clock

## Dependencies & Sequencing

- Depends on:
  - [[story-01-001-config-device-mapping-loading]] (needs config thresholds)
  - [[story-01-002-ble-scanning-loop]] (needs scan results)
  - [[story-01-003-device-state-tracking-structures]] (needs state table)
- Unblocks: 03-001 (needs presence events to notify)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(detection): add enter/exit debounce engine`

## Changelog

- 2026-05-27: initialized story file
