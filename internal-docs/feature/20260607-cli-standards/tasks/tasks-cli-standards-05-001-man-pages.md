---
story_id: "05-001"
story_title: "Generate man pages using clap_mangen"
story_name: "man-pages"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 5
parallel_id: 1
branch: "feature/current/cli-standards/story-05-001-man-pages"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/cli/man"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "docs"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Generate traditional Unix man pages for proximityd using clap_mangen. Man pages should be generated for the main command and all major subcommands. Make man pages accessible via `man proximityd` and add a `--man` flag to display man page content.

## Sub-Tasks

- [ ] Add clap_mangen dependency to Cargo.toml
- [ ] Create `src/cli/man.rs` module
- [ ] Implement man page generation function:
  - Generate man page for main proximityd command
  - Generate man pages for subcommands: status, export, discover, install, completion
  - Use clap_mangen to generate from CLI definition
- [ ] Add `man` subcommand to CLI: `proximityd man [command]`
- [ ] Add `--man` flag to display man page content to stdout
- [ ] Implement man page installation in install functionality (story 02-001):
  - Install to `/usr/local/share/man/man1/`
  - Handle permission errors gracefully
  - Skip installation if directory not writable
- [ ] Add man page generation to build process:
  - Add `man` target to justfile
  - Generate man pages during release build
- [ ] Add tests for man page generation
- [ ] Verify generated man pages are valid
- [ ] Test man page display with `man proximityd`

## Relevant Files

- `Cargo.toml` — Add clap_mangen dependency
- `src/cli/man.rs` — NEW FILE for man page generation
- `src/cli/mod.rs` — Export man module
- `src/main.rs` — Add man subcommand and --man flag
- `src/lib.rs` — Add cli module export
- `justfile` — Add man generation target
- `tests/cli_tests.rs` — Add man page tests

## Acceptance Criteria

- [ ] `proximityd man` generates man page for main command
- [ ] `proximityd man status` generates man page for status command
- [ ] `proximityd man export` generates man page for export command
- [ ] `proximityd man discover` generates man page for discover command
- [ ] `proximityd --man` displays man page content to stdout
- [ ] Generated man pages are valid roff format
- [ ] Man pages can be installed to system directory
- [ ] Man pages display correctly with `man proximityd`
- [ ] All man page generation has tests

## Test Plan

- Unit: Tests for man page generation
- Integration: Test man subcommand and --man flag
- Manual: Generate man pages and verify with `man` command
- Manual: Test man page installation

## Observability

- Log man page generation at DEBUG level
- Log installation success/failure at INFO level

## Compliance

- Man pages must not contain sensitive information
- Installation must respect system permissions

## Risks & Mitigations

- Risk: Man page installation might fail due to permissions
  - Mitigation: Handle permission errors gracefully
  - Mitigation: Provide clear error messages with sudo suggestions
- Risk: Generated man pages might be incomplete
  - Mitigation: Use clap_mangen for automatic generation from CLI definition
  - Mitigation: Manually review generated content

## Dependencies & Sequencing

- Depends on: None
- Unblocks: None

## Definition of Done

- Man page generation is implemented
- All major commands have man pages
- Man pages can be displayed and installed
- Tests pass
- Manual testing confirms man pages work

## Commit Conventions

- `feat(cli): add man page generation with clap_mangen`
- `feat(cli): add man subcommand and --man flag`
- `test(cli): add man page tests`