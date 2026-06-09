---
story_id: "01-002"
story_title: "TOON Format Implementation"
story_name: "toon-format"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 1
parallel_id: 2
branch: "feature/current/cli-axi/story-01-002-toon-format"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["output.rs", "toon.rs (new)"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement TOON (Token-Oriented Object Notation) format for ~40% token savings over JSON. Add --toon flag and --format flag for explicit format selection. Default to TOON in agent mode, JSON/human in human mode. Implement TOON encoder/decoder following the specification.

## Sub-Tasks

- [x] Create new toon.rs module with TOON encoder/decoder
- [x] Implement TOON format parser following spec (https://toonformat.dev/reference/spec.html)
- [x] Add --toon CLI flag for TOON output
- [x] Add --format=toon|json|human CLI flag
- [x] Integrate TOON output into existing command outputs
- [x] Default to TOON format in agent mode
- [x] Default to JSON/human format in human mode
- [x] Add TOON encoding tests with sample data
- [x] Add TOON decoding tests for round-trip validation
- [x] Update CLI help text to document format options
- [x] Benchmark TOON vs JSON token usage

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/toon.rs` (new) — TOON encoder/decoder implementation
- `src/output/mod.rs` — Output format abstraction
- `src/cli.rs` — Add format flags
- `tests/toon_test.rs` (new) — TOON format tests

## Acceptance Criteria

- [x] TOON format is correctly implemented per specification
- [x] --toon flag produces TOON output
- [x] --format flag allows explicit format selection
- [x] Agent mode defaults to TOON
- [x] Human mode defaults to JSON/human
- [x] TOON achieves ~40% token savings over JSON
- [x] TOON encoder/decoder round-trip correctly
- [x] All commands support TOON output

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test output::toon`
- Integration: Test TOON output for all commands
- Benchmark: Compare token counts between TOON and JSON
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log format selection decisions
- Track TOON vs JSON usage metrics

## Compliance

- Follow TOON format specification exactly
- Ensure backward compatibility with JSON output

## Risks & Mitigations

- Risk: TOON spec changes — Mitigation: Pin to specific version in dependencies
- Risk: Performance overhead — Mitigation: Benchmark and optimize encoder/decoder

## Dependencies

- None

## Notes

- TOON is critical for agent mode token efficiency
- Internal logic remains in JSON, convert at output boundary
