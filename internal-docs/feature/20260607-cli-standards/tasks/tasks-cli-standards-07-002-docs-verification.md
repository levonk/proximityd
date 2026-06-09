---
story_id: "07-002"
story_title: "Update documentation and verify all CLI standards"
story_name: "docs-verification"
prd_name: "cli-standards"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-20260607-cli-standards.md"
phase: 7
parallel_id: 2
branch: "feature/current/cli-standards/story-07-002-docs-verification"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["02-001", "02-002", "02-003", "03-001", "04-001", "04-002", "04-003", "05-001", "05-002", "06-001", "06-002", "06-003", "06-004"]
parallel_safe: true
modules: ["README.md", "AGENTS.md"]
priority: "MUST"
risk_level: "low"
tags: ["docs", "quality"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Update all project documentation to reflect new CLI standards features and verify that all 35 CLI standards from ADR-20260607001 are implemented. This includes updating README.md with new features, updating AGENTS.md with new patterns, and creating a verification checklist to ensure full compliance.

## Sub-Tasks

- [x] Update README.md with new features:
  - Document install/uninstall functionality
  - Document shell completion usage
  - Document config initialization
  - Document TUI mode (--interactive flag)
  - Document man pages (--man flag)
  - Document pager integration (--no-pager flag)
  - Document dry-run mode
  - Document confirmation prompts and --force flag
  - Document progress indicators
  - Document standard exit codes
- [x] Update AGENTS.md with new patterns:
  - Document install/uninstall usage for agents
  - Document TUI mode considerations for agents
  - Document new CLI flags and their usage
  - Update build system commands if needed
- [x] Create CLI standards verification checklist:
  - Go through all 35 standards from ADR-20260607001
  - Mark each as implemented or not implemented
  - Add evidence/test case for each implemented standard
- [x] Verify all implemented CLI standards:
  - Standard 1: Standard arguments (--help, --version, --usage)
  - Standard 2: Configuration precedence
  - Standard 3: Config file initialization
  - Standard 4: Install/uninstall functionality
  - Standard 5: Input & globbing
  - Standard 6: Output discipline
  - Standard 7: Logging modes
  - Standard 8: Signals & exit codes
  - Standard 9: TUI mode
  - Standard 10: Dry-run mode
  - Standard 11: Confirmation prompts
  - Standard 12: Progress indicators
  - Standard 13: Daemon process support
  - Standard 14: Error message formatting
  - Standard 15: File reference formatting
  - Standard 16: URL formatting
  - Standard 17: Shell completion
  - Standard 18: Man pages
  - Standard 19: Pager integration
  - Standard 20: Subcommand organization
  - Standard 21: Configuration validation
  - Standard 22: Terminal size awareness
  - Standard 23: Environment variable naming
  - Standard 24: Cross-platform path handling
  - Standard 25: Credential/secret handling
  - Standard 26: Resource limits
  - Standard 27: Testing
  - Standard 28: Collection vs processing separation
  - Standard 29: Config file auto-migration (REMOVED per this PRD)
  - Standard 30: Structured logging with format auto-detection
  - Standard 31: Signal-based config reload
  - Standard 32: Health check for containers
  - Standard 33: Privacy mode with anonymous lists
  - Standard 34: Audit logging with retention
  - Standard 35: Legacy deprecation policy
- [x] Document any standards not implemented with rationale
- [x] Add breaking change notice to release notes
- [x] Update CHANGELOG.md with all new features
- [x] Verify documentation is accurate and complete
- [x] Spell check all documentation
- [x] Link documentation to ADR-20260607001

## Relevant Files

- `README.md` — Update with new features
- `AGENTS.md` — Update with new patterns
- `CHANGELOG.md` — Add new features and breaking changes
- `internal-docs/feature/20260607-cli-standards/verification-checklist.md` — NEW FILE for verification checklist

## Acceptance Criteria

- [x] README.md documents all new CLI features
- [x] AGENTS.md documents new patterns for agents
- [x] Verification checklist covers all 35 CLI standards
- [x] All implemented standards have evidence/test cases
- [x] Non-implemented standards have documented rationale
- [x] Breaking change is documented in release notes
- [x] CHANGELOG.md is comprehensive
- [x] Documentation is accurate and complete
- [x] No spelling errors in documentation
- [x] Documentation links to ADR-20260607001

## Test Plan

- Manual: Review all documentation for accuracy
- Manual: Verify all examples in documentation work
- Manual: Check all links in documentation
- Manual: Verify verification checklist is complete

## Observability

- Documentation is version-controlled
- Verification checklist is maintained

## Compliance

- Documentation must accurately reflect implementation
- Must not document features that don't exist
- Must document breaking changes clearly

## Risks & Mitigations

- Risk: Documentation might become outdated quickly
  - Mitigation: Keep documentation close to code
  - Mitigation: Update documentation as part of each story
- Risk: Some CLI standards might not be applicable
  - Mitigation: Document rationale for non-implementation
  - Mitigation: Review with stakeholders if needed

## Dependencies & Sequencing

- Depends on: All implementation stories (02-001 through 06-004)
- Unblocks: None

## Definition of Done

- All documentation is updated
- Verification checklist is complete
- All 35 CLI standards are verified
- Breaking change is documented
- Documentation is accurate and complete

## Commit Conventions

- `docs(readme): update with new CLI standards features`
- `docs(agents): update with new CLI patterns`
- `docs(verification): add CLI standards verification checklist`
- `docs(changelog): add new features and breaking changes`