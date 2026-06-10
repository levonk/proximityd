---
story_id: "08-001"
story_title: "Update documentation and verify all AXI requirements"
story_name: "axi-docs-verification"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 8
parallel_id: 1
branch: "feature/current/cli-axi/story-08-001-axi-docs-verification"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "01-003", "02-001", "02-002", "02-003", "03-001", "04-001", "04-002", "05-001", "05-002", "06-001", "06-002"]
parallel_safe: true
modules: ["README.md", "AGENTS.md", "docs/"]
priority: "MUST"
risk_level: "low"
tags: ["docs", "quality"]
due: "2026-06-15"
created_at: "2026-06-09"
updated_at: "2026-06-09"
---

## Summary

Update all project documentation to reflect new CLI AXI features and verify that all AXI requirements from the PRD are implemented. This includes updating README.md with agent mode features, updating AGENTS.md with AXI patterns, and creating a verification checklist to ensure full AXI compliance.

## Sub-Tasks

- [x] Update README.md with AXI features:
  - Document agent mode overview and benefits
  - Document mode selection (--human, --mode, PROXIMITYD_MODE)
  - Document TOON format and --format flag
  - Document --fields flag and field selection
  - Document --full flag and content truncation
  - Document session context and hooks
  - Document agent skill generation
  - Document content-first no-args behavior
  - Document contextual disclosure and suggestions
  - Provide agent mode usage examples
- [x] Update AGENTS.md with AXI patterns:
  - Document agent mode detection and usage
  - Document TOON format for agents
  - Document minimal schema patterns
  - Document truncation escape hatches
  - Document aggregate usage
  - Document empty state handling
  - Document structured error parsing
  - Document session hook integration
  - Document skill generation workflows
- [x] Create AXI verification checklist:
  - Go through all AXI requirements from PRD-20260608-cli-axi
  - Mark each as implemented or not implemented
  - Add evidence/test case for each implemented requirement
  - Document any partial implementations
- [x] Verify all implemented AXI requirements:
  - Requirement 1: Mode Selection (agent/human/auto detection)
  - Requirement 2: TOON Format Implementation
  - Requirement 3: Minimal Default Schemas
  - Requirement 4: Content Truncation
  - Requirement 5: Pre-computed Aggregates
  - Requirement 6: Definitive Empty States
  - Requirement 7: Structured Errors & Exit Codes
  - Requirement 8: Session Hook Infrastructure
  - Requirement 9: Installable Agent Skill
  - Requirement 10: Content-First No-Args
  - Requirement 11: Contextual Disclosure
  - Requirement 12: Integration Testing
  - Requirement 13: Documentation Completion
- [x] Document any requirements not implemented with rationale
- [x] Add breaking change notice to release notes if applicable
- [x] Update CHANGELOG.md with all AXI features
- [x] Verify documentation is accurate and complete
- [x] Spell check all documentation
- [x] Link documentation to AXI specification reference

## Relevant Files

- `README.md` — Update with AXI features
- `AGENTS.md` — Update with AXI patterns
- `CHANGELOG.md` — Add AXI features and any breaking changes
- `internal-docs/feature/20260608-cli-axi/verification-checklist.md` — NEW FILE for AXI verification checklist

## Acceptance Criteria

- [x] README.md documents all AXI features
- [x] AGENTS.md documents AXI patterns for agents
- [x] Verification checklist covers all AXI requirements from PRD
- [x] All implemented requirements have evidence/test cases
- [x] Non-implemented requirements have documented rationale
- [x] Breaking changes (if any) are documented in release notes
- [x] CHANGELOG.md is comprehensive
- [x] Documentation is accurate and complete
- [x] No spelling errors in documentation
- [x] Documentation links to AXI specification

## Test Plan

- Manual: Review all documentation for accuracy
- Manual: Verify all examples in documentation work
- Manual: Check all links in documentation
- Manual: Verify verification checklist is complete
- Manual: Test documentation examples with actual CLI

## Observability

- Documentation is version-controlled
- Verification checklist is maintained
- AXI compliance is trackable

## Compliance

- Documentation must accurately reflect AXI implementation
- Must not document features that don't exist
- Must document breaking changes clearly
- Must align with AXI specification reference

## Risks & Mitigations

- Risk: Documentation might become outdated quickly
  - Mitigation: Keep documentation close to code
  - Mitigation: Update documentation as part of each story
- Risk: Some AXI requirements might not be applicable
  - Mitigation: Document rationale for non-implementation
  - Mitigation: Review with stakeholders if needed
- Risk: AXI specification might evolve
  - Mitigation: Link to specific specification version
  - Mitigation: Document which version was implemented

## Dependencies & Sequencing

- Depends on: All AXI implementation stories (01-001 through 06-002)
- Unblocks: AXI feature release

## Definition of Done

- All documentation is updated
- Verification checklist is complete
- All AXI requirements are verified
- Breaking changes (if any) are documented
- Documentation is accurate and complete
- AXI compliance is demonstrated

## Commit Conventions

- `docs(readme): update with AXI features`
- `docs(agents): update with AXI patterns`
- `docs(verification): add AXI verification checklist`
- `docs(changelog): add AXI features and breaking changes`
