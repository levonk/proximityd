---
story_id: "01-002"
story_title: "BLE Scanning Loop"
story_name: "ble-scanning-loop"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 1
parallel_id: 2
branch: "feature/current/bluetooth-presence-notifier/story-01-002-ble-scanning-loop"
status: "todo"
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
updated_at: "2026-05-27"
---

## Summary

Implement a continuous BLE discovery scan loop using the `blurz` or `bluez-async` crate. Collect MAC address and RSSI for each discovered device. Abstract the Bluetooth backend behind a trait so future Windows/macOS implementations are possible.

## Sub-Tasks

- [ ] Research and select `blurz` vs `bluez-async` based on async model compatibility with existing `tokio` usage
- [ ] Add chosen crate + `tokio` to `Cargo.toml`
- [ ] Create `src/bluetooth/mod.rs` — module root with re-exports
- [ ] Create `src/bluetooth/adapter.rs` — `BluetoothAdapter` trait with `scan()` method returning a stream of `ScannedDevice`
- [ ] Create `src/bluetooth/bluez.rs` — BlueZ implementation using D-Bus via chosen crate
- [ ] Create `src/bluetooth/types.rs` — `ScannedDevice { mac: String, rssi: i16, last_seen: DateTime<Utc> }`
- [ ] Create `src/bluetooth/scan_loop.rs` — `run_scan_loop(scan_interval)` that repeatedly scans and yields results via a `tokio::sync::mpsc` channel
- [ ] Add unit tests with mocked `BluetoothAdapter` trait for scan loop timing
- [ ] Add integration test that validates scan cycle completes within 5 seconds (NFR-1)

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

- [ ] Scan loop discovers BLE devices and extracts MAC + RSSI
- [ ] Scan cycle completes within 5 seconds on typical hardware
- [ ] Adapter trait is well-defined and mockable for tests
- [ ] If Bluetooth adapter is unavailable, loop retries every 30 seconds (NFR-2)
- [ ] Abstracted so a future `WindowsBleAdapter` or `MacBleAdapter` can be dropped in

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
