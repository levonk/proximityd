---
story_id: "06-001"
story_title: "Integration Testing"
story_name: "integration-testing"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 6
parallel_id: 1
branch: "feature/current/cli-axi/story-06-001-integration-testing"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "01-003", "02-001", "02-002", "02-003", "03-001", "04-001", "04-002", "05-001", "05-002"]
parallel_safe: false
modules: ["tests/"]
priority: "MUST"
risk_level: "medium"
tags: ["test", "quality"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement comprehensive integration testing for all agent mode features. Test mode selection, TOON format, minimal schemas, content truncation, pre-computed aggregates, empty states, structured errors, session hooks, agent skills, content-first no-args, and contextual disclosure. Ensure 90%+ test coverage.

## Sub-Tasks

- [ ] Create integration test suite for mode selection
- [ ] Create integration test suite for TOON format
- [ ] Create integration test suite for minimal schemas
- [ ] Create integration test suite for content truncation
- [ ] Create integration test suite for pre-computed aggregates
- [ ] Create integration test suite for empty states
- [ ] Create integration test suite for structured errors
- [ ] Create integration test suite for session hooks
- [ ] Create integration test suite for agent skills
- [ ] Create integration test suite for content-first no-args
- [ ] Create integration test suite for contextual disclosure
- [ ] Add end-to-end agent mode workflow tests
- [ ] Add cross-platform integration tests
- [ ] Measure and document test coverage
- [ ] Add performance benchmarks for agent mode features
- [ ] Add CI integration test execution

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `tests/integration/mode_selection_test.rs` (new) — Mode selection integration tests
- `tests/integration/toon_format_test.rs` (new) — TOON format integration tests
- `tests/integration/minimal_schemas_test.rs` (new) — Minimal schemas integration tests
- `tests/integration/content_truncation_test.rs` (new) — Content truncation integration tests
- `tests/integration/aggregates_test.rs` (new) — Aggregates integration tests
- `tests/integration/empty_states_test.rs` (new) — Empty states integration tests
- `tests/integration/structured_errors_test.rs` (new) — Structured errors integration tests
- `tests/integration/session_hooks_test.rs` (new) — Session hooks integration tests
- `tests/integration/agent_skills_test.rs` (new) — Agent skills integration tests
- `tests/integration/content_first_test.rs` (new) — Content-first integration tests
- `tests/integration/contextual_disclosure_test.rs` (new) — Contextual disclosure integration tests
- `tests/integration/agent_workflow_test.rs` (new) — End-to-end agent workflow tests

## Acceptance Criteria

- [ ] All agent mode features have integration tests
- [ ] Test coverage is 90%+ for agent mode code
- [ ] Integration tests pass consistently
- [ ] End-to-end agent workflows are tested
- [ ] Cross-platform tests work on Linux, macOS, Windows
- [ ] Performance benchmarks are established
- [ ] CI runs integration tests automatically

## Test Plan

- Integration: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test --test integration`
- Coverage: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test -- --nocapture`
- Performance: Run benchmarks for agent mode features
- Cross-platform: Test on Linux, macOS, Windows

## Observability

- Track test coverage metrics
- Monitor integration test pass rates
- Log performance benchmark results

## Compliance

- Ensure all AXI requirements are tested
- Follow testing best practices

## Risks & Mitigations

- Risk: Flaky integration tests — Mitigation: Use deterministic test data and proper cleanup
- Risk: Long test execution time — Mitigation: Parallelize tests where possible

## Dependencies

- All previous stories (01-001 through 05-002) — Integration tests require all features

## Notes

- Integration testing is critical for agent mode reliability
- Focus on real-world agent usage patterns
