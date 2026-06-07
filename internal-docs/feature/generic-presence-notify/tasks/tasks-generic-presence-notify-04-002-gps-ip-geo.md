---
story_id: "04-002"
story_title: "GPS and IP Geolocation Logging"
story_name: "gps-ip-geo"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 4
parallel_id: 2
branch: "feature/current/generic-presence-notify/story-04-002-gps-ip-geo"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001", "04-001"]
parallel_safe: true
modules: ["src/location/"]
priority: "COULD"
risk_level: "low"
tags: ["feat", "backend"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Log GPS coordinates and IP geolocation hints per sighting. GPS via `geoclue` on Linux (non-blocking); IP via STUN or `icanhazip.com`.

## Sub-Tasks

- [x] Add `geoclue-zbus` to `Cargo.toml` (Linux-only dependency)
- [x] Create `src/location/gps.rs` — `GpsSource` trait; `GeoclueGps` implementation for Linux
- [x] Create `src/location/ip_geo.rs` — fetch public IP via HTTP (non-blocking, short timeout)
- [x] Integrate into `SignalLogger::log()`: write `gps_lat`, `gps_lon`, `public_ip` if available
- [x] Ensure GPS never blocks scanning: fire-and-forget with timeout
- [x] Add `src/location/gps_tests.rs` — unit tests with mocked geoclue responses

## Relevant Files

- `src/location/gps.rs`
- `src/location/ip_geo.rs`
- `src/location/gps_tests.rs`
- `src/signals/logger.rs`
- `Cargo.toml`

## Acceptance Criteria

- [x] GPS coordinates logged when geoclue available
- [x] Public IP logged when network available
- [x] Missing sources do not block scanning
- [x] Unit tests pass

## Test Plan

- Unit: `cargo test gps_ip_geo`

## Risks & Mitigations

- Risk: GPS probe adds latency — Mitigation: fire-and-forget; use timeout

## Dependencies & Sequencing

- Depends on: 01-001, 04-001
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(location): add GPS and IP geolocation logging`

## Changelog

- 2026-06-03: initialized story file
