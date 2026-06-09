---
story_id: "06-003"
story_title: "Implement file reference formatting and terminal size awareness"
story_name: "file-ref-terminal"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 6
parallel_id: 3
branch: "feature/current/cli-standards/story-06-003-file-ref-terminal"
status: "done"
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

- [x] Implement file reference formatting function:
  - Format: `file:///absolute/path/to/file:line:column`
  - Alternative format: `file:line:column`
  - Use VSCode-compatible format for modern terminals
- [x] Update all error messages to use file reference formatting:
  - Config file errors
  - Validation errors with file locations
  - Test failure messages
- [x] Update log output to use file reference formatting where applicable
- [x] Implement terminal size detection:
  - Detect terminal width on startup
  - Detect terminal height on startup
  - Handle detection errors gracefully
- [x] Implement responsive output formatting:
  - Adjust table widths based on terminal width
  - Wrap long lines at terminal width
  - Truncate with ellipsis if needed
- [x] Implement terminal resize handling (where possible):
  - Detect SIGWINCH signal
  - Reformat output on resize
  - Handle resize in TUI mode
- [x] Ensure file references work on all platforms (Windows, Linux, macOS)
- [x] Add tests for file reference formatting
- [x] Add tests for terminal size detection
- [x] Add tests for responsive formatting

## Relevant Files

- `src/cli/format.rs` — NEW FILE for formatting utilities (file references, terminal size, text wrapping)
- `src/cli/mod.rs` — Export format module
- `src/config/loader.rs` — Updated to use file reference formatting in error messages
- `src/error.rs` — Added format_error_with_file helper function
- `Cargo.toml` — Added textwrap dependency

## Acceptance Criteria

- [x] File references use VSCode-compatible format
- [x] Error messages include file references with line numbers
- [x] Log output uses file references where applicable
- [x] Terminal size is detected on startup
- [x] Output formatting adjusts to terminal width
- [x] Long lines wrap or truncate at terminal width
- [x] Terminal resize is detected and handled where possible
- [x] File references work on Windows, Linux, and macOS
- [x] All file reference formatting has tests
- [x] All terminal size detection has tests
- [x] All responsive formatting has tests

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