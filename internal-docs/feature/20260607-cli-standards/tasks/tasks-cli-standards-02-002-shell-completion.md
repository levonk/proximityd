---
story_id: "02-002"
story_title: "Generate shell completion scripts"
story_name: "shell-completion"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 2
parallel_id: 2
branch: "feature/current/cli-standards/story-02-002-shell-completion"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["src/cli/completion"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement shell completion script generation for bash, zsh, and fish shells. This story focuses on the completion generation logic, which will be used by the install functionality in story 02-001. Completions should cover all commands, subcommands, flags, and arguments.

## Sub-Tasks

- [ ] Create `src/cli/completion.rs` module
- [ ] Implement bash completion generation using clap
- [ ] Implement zsh completion generation using clap
- [ ] Implement fish completion generation using clap
- [ ] Ensure completions cover all CLI commands and subcommands
- [ ] Ensure completions cover all flags and arguments
- [ ] Add completion subcommand to CLI: `proximityd completion <shell>`
- [ ] Support shells: bash, zsh, fish
- [ ] Output completion script to stdout by default
- [ ] Add `--output` flag to write completion to file
- [ ] Add tests for completion generation
- [ ] Test completion scripts manually in each shell

## Relevant Files

- `src/cli/completion.rs` — NEW FILE for completion generation
- `src/cli/mod.rs` — Export completion module
- `src/main.rs` — Add completion subcommand
- `src/lib.rs` — Add cli module export
- `tests/cli_tests.rs` — Add completion tests

## Acceptance Criteria

- [ ] `proximityd completion bash` generates valid bash completion
- [ ] `proximityd completion zsh` generates valid zsh completion
- [ ] `proximityd completion fish` generates valid fish completion
- [ ] Completions cover all commands: status, export, discover, etc.
- [ ] Completions cover all flags: --help, --version, --config, etc.
- [ ] `proximityd completion bash --output /path/to/file` writes to file
- [ ] Generated completion scripts are syntactically valid
- [ ] All completion functionality has tests

## Test Plan

- Unit: Tests for completion generation in completion.test.rs
- Integration: CLI tests for completion command
- Manual: Source completion scripts and verify they work in bash, zsh, fish

## Observability

- No specific logging needed (completion is output-only)

## Compliance

- Completion scripts must not expose sensitive data
- Completion must work with standard shell versions (bash 4.0+, zsh 5.0+, fish 3.0+)

## Risks & Mitigations

- Risk: Completion might not work with older shell versions
  - Mitigation: Document minimum shell version requirements
- Risk: Completion might become outdated as CLI changes
  - Mitigation: Auto-generate from clap definition, not hand-written

## Dependencies & Sequencing

- Depends on: None (can be done independently)
- Unblocks: Story 02-001 (install functionality uses this)

## Definition of Done

- Shell completion generation is implemented for all three shells
- Completion subcommand works correctly
- All completions cover commands, flags, and arguments
- Tests pass
- Manual testing confirms completions work in each shell

## Commit Conventions

- `feat(cli): add shell completion generation`
- `test(cli): add completion tests`