---
story_id: "05-002"
story_title: "Contextual Disclosure"
story_name: "contextual-disclosure"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 5
parallel_id: 2
branch: "feature/current/cli-axi/story-05-002-contextual-disclosure"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-003", "02-002"]
parallel_safe: true
modules: ["output.rs", "cli.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement contextual disclosure with next step suggestions. Include 2-4 relevant suggestions ranked by relevance. Make suggestions actionable (complete commands with flags). Format as structured help[] array in TOON output. Enable organic CLI surface area discovery.

## Sub-Tasks

- [x] Implement suggestion engine for each command
- [x] Generate contextual help based on current state and output
- [x] Format suggestions as structured help[] array in TOON
- [x] Include suggestions in all command outputs
- [x] Make suggestions context-aware (not generic)
- [x] Limit to 2-4 suggestions maximum
- [x] Rank suggestions by relevance
- [x] Ensure suggestions are complete commands with flags
- [x] Add suggestion tests for various contexts
- [~] Update CLI help text to document suggestion behavior

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/suggestions.rs` (new) — Suggestion engine
- `src/cli/parties.rs` — Party-specific suggestions
- `src/cli/devices.rs` — Device-specific suggestions
- `tests/suggestions_test.rs` (new) — Suggestion tests

## Acceptance Criteria

- [ ] Suggestions are included in all command outputs
- [ ] Suggestions are formatted as help[] array in TOON
- [ ] Suggestions are context-aware and relevant
- [ ] Suggestions are complete commands with flags
- [ ] 2-4 suggestions maximum per output
- [ ] Suggestions are ranked by relevance
- [ ] Suggestions enable organic CLI discovery

## Test Plan

- Unit: `devbox run -c ~/p/gh/levonk/proximityd -- cargo test suggestions`
- Integration: Test suggestions for all commands and contexts
- Lint: `devbox run -c ~/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c ~/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log suggestion generation events
- Track suggestion relevance and usage

## Compliance

- Follow AXI contextual disclosure requirements
- Ensure suggestions are helpful, not overwhelming

## Risks & Mitigations

- Risk: Irrelevant suggestions — Mitigation: Use context-aware ranking and limit to 2-4
- Risk: Complex suggestion logic — Mitigation: Keep suggestion rules simple and well-documented

## Dependencies

- 01-003 (Minimal Default Schemas) — Suggestions reference schema fields
- 02-002 (Pre-computed Aggregates) — Suggestions use aggregate data

## Notes

- Contextual disclosure enables organic CLI discovery
- Suggestions should be smart, not generic
