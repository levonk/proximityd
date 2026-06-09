---
story_id: "04-002"
story_title: "Installable Agent Skill"
story_name: "agent-skill"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 4
parallel_id: 2
branch: "feature/current/cli-axi/story-04-002-agent-skill"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["04-001"]
parallel_safe: true
modules: ["skill.rs (new)", "docs/"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement installable agent skill support. Generate SKILL.md from no-args home view content. Add --check-skill build step for CI. Strip live state from skill. Rewrite command examples to non-interactive form. Include trigger-shaped frontmatter.

## Sub-Tasks

- [x] Create skill.rs module for skill generation
- [x] Implement --generate-skill command to output SKILL.md content
- [x] Generate SKILL.md from no-args home view
- [x] Strip live state from skill (static only, no dynamic data)
- [x] Rewrite command examples to non-interactive form
- [x] Include trigger-shaped frontmatter (name, description)
- [x] Add --check-skill build step to CI
- [x] Implement skill staleness detection
- [x] Add skill generation tests
- [x] Update README.md to document hook and skill paths
- [x] Support skill installation via agentskills.io
- [x] Add template-based skill generation from CLI metadata

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/skill.rs` (new) — Skill generation logic
- `src/cli/skill.rs` (new) — Skill CLI commands
- `SKILL.md` (new) — Generated agent skill
- `tests/skill_test.rs` (new) — Skill tests
- `README.md` — Documentation for hook and skill paths

## Acceptance Criteria

- [x] --generate-skill outputs valid SKILL.md content
- [x] SKILL.md is generated from no-args home view
- [x] Live state is stripped from skill
- [x] Command examples are non-interactive
- [x] Trigger frontmatter is included
- [x] --check-skill fails if skill is stale
- [x] Skill generation is template-based
- [x] README documents both hook and skill paths

## Test Plan

- Unit: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- cargo test skill`
- Integration: Test skill generation and staleness detection
- CI: Test --check-skill in CI pipeline
- Lint: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just lint-internal`
- Types: `devbox run -c /Users/micro/p/gh/levonk/proximityd -- just typecheck-internal`

## Observability

- Log skill generation events
- Track skill staleness detection

## Compliance

- Follow AXI agent skill requirements
- Ensure skill format is compatible with agentskills.io

## Risks & Mitigations

- Risk: Skill format changes — Mitigation: Version skill format and detect compatibility
- Risk: Stale skill in commits — Mitigation: CI check-skill prevents this

## Dependencies

- 04-001 (Session Hook Infrastructure) — Skill generation uses session context

## Notes

- Agent skills provide alternative to hooks
- Users only need one (hook or skill), not both
