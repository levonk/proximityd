---
story_id: "01-003"
story_title: "Minimal Default Schemas"
story_name: "minimal-schemas"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 1
parallel_id: 3
branch: "feature/current/cli-axi/story-01-003-minimal-schemas"
status: "todo"
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

- [ ] Define default output schemas for each command (parties, devices, status, export, discover)
- [ ] Implement field selection logic with --fields flag
- [ ] Add field validation against available fields
- [ ] Update parties list command to use minimal schema (name, device_count, location)
- [ ] Update devices list command to use minimal schema (name, identifier_count, status)
- [ ] Update status command to use minimal schema (daemon_status, active_parties)
- [ ] Move long-form content to detail views
- [ ] Apply schema logic to both TOON and JSON outputs
- [ ] Add field selection tests
- [ ] Update CLI help text to document --fields flag

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/schema.rs` (new) — Schema definition and field selection
- `src/cli/parties.rs` — Apply minimal schema to parties commands
- `src/cli/devices.rs` — Apply minimal schema to devices commands
- `src/cli/status.rs` — Apply minimal schema to status command
- `tests/schema_test.rs` (new) — Schema and field selection tests

## Acceptance Criteria

- [ ] Default schemas have 3-4 fields maximum
- [ ] --fields flag allows explicit field selection
- [ ] Field names are validated against available fields
- [ ] Long-form content is in detail views only
- [ ] Schemas apply to both TOON and JSON
- [ ] All commands use minimal schemas by default
- [ ] Field selection works correctly

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
