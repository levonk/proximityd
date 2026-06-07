---
story_id: "05-002"
story_title: "status/export CLI and Documentation"
story_name: "status-export-cli"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 5
parallel_id: 2
branch: "feature/current/generic-presence-notify/story-05-002-status-export-cli"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-004"]
parallel_safe: true
modules: ["src/main.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "cli", "docs"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Implement `proximityd status` (print present parties, last-seen signals, locations) and `proximityd export --format jsonl --since YYYY-MM-DD`. Update README, devbox, and flake docs.

## Sub-Tasks

- [x] Add `status` CLI command: query `PresenceStateTable`, print table of parties, last seen, location, signal source
- [x] Add `export` CLI command: query `signal_log`, output JSONL with `--since` and `--format` filters
- [x] Update `README.md` with full setup guide, config examples, and success metrics
- [x] Update `devbox.json` and `flake.nix` with new binary name and dependencies
- [x] Add `SIGHUP` handler for config reload (NF6)
- [x] Add `tests/cli_tests.rs` cases for `status` and `export`

## Relevant Files

- `src/main.rs`
- `src/state/table.rs`
- `src/signals/logger.rs`
- `README.md`
- `devbox.json`
- `flake.nix`

## Acceptance Criteria

- [x] `status` shows present parties with location
- [x] `export` outputs valid JSONL with correct `--since` filter
- [x] README covers installation, config, and all CLI commands

## Test Plan

- Unit: `cargo test cli`
- Integration: `cargo test --test cli_tests`

## Risks & Mitigations

- Risk: Config reload race conditions — Mitigation: atomic swap of config structs

## Dependencies & Sequencing

- Depends on: 01-001, 01-004
- Unblocks: None

## Definition of Done

- Code, tests, docs updated; CI green

## Commit Conventions

- `feat(cli): add status and export commands`
- `docs: update README for proximityd`

## Changelog

- 2026-06-03: initialized story file
