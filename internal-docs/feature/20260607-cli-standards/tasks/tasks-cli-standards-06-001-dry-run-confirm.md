---
story_id: "06-001"
story_title: "Implement dry-run mode and confirmation prompts"
story_name: "dry-run-confirm"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 6
parallel_id: 1
branch: "feature/current/cli-standards/story-06-001-dry-run-confirm"
status: "done"
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

Implement dry-run mode for applicable commands and confirmation prompts for destructive operations. Dry-run should show exactly what would be done without making changes. Confirmation prompts should require user approval for destructive operations with a `--force` flag to bypass.

## Sub-Tasks

- [x] Add `--dry-run` flag to applicable commands:
  - migrate command (if kept)
  - Any future config validation commands
  - Any future database operations
- [x] Implement dry-run logic:
  - Log all actions that would be taken
  - Do not execute any changes
  - Display summary of what would be done
- [x] Add confirmation prompts for destructive operations:
  - `--uninstall` command
  - Config file overwrites
  - Any future database deletion commands
- [x] Implement `--force` flag to bypass confirmation prompts:
  - Add to all commands with confirmation prompts
  - Skip prompts when --force is set
- [x] Implement prompt format:
  - "Are you sure you want to [action]? [y/N]"
  - Default to No (require explicit Yes)
  - Case-insensitive input
- [x] Ensure prompts respect `--quiet` flag (auto-confirm No in quiet mode)
- [x] Add tests for dry-run mode
- [x] Add tests for confirmation prompts
- [x] Add tests for --force flag

## Relevant Files

- `src/main.rs` — Add --dry-run, --force flags and confirmation logic
- `src/cli/confirm.rs` — NEW FILE for confirmation prompt utilities
- `src/cli/mod.rs` — Export confirm module
- `tests/cli_tests.rs` — Add dry-run and confirmation tests

## Acceptance Criteria

- [x] `--dry-run` flag shows what would be done without making changes
- [x] Dry-run output clearly indicates no changes were made
- [x] Destructive operations require confirmation prompt
- [x] Confirmation prompt has clear format with default No
- [x] `--force` flag bypasses confirmation prompts
- [x] Prompts respect `--quiet` flag (auto-confirm No)
- [x] All dry-run functionality has tests
- [x] All confirmation functionality has tests
- [x] --force flag has tests

## Test Plan

- Unit: Tests for dry-run logic
- Unit: Tests for confirmation prompt logic
- Integration: Test dry-run with applicable commands
- Integration: Test confirmation prompts with destructive operations
- Manual: Test --force flag bypasses prompts

## Observability

- Log dry-run actions at INFO level
- Log confirmation responses at DEBUG level

## Compliance

- Confirmation prompts must default to safe choice (No)
- --quiet mode must not auto-confirm Yes (auto-confirm No instead)

## Risks & Mitigations

- Risk: Users might accidentally skip confirmation with --force
  - Mitigation: Document --force flag clearly
  - Mitigation: Use --force only when explicitly set
- Risk: Dry-run might not accurately reflect what would happen
  - Mitigation: Use same logic path as real execution
  - Mitigation: Clearly mark dry-run output

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- Dry-run mode is implemented for applicable commands
- Confirmation prompts work for destructive operations
- --force flag bypasses prompts correctly
- All functionality has tests
- Manual testing confirms behavior

## Commit Conventions

- `feat(cli): add dry-run mode`
- `feat(cli): add confirmation prompts for destructive operations`
- `feat(cli): add --force flag to bypass prompts`
- `test(cli): add dry-run and confirmation tests`