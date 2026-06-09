---
story_id: "07-001"
story_title: "Add comprehensive tests for all new CLI standards"
story_name: "comprehensive-tests"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 7
parallel_id: 1
branch: "feature/current/cli-standards/story-07-001-comprehensive-tests"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["02-001", "02-002", "02-003", "03-001", "04-001", "04-002", "04-003", "05-001", "05-002", "06-001", "06-002", "06-003", "06-004"]
parallel_safe: false
modules: ["tests/"]
priority: "MUST"
risk_level: "medium"
tags: ["test", "quality"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Add comprehensive test coverage for all new CLI standards functionality implemented in previous stories. This includes unit tests for new modules, integration tests for CLI commands, and end-to-end tests for user workflows. Ensure code coverage exceeds 80% for new functionality.

## Sub-Tasks

- [x] Add unit tests for install/uninstall functionality (story 02-001)
- [x] Add unit tests for shell completion generation (story 02-002)
- [x] Add unit tests for config initialization (story 02-003)
- [x] Add unit tests for TUI framework (story 03-001) with mocked terminal
- [x] Add unit tests for TUI config editors (story 04-001)
- [x] Add unit tests for TUI party management (story 04-002)
- [x] Add unit tests for TUI notifier testing (story 04-003)
- [x] Add unit tests for man page generation (story 05-001)
- [x] Add unit tests for pager integration (story 05-002)
- [x] Add unit tests for dry-run mode (story 06-001)
- [x] Add unit tests for confirmation prompts (story 06-001)
- [x] Add unit tests for progress indicators (story 06-002)
- [x] Add unit tests for exit codes (story 06-002)
- [x] Add unit tests for file reference formatting (story 06-003)
- [x] Add unit tests for terminal size detection (story 06-003)
- [x] Add unit tests for resource limits (story 06-004)
- [x] Add unit tests for globbing patterns (story 06-004)
- [x] Add integration tests for install/uninstall commands
- [x] Add integration tests for completion command
- [x] Add integration tests for config initialization
- [x] Add integration tests for TUI launch and navigation
- [x] Add integration tests for man command and --man flag
- [x] Add integration tests for pager integration
- [x] Add integration tests for dry-run mode
- [x] Add integration tests for confirmation prompts
- [x] Add integration tests for --force flag
- [x] Add integration tests for exit codes across error scenarios
- [x] Run coverage analysis and verify >80% coverage for new code
- [x] Fix any coverage gaps
- [x] Run full test suite and ensure all tests pass
- [x] Run lint checks and ensure no warnings

## Relevant Files

- `tests/cli_tests.rs` — Add integration tests
- `src/cli/install.test.rs` — NEW FILE for install tests
- `src/cli/completion.test.rs` — NEW FILE for completion tests
- `src/config/loader.test.rs` — Add initialization tests
- `tests/tui_test.rs` — Add TUI tests
- `src/cli/man.test.rs` — NEW FILE for man page tests
- `src/cli/pager.test.rs` — NEW FILE for pager tests
- `src/cli/confirm.test.rs` — NEW FILE for confirmation tests
- `src/cli/progress.test.rs` — NEW FILE for progress tests
- `src/cli/format.test.rs` — NEW FILE for formatting tests
- `src/cli/limits.test.rs` — NEW FILE for resource limit tests
- `src/cli/glob.test.rs` — NEW FILE for globbing tests

## Acceptance Criteria

- [x] All new modules have unit tests
- [x] All new CLI commands have integration tests
- [x] Code coverage for new functionality exceeds 80%
- [x] All tests pass (0 failures)
- [x] No lint warnings
- [x] Test suite completes in reasonable time (<5 minutes)
- [x] Tests are deterministic (no flaky tests)

## Test Plan

- Unit: Run `devbox run just test-internal` for unit tests
- Integration: Run `devbox run just test-internal` for all tests
- Coverage: Use tarpaulin or similar tool for coverage analysis
- Lint: Run `devbox run just lint-internal`
- Types: Run `devbox run just typecheck-internal`

## Observability

- Test results are logged
- Coverage metrics are recorded

## Compliance

- Tests must not require external services
- Tests must be deterministic
- Tests must not leave artifacts

## Risks & Mitigations

- Risk: TUI tests might be flaky due to terminal emulation
  - Mitigation: Use mocked terminal for unit tests
  - Mitigation: Keep integration TUI tests minimal
- Risk: Coverage target might be difficult to reach
  - Mitigation: Focus on critical paths first
  - Mitigation: Accept slightly lower coverage for complex UI code

## Dependencies & Sequencing

- Depends on: All implementation stories (02-001 through 06-004)
- Unblocks: Story 07-002 (documentation)

## Definition of Done

- All new functionality has comprehensive tests
- Code coverage exceeds 80%
- All tests pass
- No lint warnings
- Test suite is fast and deterministic

## Commit Conventions

- `test(cli): add comprehensive tests for install/uninstall`
- `test(cli): add comprehensive tests for shell completion`
- `test(cli): add comprehensive tests for config initialization`
- `test(tui): add comprehensive tests for TUI framework`
- `test(tui): add comprehensive tests for TUI features`
- `test(cli): add comprehensive tests for man pages and pager`
- `test(cli): add comprehensive tests for dry-run and confirmation`
- `test(cli): add comprehensive tests for progress and exit codes`
- `test(cli): add comprehensive tests for formatting and limits`
- `test(ci): verify code coverage exceeds 80%`