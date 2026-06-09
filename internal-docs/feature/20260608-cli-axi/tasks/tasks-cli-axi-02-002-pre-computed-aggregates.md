---
story_id: "02-002"
story_title: "Pre-computed Aggregates"
story_name: "pre-computed-aggregates"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 2
parallel_id: 2
branch: "feature/current/cli-axi/story-02-002-pre-computed-aggregates"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["cli.rs", "output.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement pre-computed aggregate counts and derived status fields in list outputs. Include total counts (not just page size) and lightweight summaries (identifiers active, devices present). Compute counts efficiently at query time.

## Sub-Tasks

- [x] Implement aggregate count computation for list queries
- [x] Add total count to list output format (count: X of Y total)
- [x] Implement derived status fields (identifiers: 3/3 active, devices: 2 present)
- [x] Optimize count queries for efficiency
- [x] Add derived fields to detail views where relevant
- [x] Update parties list to include device count aggregates
- [x] Update devices list to include identifier count aggregates
- [x] Add aggregate computation tests
- [x] Ensure aggregates work with both TOON and JSON
- [~] Update CLI help text to document aggregate fields

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/aggregates.rs` (new) — Aggregate computation logic with ListAggregate, PartyAggregate, DeviceAggregate, SystemAggregate
- `src/output/mod.rs` — Added aggregates module exports
- `src/output/schema.rs` — Added aggregate field to PartyOutput and DeviceOutput structs
- `src/main.rs` — Added aggregate computation to run_parties and run_devices, updated CLI help text
- `tests/aggregates_test.rs` (new) — Aggregate computation tests (15 tests)
- `tests/schema_test.rs` — Updated PartyOutput and DeviceOutput test fixtures to include aggregate field

## Acceptance Criteria

- [x] List outputs show total counts (count: X of Y total)
- [x] Derived status fields are included inline
- [x] Aggregate computation is efficient
- [x] Aggregates work with both TOON and JSON
- [x] Parties list shows device count
- [x] Devices list shows identifier count
- [x] Aggregates are computed at query time

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test output::aggregates`
- Integration: Test aggregates for all list commands
- Performance: Benchmark aggregate computation
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log aggregate computation performance
- Track aggregate field usage

## Compliance

- Follow AXI pre-computed aggregate requirements
- Ensure aggregates don't significantly impact performance

## Risks & Mitigations

- Risk: Performance impact from aggregate computation — Mitigation: Optimize queries and cache where appropriate
- Risk: Complex aggregate logic — Mitigation: Keep aggregates simple and well-documented

## Dependencies

- 01-003 (Minimal Default Schemas) — Schemas define which aggregates to include

## Notes

- Aggregates reduce need for follow-up API calls
- Keep aggregates lightweight and cheap to compute
