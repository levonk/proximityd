---
story_id: "05-001"
story_title: "New Notifiers (Slack, Webhook, MQTT)"
story_name: "new-notifiers"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 5
parallel_id: 1
branch: "feature/current/generic-presence-notify/story-05-001-new-notifiers"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-004"]
parallel_safe: true
modules: ["src/notifier/"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "backend", "notify"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Expand notifiers beyond Discord to Slack webhook, generic HTTP webhook, and MQTT publisher. All include rich payload with party name, signal source, identifier type, and location context.

## Sub-Tasks

- [x] Update `Notifier` trait to include rich payload fields (party, source, id_type, location)
- [x] Create `src/notifier/slack.rs` — `SlackNotifier` using `reqwest`
- [x] Create `src/notifier/webhook.rs` — `WebhookNotifier` with configurable URL, method, payload template
- [x] Add `rumqttc` to `Cargo.toml` (optional feature flag)
- [x] Create `src/notifier/mqtt.rs` — `MqttNotifier` publishing JSON presence events to topic
- [x] Update `src/notifier/registry.rs` — register new notifier types
- [x] Add `src/notifier/slack_tests.rs`, `webhook_tests.rs`, `mqtt_tests.rs`

## Relevant Files

- `src/notifier/slack.rs`
- `src/notifier/webhook.rs`
- `src/notifier/mqtt.rs`
- `src/notifier/registry.rs`
- `src/notifier/mod.rs`
- `Cargo.toml`

## Acceptance Criteria

- [x] Slack notifier sends formatted message with party + location
- [x] Webhook notifier respects custom method, URL, and payload template
- [x] MQTT notifier publishes JSON to configured topic
- [x] All new notifiers have unit tests

## Test Plan

- Unit: `cargo test notifier`

## Risks & Mitigations

- Risk: MQTT adds dependency bloat — Mitigation: make optional feature flag

## Dependencies & Sequencing

- Depends on: 01-004
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(notifier): add Slack, Webhook, and MQTT notifiers`

## Changelog

- 2026-06-03: initialized story file
