---
story_id: "03-002"
story_title: "Docker Multi-Arch Packaging"
story_name: "docker-multi-arch-packaging"
prd_name: "bluetooth-presence-notifier"
prd_file: "docs/requirements/20260527-initial-reqs/prd-bluetooth-presence-notifier.md"
phase: 3
parallel_id: 2
branch: "feature/current/bluetooth-presence-notifier/story-03-002-docker-multi-arch-packaging"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "01-003"]
parallel_safe: true
modules: ["Dockerfile", "docker-compose.yml"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "devops", "docker"]
due: "2026-06-17"
created_at: "2026-05-27"
updated_at: "2026-05-27"
---

## Summary

Replace the existing stub `Dockerfile` and `docker-compose.yml` with production-hardened, multi-arch (`amd64` + `arm64`) container packaging. The container must run as a non-root `btnotify` user, use minimal Linux capabilities for Bluetooth access, and include a `HEALTHCHECK`.

## Sub-Tasks

- [ ] Rewrite `Dockerfile`:
  - Multi-stage build from `rust:1-slim` builder + `debian:bookworm-slim` runtime
  - Install BlueZ client libraries (`libbluetooth3`, `dbus`) in runtime image
  - Create `btnotify` user with UID/GID 1000 and `USER btnotify`
  - Add `HEALTHCHECK` command (e.g., `btnotify --health-check` or PID check)
  - Remove package managers and build tools from final image
- [ ] Rewrite `docker-compose.yml`:
  - Mount `/var/run/dbus:/var/run/dbus` for D-Bus access
  - Use `host` network mode (required for Bluetooth discovery)
  - Mount `BTNOTIFY_CONFIG_DIR` volume for config persistence
  - Drop unnecessary capabilities; document required ones
- [ ] Add `.dockerignore` to minimize build context
- [ ] Test build on both `linux/amd64` and `linux/arm64` via `docker buildx`
- [ ] Add `run.sh` or `just docker-build` command for developer convenience
- [ ] Update `Cargo.toml` description to match PRD

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `Dockerfile` — multi-stage container build
- `docker-compose.yml` — compose service definition
- `.dockerignore` — build context exclusions
- `run.sh` — convenience build/run script
- `justfile` — add `docker-build` and `docker-run` recipes
- `Cargo.toml` — update description field

## Acceptance Criteria

- [ ] Container builds and runs on both `amd64` and `arm64`
- [ ] Container does not run as root
- [ ] `HEALTHCHECK` reports healthy when scan loop is active
- [ ] Bluetooth adapter is accessible inside container with minimal capabilities
- [ ] Config and device mapping files can be mounted in via volume
- [ ] Image size is reasonable (< 200 MB target)

## Test Plan

- Build: `docker buildx build --platform linux/amd64,linux/arm64 -t btnotify:latest .`
- Run: `docker-compose up --build`
- Lint: `hadolint Dockerfile` (if available)

## Observability

- `HEALTHCHECK` logs visible in `docker ps` and container logs
- `tracing` JSON logs available via `docker logs`

## Compliance

- No secrets in image layers
- Non-root execution enforced
- Minimal attack surface (no shell, no package manager in final image if possible)

## Risks & Mitigations

- Risk: `--privileged` is tempting for Bluetooth but violates NFR-3 — Mitigation: document exact `--cap-add` flags; test without `--privileged`
- Risk: `host` network mode limits Swarm/K8s deployability — Mitigation: document as known limitation for BLE; future work could use macvlan
- Risk: Multi-arch builds are slow — Mitigation: use `docker buildx` with cross-compilation or CI runners

## Dependencies & Sequencing

- Depends on:
  - [[story-01-001-config-device-mapping-loading]] (needs config path for volume mount)
  - [[story-01-002-ble-scanning-loop]] (needs Bluetooth runtime deps)
  - [[story-01-003-device-state-tracking-structures]] (needs daemon runtime)
- Unblocks: 04-002 (docs reference Docker packaging)

## Definition of Done

- Dockerfile, compose file, and docs updated; image builds on both arches; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(docker): add multi-arch hardened container packaging`

## Changelog

- 2026-05-27: initialized story file
