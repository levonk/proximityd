---
story_id: "04-003"
story_title: "Add notifier testing in TUI"
story_name: "tui-notifier-testing"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 4
parallel_id: 3
branch: "feature/current/cli-standards/story-04-003-tui-notifier-testing"
status: "todo"
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

Implement TUI screen for testing notifier configurations. Users should be able to send test notifications through configured notifiers (Discord, Slack, Webhook, MQTT) to verify their setup without needing to trigger actual presence events.

## Sub-Tasks

- [ ] Implement notifier test screen:
  - List all configured notifiers
  - Select notifier to test
  - Send test notification button
- [ ] Implement test notification builder:
  - Create sample presence event (enter/exit)
  - Include sample party name, device name, location
  - Format notification according to notifier type
- [ ] Implement Discord test notification:
  - Send test message to configured webhook
  - Display success/error status
  - Show response/error details
- [ ] Implement Slack test notification:
  - Send test message to configured webhook
  - Display success/error status
  - Show response/error details
- [ ] Implement Webhook test notification:
  - Send test POST request to configured URL
  - Display success/error status
  - Show response/error details
- [ ] Implement MQTT test notification:
  - Publish test message to configured topic
  - Display success/error status
  - Show connection/publish errors
- [ ] Implement test result display:
  - Success message with timestamp
  - Error message with details
  - Retry button for failed tests
- [ ] Add loading indicator during notification sending
- [ ] Add tests for notifier testing functionality

## Relevant Files

- `src/cli/tui.rs` — Add notifier test screen
- `src/cli/tui/notifier.rs` — NEW FILE for notifier test logic
- `src/notifier/` — Reference notifier implementations
- `tests/tui_test.rs` — Add notifier test tests

## Acceptance Criteria

- [ ] Notifier test screen displays all configured notifiers
- [ ] Discord test notifications send successfully
- [ ] Slack test notifications send successfully
- [ ] Webhook test notifications send successfully
- [ ] MQTT test notifications send successfully
- [ ] Test results display clearly (success or error with details)
- [ ] Failed tests can be retried
- [ ] Loading indicator shows during sending
- [ ] All notifier test functionality has tests

## Test Plan

- Unit: Tests for notifier test logic
- Integration: Test sending notifications through TUI
- Manual: Test each notifier type with valid configuration
- Manual: Test with invalid configuration to verify error handling

## Observability

- Log test notification attempts at INFO level
- Log test notification results at INFO level
- Log errors at ERROR level

## Compliance

- Must not expose sensitive webhook URLs in logs
- Test notifications must be clearly marked as test messages
- Must handle network errors gracefully

## Risks & Mitigations

- Risk: Test notifications might be sent to production systems accidentally
  - Mitigation: Clearly label test messages
  - Mitigation: Add confirmation before sending test
- Risk: Network errors might cause TUI to hang
  - Mitigation: Use timeout for notification sending
  - Mitigation: Show loading indicator with cancel option

## Dependencies & Sequencing

- Depends on: 03-001 (TUI framework)
- Unblocks: None

## Definition of Done

- Notifier testing screen is implemented
- All notifier types can be tested
- Test results display correctly
- Error handling is robust
- Tests pass
- Manual testing confirms all notifier tests work

## Commit Conventions

- `feat(tui): add notifier testing screen`
- `feat(tui): implement Discord test notifications`
- `feat(tui): implement Slack test notifications`
- `feat(tui): implement Webhook test notifications`
- `feat(tui): implement MQTT test notifications`
- `test(tui): add notifier test tests`