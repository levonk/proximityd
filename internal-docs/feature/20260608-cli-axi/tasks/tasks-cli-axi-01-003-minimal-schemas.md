---
story_id: "01-003"
story_title: "Minimal Default Schemas"
story_name: "minimal-schemas"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 1
parallel_id: 3
branch: "feature/current/cli-axi/story-01-003-minimal-schemas"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
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

Implement minimal default schemas (3-4 fields) for command outputs to reduce token consumption. Add --fields flag for explicit field selection. Move long-form content to detail views. Apply schemas to both TOON and JSON output formats.

## Sub-Tasks

- [x] Define default output schemas for each command (parties, devices, status, export, discover)
- [x] Implement field selection logic with --fields flag
- [x] Add field validation against available fields
- [x] Update parties list command to use minimal schema (name, device_count, location)
- [x] Update devices list command to use minimal schema (name, identifier_count, status)
- [x] Update status command to use minimal schema (daemon_status, active_parties)
- [x] Move long-form content to detail views
- [x] Apply schema logic to both TOON and JSON outputs
- [x] Add field selection tests
- [x] Update CLI help text to document --fields flag

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/schema.rs` (new) — Schema definition and field selection
- `src/output/mod.rs` — Added schema module exports
- `src/main.rs` — Added --fields flag, Parties/Devices commands, schema integration
- `tests/schema_test.rs` (new) — Schema and field selection tests
- `tests/cli_tests.rs` — Updated status JSON test for new schema

## Acceptance Criteria

- [x] Default schemas have 3-4 fields maximum
- [x] --fields flag allows explicit field selection
- [x] Field names are validated against available fields
- [x] Long-form content is in detail views only
- [x] Schemas apply to both TOON and JSON
- [x] All commands use minimal schemas by default
- [x] Field selection works correctly

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test output::schema`
- Integration: Test field selection for all commands
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log field selection requests
- Track schema usage patterns

## Compliance

- Follow AXI minimal schema requirements
- Ensure backward compatibility with full output via --fields

## Risks & Mitigations

- Risk: Breaking existing scripts expecting full output — Mitigation: Maintain backward compatibility with --fields flag
- Risk: Schema complexity — Mitigation: Keep schema logic simple and well-documented

## Dependencies

- None

## Notes

- Minimal schemas are critical for agent mode token efficiency
- Balance between minimal output and usability
