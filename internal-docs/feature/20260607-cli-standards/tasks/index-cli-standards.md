# CLI Standards Compliance and Legacy Migration Removal - Task Index

## Overview

This index summarizes all stories for implementing CLI standards compliance and removing legacy config migration logic, as defined in PRD `prd-20260607-cli-standards.md`.

## Stories by Phase

### Phase 01: Legacy Migration Removal

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 01-001 | Remove migration module and update config loader | feature/current/cli-standards/story-01-001-remove-migration-module | None | Parallel-safe: true | src/config | [x] Done |
| 01-002 | Update documentation for legacy config removal | feature/current/cli-standards/story-01-002-update-legacy-docs | None | Parallel-safe: true | README.md, AGENTS.md | [x] Done |

### Phase 02: Install/Uninstall and Shell Completion

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 02-001 | Implement install/uninstall functionality | feature/current/cli-standards/story-02-001-install-uninstall | 01-001 | Parallel-safe: true | src/cli/install | [x] Done |
| 02-002 | Generate shell completion scripts | feature/current/cli-standards/story-02-002-shell-completion | None | Parallel-safe: true | src/cli/completion | [x] Done |
| 02-003 | Add config file initialization | feature/current/cli-standards/story-02-003-config-init | 01-001 | Parallel-safe: true | src/config/loader | [x] Done |

### Phase 03: TUI Mode Foundation

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 03-001 | Implement TUI framework and basic structure | feature/current/cli-standards/story-03-001-tui-framework | None | Parallel-safe: false | src/cli/tui | [x] Done |

### Phase 04: TUI Mode Features

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 04-001 | Add config section editors to TUI | feature/current/cli-standards/story-04-001-tui-config-editors | 03-001 | Parallel-safe: true | src/cli/tui | [x] Done |
| 04-002 | Add party/device/identifier management in TUI | feature/current/cli-standards/story-04-002-tui-party-management | 03-001 | Parallel-safe: true | src/cli/tui | [x] Done |
| 04-003 | Add notifier testing in TUI | feature/current/cli-standards/story-04-003-tui-notifier-testing | 03-001 | Parallel-safe: true | src/cli/tui | [x] Done |

### Phase 05: Man Pages and Pager Integration

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 05-001 | Generate man pages using clap_mangen | feature/current/cli-standards/story-05-001-man-pages | None | Parallel-safe: true | src/cli/man | [x] Done |
| 05-002 | Implement pager integration | feature/current/cli-standards/story-05-002-pager-integration | None | Parallel-safe: true | src/main.rs | [x] Done |

### Phase 06: Remaining CLI Standards

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 06-001 | Implement dry-run mode and confirmation prompts | feature/current/cli-standards/story-06-001-dry-run-confirm | None | Parallel-safe: true | src/main.rs | [x] Done |
| 06-002 | Add progress indicators and standard exit codes | feature/current/cli-standards/story-06-002-progress-exit-codes | None | Parallel-safe: true | src/main.rs | [x] Done |
| 06-003 | Implement file reference formatting and terminal size awareness | feature/current/cli-standards/story-06-003-file-ref-terminal | None | Parallel-safe: true | src/main.rs | [x] Done |
| 06-004 | Add resource limits and input globbing support | feature/current/cli-standards/story-06-004-resource-limits-globbing | None | Parallel-safe: true | src/main.rs | [x] Done |

### Phase 07: Testing and Documentation

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules |
| -------- | ----------- | ------ | ------------ | ------------- | ------- |
| 07-001 | Add comprehensive tests for all new CLI standards | feature/current/cli-standards/story-07-001-comprehensive-tests | 02-001, 02-002, 02-003, 03-001, 04-001, 04-002, 04-003, 05-001, 05-002, 06-001, 06-002, 06-003, 06-004 | Parallel-safe: false | tests/ | [x] Done |
| 07-002 | Update documentation and verify all CLI standards | feature/current/cli-standards/story-07-002-docs-verification | 02-001, 02-002, 02-003, 03-001, 04-001, 04-002, 04-003, 05-001, 05-002, 06-001, 06-002, 06-003, 06-004 | Parallel-safe: true | README.md, AGENTS.md |

## Summary

- **Total Stories**: 16
- **Total Phases**: 7
- **Parallel-safe Stories**: 14 (can be developed concurrently within phases)
- **Sequential Stories**: 2 (03-001, 07-001 - must be done before dependent phases)

## Implementation Notes

1. **Phase 01** removes legacy migration logic and updates documentation. Stories can be done in parallel.
2. **Phase 02** implements install/uninstall, shell completion, and config initialization. Stories 02-001 and 02-003 depend on 01-001 being complete. Story 02-002 is independent.
3. **Phase 03** is a foundation phase for TUI. Must be completed before Phase 04 TUI features.
4. **Phase 04** implements TUI features. All stories depend on 03-001 but can be done in parallel with each other.
5. **Phase 05** implements man pages and pager integration. Stories are independent and can be done in parallel.
6. **Phase 06** implements remaining CLI standards. All stories are independent and can be done in parallel.
7. **Phase 07** is testing and documentation. Story 07-001 depends on all implementation stories. Story 07-002 can be done in parallel with 07-001 once implementation is complete.

## Breaking Changes

- **Story 01-001**: Removes legacy `devices.toml` format support. Users with legacy configs will need to manually migrate to `presence.toml` format.
- **Story 06-002**: Updates exit codes to follow standard conventions. May break existing scripts that rely on specific exit codes.

## Dependencies External to This Feature

- ADR-20260607001: CLI Tool Standards (must conform to all 35 standards)
- Rust crates: ratatui, crossterm, clap_mangen, indicatif, console, pager (versions TBD)

## Target Release

- Include in next minor version release (vX.Y.0)
- Breaking changes must be prominently documented in release notes