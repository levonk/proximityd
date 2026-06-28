---
story_id: "05-001"
story_title: "Content-First No-Args"
story_name: "content-first"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 5
parallel_id: 1
branch: "feature/current/cli-axi/story-05-001-content-first"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-003", "02-002"]
parallel_safe: true
modules: ["cli.rs", "main.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement content-first no-args behavior that shows most relevant live state instead of usage manual. Redesign no-args invocation to show state summary. Move detailed help to --help flag. Show different content based on current directory/context.

## Sub-Tasks

- [x] Redesign no-args invocation to show state summary
- [x] Implement directory/context-aware content display
- [x] Show parties summary when in config directory
- [x] Show daemon status when daemon is running
- [x] Include contextual help suggestions in no-args output
- [x] Move detailed help to --help flag (unchanged)
- [x] Apply content-first to both agent and human modes
- [x] Add no-args tests for different contexts
- [x] Update CLI help text to document no-args behavior
- [x] Ensure no-args output is TOON-formatted in agent mode

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/main.rs` — No-args behavior logic
- `src/cli.rs` — Context detection and content display
- `tests/noargs_test.rs` (new) — No-args tests

## Acceptance Criteria

- [x] No-args shows live state, not usage manual
- [x] Content is context-aware (directory, daemon status)
- [x] Parties summary shown in config directory
- [x] Daemon status shown when running
- [x] Contextual help suggestions included
- [x] --help flag shows detailed help
- [x] Works in both agent and human modes
- [x] Agent mode uses TOON format

## Test Plan

- Unit: `devbox run -c ~/p/gh/levonk/proximityd -- cargo test noargs`
- Integration: Test no-args in different contexts
- Lint: `devbox run -c ~/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c ~/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log no-args invocations and context
- Track context detection accuracy

## Compliance

- Follow AXI content-first requirements
- Ensure backward compatibility with --help

## Risks & Mitigations

- Risk: Breaking existing user expectations — Mitigation: Keep --help unchanged and document new behavior
- Risk: Context detection complexity — Mitigation: Keep context logic simple and well-documented

## Dependencies

- 01-003 (Minimal Default Schemas) — No-args uses minimal schemas
- 02-002 (Pre-computed Aggregates) — No-args shows aggregate summaries

## Notes

- Content-first reduces agent API calls
- Context awareness makes output more relevant
