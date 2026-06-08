---
story_id: "04-001"
story_title: "Session Hook Infrastructure"
story_name: "session-hooks"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 4
parallel_id: 1
branch: "feature/current/cli-axi/story-04-001-session-hooks"
status: "todo"
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

- [ ] Create hooks.rs module for session hook infrastructure
- [ ] Implement --session-context command with TOON output
- [ ] Make session-context output token-budget-aware
- [ ] Implement directory-scoped context (current working directory)
- [ ] Add --install-agent-hooks command
- [ ] Implement hook registration for Claude Code (~/.claude/settings.json)
- [ ] Implement hook registration for Codex (~/.codex/hooks.json)
- [ ] Add idempotent hook installation (silent no-op if unchanged)
- [ ] Use PATH-verified binary names with absolute path fallback
- [ ] Implement session-end hook registration
- [ ] Add session metadata caching for future context enrichment
- [ ] Add session hook tests
- [ ] Update CLI help text to document hook commands

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/hooks.rs` (new) — Session hook infrastructure
- `src/cli/hooks.rs` (new) — Hook CLI commands
- `src/config.rs` — Hook configuration
- `tests/hooks_test.rs` (new) — Hook tests

## Acceptance Criteria

- [ ] --session-context outputs compact state in TOON format
- [ ] Session-context is token-budget-aware
- [ ] Context is directory-scoped
- [ ] --install-agent-hooks registers hooks correctly
- [ ] Hooks work for Claude Code and Codex
- [ ] Hook installation is idempotent
- [ ] Binary names are PATH-verified with fallback
- [ ] Session-end hooks are registered
- [ ] Session metadata is cached

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
