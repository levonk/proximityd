---
story_id: "01-001"
story_title: "Config & Device Mapping Loading"
story_name: "config-device-mapping-loading"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 1
parallel_id: 1
branch: "feature/current/bluetooth-presence-notifier/story-01-001-config-device-mapping-loading"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/config/"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "backend", "config"]
due: "2026-06-03"
created_at: "2026-05-27"
updated_at: "2026-05-31"
---

## Summary

Implement TOML configuration loading for app behavior (`config.toml`) and device identity mapping (`devices.toml`). Include environment variable overrides, XDG config directory resolution, and clear error messages for malformed files.

## Sub-Tasks

- [x] Create `src/config/mod.rs` — module root with re-exports
- [x] Create `src/config/app.rs` — `AppConfig` struct with serde derive for `scan_interval_seconds`, `enter_rssi_threshold_dbm`, `enter_duration_seconds`, `exit_timeout_seconds`, `notifiers` array
- [x] Create `src/config/devices.rs` — `DevicesConfig` struct with MAC-to-name mapping table
- [x] Create `src/config/loader.rs` — `load_config()` and `load_devices()` functions with XDG path resolution via `directories` crate and `BTNOTIFY_CONFIG_DIR` env override
- [x] Add `toml` crate to `Cargo.toml` dependencies
- [x] Add unit tests in `src/config/` — valid TOML parse, missing file error, malformed TOML error, env override behavior
- [x] Update `src/main.rs` to initialize config at startup and wire into CLI `--config` flag

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/config/mod.rs` — module root
- `src/config/app.rs` — app behavior config schema
- `src/config/devices.rs` — device identity mapping schema
- `src/config/loader.rs` — file loading and path resolution logic
- `src/config/app.test.rs` — unit tests for AppConfig parsing
- `src/config/devices.test.rs` — unit tests for DevicesConfig parsing
- `src/config/loader.test.rs` — unit tests for loader behavior
- `Cargo.toml` — add `toml` dependency
- `config.example.toml` — example config for users
- `devices.example.toml` — example device mapping for users

## Acceptance Criteria

- [x] `config.toml` and `devices.toml` are parsed correctly with all fields optional and sensible defaults
- [x] `BTNOTIFY_CONFIG_DIR` overrides the default XDG config path
- [x] Malformed TOML produces a clear error and exits with code `1`
- [x] Missing `devices.toml` is allowed (no known devices yet)
- [x] All config fields have documented defaults matching the PRD

## Test Plan

- Unit: `cargo test config::`
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Add `tracing::info!` log on successful config load with path
- Add `tracing::warn!` when `devices.toml` is missing

## Compliance

- MAC addresses and names stay local; no cloud upload
- Discord tokens must be loaded from env, never in config files

## Risks & Mitigations

- Risk: XDG path resolution fails on non-standard systems — Mitigation: use `directories` crate which handles edge cases
- Risk: TOML parse errors are cryptic — Mitigation: wrap `toml::de::Error` with `anyhow::Context`

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001 (needs config values), 03-002 (needs config for Docker env)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(config): add app and device config loading`

## Changelog

- 2026-05-27: initialized story file
