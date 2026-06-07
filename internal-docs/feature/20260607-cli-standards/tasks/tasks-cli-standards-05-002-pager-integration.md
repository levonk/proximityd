---
story_id: "05-002"
story_title: "Implement pager integration"
story_name: "pager-integration"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 5
parallel_id: 2
branch: "feature/current/cli-standards/story-05-002-pager-integration"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/main.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement automatic pager integration for long output. Commands that produce long output (status, export, discover, help text) should automatically pipe through a pager (respecting PAGER env var, defaulting to less). Add a `--no-pager` flag to bypass paging when desired.

## Sub-Tasks

- [ ] Add pager library dependency to Cargo.toml (e.g., `pager` crate)
- [ ] Implement pager detection function:
  - Check PAGER environment variable
  - Default to `less` if PAGER not set
  - Fall back to no pager if pager not found
- [ ] Implement pager invocation for long output:
  - Detect if output is long (> terminal lines)
  - Pipe output through pager
  - Handle pager errors gracefully
- [ ] Add `--no-pager` flag to commands with long output:
  - status command
  - export command
  - discover command
  - help text
- [ ] Implement pager for status command output
- [ ] Implement pager for export command output
- [ ] Implement pager for discover command output
- [ ] Implement pager for help text
- [ ] Ensure pager respects `--quiet` flag (no pager in quiet mode)
- [ ] Add tests for pager integration
- [ ] Test with different pagers (less, more, etc.)

## Relevant Files

- `Cargo.toml` — Add pager dependency
- `src/main.rs` — Add pager integration and --no-pager flag
- `src/cli/pager.rs` — NEW FILE for pager utilities
- `src/cli/mod.rs` — Export pager module
- `tests/cli_tests.rs` — Add pager tests

## Acceptance Criteria

- [ ] Long output automatically uses pager (respects PAGER env var)
- [ ] Default pager is `less` when PAGER not set
- [ ] `--no-pager` flag bypasses paging
- [ ] Status command output uses pager when long
- [ ] Export command output uses pager when long
- [ ] Discover command output uses pager when long
- [ ] Help text uses pager when long
- [ ] Pager is not used in `--quiet` mode
- [ ] Pager errors are handled gracefully
- [ ] All pager integration has tests

## Test Plan

- Unit: Tests for pager detection and invocation
- Integration: Test pager with each command
- Manual: Test with different PAGER values
- Manual: Test --no-pager flag
- Manual: Test in --quiet mode

## Observability

- Log pager usage at DEBUG level
- Log pager errors at WARN level

## Compliance

- Pager must not interfere with programmatic output (JSON mode)
- Pager must respect user preferences (PAGER env var)

## Risks & Mitigations

- Risk: Pager might not work on all systems
  - Mitigation: Fall back to no pager if pager not found
  - Mitigation: Handle pager errors gracefully
- Risk: Pager might interfere with piping
  - Mitigation: Detect if output is being piped and skip pager
  - Mitigation: --no-pager flag always available

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- Pager integration is implemented
- All long-output commands use pager
- --no-pager flag works correctly
- Tests pass
- Manual testing confirms pager works

## Commit Conventions

- `feat(cli): add pager integration for long output`
- `feat(cli): add --no-pager flag`
- `test(cli): add pager tests`