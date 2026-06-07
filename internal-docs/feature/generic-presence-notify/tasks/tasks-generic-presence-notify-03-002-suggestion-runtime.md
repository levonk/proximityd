---
story_id: "03-002"
story_title: "Suggestion Runtime Toggle"
story_name: "suggestion-runtime"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 3
parallel_id: 2
branch: "feature/current/generic-presence-notify/story-03-002-suggestion-runtime"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["src/discovery/"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "backend"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Wire discovery suggestions into runtime. When `config.toml` `[discovery] use_suggestions = true`, high-confidence suggestions above `auto_promote_threshold` (default 0.95) are used as runtime mappings with a logged warning.

## Sub-Tasks

- [x] Add `[discovery]` section to `AppConfig`: `use_suggestions` (default false), `auto_promote_threshold` (default 0.95)
- [x] Create `src/discovery/runtime.rs` — `SuggestionRuntime` that loads `suggestions.toml` and resolves identifiers to parties at runtime
- [x] Integrate into `DetectionEngine`: check suggestion mappings when normal config lookup fails
- [x] Log warning whenever a suggestion-based mapping is used
- [x] Add `src/discovery/runtime_tests.rs` — unit tests for promote/demote logic

## Relevant Files

- `src/discovery/runtime.rs`
- `src/discovery/runtime_tests.rs`
- `src/config/app.rs`
- `src/detection/engine.rs`

## Acceptance Criteria

- [x] When `use_suggestions = false`, suggestions are never used at runtime
- [x] When `use_suggestions = true`, suggestions above threshold are used with warning log
- [x] Unit tests pass: `cargo test discovery_runtime`

## Test Plan

- Unit: `cargo test discovery_runtime`

## Risks & Mitigations

- Risk: Wrong suggestion causes false presence — Mitigation: default to false; require explicit opt-in

## Dependencies & Sequencing

- Depends on: 03-001
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(discovery): add suggestion runtime toggle`

## Changelog

- 2026-06-03: initialized story file
