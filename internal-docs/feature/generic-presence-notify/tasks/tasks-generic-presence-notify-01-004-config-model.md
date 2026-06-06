---
story_id: "01-004"
story_title: "Config Model and Legacy Migration"
story_name: "config-model"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 1
parallel_id: 4
branch: "feature/current/generic-presence-notify/story-01-004-config-model"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/config/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend", "config"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Refactor config to support `[scanner.*]` toggles, `[general]` settings, `[privacy]`, `[detection]`, `[discovery]`, and `[[notifier]]` sections. Introduce `presence.toml` with `party` → `device` → `identifier` hierarchy. Auto-migrate legacy `devices.toml`.

## Sub-Tasks

- [x] Refactor `src/config/app.rs` — `AppConfig` with `general`, `privacy`, `scanner` map, `detection`, `discovery`, `notifiers` fields
- [x] Create `src/config/presence.rs` — `Party`, `Device`, `Identifier` structs; `IdentifierType` enum (`ble_mac`, `wifi_mac`, `ip_v4`, `ip_v6`, `hostname`, `card_id`, `door_sensor`)
- [x] Create `src/config/loader.rs` — load `config.toml` and `presence.toml` from `~/.config/proximityd/`
- [x] Create `src/config/migrate.rs` — read legacy `devices.toml`, convert to single default `Party`, write `presence.toml`, rename old file to `.bak`
- [x] Add identifier normalization: lowercase + trim on load
- [~] Add backward compatibility: if `presence.toml` missing but `devices.toml` exists, auto-migrate on first run
- [x] Add `src/config/tests.rs` — test load, migration, normalization (tests integrated in modules)

## Relevant Files

- `src/config/app.rs`
- `src/config/presence.rs`
- `src/config/loader.rs`
- `src/config/migrate.rs`
- `src/config/tests.rs`
- `config.example.toml`
- `devices.example.toml`

## Acceptance Criteria

- [x] `config.toml` parses with all new sections
- [x] `presence.toml` parses with nested party/device/identifier structure
- [x] Legacy `devices.toml` auto-migrates correctly
- [x] Identifier values are normalized (lowercase, trimmed)
- [x] Unit tests pass: `cargo test config`

## Test Plan

- Unit: `cargo test config`
- Integration: test migration with fixture files

## Risks & Mitigations

- Risk: Migration corrupts user config — Mitigation: always write `.bak` before modifying; never delete original until success

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 03-001, 04-001, 05-001, 05-002

## Definition of Done

- Code, tests, docs updated; CI green; story file updated

## Commit Conventions

- `feat(config): add presence.toml model with party/device/identifier hierarchy`
- `feat(config): auto-migrate legacy devices.toml`

## Changelog

- 2026-06-03: initialized story file
