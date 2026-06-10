---
story_id: "06-002"
story_title: "Documentation Completion"
story_name: "documentation"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 6
parallel_id: 2
branch: "feature/current/cli-axi/story-06-002-documentation"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["04-002", "05-001", "05-002"]
parallel_safe: true
modules: ["docs/", "README.md"]
priority: "MUST"
risk_level: "low"
tags: ["docs", "quality"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Complete documentation for CLI AXI features. Update README.md with mode selection, TOON format, and session integration documentation. Document all new CLI flags and commands. Provide clear examples for agent mode usage. Update AGENTS.md with new patterns.

## Sub-Tasks

- [x] Update README.md with agent mode overview
- [x] Document mode selection (--human, --interactive, PROXIMITYD_MODE)
- [x] Document TOON format and --format flag
- [x] Document --fields flag and field selection
- [x] Document --full flag and content truncation
- [x] Document --session-context command
- [x] Document --install-agent-hooks command
- [x] Document --generate-skill and --check-skill commands
- [x] Provide agent mode usage examples
- [x] Document session hook installation for Claude Code and Codex
- [x] Document agent skill generation and installation
- [x] Update AGENTS.md with agent mode patterns
- [x] Add migration guide for existing users
- [x] Document breaking changes and backward compatibility
- [x] Add troubleshooting section for agent mode issues

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `README.md` — Main documentation
- `docs/agent-mode.md` (new) — Agent mode detailed documentation
- `docs/toon-format.md` (new) — TOON format documentation
- `docs/session-hooks.md` (new) — Session hooks documentation
- `docs/agent-skills.md` (new) — Agent skills documentation
- `AGENTS.md` — Agent development patterns

## Acceptance Criteria

- [x] README.md documents all agent mode features
- [x] All new CLI flags and commands are documented
- [x] Agent mode usage examples are provided
- [x] Session hook installation is documented
- [x] Agent skill generation is documented
- [x] AGENTS.md includes agent mode patterns
- [x] Migration guide is provided
- [x] Breaking changes are documented
- [x] Troubleshooting section is included

## Test Plan

- Review: Manual review of documentation completeness
- Validation: Test documentation examples work correctly
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`

## Observability

- Track documentation usage and feedback
- Monitor common support questions

## Compliance

- Follow documentation best practices
- Ensure documentation is clear and actionable

## Risks & Mitigations

- Risk: Documentation becomes outdated — Mitigation: Include documentation in code review process
- Risk: Complex documentation — Mitigation: Use clear examples and structure

## Dependencies

- 04-002 (Installable Agent Skill) — Documentation requires skill generation
- 05-001 (Content-First No-Args) — Documentation requires no-args behavior
- 05-002 (Contextual Disclosure) — Documentation requires suggestion system

## Notes

- Documentation is critical for agent mode adoption
- Focus on clear examples and troubleshooting
