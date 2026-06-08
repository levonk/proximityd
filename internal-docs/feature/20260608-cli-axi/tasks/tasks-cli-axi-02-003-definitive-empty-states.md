---
story_id: "02-003"
story_title: "Definitive Empty States"
story_name: "definitive-empty-states"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 2
parallel_id: 3
branch: "feature/current/cli-axi/story-02-003-definitive-empty-states"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["cli.rs", "output.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement definitive empty states that explicitly state when a query has no results. Include context (filter criteria, scope) and ensure exit code 0 for successful empty queries. Format empty states consistently across all commands.

## Sub-Tasks

- [ ] Detect empty result sets across all commands (parties, devices, signal logs)
- [ ] Format empty states with explicit "0 results" message
- [ ] Include context in empty state messages (filter criteria, scope)
- [ ] Ensure exit code 0 for successful empty queries
- [ ] Apply consistent empty state formatting across commands
- [ ] Add empty state tests for all commands
- [ ] Update parties list empty state format
- [ ] Update devices list empty state format
- [ ] Update status command empty state format
- [ ] Update CLI help text to document empty state behavior

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/empty.rs` (new) — Empty state formatting logic
- `src/cli/parties.rs` — Apply empty state formatting
- `src/cli/devices.rs` — Apply empty state formatting
- `src/cli/status.rs` — Apply empty state formatting
- `tests/empty_test.rs` (new) — Empty state tests

## Acceptance Criteria

- [ ] Empty states explicitly state "0 results"
- [ ] Empty states include context (filter criteria, scope)
- [ ] Exit code 0 for successful empty queries
- [ ] Empty state formatting is consistent
- [ ] All commands have proper empty states
- [ ] Empty states work with both TOON and JSON

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test output::empty`
- Integration: Test empty states for all commands
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log empty query results
- Track empty state patterns

## Compliance

- Follow AXI definitive empty state requirements
- Ensure agents can distinguish empty results from errors

## Risks & Mitigations

- Risk: Confusion between empty results and errors — Mitigation: Use clear formatting and exit codes
- Risk: Inconsistent empty state messages — Mitigation: Use centralized formatting logic

## Dependencies

- 01-003 (Minimal Default Schemas) — Schemas define empty state structure

## Notes

- Empty states are critical for agent understanding
- Clear distinction between "no results" and "error"
