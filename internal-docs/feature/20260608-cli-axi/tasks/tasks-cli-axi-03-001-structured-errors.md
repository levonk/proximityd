---
story_id: "03-001"
story_title: "Structured Errors & Exit Codes"
story_name: "structured-errors"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 3
parallel_id: 1
branch: "feature/current/cli-axi/story-03-001-structured-errors"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002"]
parallel_safe: true
modules: ["error.rs", "main.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement idempotent mutations, structured errors on stdout, no interactive prompts, and proper output channels. Ensure errors go to stdout in structured format with actionable suggestions. Reserve non-zero exit codes for situations where agent intent cannot be satisfied.

## Sub-Tasks

- [ ] Implement idempotent mutation logic (no error when state already exists)
- [ ] Add structured error formatting on stdout
- [ ] Include actionable suggestions in error messages
- [ ] Validate required flags before calling dependencies
- [ ] Translate errors to extract actionable meaning
- [ ] Never leak dependency names in error messages
- [ ] Suppress interactive prompts in agent mode
- [ ] Ensure every operation is completable with flags alone
- [ ] Implement proper output channels (stdout: data/errors, stderr: logs)
- [ ] Add structured error tests
- [ ] Update exit code logic (0: success/no-op, 1: error, 2: usage)

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/error.rs` — Structured error types and formatting
- `src/main.rs` — Exit code logic and output channel separation
- `src/cli.rs` — Flag validation and prompt suppression
- `tests/error_test.rs` (new) — Structured error tests

## Acceptance Criteria

- [ ] Idempotent operations don't error when state already exists
- [ ] Errors go to stdout in structured format
- [ ] Error messages include actionable suggestions
- [ ] Required flags are validated before dependency calls
- [ ] Dependency names are never leaked in errors
- [ ] Interactive prompts are suppressed in agent mode
- [ ] All operations are completable with flags alone
- [ ] Exit codes follow standard (0: success, 1: error, 2: usage)
- [ ] Output channels are properly separated

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test error`
- Integration: Test error scenarios for all commands
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log error occurrences and types
- Track exit code distribution

## Compliance

- Follow AXI structured error requirements
- Ensure backward compatibility with human mode prompts

## Risks & Mitigations

- Risk: Breaking existing error handling — Mitigation: Maintain error message clarity while adding structure
- Risk: Complex error translation logic — Mitigation: Keep translation simple and well-documented

## Dependencies

- 01-001 (Mode Selection) — Agent mode detection for prompt suppression
- 01-002 (TOON Format) — Structured errors in TOON format

## Notes

- Structured errors are critical for agent understanding
- Idempotency reduces agent retry complexity
