---
story_id: "01-002"
story_title: "Update documentation for legacy config removal"
story_name: "update-legacy-docs"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 1
parallel_id: 2
branch: "feature/current/cli-standards/story-01-002-update-legacy-docs"
status: "done"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["README.md", "AGENTS.md"]
priority: "MUST"
risk_level: "medium"
tags: ["docs", "breaking"]
due: "2026-06-14"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Update all project documentation to remove references to the legacy `devices.toml` format and migration logic. This includes updating README.md, AGENTS.md, and any other documentation files that mention the old format. Add clear migration guide for users who need to convert from the old format to the new `presence.toml` format.

## Sub-Tasks

- [x] Search for all references to `devices.toml` in documentation files
- [x] Remove legacy config references from README.md
- [x] Remove migration guide or deprecation notices from README.md
- [x] Update config examples in README.md to use only `presence.toml` format
- [x] Add migration guide section to README.md for users with legacy configs:
  - Step-by-step instructions to convert `devices.toml` to `presence.toml`
  - Example conversion showing old vs new format
  - Link to presence.toml schema documentation
- [x] Update AGENTS.md to remove any references to legacy migration logic
- [x] Update AGENTS.md config section to reference only `presence.toml`
- [x] Check and update any example config files in repository root
- [x] Add breaking change notice to CHANGELOG.md (if exists)
- [x] Verify all documentation is consistent with new config-only approach

## Relevant Files

- `README.md` — Remove legacy references, add migration guide
- `AGENTS.md` — Update config section, remove migration references
- `config.example.toml` — Ensure it uses new format only
- `devices.example.toml` — DELETE or rename to presence.example.toml
- `CHANGELOG.md` — Add breaking change notice (if exists)
- `internal-docs/` — Check for any internal docs mentioning legacy format

## Acceptance Criteria

- [x] No references to `devices.toml` or legacy migration exist in README.md
- [x] Migration guide is present in README.md with clear steps
- [x] AGENTS.md references only `presence.toml` format
- [x] Example config files use new format only
- [x] Breaking change is documented in CHANGELOG.md
- [x] All documentation is consistent and accurate

## Test Plan

- Manual: Review all documentation files for legacy references
- Manual: Follow migration guide to verify instructions are correct
- Manual: Verify example configs match new schema

## Observability

- No code changes, only documentation updates

## Compliance

- Documentation must accurately reflect breaking change
- Migration guide must be clear and actionable

## Risks & Mitigations

- Risk: Users might miss migration guide
  - Mitigation: Place migration guide prominently in README near config section
  - Mitigation: Add breaking change notice at top of release notes

## Dependencies & Sequencing

- Depends on: None (can be done in parallel with 01-001)
- Unblocks: None

## Definition of Done

- All documentation updated to remove legacy references
- Migration guide is clear and complete
- Example configs use new format only
- Breaking change is documented

## Commit Conventions

- `docs(readme): remove legacy config references and add migration guide`
- `docs(agents): update config section to reference presence.toml only`
- `docs(changelog): add breaking change notice for legacy config removal`