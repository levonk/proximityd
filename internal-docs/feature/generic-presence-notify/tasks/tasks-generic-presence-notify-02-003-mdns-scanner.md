---
story_id: "02-003"
story_title: "mDNS Scanner"
story_name: "mdns-scanner"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 2
parallel_id: 3
branch: "feature/current/generic-presence-notify/story-02-003-mdns-scanner"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["src/scanner/"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "backend", "network"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Implement mDNS scanner that listens for multicast DNS announcements via `avahi-browse` or `dns-sd` wrappers. Emits `RawSignal` with `hostname` identifiers.

## Sub-Tasks

- [x] Create `src/scanner/mdns.rs` — `MdnsScanner` implementing `Scanner`
- [x] Implement `avahi-browse` wrapper for Linux (`tokio::process`)
- [x] Implement `dns-sd` wrapper for macOS
- [x] Add fallback: skip mDNS if neither tool is available (log at `warn`)
- [x] Respect `[scanner.mdns]` config: `enabled`, `scan_interval_sec`
- [x] Add unit tests with mocked output (in mdns.rs)

## Relevant Files

- `src/scanner/mdns.rs`
- `src/scanner/mdns_tests.rs`

## Acceptance Criteria

- [x] Scanner emits `RawSignal` with `id_type = "hostname"`
- [x] Gracefully handles missing `avahi-browse`/`dns-sd`
- [x] Unit tests pass

## Test Plan

- Unit: `cargo test mdns`

## Risks & Mitigations

- Risk: `avahi-browse` not installed on minimal systems — Mitigation: make optional; log warning

## Dependencies & Sequencing

- Depends on: 01-003
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(scanner): add mDNS scanner`

## Changelog

- 2026-06-03: initialized story file
