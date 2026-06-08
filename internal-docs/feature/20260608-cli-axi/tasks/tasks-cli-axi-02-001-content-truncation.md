---
story_id: "02-001"
story_title: "Content Truncation"
story_name: "content-truncation"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 2
parallel_id: 1
branch: "feature/current/cli-axi/story-02-001-content-truncation"
status: "todo"
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

- [ ] Implement truncation logic with configurable limit (default 1000 chars)
- [ ] Add --full CLI flag to disable truncation
- [ ] Include truncation metadata in output (total size, truncated indicator)
- [ ] Add help suggestions for --full flag when content is truncated
- [ ] Apply truncation to device names, identifier values, signal logs
- [ ] Add truncation tests for various field types
- [ ] Update CLI help text to document truncation behavior
- [ ] Add config option for default truncation limit
- [ ] Test truncation with different content sizes
- [ ] Ensure truncation works in both agent and human modes

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/truncation.rs` (new) — Truncation logic
- `src/output/mod.rs` — Integrate truncation into output pipeline
- `src/config/app.rs` — Add truncation limit config option
- `tests/truncation_test.rs` (new) — Truncation tests

## Acceptance Criteria

- [ ] Large text fields are truncated by default (1000 chars)
- [ ] --full flag disables truncation
- [ ] Truncation metadata shows total size
- [ ] Help suggestions appear when content is truncated
- [ ] Truncation applies to all large text fields
- [ ] Config option sets default truncation limit
- [ ] Truncation works in both agent and human modes

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
