---
story_id: "02-001"
story_title: "Content Truncation"
story_name: "content-truncation"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 2
parallel_id: 1
branch: "feature/current/cli-axi/story-02-001-content-truncation"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["output.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement content truncation for large text fields (500-1500 chars default). Add --full flag to disable truncation. Include truncation metadata and help suggestions. Apply to both agent and human modes for consistent behavior.

## Sub-Tasks

- [x] Implement truncation logic with configurable limit (default 1000 chars)
- [x] Add --full CLI flag to disable truncation
- [x] Include truncation metadata in output (total size, truncated indicator)
- [x] Add help suggestions for --full flag when content is truncated
- [x] Apply truncation to device names, identifier values, signal logs
- [x] Add truncation tests for various field types
- [x] Update CLI help text to document truncation behavior
- [x] Add config option for default truncation limit
- [x] Test truncation with different content sizes
- [x] Ensure truncation works in both agent and human modes

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/truncation.rs` (new) — Truncation logic
- `src/output/mod.rs` — Integrate truncation into output pipeline
- `src/config/app.rs` — Add truncation limit config option
- `tests/truncation_test.rs` (new) — Truncation tests

## Acceptance Criteria

- [x] Large text fields are truncated by default (1000 chars)
- [x] --full flag disables truncation
- [x] Truncation metadata shows total size
- [x] Help suggestions appear when content is truncated
- [x] Truncation applies to all large text fields
- [x] Config option sets default truncation limit
- [x] Truncation works in both agent and human modes

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test output::truncation`
- Integration: Test truncation with various content sizes
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log truncation decisions
- Track --full flag usage

## Compliance

- Follow AXI truncation requirements
- Ensure backward compatibility with --full flag

## Risks & Mitigations

- Risk: Truncation hides important information — Mitigation: Always show truncated preview with metadata
- Risk: Config complexity — Mitigation: Keep config simple with sensible defaults

## Dependencies

- 01-003 (Minimal Default Schemas) — Schemas define which fields to truncate

## Notes

- Truncation is critical for token efficiency
- Balance between preview and completeness
