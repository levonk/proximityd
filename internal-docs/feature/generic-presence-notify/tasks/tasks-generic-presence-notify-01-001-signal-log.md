---
story_id: "01-001"
story_title: "Signal Log SQLite Schema and Logger"
story_name: "signal-log"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 1
parallel_id: 1
branch: "feature/current/generic-presence-notify/story-01-001-signal-log"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/signals/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend", "database"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-04"
---

## Summary

Create the append-only SQLite `signal_log` table and a `SignalLogger` that inserts every raw signal sighting before the detection engine evaluates it. Includes auto-prune on startup.

## Sub-Tasks

- [x] Create `src/signals/mod.rs` and `src/signals/schema.rs` — SQLite schema with columns: `ts`, `scanner`, `id_type`, `id_value`, `rssi`, `party_name`, `device_name`, `location_building`, `location_floor`, `location_room`, `location_zone`, `gps_lat`, `gps_lon`, `public_ip`, `metadata` (JSON)
- [x] Create `src/signals/logger.rs` — `SignalLogger` struct with `log(raw_signal: &RawSignal)` method; inserts row with nullable party/device resolved later
- [x] Create `src/signals/db_path.rs` — cross-platform DB path resolution: Linux (`~/.local/share/proximityd/signals.db`), macOS (`~/Library/Application Support/proximityd/signals.db`), Windows (`%LOCALAPPDATA%\proximityd\signals.db`)
- [x] Add `rusqlite` to `Cargo.toml`
- [x] Implement auto-prune: on `SignalLogger` init, delete rows older than `max_log_age_days` (default 7); log at `info` level
- [x] Add `src/signals/tests.rs` — unit tests for schema creation, insert, query, and prune
- [x] Wire `SignalLogger::log()` call point in detection pipeline (stub for now; full wiring in 01-003/01-004)

## Relevant Files

- `src/signals/mod.rs` — module exports
- `src/signals/schema.rs` — table creation SQL
- `src/signals/logger.rs` — insert and prune logic
- `src/signals/db_path.rs` — platform-specific paths
- `src/signals/tests.rs` — unit tests
- `Cargo.toml` — add `rusqlite` dependency

## Acceptance Criteria

- [x] `signal_log` table is created on first run
- [x] Every inserted row has `ts`, `scanner`, `id_type`, `id_value`
- [x] Auto-prune deletes only rows older than configured threshold
- [x] Unit tests pass: `cargo test signals`

## Test Plan

- Unit: `cargo test signals`
- Lint: `cargo clippy -- -D warnings`
- Typecheck: `cargo check`

## Observability

- Log prune count at `info` level on startup
- Log insert errors at `warn` level (must not block scanning)

## Risks & Mitigations

- Risk: DB path fails on macOS/Windows — Mitigation: use `dirs` crate for path resolution
- Risk: Prune on large DB is slow — Mitigation: add index on `ts`; run in background task

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001, 02-002, 02-003, 03-001, 05-002

## Definition of Done

- Code, tests, docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(signals): add signal_log SQLite schema`

## Changelog

- 2026-06-03: initialized story file
