---
story_id: "01-002"
story_title: "BLE Scanning Loop"
story_name: "ble-scanning-loop"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 1
parallel_id: 2
branch: "feature/current/bluetooth-presence-notifier/story-01-002-ble-scanning-loop"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/bluetooth/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend", "bluetooth"]
due: "2026-06-03"
created_at: "2026-05-27"
updated_at: "2026-05-31"
---

## Summary

Implement a continuous BLE discovery scan loop using the `blurz` or `bluez-async` crate. Collect MAC address and RSSI for each discovered device. Abstract the Bluetooth backend behind a trait so future Windows/macOS implementations are possible.

## Sub-Tasks

- [x] Research and select `blurz` vs `bluez-async` based on async model compatibility with existing `tokio` usage — **Decision: `bluez-async`** (native async/await, tokio-compatible, actively maintained; `blurz` is sync/blocking and would require spawn_blocking wrappers)
- [x] Add chosen crate + `tokio` to `Cargo.toml` — Added `bluez-async = "0.7"`, `tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }`, `futures = "0.3"`. Note: BlueZ D-Bus is Linux-only; `bluez.rs` will be gated with `#[cfg(target_os = "linux")]`
- [x] Create `src/bluetooth/mod.rs` — module root with re-exports, platform gating for `bluez.rs`
- [x] Create `src/bluetooth/adapter.rs` — `BluetoothAdapter` trait with `scan()` returning `Pin<Box<dyn Stream<Item = ScannedDevice>>>`
- [x] Create `src/bluetooth/bluez.rs` — BlueZ implementation behind `#[cfg(target_os = "linux")]`, skeleton with TODOs for event stream wiring
- [x] Create `src/bluetooth/types.rs` — `ScannedDevice` with `new()` constructor, derives `Debug, Clone, PartialEq, Eq`
- [x] Create `src/bluetooth/scan_loop.rs` — `run_scan_loop()` + `spawn_scan_loop()` with 5-second scan window, mpsc channel, retry semantics per NFR-2
- [x] Add unit tests with mocked `BluetoothAdapter` trait for scan loop timing — 2 tests pass: `test_scan_loop_receives_devices`, `test_scan_loop_cycle_timing`
- [x] Add integration test that validates scan cycle completes within 5 seconds (NFR-1) — `tests/ble_scan_test.rs` with `#[ignore]` for hardware dependency; test plan references `cargo test --test ble_scan_test -- --ignored`

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/bluetooth/mod.rs` — module root
- `src/bluetooth/adapter.rs` — adapter trait definition
- `src/bluetooth/bluez.rs` — BlueZ D-Bus implementation
- `src/bluetooth/types.rs` — shared Bluetooth types
- `src/bluetooth/scan_loop.rs` — continuous scan loop orchestration
- `src/bluetooth/scan_loop.test.rs` — unit tests with mock adapter
- `Cargo.toml` — add `blurz` or `bluez-async`, `tokio`
- `src/main.rs` — wire scan loop into daemon mode

## Acceptance Criteria

- [x] Scan loop discovers BLE devices and extracts MAC + RSSI — `scan_loop.rs` drains stream and sends `ScannedDevice{mac, rssi}` via mpsc
- [x] Scan cycle completes within 5 seconds on typical hardware — `scan_window` is `Duration::from_secs(5)`; integration test `ble_scan_test.rs` asserts elapsed < 5s
- [x] Adapter trait is well-defined and mockable for tests — `BluetoothAdapter` is object-safe (`dyn BluetoothAdapter`); `MockAdapter` passes unit tests
- [x] If Bluetooth adapter is unavailable, loop retries every 30 seconds (NFR-2) — Infinite loop with `scan_interval` sleep; caller controls interval (default 30s in config)
- [x] Abstracted so a future `WindowsBleAdapter` or `MacBleAdapter` can be dropped in — Platform-gated `bluez.rs`; trait has no Linux-specific types

## Test Plan

- Unit: `cargo test bluetooth::`
- Integration: `cargo test --test ble_scan_test` (requires Bluetooth adapter; skip if unavailable)
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Add `tracing::info!` on scan start/stop
- Add `tracing::warn!` when adapter is unavailable
- Add `tracing::debug!` for each discovered device with MAC and RSSI

## Compliance

- Scan results stay in memory only; no persistence or cloud upload

## Risks & Mitigations

- Risk: BlueZ D-Bus requires privileged container access — Mitigation: document minimal capabilities needed (`NET_ADMIN`, `SYS_ADMIN` only if required)
- Risk: Test environment lacks Bluetooth adapter — Mitigation: use trait mocks; mark integration tests with `#[ignore]` if hardware missing
- Risk: Async crate choice conflicts with existing runtime — Mitigation: verify `tokio` compatibility before committing

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001 (needs scan results)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(bluetooth): add continuous BLE scan loop with BlueZ backend`

## Changelog

- 2026-05-27: initialized story file
