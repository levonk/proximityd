---
story_id: "06-002"
story_title: "Add progress indicators and standard exit codes"
story_name: "progress-exit-codes"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 6
parallel_id: 2
branch: "feature/current/cli-standards/story-06-002-progress-exit-codes"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/main.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-07-12"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement progress indicators for long-running operations and ensure all exit codes follow standard conventions. Progress indicators must respect the `--quiet` flag. Exit codes must use standard values: 0 for success, 1 for generic error, 2 for usage error, and specific codes for different error types.

## Sub-Tasks

- [ ] Add progress indicator library dependency to Cargo.toml (indicatif)
- [ ] Implement progress bar for daemon startup
- [ ] Implement progress bar for signal log export
- [ ] Implement progress bar for discovery analysis
- [ ] Implement progress spinner for other long operations
- [ ] Ensure progress indicators respect `--quiet` flag (no progress in quiet mode)
- [ ] Define standard exit codes:
  - 0: success
  - 1: generic error
  - 2: usage error
  - 3: network error
  - 4: validation error
  - 5: file not found
  - 6: permission denied
  - 130: SIGINT (Ctrl+C)
- [ ] Audit all existing exit codes and update to standard values
- [ ] Update error handling to use appropriate exit codes
- [ ] Add exit code documentation to help text
- [ ] Add tests for progress indicators
- [ ] Add tests for exit codes

## Relevant Files

- `Cargo.toml` — Add indicatif dependency
- `src/main.rs` — Add progress indicators and update exit codes
- `src/cli/progress.rs` — NEW FILE for progress utilities
- `src/cli/mod.rs` — Export progress module
- `src/error.rs` — Define exit code constants
- `tests/cli_tests.rs` — Add progress and exit code tests

## Acceptance Criteria

- [ ] Progress bars show for daemon startup
- [ ] Progress bars show for signal log export
- [ ] Progress bars show for discovery analysis
- [ ] Progress indicators do not show in `--quiet` mode
- [ ] Exit code 0 used for success
- [ ] Exit code 1 used for generic errors
- [ ] Exit code 2 used for usage errors
- [ ] Exit code 3 used for network errors
- [ ] Exit code 4 used for validation errors
- [ ] Exit code 5 used for file not found
- [ ] Exit code 6 used for permission denied
- [ ] Exit code 130 used for SIGINT
- [ ] All progress functionality has tests
- [ ] All exit code behavior has tests

## Test Plan

- Unit: Tests for progress indicator logic
- Integration: Test progress indicators with long operations
- Integration: Test exit codes for various error scenarios
- Manual: Verify progress indicators work correctly
- Manual: Verify exit codes match standards

## Observability

- Progress indicators are visible to user
- Exit codes are documented

## Compliance

- Progress indicators must respect --quiet flag
- Exit codes must follow Unix conventions

## Risks & Mitigations

- Risk: Progress indicators might not work on all terminals
  - Mitigation: Gracefully fall back to no progress if terminal not supported
- Risk: Changing exit codes might break existing scripts
  - Mitigation: Document breaking change in release notes
  - Mitigation: Use semantic versioning (minor version for breaking change)

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- Progress indicators work for long operations
- Progress indicators respect --quiet flag
- All exit codes follow standard conventions
- Exit codes are documented
- All functionality has tests
- Manual testing confirms behavior

## Commit Conventions

- `feat(cli): add progress indicators for long operations`
- `refactor(cli): standardize exit codes`
- `test(cli): add progress and exit code tests`