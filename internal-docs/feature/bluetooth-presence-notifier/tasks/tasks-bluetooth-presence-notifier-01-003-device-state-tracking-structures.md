---
story_id: "01-003"
story_title: "Device State Tracking Structures"
story_name: "device-state-tracking-structures"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 1
parallel_id: 3
branch: "feature/current/bluetooth-presence-notifier/story-01-003-device-state-tracking-structures"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/state/"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "backend", "state"]
due: "2026-06-03"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Implement the core data model and in-memory state table for tracking which labeled devices are present, pending, or exited. Define `PresenceState`, `TrackedDevice`, and a thread-safe `PresenceStateTable`.

## Sub-Tasks

- [ ] Create `src/state/mod.rs` — module root with re-exports
- [ ] Create `src/state/types.rs` — `PresenceState` enum (`Entered`, `Exited`, `Pending`), `TrackedDevice` struct with `mac`, `name`, `last_seen`, `rssi`, `state`
- [ ] Create `src/state/table.rs` — `PresenceStateTable` backed by `std::sync::RwLock<HashMap<String, TrackedDevice>>` with methods: `update(mac, rssi)`, `get_state(mac)`, `list_present()`, `list_exited()`
- [ ] Create `src/state/events.rs` — `PresenceEvent` enum (`Entered { name, mac }`, `Exited { name, mac }`) for state transition notifications
- [ ] Add unit tests for state transitions and concurrent access
- [ ] Add property-based tests (if feasible) for state machine correctness

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/state/mod.rs` — module root
- `src/state/types.rs` — core data types
- `src/state/table.rs` — thread-safe state table
- `src/state/events.rs` — state transition events
- `src/state/table.test.rs` — unit tests for concurrent updates and transitions
- `src/state/types.test.rs` — unit tests for enum/struct behavior

## Acceptance Criteria

- [ ] `PresenceStateTable` is thread-safe and supports concurrent reads/writes
- [ ] `update()` correctly records `last_seen` and `rssi`
- [ ] `list_present()` and `list_exited()` return accurate filtered views
- [ ] `PresenceEvent` variants carry all necessary context for notifiers
- [ ] Memory footprint stays minimal (target < 64 MB total daemon)

## Test Plan

- Unit: `cargo test state::`
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Add `tracing::debug!` on every state table update
- Add `tracing::info!` on state transitions (Entered/Exited)

## Compliance

- All device data is in-memory only; no persistence layer for privacy

## Risks & Mitigations

- Risk: `RwLock` contention under high scan frequency — Mitigation: benchmark with `criterion`; switch to `dashmap` if needed
- Risk: Memory growth with many unique MACs — Mitigation: prune unknown MACs after timeout (configurable)

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001 (needs state table to evaluate transitions)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(state): add PresenceStateTable and transition events`

## Changelog

- 2026-05-27: initialized story file
