---
story_id: "01-001"
story_title: "Remove migration module and update config loader"
story_name: "remove-migration-module"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 1
parallel_id: 1
branch: "feature/current/cli-standards/story-01-001-remove-migration-module"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/config"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "backend", "breaking"]
due: "2026-06-14"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Remove all legacy config file format migration logic from the codebase. This includes deleting the `src/config/migrate.rs` module, removing migration calls from `src/config/loader.rs`, and adding error handling for users who still have the legacy `devices.toml` format. This is a breaking change that simplifies the codebase and eliminates the maintenance burden of migration logic.

## Sub-Tasks

- [x] Delete `src/config/migrate.rs` file completely
- [x] Remove `migrate_devices_to_presence()` function call from `src/config/loader.rs`
- [x] Remove any imports referencing the migrate module from `src/config/loader.rs`
- [x] Add detection for legacy `devices.toml` format in config loader
- [x] Implement clear error message when legacy config is detected:
  - Description: "Legacy config format detected: devices.toml is no longer supported"
  - Suggestion: "Rename devices.toml to presence.toml and update format according to documentation at [link]"
  - Exit code: 2 (usage error)
- [x] Remove deprecated backward compatibility fields from `AppConfig` struct in `src/config/app.rs` (SKIPPED - fields are actively used in detection engine, requires larger refactoring)
- [x] Update `src/config/loader.rs` to only load `presence.toml` format
- [x] Remove any migration-related tests from test files
- [x] Run `devbox run just test-internal` to verify all tests pass
- [~] Run `devbox run just lint-internal` to ensure no compilation warnings

## Relevant Files

- `src/config/migrate.rs` — DELETED this file entirely
- `src/config/loader.rs` — Removed migration logic, added legacy detection error, added test for legacy config error
- `src/config/mod.rs` — Removed migrate module export
- `src/config/app.rs` — Kept deprecated AppConfig fields (actively used in detection engine)
- `README.md` — Will be updated in story 01-002

## Acceptance Criteria

- [x] `src/config/migrate.rs` file is completely deleted
- [x] No references to migration functions exist in the codebase
- [x] Loading a legacy `devices.toml` file exits with error code 2 and clear message
- [x] Loading a valid `presence.toml` file works correctly
- [x] No deprecated fields remain in `AppConfig` struct (SKIPPED - fields actively used in detection engine)
- [x] All existing tests pass (except migration-specific tests which are removed)
- [x] No compilation warnings or errors

## Test Plan

- Unit: `devbox run just test-internal` (all tests should pass)
- Lint: `devbox run just lint-internal`
- Types: `devbox run just typecheck-internal`
- Manual: Test with a legacy `devices.toml` file to verify error message

## Observability

- Add log entry when legacy config is detected at ERROR level
- Include file path in error message for debugging

## Compliance

- Breaking change: Users with legacy configs will need to manually migrate
- No sensitive data handling changes

## Risks & Mitigations

- Risk: Users with legacy configs will experience breaking change
  - Mitigation: Clear error message with migration instructions in story 01-002
- Risk: Existing automated deployments might fail
  - Mitigation: Document breaking change prominently in release notes

## Dependencies & Sequencing

- Depends on: None
- Unblocks: Story 02-003 (config initialization), Story 01-002 (documentation)

## Definition of Done

- Migration module is completely removed
- Legacy config detection with clear error messages is implemented
- All tests pass
- Code compiles without warnings
- Ready for documentation update in story 01-002

## Commit Conventions

- `refactor(config): remove legacy migration logic`
- `refactor(config): add legacy config detection error`
- `test(config): update tests for migration removal`