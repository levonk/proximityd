---
story_id: "04-001"
story_title: "Observability, Error Handling & Retry Logic"
story_name: "observability-error-handling-and-retry-logic"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 4
parallel_id: 1
branch: "feature/current/bluetooth-presence-notifier/story-04-001-observability-error-handling-and-retry-logic"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["src/"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "backend", "observability", "reliability"]
due: "2026-06-24"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Harden the daemon for production: add structured JSON logging, retry logic with exponential backoff for Discord delivery, graceful error recovery when the Bluetooth adapter disappears, and a health check endpoint/signal for Docker.

## Sub-Tasks

- [ ] Refactor `src/main.rs` logging to default to JSON format in container; pretty ANSI in terminal (already partially done, align with PRD)
- [ ] Add `BTNOTIFY_LOG_LEVEL` env var override with clear precedence: env > config > default (`INFO`)
- [ ] Implement retry logic in `src/notifier/discord.rs`: 3 attempts with exponential backoff (1s, 2s, 4s)
- [ ] Implement adapter recovery in `src/bluetooth/scan_loop.rs`: if scan fails, retry every 30 seconds; log error each time
- [ ] Add `src/health.rs` — `HealthCheck` struct with `is_healthy()` based on last successful scan time and last notification delivery
- [ ] Add `--health-check` CLI flag for Docker `HEALTHCHECK`
- [ ] Add graceful shutdown on `SIGTERM`/`SIGINT` (improve existing `ctrlc` handler to finish current scan cycle before exit)
- [ ] Add memory usage guard: warn if RSS exceeds 64 MB (NFR-1)
- [ ] Add unit tests for retry backoff timing
- [ ] Add integration test for graceful shutdown

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/main.rs` — logging init and shutdown handling
- `src/notifier/discord.rs` — add retry wrapper
- `src/bluetooth/scan_loop.rs` — add adapter recovery
- `src/health.rs` — health check logic
- `src/health.test.rs` — unit tests for health criteria
- `Dockerfile` — wire `--health-check` into `HEALTHCHECK` instruction
- `Cargo.toml` — add `tokio` signal handling if not already present

## Acceptance Criteria

- [ ] Discord retry succeeds on transient failures; gives up after 3 tries and logs error
- [ ] Bluetooth adapter loss triggers retry every 30 seconds without crash
- [ ] `HEALTHCHECK` fails if no scan completed in last 2 minutes
- [ ] Graceful shutdown completes current scan cycle before exiting
- [ ] JSON logs include `level`, `timestamp`, `module`, `message`, and `correlation_id` fields
- [ ] App does not crash on malformed TOML; exits with code `1` and clear error

## Test Plan

- Unit: `cargo test health::`, `cargo test notifier::retry`, `cargo test bluetooth::recovery`
- Integration: `cargo test --test graceful_shutdown`
- Lint: `cargo clippy --all-targets`
- Types: `cargo check`

## Observability

- Structured JSON logs with stable field names
- Health check status exposed via CLI and container runtime
- Metrics: scan_cycle_duration_ms, notification_delivery_duration_ms, presence_state_count

## Compliance

- No PII in logs beyond device names (user-controlled config)
- Error messages do not leak Discord webhook URLs

## Risks & Mitigations

- Risk: Retry logic hammers Discord during outages — Mitigation: max 3 retries; consider circuit breaker in future
- Risk: Health check is too aggressive and kills container during normal gaps — Mitigation: 2-minute window is generous for 15s scan intervals

## Dependencies & Sequencing

- Depends on:
  - [[story-03-001-pluggable-notifier-trait-and-discord-webhook]] (needs notifier to retry)
- Unblocks: None (last backend story)

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(health): add health check and graceful shutdown`

## Changelog

- 2026-05-27: initialized story file
