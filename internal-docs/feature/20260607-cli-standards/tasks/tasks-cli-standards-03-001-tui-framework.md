---
story_id: "03-001"
story_title: "Implement TUI framework and basic structure"
story_name: "tui-framework"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 3
parallel_id: 1
branch: "feature/current/cli-standards/story-03-001-tui-framework"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: false
modules: ["src/cli/tui"]
priority: "SHOULD"
risk_level: "high"
tags: ["feat", "tui"]
due: "2026-06-28"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement the foundational TUI (Terminal User Interface) framework for interactive configuration. This includes setting up the TUI library (ratatui or similar), implementing the basic application structure, event loop, and navigation framework. This is the foundation for all TUI features in subsequent stories.

## Sub-Tasks

- [x] Add TUI library dependency to Cargo.toml (ratatui + crossterm)
- [x] Create `src/cli/tui.rs` module
- [x] Implement basic TUI application structure:
  - Terminal initialization and cleanup
  - Event loop for keyboard input
  - Basic rendering loop
  - Error handling for terminal resize
- [x] Implement main menu navigation:
  - Menu items: Config, Parties, Devices, Notifiers, Test, Save, Exit
  - Keyboard navigation (arrow keys, Enter, Esc)
  - Visual selection indicators
- [x] Implement basic screen management:
  - Screen stack for navigation
  - Push/pop screens
  - Title bar and status bar
- [x] Add `--interactive` / `--tui` flag to main command
- [x] Add TUI mode detection and entry point in `src/main.rs`
- [x] Implement graceful fallback to text mode on Windows if TUI not supported
- [x] Add keyboard shortcut help screen (F1 or ?)
- [x] Add basic error handling and user feedback in TUI
- [x] Add unit tests for TUI framework (mocked terminal)
- [x] Test TUI on Linux and macOS

## Relevant Files

- `Cargo.toml` — Add ratatui and crossterm dependencies
- `src/cli/tui.rs` — NEW FILE for TUI framework
- `src/cli/mod.rs` — Export tui module
- `src/main.rs` — Add --interactive flag and TUI entry point
- `src/lib.rs` — Add cli module export
- `tests/tui_test.rs` — NEW FILE for TUI tests

## Acceptance Criteria

- [x] `proximityd --interactive` launches TUI mode
- [x] TUI initializes terminal correctly
- [x] TUI cleans up terminal on exit
- [x] Main menu displays all options
- [x] Keyboard navigation works (arrows, Enter, Esc)
- [x] Screen navigation works (push/pop)
- [x] Help screen displays keyboard shortcuts
- [x] TUI handles terminal resize gracefully
- [x] TUI falls back gracefully on Windows if needed
- [x] All TUI framework code has tests

## Test Plan

- Unit: Tests for TUI framework with mocked terminal
- Integration: Test TUI launch and basic navigation
- Manual: Test TUI on Linux
- Manual: Test TUI on macOS
- Manual: Test TUI on Windows (verify fallback)

## Observability

- Log TUI mode entry at INFO level
- Log terminal initialization errors at ERROR level
- Log keyboard events at DEBUG level (for debugging)

## Compliance

- TUI must not expose sensitive data in terminal history
- TUI must handle terminal size constraints gracefully
- TUI must be accessible with keyboard-only navigation

## Risks & Mitigations

- Risk: TUI library might have compatibility issues
  - Mitigation: Use well-maintained library (ratatui) with good cross-platform support
- Risk: TUI might not work on all terminals
  - Mitigation: Implement graceful fallback to CLI mode
  - Mitigation: Document terminal requirements
- Risk: TUI might increase binary size significantly
  - Mitigation: Consider making TUI optional via feature flag

## Dependencies & Sequencing

- Depends on: None
- Unblocks: Stories 04-001, 04-002, 04-003 (all TUI features)

## Definition of Done

- TUI framework is implemented and functional
- Basic navigation works
- Terminal handling is robust
- Tests pass
- Manual testing confirms TUI works on supported platforms

## Commit Conventions

- `feat(tui): add TUI framework with ratatui`
- `feat(tui): implement basic navigation and screen management`
- `test(tui): add framework tests`