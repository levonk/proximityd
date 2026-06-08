---
story_id: "06-002"
story_title: "Documentation Completion"
story_name: "documentation"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 6
parallel_id: 2
branch: "feature/current/cli-axi/story-06-002-documentation"
status: "todo"
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

- [ ] Update README.md with agent mode overview
- [ ] Document mode selection (--human, --interactive, PROXIMITYD_MODE)
- [ ] Document TOON format and --format flag
- [ ] Document --fields flag and field selection
- [ ] Document --full flag and content truncation
- [ ] Document --session-context command
- [ ] Document --install-agent-hooks command
- [ ] Document --generate-skill and --check-skill commands
- [ ] Provide agent mode usage examples
- [ ] Document session hook installation for Claude Code and Codex
- [ ] Document agent skill generation and installation
- [ ] Update AGENTS.md with agent mode patterns
- [ ] Add migration guide for existing users
- [ ] Document breaking changes and backward compatibility
- [ ] Add troubleshooting section for agent mode issues

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `README.md` — Main documentation
- `docs/agent-mode.md` (new) — Agent mode detailed documentation
- `docs/toon-format.md` (new) — TOON format documentation
- `docs/session-hooks.md` (new) — Session hooks documentation
- `docs/agent-skills.md` (new) — Agent skills documentation
- `AGENTS.md` — Agent development patterns

## Acceptance Criteria

- [ ] README.md documents all agent mode features
- [ ] All new CLI flags and commands are documented
- [ ] Agent mode usage examples are provided
- [ ] Session hook installation is documented
- [ ] Agent skill generation is documented
- [ ] AGENTS.md includes agent mode patterns
- [ ] Migration guide is provided
- [ ] Breaking changes are documented
- [ ] Troubleshooting section is included

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
