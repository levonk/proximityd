---
story_id: "02-003"
story_title: "Add config file initialization"
story_name: "config-init"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 2
parallel_id: 3
branch: "feature/current/cli-standards/story-02-003-config-init"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["src/config/loader"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "config"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement automatic config file initialization on first run. When no config file exists in the expected location, create default config files with all settings commented out, including default values and explanations for each option. This applies to both `config.toml` and `presence.toml`.

## Sub-Tasks

- [x] Add default config template for `config.toml` with:
  - All settings commented out with `#`
  - Default values shown in comments
  - Explanations for each option
  - Example configurations for common use cases
- [x] Add default config template for `presence.toml` with:
  - Example party structure
  - Example device with identifiers
  - All fields commented with explanations
  - Common identifier type examples
- [x] Add function to detect if config directory exists
- [x] Add function to detect if config files exist
- [x] Add function to create config directory if missing
- [x] Add function to write default config templates
- [x] Integrate initialization into config loader in `src/config/loader.rs`:
  - On first run, detect missing config files
  - Create default templates automatically
  - Log INFO message when creating default configs
  - Do not overwrite existing files
- [x] Add `--init-config` flag to force re-initialization
- [x] Add tests for config initialization
- [x] Test first-run scenario with no config directory
- [x] Test scenario with partial config files

## Relevant Files

- `src/config/loader.rs` — Add initialization logic
- `src/config/templates.rs` — NEW FILE for config templates
- `src/config/mod.rs` — Export templates module
- `src/main.rs` — Add --init-config flag
- `tests/config_test.rs` — Add initialization tests

## Acceptance Criteria

- [x] First run with no config directory creates `~/.config/proximityd/`
- [x] First run creates default `config.toml` with commented settings
- [x] First run creates default `presence.toml` with examples
- [x] Existing config files are never overwritten
- [x] `--init-config` flag forces re-initialization with confirmation
- [x] Default configs are valid TOML and can be loaded
- [x] All config options have clear explanations in comments
- [x] Initialization is logged at INFO level
- [x] All initialization logic has tests

## Test Plan

- Unit: Tests for initialization logic in loader.test.rs
- Integration: Test first-run scenario
- Manual: Delete config directory and run proximityd
- Manual: Verify generated config files are valid

## Observability

- Log config directory creation at INFO level
- Log config file creation at INFO level
- Log when skipping existing files at DEBUG level

## Compliance

- Config initialization must respect user permissions
- Must not overwrite existing files without explicit flag
- Default configs must not contain sensitive information

## Risks & Mitigations

- Risk: Initialization might fail due to permission issues
  - Mitigation: Detect permission errors and provide clear error messages
- Risk: Default configs might not match user's needs
  - Mitigation: Provide comprehensive examples and documentation
- Risk: Users might be surprised by automatic file creation
  - Mitigation: Log clearly what files are being created and why

## Dependencies & Sequencing

- Depends on: 01-001 (clean config loading without migration logic)
- Unblocks: Story 02-001 (install uses this)

## Definition of Done

- Config initialization is implemented
- Default templates are comprehensive and well-documented
- First-run experience works smoothly
- All tests pass
- Documentation is updated

## Commit Conventions

- `feat(config): add automatic config file initialization`
- `feat(config): add default config templates`
- `test(config): add initialization tests`