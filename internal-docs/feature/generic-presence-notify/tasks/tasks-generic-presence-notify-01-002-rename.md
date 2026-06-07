---
story_id: "01-002"
story_title: "Rename btnotify to proximityd + Deprecation Wrapper"
story_name: "rename"
prd_name: "generic-presence-notify"
prd_file: "internal-docs/feature/generic-presence-notify/prd-generic-presence-notify.md"
phase: 1
parallel_id: 2
branch: "feature/current/generic-presence-notify/story-01-002-rename"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["root", "Cargo.toml"]
priority: "MUST"
risk_level: "low"
tags: ["chore", "rename"]
due: ""
created_at: "2026-06-03"
updated_at: "2026-06-03"
---

## Summary

Rebrand `btnotify` to `proximityd` in `Cargo.toml`, README, and docs. No `btnotify` alias or wrapper retained.

## Sub-Tasks

- [x] Update `Cargo.toml` `name`, `description`, and binary targets (`proximityd` primary)
- [x] Update `README.md` — change all references from `btnotify` to `proximityd`
- [x] Update `AGENTS.md` with new project context
- [x] Update `devbox.json`, `justfile`, `run.sh`, `project.json` references
- [x] ~~Add deprecation warning in `btnotify` wrapper~~ — Removed per user direction (no alias needed)
- [x] Update Docker labels and compose service names
- [x] Update `config.example.toml`, `devices.example.toml`, source env vars, and docs

## Relevant Files

- `Cargo.toml`
- `README.md`
- `AGENTS.md`
- `devbox.json`
- `flake.nix`
- `justfile`
- `Dockerfile`
- `docker-compose.yml`

## Acceptance Criteria

- [x] `cargo build --release` produces `proximityd` binary
- [x] ~~`btnotify` binary exists and prints deprecation warning~~ — Removed per user direction
- [x] All CI/build scripts reference new name

## Test Plan

- Build: `cargo build --release`
- CLI: `./target/release/proximityd --help` shows correct name

## Risks & Mitigations

- Risk: Downstream scripts break — Mitigation: keep `btnotify` alias for one release

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None (independent branding change)

## Definition of Done

- Code, docs, CI updated; story file updated

## Commit Conventions

- `chore(project): rename btnotify to proximityd`

## Changelog

- 2026-06-03: initialized story file
