---
story_id: "04-002"
story_title: "Documentation & Repo Polish"
story_name: "documentation-and-repo-polish"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 4
parallel_id: 2
branch: "feature/current/bluetooth-presence-notifier/story-04-002-documentation-and-repo-polish"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-002"]
parallel_safe: true
modules: ["README.md", "docs/", "examples/"]
priority: "MUST"
risk_level: "low"
tags: ["docs", "devops", "polish"]
due: "2026-06-24"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Polish the repository so a junior developer can clone, configure, and run the project within 10 minutes. Update README, add example configs, document Docker usage, and verify all success metrics from the PRD.

## Sub-Tasks

- [ ] Rewrite `README.md`:
  - Product description and features
  - Quick start (clone, copy examples, Discord webhook, run)
  - Configuration reference for `config.toml` and `devices.toml`
  - Docker build and run instructions
  - Troubleshooting section (adapter not found, Discord not delivering, permissions)
- [ ] Add `config.example.toml` with all options documented
- [ ] Add `devices.example.toml` with sample entries
- [ ] Add `.env.example` for `BTNOTIFY_CONFIG_DIR`, `BTNOTIFY_DISCORD_WEBHOOK`, `BTNOTIFY_LOG_LEVEL`
- [ ] Verify and update `Cargo.toml` metadata (description, repository, keywords, categories)
- [ ] Add `LICENSE` file if missing
- [ ] Add `CONTRIBUTING.md` with build/test instructions
- [ ] Run `cargo fmt`, `cargo clippy --all-targets`, `cargo test` and ensure all green
- [ ] Verify success metrics:
  - [ ] Notification latency under 60 seconds of real-world enter/exit
  - [ ] Zero false events under normal home conditions (7-day test)
  - [ ] 30-day uptime without memory leaks or crashes
  - [ ] Junior developer 10-minute setup time

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `README.md` — primary user-facing documentation
- `config.example.toml` — example app config
- `devices.example.toml` — example device mapping
- `.env.example` — example environment variables
- `Cargo.toml` — crate metadata
- `LICENSE` — software license
- `CONTRIBUTING.md` — contributor guide
- `docs/usage.md` — extended usage documentation
- `docs/troubleshooting.md` — troubleshooting guide

## Acceptance Criteria

- [ ] README contains quick start with all 4 success metrics addressed
- [ ] Example configs are copy-paste ready and valid
- [ ] All CI checks pass (`cargo test`, `cargo clippy`, `cargo fmt`)
- [ ] Docker instructions work on both `amd64` and `arm64`
- [ ] No hardcoded secrets in any committed file

## Test Plan

- Manual: follow README quick start on a clean machine
- Lint: `cargo clippy --all-targets && cargo fmt --check`
- Test: `cargo test --all-targets`

## Observability

- README documents how to enable `DEBUG` and `TRACE` logs
- README documents how to check `docker logs` for issues

## Compliance

- No secrets in examples (use placeholder URLs)
- License is permissive (MIT/Apache-2.0 recommended for Rust)

## Risks & Mitigations

- Risk: README becomes stale after code changes — Mitigation: include README review in PR template
- Risk: 10-minute setup claim is not achievable — Mitigation: time a clean install and iterate

## Dependencies & Sequencing

- Depends on:
  - [[story-03-002-docker-multi-arch-packaging]] (needs Docker instructions)
- Unblocks: None (last story)

## Definition of Done

- Code, tests, docs updated; CI green; story file updated; README quick start verified

## Commit Conventions

- Use conventional commits with module scoping, e.g., `docs(readme): rewrite quick start and add example configs`

## Changelog

- 2026-05-27: initialized story file
