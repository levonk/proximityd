---
story_id: "04-001"
story_title: "Hierarchical Location Model"
story_name: "hierarchical-location"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 4
parallel_id: 1
branch: "feature/current/generic-presence-notify/story-04-001-hierarchical-location"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-004"]
parallel_safe: true
modules: ["src/location/"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "backend"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Implement static hierarchical location model (building → floor → room → zone) in `presence.toml`. Device-level overrides party-level. Scanner-node location mappings also stored in `presence.toml`.

## Sub-Tasks

- [ ] Create `src/location/mod.rs` — `Location` struct with `building`, `floor`, `room`, `zone`
- [ ] Update `presence.toml` schema: support `location` on `party` and `device` levels
- [ ] Update `PartyConfig` loader to parse location fields
- [ ] Add location resolution logic: device-level > party-level > scanner-node default > none
- [ ] Add `src/location/tests.rs` — unit tests for resolution priority

## Relevant Files

- `src/location/mod.rs`
- `src/location/tests.rs`
- `src/config/presence.rs`

## Acceptance Criteria

- [ ] `presence.toml` parses location on party and device
- [ ] Resolution priority correct: device > party > scanner > none
- [ ] Unit tests pass

## Test Plan

- Unit: `cargo test location`

## Dependencies & Sequencing

- Depends on: 01-004
- Unblocks: 04-002

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(location): add hierarchical location model`

## Changelog

- 2026-06-03: initialized story file
