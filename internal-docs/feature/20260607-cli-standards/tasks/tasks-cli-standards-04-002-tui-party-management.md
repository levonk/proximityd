---
story_id: "04-002"
story_title: "Add party/device/identifier management in TUI"
story_name: "tui-party-management"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 4
parallel_id: 2
branch: "feature/current/cli-standards/story-04-002-tui-party-management"
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

Implement TUI screens for managing parties, devices, and identifiers from presence.toml. Users should be able to add, edit, and delete parties, add devices to parties, and add identifiers to devices through the TUI interface.

## Sub-Tasks

- [x] Implement party list screen:
  - Display all parties with device count
  - Add new party button
  - Edit party button
  - Delete party button (with confirmation)
- [x] Implement party detail/edit screen:
  - Edit party name
  - Edit party-level location
  - View devices in party
  - Add device to party
- [x] Implement device list screen (within party):
  - Display all devices in party with identifier count
  - Add new device button
  - Edit device button
  - Delete device button (with confirmation)
- [x] Implement device detail/edit screen:
  - Edit device name
  - Edit device-level location
  - View identifiers for device
  - Add identifier to device
- [x] Implement identifier list screen (within device):
  - Display all identifiers with type and value
  - Add new identifier button
  - Edit identifier button
  - Delete identifier button (with confirmation)
- [x] Implement identifier edit screen:
  - Select identifier type (ble_mac, wifi_mac, ip_v4, ip_v6, hostname, card_id, door_sensor)
  - Edit identifier value
  - Edit identifier name/notes
- [x] Implement hierarchical navigation:
  - Party list → Party detail → Device list → Device detail → Identifier list → Identifier edit
  - Back navigation at each level
- [x] Implement add/edit/delete operations with confirmation for destructive actions
- [x] Implement save changes to presence.toml
- [x] Add validation for identifier values based on type (MAC format, IP format, etc.)
- [x] Add tests for party/device/identifier management screens

## Relevant Files

- `src/cli/tui.rs` — Add party management screens
- `src/cli/tui/presence.rs` — NEW FILE for presence-specific TUI screens
- `src/config/presence.rs` — Reference for presence config structure
- `tests/tui_test.rs` — Add party management tests

## Acceptance Criteria

- [x] Parties can be viewed, added, edited, and deleted in TUI
- [x] Devices can be viewed, added, edited, and deleted in TUI
- [x] Identifiers can be viewed, added, edited, and deleted in TUI
- [x] Hierarchical navigation works smoothly
- [x] Back navigation works at each level
- [x] Identifier values are validated based on type
- [x] Changes save to presence.toml correctly
- [x] Destructive actions require confirmation
- [x] All party management screens have tests

## Test Plan

- Unit: Tests for each screen in party management
- Integration: Test adding/editing/deleting parties, devices, identifiers
- Manual: Create a complete party structure through TUI
- Manual: Verify saved presence.toml matches TUI input

## Observability

- Log party/device/identifier CRUD operations at INFO level
- Log validation errors at WARN level

## Compliance

- Must validate identifier formats before saving
- Must confirm before destructive operations
- Must handle file write errors gracefully

## Risks & Mitigations

- Risk: Complex hierarchy might be confusing to navigate
  - Mitigation: Clear breadcrumbs showing current location
  - Mitigation: Consistent back navigation
- Risk: Validation logic might be complex for different identifier types
  - Mitigation: Use existing validation logic from config module
  - Mitigation: Provide clear error messages for invalid formats

## Dependencies & Sequencing

- Depends on: 03-001 (TUI framework)
- Unblocks: None

## Definition of Done

- Party management is fully functional in TUI
- Device management is fully functional in TUI
- Identifier management is fully functional in TUI
- Hierarchical navigation works smoothly
- Validation works correctly
- Tests pass
- Manual testing confirms all operations work

## Commit Conventions

- `feat(tui): add party management screens`
- `feat(tui): add device management screens`
- `feat(tui): add identifier management screens`
- `test(tui): add party management tests`