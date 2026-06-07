---
story_id: "02-001"
story_title: "Implement install/uninstall functionality"
story_name: "install-uninstall"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 2
parallel_id: 1
branch: "feature/current/cli-standards/story-02-001-install-uninstall"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["src/cli/install"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "cli"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement `--install` and `--uninstall` flags (and corresponding subcommands) for proximityd. The install flag will generate shell completion scripts, initialize default config files, and set up required environment. The uninstall flag will clean up installed artifacts with confirmation prompts.

## Sub-Tasks

- [x] Create `src/cli/install.rs` module with install/uninstall logic
- [x] Add `install` subcommand to CLI in `src/main.rs`
- [x] Add `--install` flag to main command that triggers install subcommand
- [x] Add `--uninstall` flag to main command that triggers uninstall subcommand
- [x] Implement shell completion script generation:
  - Bash completion script
  - Zsh completion script
  - Fish completion script
  - Use clap's completion generation features
- [x] Implement default config file initialization:
  - Create `~/.config/proximityd/` directory if it doesn't exist
  - Generate `config.toml` with all settings commented out
  - Generate `presence.toml` with example parties/devices/identifiers
  - Include default values and explanations for each option
  - Do not overwrite existing files without confirmation
- [x] Implement environment setup (print instructions for required env vars)
- [x] Implement uninstall functionality:
  - Remove shell completion scripts from system directories
  - Offer to remove config files with confirmation prompt
  - Clean up any generated artifacts
  - Require `--force` flag to bypass confirmation
- [x] Add confirmation prompt for destructive operations
- [x] Print installation summary with next steps
- [x] Add tests for install/uninstall functionality
- [~] Update AGENTS.md with install/uninstall usage patterns

## Relevant Files

- `src/cli/install.rs` — NEW FILE for install/uninstall logic
- `src/cli/mod.rs` — Export install module
- `src/main.rs` — Add install/uninstall flags and subcommands
- `src/lib.rs` — Add cli module export
- `tests/cli_tests.rs` — Add install/uninstall tests
- `AGENTS.md` — Document install/uninstall usage

## Acceptance Criteria

- [x] `proximityd install` generates shell completion scripts
- [x] `proximityd install` initializes default config files in `~/.config/proximityd/`
- [x] `proximityd install` does not overwrite existing files without confirmation
- [x] `proximityd uninstall` removes completion scripts
- [x] `proximityd uninstall` offers to remove config files with confirmation
- [x] `proximityd uninstall --force` bypasses confirmation
- [x] Installation summary is printed with next steps
- [x] Shell completion scripts work for bash, zsh, and fish
- [x] All new functionality has tests

## Test Plan

- Unit: Tests for install/uninstall logic in install.test.rs
- Integration: CLI tests for install/uninstall commands
- Manual: Test install on fresh system
- Manual: Test uninstall and verify cleanup

## Observability

- Log installation actions at INFO level
- Log file paths of created/removed files
- Log confirmation prompt responses

## Compliance

- Install should not require elevated privileges except for system-wide completion scripts
- Config file initialization must respect user permissions
- Uninstall must not remove files without confirmation unless --force is used

## Risks & Mitigations

- Risk: Install might fail due to permission issues
  - Mitigation: Detect permission errors and provide clear error messages with sudo suggestions
- Risk: Uninstall might remove user data unexpectedly
  - Mitigation: Require explicit confirmation for config file removal
  - Mitigation: Default to preserving config files

## Dependencies & Sequencing

- Depends on: 01-001 (clean config loading without migration)
- Unblocks: None

## Definition of Done

- Install/uninstall functionality is fully implemented
- Shell completion scripts are generated correctly
- Config initialization works as expected
- All tests pass
- Documentation is updated

## Commit Conventions

- `feat(cli): add install/uninstall functionality`
- `feat(cli): add shell completion generation`
- `feat(cli): add config file initialization`
- `test(cli): add install/uninstall tests`