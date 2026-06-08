---
story_id: "04-001"
story_title: "Add config section editors to TUI"
story_name: "tui-config-editors"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 4
parallel_id: 1
branch: "feature/current/cli-standards/story-04-001-tui-config-editors"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["src/cli/tui"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "tui"]
due: "2026-06-28"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement TUI screens for editing all configuration sections from config.toml. This includes general settings, scanner settings, detection settings, discovery settings, and notifier configurations. Users should be able to view and modify all config options through the TUI.

## Sub-Tasks

- [x] Implement general settings editor screen:
  - Log level selection
  - Log format selection
  - Scan interval settings
  - Debounce settings
  - Privacy mode toggle
- [x] Implement scanner settings editor screen:
  - BLE scanner toggle and interval
  - WiFi ARP scanner toggle and interval
  - Ping sweep scanner toggle and interval
  - mDNS scanner toggle and interval
  - Scanner-specific settings (router IP, SNMP community, etc.)
- [x] Implement detection settings editor screen:
  - Exit timeout
  - RSSI threshold
  - Location defaults
- [x] Implement discovery settings editor screen:
  - Use suggestions toggle
  - Auto-promote threshold
  - Confidence threshold
- [x] Implement notifier configuration editor screen:
  - Discord webhook URL
  - Slack webhook URL
  - Webhook URL and method
  - MQTT broker and topic
  - Notifier-specific settings
- [x] Implement form input widgets:
  - Text input fields
  - Number input fields
  - Dropdown/select for enums
  - Toggle switches for booleans
  - Validation for input types
- [x] Implement save changes functionality:
  - Validate all inputs before saving
  - Write changes to config.toml
  - Show success/error feedback
- [x] Implement cancel/rollback functionality
- [x] Add keyboard shortcuts for common actions (Save: Ctrl+S, Cancel: Esc)
- [x] Add input validation with inline error messages
- [x] Add tests for config editor screens

## Relevant Files

- `src/cli/tui.rs` — Add config editor screens
- `src/cli/tui/config.rs` — NEW FILE for config-specific TUI screens
- `src/config/app.rs` — Reference for config structure
- `tests/tui_test.rs` — Add config editor tests

## Acceptance Criteria

- [x] General settings can be viewed and modified in TUI
- [x] Scanner settings can be viewed and modified in TUI
- [x] Detection settings can be viewed and modified in TUI
- [x] Discovery settings can be viewed and modified in TUI
- [x] Notifier configurations can be viewed and modified in TUI
- [x] All input types work correctly (text, number, select, toggle)
- [x] Input validation prevents invalid values
- [x] Save writes changes to config.toml correctly
- [x] Cancel discards changes correctly
- [x] Keyboard shortcuts work as expected
- [x] All config editor screens have tests

## Test Plan

- Unit: Tests for each config editor screen
- Integration: Test editing and saving config through TUI
- Manual: Edit all config sections through TUI
- Manual: Verify saved config matches TUI input

## Observability

- Log config save actions at INFO level
- Log validation errors at WARN level

## Compliance

- Config editing must validate input before saving
- Must not allow invalid configurations to be saved
- Must handle file write errors gracefully

## Risks & Mitigations

- Risk: Complex config structure might be hard to edit in TUI
  - Mitigation: Group related settings logically
  - Mitigation: Provide help text for each setting
- Risk: File write errors might lose user changes
  - Mitigation: Validate before writing
  - Mitigation: Show clear error messages on failure

## Dependencies & Sequencing

- Depends on: 03-001 (TUI framework)
- Unblocks: None

## Definition of Done

- All config sections can be edited in TUI
- Input validation works correctly
- Save/cancel functionality works
- Tests pass
- Manual testing confirms all editors work

## Commit Conventions

- `feat(tui): add general settings editor`
- `feat(tui): add scanner settings editor`
- `feat(tui): add detection settings editor`
- `feat(tui): add discovery settings editor`
- `feat(tui): add notifier configuration editor`
- `test(tui): add config editor tests`