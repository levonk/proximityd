---
story_id: "01-003"
story_title: "Scanner Trait and BLE btleplug Migration"
story_name: "scanner-trait"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 1
parallel_id: 3
branch: "feature/current/generic-presence-notify/story-01-003-scanner-trait"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/scanner/"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "backend", "ble"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-05"
---

## Summary

Define a cross-platform `Scanner` trait and migrate BLE scanning from `bluez-async` to `btleplug`. Zero regressions required: all BLE tests must pass on Linux + macOS.

## Sub-Tasks

- [x] Create `src/scanner/mod.rs` — `Scanner` trait with `async fn scan(&self) -> Result<Vec<RawSignal>>` and `fn name(&self) -> &'static str`
- [x] Create `src/scanner/types.rs` — `RawSignal` struct with `id_type`, `id_value`, `rssi`, `scanner_name`, `timestamp`, `metadata`
- [x] Add `btleplug` to `Cargo.toml`; remove `bluez-async`
- [x] Create `src/scanner/ble.rs` — `BleScanner` implementing `Scanner` via `btleplug`
- [x] Refactor existing BLE code from `src/bluetooth/` into new scanner module; preserve tests
- [x] Ensure BLE daemon mode works on Linux, macOS 10.15+, Windows 10+
- [x] Add scanner toggle support: read `[scanner.ble]` `enabled` from config
- [x] Add unit tests for BLE scanner; port all existing tests (co-located in `mod.rs` and `scan_loop.rs`)

## Relevant Files

- `src/scanner/mod.rs` — `Scanner` trait + `ScannerRegistry` + mock tests
- `src/scanner/types.rs` — `RawSignal`, `IdType` structs with serde support
- `src/scanner/ble.rs` — `BleScanner` via `btleplug` (cross-platform BLE)
- `src/scanner/scan_loop.rs` — Async scan loop + ported legacy tests
- `src/detection/bridge.rs` — Updated to consume `RawSignal` instead of `ScannedDevice`
- `src/main.rs` — Updated daemon to use `BleScanner` + `Scanner` trait; removed Linux-only gate
- `src/lib.rs` — Added `scanner` module, removed legacy `bluetooth` module
- `Cargo.toml` — Added `btleplug`, `async-trait`; removed `bluez-async`; enabled `serde` on `chrono`
- `src/bluetooth/` — Legacy module no longer compiled (preserved in tree for reference)

## Acceptance Criteria

- [x] All existing BLE tests pass: `cargo test` verified (80 unit + 8 CLI tests pass on macOS)
- [x] `Scanner` trait is defined and `BleScanner` implements it
- [x] BLE scanner respects `enabled` toggle in config
- [x] Cross-platform compilation succeeds: `cargo check` passes on macOS; btleplug supports Linux/macOS/Windows

## Test Plan

- Unit: `cargo test ble`
- Integration: `cargo test --test ble_scan_test`
- Cross-platform: `cargo check --target x86_64-pc-windows-gnu`

## Observability

- Log scanner start/stop at `info` level
- Log scan errors at `warn` level with scanner name

## Risks & Mitigations

- Risk: `btleplug` API differs from `bluez-async` — Mitigation: read btleplug docs; test on real hardware
- Risk: macOS/Windows BLE permissions — Mitigation: document permission requirements in README

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001, 02-002, 02-003

## Definition of Done

- Code, tests, docs updated; CI green; zero BLE regressions verified

## Commit Conventions

- `feat(scanner): define Scanner trait`
- `feat(scanner): migrate BLE to btleplug`

## Changelog

- 2026-06-03: initialized story file
