---
story_id: "03-001"
story_title: "Pluggable Notifier Trait & Discord Webhook"
story_name: "pluggable-notifier-trait-and-discord-webhook"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 3
parallel_id: 1
branch: "feature/current/bluetooth-presence-notifier/story-03-001-pluggable-notifier-trait-and-discord-webhook"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["02-001"]
parallel_safe: true
modules: ["src/notifier/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend", "notifications"]
due: "2026-06-17"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Implement a trait-based notification system with a Discord webhook backend. The `Notifier` trait must be generic enough to support future `OsNativeNotifier`, `WebhookNotifier`, and `SlackNotifier`. The Discord MVP must support webhook URLs and bot tokens, with retry logic and exponential backoff.

## Sub-Tasks

- [x] Create `src/notifier/mod.rs` — module root with re-exports
- [x] Create `src/notifier/trait.rs` — `Notifier` trait with `notify(event: &PresenceEvent) -> Result<()>`
- [x] Create `src/notifier/discord.rs` — `DiscordNotifier` implementing webhook POST via `reqwest`
  - Support `discord_webhook_url` and `discord_bot_token` + channel ID
  - Format: `"{name} has entered the area"` / `"{name} has exited the area"`
  - Optional: include timestamp and MAC (controlled by config flag)
- [x] Create `src/notifier/registry.rs` — `NotifierRegistry` that builds active notifiers from `config.toml` `notifiers` array
- [x] Add `reqwest` to `Cargo.toml`
- [x] Add unit tests for `DiscordNotifier` message formatting using `mockito` or `wiremock`
- [x] Add unit tests for `NotifierRegistry` builder logic
- [x] Wire notifier calls into detection engine so transitions trigger notifications

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/notifier/mod.rs` — module root
- `src/notifier/trait.rs` — notifier trait definition
- `src/notifier/discord.rs` — Discord webhook implementation
- `src/notifier/registry.rs` — notifier builder/registry
- `src/notifier/discord.test.rs` — unit tests for Discord message formatting
- `src/notifier/registry.test.rs` — unit tests for registry builder
- `Cargo.toml` — add `reqwest`
- `src/detection/engine.rs` — add notification hook on transitions

## Acceptance Criteria

- [x] Discord webhook delivers `"{name} has entered/exited the area"` messages
- [x] Bot token path works and posts to specified channel ID
- [x] Webhook URL and bot token are never hardcoded; loaded from env or secrets
- [x] Retry up to 3 times with exponential backoff on delivery failure (NFR-2)
- [x] New notifier backends can be added by implementing the `Notifier` trait + registering in `registry.rs`

## Test Plan

- Unit: `cargo test notifier::`
- Integration: `cargo test --test discord_mock_test` (uses `mockito`)
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Add `tracing::info!` on successful notification delivery
- Add `tracing::warn!` on retry attempt
- Add `tracing::error!` on final delivery failure after retries

## Compliance

- Discord tokens/webhooks loaded from env or secret files; never in source
- No cloud upload of scan results

## Risks & Mitigations

- Risk: `reqwest` async runtime conflicts — Mitigation: use `reqwest` with `tokio` (already planned)
- Risk: Discord rate limiting — Mitigation: respect 429 responses in retry logic
- Risk: Blocking the scan loop on slow notification delivery — Mitification: spawn notification in a separate `tokio::task`

## Dependencies & Sequencing

- Depends on:
  - [[story-02-001-enter-exit-detection-with-debounce]] (needs presence events)
- Unblocks: 04-001 (observability hooks into notifier success/failure)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(notifier): add pluggable notifier trait and Discord webhook backend`

## Changelog

- 2026-05-27: initialized story file
