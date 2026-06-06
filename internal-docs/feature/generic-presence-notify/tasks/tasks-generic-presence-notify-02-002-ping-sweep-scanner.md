---
story_id: "02-002"
story_title: "Ping Sweep Scanner"
story_name: "ping-sweep-scanner"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 2
parallel_id: 2
branch: "feature/current/generic-presence-notify/story-02-002-ping-sweep-scanner"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["src/scanner/"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "backend", "network"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Implement ICMP ping sweep scanner using `fping` or raw ICMP sockets. Emits `RawSignal` with `ip_v4` identifiers. Disabled by default (requires subnet config).

## Sub-Tasks

- [x] Create `src/scanner/ping_sweep.rs` — `PingSweepScanner` implementing `Scanner`
- [x] Implement `fping` wrapper via `tokio::process` (preferred)
- [x] Implement raw ICMP fallback using `tokio` sockets if `fping` unavailable
- [x] Respect `[scanner.ping_sweep]` config: `enabled` (default false), `subnet`, `scan_interval_sec`
- [x] Add unit tests with mocked `fping` output

## Relevant Files

- `src/scanner/ping_sweep.rs`
- `src/scanner/ping_sweep_tests.rs`

## Acceptance Criteria

- [x] Scanner emits `RawSignal` with `id_type = "ip_v4"` for each responsive host
- [x] Disabled by default
- [x] Works with or without `fping` installed
- [x] Unit tests pass

## Test Plan

- Unit: `cargo test ping_sweep`

## Risks & Mitigations

- Risk: Raw ICMP requires root on Linux — Mitigation: document `fping` as preferred; setcap if needed

## Dependencies & Sequencing

- Depends on: 01-003
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(scanner): add ping sweep scanner`

## Changelog

- 2026-06-03: initialized story file
