---
story_id: "01-001"
story_title: "Mode Selection Implementation"
story_name: "mode-selection"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 1
parallel_id: 1
branch: "feature/current/cli-axi/story-01-001-mode-selection"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["cli.rs", "config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement agent mode as default behavior with auto-detection based on TTY presence and agent session environment variables. Add human mode triggers via CLI flags and config file settings. Establish mode precedence chain for consistent behavior across different invocation contexts.

## Sub-Tasks

- [x] Add mode enum to config structure (Agent, Human, Auto)
- [x] Implement agent session detection logic (check CLAUDE_SESSION, CODEX_SESSION env vars)
- [x] Implement TTY detection logic for auto mode selection
- [x] Add --human and --interactive CLI flags to force human mode
- [x] Add PROXIMITYD_MODE environment variable support
- [x] Add mode setting to config file with precedence logic
- [x] Implement mode precedence chain (CLI flags > env var > config > auto-detection)
- [x] Add mode detection tests for various scenarios
- [x] Update CLI help text to document mode selection
- [x] Test mode selection with different environments (TTY, non-TTY, agent sessions)

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/config/app.rs` — Add mode enum and config field
- `src/config/loader.rs` — Load mode from config file
- `src/cli.rs` — Add mode flags and detection logic
- `src/main.rs` — Apply mode selection at startup
- `tests/cli_test.rs` — Test mode selection logic

## Acceptance Criteria

- [x] Agent mode is default when no explicit mode selection
- [x] Auto-detection works correctly (TTY + agent session detection)
- [x] --human flag forces human mode
- [x] --interactive flag forces human mode
- [x] PROXIMITYD_MODE environment variable overrides config
- [x] Config file mode setting works correctly
- [x] Mode precedence chain is respected
- [x] All mode selection scenarios have tests

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test cli::mode`
- Integration: Test with different TTY states and environment variables
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Add logging for mode selection decisions
- Log detected environment (TTY, agent session)

## Compliance

- Follow ADR-20260607001 v4.0.0 agent mode standards
- Ensure backward compatibility with existing behavior

## Risks & Mitigations

- Risk: Auto-detection may incorrectly identify agent sessions — Mitigation: Use multiple detection methods and allow manual override
- Risk: Breaking existing user workflows — Mitigation: Maintain human mode as default for TTY sessions

## Dependencies

- None

## Notes

- Mode selection is foundational for all other AXI features
- Must work correctly in CI/CD environments (non-TTY)
