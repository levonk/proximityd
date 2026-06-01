---
story_id: "01-003"
story_title: "Device State Tracking Structures"
story_name: "device-state-tracking-structures"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 1
parallel_id: 3
branch: "feature/current/bluetooth-presence-notifier/story-01-003-device-state-tracking-structures"
status: "done"
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

- [x] Create `src/state/mod.rs` — module root with re-exports
- [x] Create `src/state/types.rs` — `PresenceState` enum (`Entered`, `Exited`, `Pending`), `TrackedDevice` struct with `mac`, `name`, `last_seen`, `rssi`, `state`
- [x] Create `src/state/table.rs` — `PresenceStateTable` backed by `std::sync::RwLock<HashMap<String, TrackedDevice>>` with methods: `update(mac, rssi)`, `get_state(mac)`, `list_present()`, `list_exited()`
- [x] Create `src/state/events.rs` — `PresenceEvent` enum (`Entered { name, mac }`, `Exited { name, mac }`) for state transition notifications
- [x] Add unit tests for state transitions and concurrent access
- [x] Add property-based tests (if feasible) for state machine correctness — skipped: no proptest/quickcheck dependency; unit tests provide sufficient coverage

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/state/mod.rs` — module root with re-exports
- `src/state/types.rs` — core data types (`PresenceState`, `TrackedDevice`)
- `src/state/table.rs` — thread-safe state table (`PresenceStateTable`)
- `src/state/events.rs` — state transition events (`PresenceEvent`)
- `src/lib.rs` — added `pub mod state`

## Acceptance Criteria

- [x] `PresenceStateTable` is thread-safe and supports concurrent reads/writes
- [x] `update()` correctly records `last_seen` and `rssi`
- [x] `list_present()` and `list_exited()` return accurate filtered views
- [x] `PresenceEvent` variants carry all necessary context for notifiers
- [x] Memory footprint stays minimal (target < 64 MB total daemon)

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
