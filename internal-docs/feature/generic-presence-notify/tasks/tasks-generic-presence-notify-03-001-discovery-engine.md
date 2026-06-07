---
story_id: "03-001"
story_title: "Discovery Engine + discover CLI"
story_name: "discovery-engine"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 3
parallel_id: 1
branch: "feature/current/generic-presence-notify/story-03-001-discovery-engine"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-004"]
parallel_safe: true
modules: ["src/discovery/"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "backend", "ml"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Build offline correlation engine that computes Jaccard similarity of identifier co-occurrence within a sliding time window. Implement `proximityd discover --hours N --min-confidence X` CLI.

## Sub-Tasks

- [x] Create `src/discovery/mod.rs` — `DiscoveryEngine` struct
- [x] Create `src/discovery/correlator.rs` — Jaccard similarity on signal log; sliding window (default 5 min)
- [x] Create `src/discovery/report.rs` — `Suggestion` struct with confidence, rationale, proposed party/device/identifier mappings
- [x] Implement `proximityd discover` CLI: `--hours`, `--min-confidence`, `--output stdout/file`
- [x] Write `suggestions.toml` output with grouped suggestions and evidence
- [x] Add `src/discovery/tests.rs` — unit tests with synthetic signal log data

## Relevant Files

- `src/discovery/mod.rs` — DiscoveryEngine struct with discover() method
- `src/discovery/correlator.rs` — Jaccard similarity computation with 5-minute sliding windows
- `src/discovery/report.rs` — Suggestion struct and TOML serialization with confidence scores
- `src/discovery/tests.rs` — Unit tests with synthetic signal log data (12 tests)
- `src/main.rs` — CLI subcommand implementation with discover command (--hours, --min-confidence, --output)
- `src/lib.rs` — Added discovery module export

## Acceptance Criteria

- [x] `discover` command runs offline and reads signal log
- [x] Jaccard similarity computed correctly on synthetic data
- [x] Output is valid TOML with confidence scores and rationale
- [x] Unit tests pass: `cargo test discovery`

## Test Plan

- Unit: `cargo test discovery`
- Integration: `cargo test --test cli_tests` (discover subcommand)

## Risks & Mitigations

- Risk: Large signal log makes correlation slow — Mitigation: limit query to `--hours` window; use indexed queries

## Dependencies & Sequencing

- Depends on: 01-001 (signal log), 01-004 (config model)
- Unblocks: 03-002

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(discovery): add correlation engine with Jaccard similarity`
- `feat(cli): add discover command`

## Changelog

- 2026-06-03: initialized story file
