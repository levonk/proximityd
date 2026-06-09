---
story_id: "04-001"
story_title: "Session Hook Infrastructure"
story_name: "session-hooks"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 4
parallel_id: 1
branch: "feature/current/cli-axi/story-04-001-session-hooks"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002"]
parallel_safe: true
modules: ["hooks.rs (new)", "config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement session hook infrastructure for ambient context injection. Add --session-context command for compact state output in TOON format. Add --install-agent-hooks command for hook registration. Support Claude Code, Codex, and future agent platforms.

## Sub-Tasks

- [x] Create hooks.rs module for session hook infrastructure
- [x] Implement --session-context command with TOON output
- [x] Make session-context output token-budget-aware
- [x] Implement directory-scoped context (current working directory)
- [x] Add --install-agent-hooks command
- [x] Implement hook registration for Claude Code (~/.claude/settings.json)
- [x] Implement hook registration for Codex (~/.codex/hooks.json)
- [x] Add idempotent hook installation (silent no-op if unchanged)
- [x] Use PATH-verified binary names with absolute path fallback
- [x] Implement session-end hook registration
- [x] Add session metadata caching for future context enrichment
- [x] Add session hook tests
- [x] Update CLI help text to document hook commands

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/hooks.rs` (new) — Session hook infrastructure
- `src/cli/hooks.rs` (new) — Hook CLI commands
- `src/config.rs` — Hook configuration
- `tests/hooks_test.rs` (new) — Hook tests

## Acceptance Criteria

- [x] --session-context outputs compact state in TOON format
- [x] Session-context is token-budget-aware
- [x] Context is directory-scoped
- [x] --install-agent-hooks registers hooks correctly
- [x] Hooks work for Claude Code and Codex
- [x] Hook installation is idempotent
- [x] Binary names are PATH-verified with fallback
- [x] Session-end hooks are registered
- [x] Session metadata is cached

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test hooks`
- Integration: Test hook installation and session-context output
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log hook installation events
- Track session-context usage

## Compliance

- Follow AXI session integration requirements
- Ensure hooks are portable across platforms

## Risks & Mitigations

- Risk: Hook file format changes — Mitigation: Use stable formats and version detection
- Risk: PATH resolution issues — Mitigation: Robust fallback to absolute paths

## Dependencies

- 01-001 (Mode Selection) — Agent mode detection for hook behavior
- 01-002 (TOON Format) — Session-context uses TOON output

## Notes

- Session hooks enable ambient context for agents
- Hook installation should be explicit opt-in only
