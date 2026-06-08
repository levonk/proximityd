---
story_id: "06-004"
story_title: "Add resource limits and input globbing support"
story_name: "resource-limits-globbing"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 6
parallel_id: 4
branch: "feature/current/cli-standards/story-06-004-resource-limits-globbing"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/main.rs"]
priority: "COULD"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-07-12"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement resource limits for memory and CPU-intensive operations, and add support for recursive globbing patterns with stdin input. Resource limits help prevent denial-of-service issues. Globbing support allows flexible file input patterns.

## Sub-Tasks

- [x] Add `--max-memory` flag to applicable commands:
  - discovery command (memory-intensive analysis)
  - export command (large signal log processing)
- [x] Implement memory limit enforcement:
  - Track memory usage during operations
  - Abort if limit exceeded
  - Log limit violations at ERROR level
- [x] Add `--max-cpu` flag to applicable commands:
  - discovery command (CPU-intensive correlation)
  - export command (CPU-intensive processing)
- [x] Implement CPU limit enforcement:
  - Limit parallelism based on CPU limit
  - Use platform-appropriate mechanisms (taskset on Linux, etc.)
  - Log limit application at INFO level
- [x] Implement recursive globbing support:
  - Support `**/*` pattern for recursive file matching
  - Support `*` pattern for single-level matching
  - Use glob crate for pattern matching
- [x] Implement stdin input support:
  - Accept `-` as file argument for stdin
  - Accept piped input
  - Process stdin same as file input where applicable
- [x] Apply globbing to applicable commands:
  - export command (for file path arguments if added)
  - Any future file-processing commands
- [x] Add tests for resource limit enforcement
- [x] Add tests for globbing patterns
- [x] Add tests for stdin input

## Relevant Files

- `src/main.rs` — Add --max-memory, --max-cpu flags to Discover and Export commands
- `src/cli/limits.rs` — NEW FILE for resource limit utilities (MemoryLimit, CpuLimit)
- `src/cli/glob.rs` — NEW FILE for globbing utilities (expand_glob, expand_inputs, is_glob_pattern)
- `src/cli/mod.rs` — Export limits and glob modules
- `Cargo.toml` — Add num_cpus dependency

## Acceptance Criteria

- [x] `--max-memory` flag limits memory usage
- [x] Operations abort when memory limit exceeded
- [x] `--max-cpu` flag limits CPU usage
- [x] CPU limit is enforced via platform-appropriate mechanisms
- [x] `**/*` pattern matches files recursively
- [x] `*` pattern matches files in current directory
- [x] `-` argument accepts stdin input
- [x] Piped input is processed correctly
- [x] All resource limit functionality has tests
- [x] All globbing functionality has tests
- [x] All stdin functionality has tests

## Test Plan

- Unit: Tests for memory limit enforcement
- Unit: Tests for CPU limit enforcement
- Unit: Tests for globbing patterns
- Integration: Test globbing with actual file patterns
- Integration: Test stdin input with pipes
- Manual: Test resource limits with intensive operations

## Observability

- Log resource limit application at INFO level
- Log limit violations at ERROR level
- Log matched files from globbing at DEBUG level

## Compliance

- Resource limits must be enforced to prevent abuse
- Globbing must not expose files outside intended scope
- Stdin input must be handled securely

## Risks & Mitigations

- Risk: Resource limit enforcement might not work on all platforms
  - Mitigation: Use platform-appropriate mechanisms
  - Mitigation: Fall back to best-effort enforcement
- Risk: Globbing might match too many files
  - Mitigation: Add limit on number of matched files
  - Mitigation: Provide warning for large matches

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- Resource limits are implemented and enforced
- Globbing support works for file patterns
- Stdin input is supported
- All functionality has tests
- Manual testing confirms behavior

## Commit Conventions

- `feat(cli): add resource limits for memory and CPU`
- `feat(cli): add recursive globbing support`
- `feat(cli): add stdin input support`
- `test(cli): add resource limit and globbing tests`