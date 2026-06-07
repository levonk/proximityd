---
story_id: "06-003"
story_title: "Implement file reference formatting and terminal size awareness"
story_name: "file-ref-terminal"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 6
parallel_id: 3
branch: "feature/current/cli-standards/story-06-003-file-ref-terminal"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/main.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-07-12"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement VSCode-compatible file reference formatting for all file references with line numbers. Add terminal size detection and responsive output formatting based on terminal width. Handle terminal resize events where possible.

## Sub-Tasks

- [ ] Implement file reference formatting function:
  - Format: `file:///absolute/path/to/file:line:column`
  - Alternative format: `file:line:column`
  - Use VSCode-compatible format for modern terminals
- [ ] Update all error messages to use file reference formatting:
  - Config file errors
  - Validation errors with file locations
  - Test failure messages
- [ ] Update log output to use file reference formatting where applicable
- [ ] Implement terminal size detection:
  - Detect terminal width on startup
  - Detect terminal height on startup
  - Handle detection errors gracefully
- [ ] Implement responsive output formatting:
  - Adjust table widths based on terminal width
  - Wrap long lines at terminal width
  - Truncate with ellipsis if needed
- [ ] Implement terminal resize handling (where possible):
  - Detect SIGWINCH signal
  - Reformat output on resize
  - Handle resize in TUI mode
- [ ] Ensure file references work on all platforms (Windows, Linux, macOS)
- [ ] Add tests for file reference formatting
- [ ] Add tests for terminal size detection
- [ ] Add tests for responsive formatting

## Relevant Files

- `src/cli/format.rs` — NEW FILE for formatting utilities
- `src/cli/mod.rs` — Export format module
- `src/main.rs` — Use file reference formatting in errors
- `src/error.rs` — Use file reference formatting in error types
- `tests/cli_tests.rs` — Add formatting tests

## Acceptance Criteria

- [ ] File references use VSCode-compatible format
- [ ] Error messages include file references with line numbers
- [ ] Log output uses file references where applicable
- [ ] Terminal size is detected on startup
- [ ] Output formatting adjusts to terminal width
- [ ] Long lines wrap or truncate at terminal width
- [ ] Terminal resize is detected and handled where possible
- [ ] File references work on Windows, Linux, and macOS
- [ ] All file reference formatting has tests
- [ ] All terminal size detection has tests
- [ ] All responsive formatting has tests

## Test Plan

- Unit: Tests for file reference formatting
- Unit: Tests for terminal size detection
- Integration: Test responsive formatting with different terminal widths
- Manual: Test file references in VSCode (verify clickable links)
- Manual: Test on different platforms

## Observability

- Log terminal size detection at DEBUG level
- Log resize events at DEBUG level

## Compliance

- File references must use platform-appropriate path separators
- Terminal size detection must handle errors gracefully

## Risks & Mitigations

- Risk: Terminal size detection might not work on all systems
  - Mitigation: Fall back to default width (80 columns)
  - Mitigation: Handle detection errors gracefully
- Risk: File references might not be clickable on all terminals
  - Mitigation: Use standard format that most terminals support
  - Mitigation: Provide alternative format if needed

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- File references use VSCode-compatible format
- Terminal size detection works
- Output formatting is responsive to terminal width
- All functionality has tests
- Manual testing confirms behavior

## Commit Conventions

- `feat(cli): add VSCode-compatible file reference formatting`
- `feat(cli): add terminal size detection`
- `feat(cli): add responsive output formatting`
- `test(cli): add formatting tests`