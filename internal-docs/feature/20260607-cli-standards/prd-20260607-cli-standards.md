---
# Product Requirements Document (PRD)

## Introduction / Overview
- **Feature name:** CLI Standards Compliance and Legacy Migration Removal
- **Summary:** Remove legacy config file format migration logic and implement all missing CLI standards from ADR-20260607001 to bring proximityd into full compliance with the CLI Tool Standards ADR.
- **Context:**
  - proximityd currently maintains legacy migration logic to convert old `devices.toml` format to the new `presence.toml` format introduced in story 01-004. This migration code adds complexity, has test failures (migrate_simple_devices ordering issue), and increases maintenance burden.
  - The CLI Tool Standards ADR (adr-20260607001-cli-tool-standards.md) defines 35 comprehensive standards for CLI programs. proximityd implements many of these already but is missing several key standards.
  - This feature will simplify the codebase by removing migration logic and bring proximityd into full compliance with the CLI standards ADR, improving user experience and operational posture.

## Goals
- Remove all legacy config file format migration logic (`src/config/migrate.rs` and related code)
- Reduce codebase complexity and eliminate maintenance burden of migration logic
- Fix the pre-existing test failure in `config::migrate::tests::migrate_simple_devices`
- Implement all missing CLI standards from ADR-20260607001 that proximityd does not currently have
- Provide clear error messages for users with legacy config files, directing them to manual migration
- Ensure proximityd is a reference implementation of the CLI Tool Standards ADR

## User Stories

### As a developer working on proximityd
- I want the codebase to be simpler and easier to maintain
- So that I can focus on new features rather than debugging migration logic
- I want all tests to pass without ordering dependencies
- So that CI is reliable and test suite maintenance is straightforward

### As a user upgrading proximityd
- I want clear error messages if I have an old config format
- So that I know exactly what action to take to upgrade
- I want comprehensive shell completion
- So that I can discover commands and options easily
- I want man page documentation
- So that I can access help offline without internet

### As an operator deploying proximityd
- I want install/uninstall functionality
- So that deployment is automated and consistent
- I want health check endpoints
- So that container orchestration can monitor service health
- I want TUI mode for interactive configuration
- So that I can configure complex settings without editing TOML files manually

## Functional Requirements

### Legacy Migration Removal
1. **Remove migration module**
   - Delete `src/config/migrate.rs`
   - Remove `migrate_devices_to_presence()` function from `src/config/loader.rs`
   - Remove auto-migration logic that creates `.bak` backups
   - Remove migration-related tests from `src/config/migrate.rs` (if any)

2. **Update config loader**
   - Remove automatic detection and migration of legacy `devices.toml` format
   - If `devices.toml` is detected instead of `presence.toml`, exit with clear error message
   - Error message must include:
     - Description: "Legacy config format detected"
     - Suggestion: "Rename devices.toml to presence.toml and update format according to documentation"
     - Link to migration guide in README

3. **Remove deprecated config fields**
   - Remove backward compatibility fields from `AppConfig` that were added during story 01-004
   - Ensure all config loading uses only the new presence.toml format

4. **Update documentation**
   - Remove references to legacy `devices.toml` format from README.md
   - Remove migration guide or deprecation notices
   - Update examples to use only presence.toml format

### Missing CLI Standards Implementation

5. **Install/Uninstall Functionality (Standard #4)**
   - Add `--install` flag that:
     - Generates shell completion scripts for bash/zsh/fish
     - Initializes default config files in `~/.config/proximityd/`
     - Sets up required environment variables
     - Prints installation summary with next steps
   - Add `--uninstall` flag that:
     - Removes shell completion scripts
     - Offers to remove config files (with confirmation)
     - Cleans up any generated artifacts
   - Add `install` subcommand as alternative to `--install` flag

6. **TUI Mode (Standard #9)**
   - Add `--interactive` or `--tui` flag for interactive configuration
   - TUI should allow:
     - View and modify all config sections (general, scanner, detection, discovery, notifiers)
     - Add/edit/remove parties, devices, and identifiers
     - Test notifier configurations
     - Save changes to config file
   - Use a TUI library compatible with Rust (e.g., `ratatui`, `crossterm`)

7. **Man Pages (Standard #18)**
   - Generate traditional Unix man pages for:
     - Main `proximityd` command
     - Major subcommands (status, export, discover, migrate if kept)
   - Make accessible via `man proximityd` and `--man` flag
   - Use a man page generator (e.g., `clap_mangen`)

8. **Pager Integration (Standard #19)**
   - Auto-pager for long output (respect `PAGER` env var, default to `less`)
   - Add `--no-pager` flag to bypass paging
   - Apply pager to: `status` command output, `discover` command output, help text

9. **Dry-Run Mode (Standard #10)**
   - Add `--dry-run` flag to applicable commands:
     - `migrate` (if kept)
     - Config validation commands
     - Any destructive operations
   - Dry-run should show exactly what would be done without making changes

10. **Confirmation Prompts (Standard #11)**
    - Require confirmation for destructive operations with `--force` flag to bypass
    - Apply to: `--uninstall`, config file overwrites, database deletion commands
    - Prompt format: "Are you sure you want to [action]? [y/N]"

11. **Progress Indicators (Standard #12)**
    - Show progress bars or spinners for long-running operations
    - Apply to: daemon startup, signal log export, discovery analysis
    - Must respect `--quiet` flag (no progress indicators in quiet mode)

12. **Standard Exit Codes (Standard #8)**
    - Ensure all exit codes follow standard:
      - 0: success
      - 1: generic error
      - 2: usage error
      - Additional specific codes for different error types
    - Document exit codes in man pages and help text

13. **File Reference Formatting (Standard #15)**
    - Ensure all file references with line numbers use VSCode-compatible format
    - Format: `file:///absolute/path/to/file:line:column` or `file:line:column`
    - Apply to: error messages, log output, test failures

14. **Shell Completion (Standard #17)**
    - Provide shell completion scripts for bash, zsh, and fish
    - Auto-generate using clap's completion features
    - Include completions for all subcommands, flags, and arguments
    - Install via `--install` flag or manual placement

15. **Config File Initialization (Standard #3)**
    - On first run, if no config file exists, create default config with:
     - All settings commented out
     - Default values and explanations for each option
     - Example configurations for common use cases
    - Apply to both `config.toml` and `presence.toml`

16. **Input & Globbing (Standard #5)**
    - Support recursive `**/*` globbing where applicable
    - Support stdin via `-` or piped input for applicable commands
    - Process files or stdin interchangeably where relevant

17. **Terminal Size Awareness (Standard #22)**
    - Detect terminal size on startup
    - Adjust output formatting based on terminal width
    - Handle resize events where possible (especially for TUI mode)

18. **Resource Limits (Standard #26)**
    - Provide `--max-memory` flag for memory-intensive operations
    - Provide `--max-cpu` flag for CPU-intensive operations
    - Apply to: discovery analysis, signal log export, long-running scans
    - Enforce limits via platform-appropriate mechanisms

19. **Testing Coverage (Standard #27)**
    - Add tests for all new CLI standards functionality:
     - Install/uninstall behavior
     - TUI mode (mocked terminal)
     - Man page generation
     - Pager integration
     - Dry-run mode
     - Confirmation prompts
     - Progress indicators
     - Exit codes
     - Shell completion scripts
     - Config file initialization
     - Terminal size awareness
     - Resource limits

## Non-Functional Requirements

### Performance
- TUI mode must be responsive with <100ms latency for user interactions
- Install/uninstall operations must complete within 5 seconds
- Man page generation must complete within 2 seconds
- Shell completion scripts must not slow down shell startup time

### Compatibility
- Shell completion must work on bash 4.0+, zsh 5.0+, fish 3.0+
- Man pages must be compatible with standard Unix man implementation
- TUI mode must work on Linux, macOS, and Windows (with fallback to text mode on Windows if needed)
- Pager integration must respect platform-specific pager preferences

### Usability
- Error messages must be clear, actionable, and include suggestions
- TUI mode must be intuitive with keyboard shortcuts displayed
- Install/uninstall must require minimal user interaction
- All new features must be discoverable via `--help` and shell completion

### Maintainability
- Code for new features must follow existing proximityd patterns
- TUI code must be modular and testable
- Shell completion must be auto-generated, not manually maintained
- Man pages must be generated from code, not hand-written

### Security
- Install/uninstall must not require elevated privileges except where necessary
- Config file initialization must not overwrite existing files without confirmation
- TUI mode must not expose sensitive data in terminal history
- Resource limits must be enforced to prevent denial of service

## Technical Considerations

### Dependencies to Add
- `ratatui` or similar TUI library for interactive mode
- `clap_mangen` for man page generation
- `indicatif` for progress indicators (if not already using)
- `console` for terminal size detection and pager integration

### Modules to Modify
- `src/main.rs` - Add new CLI flags and subcommands
- `src/config/loader.rs` - Remove migration logic, add config initialization
- `src/config/migrate.rs` - DELETE this file
- `src/notifier/` - Update for any new CLI patterns
- `tests/cli_tests.rs` - Add tests for new functionality

### Modules to Create
- `src/cli/install.rs` - Install/uninstall logic
- `src/cli/tui.rs` - TUI mode implementation
- `src/cli/completion.rs` - Shell completion generation
- `src/cli/man.rs` - Man page generation

### Data Model Changes
- Remove deprecated fields from `AppConfig`
- No changes to `PresenceConfig` or other core data models

### Configuration Changes
- Default config templates for initialization
- No changes to config schema (presence.toml format remains unchanged)

### Integration Points
- Shell completion scripts install to: `/usr/local/share/bash-completion/completions/`, `/usr/local/share/zsh/site-functions/`, `~/.local/share/fish/vendor_completions.d/`
- Man pages install to: `/usr/local/share/man/man1/`
- Config initialization writes to: `~/.config/proximityd/`

### Backward Compatibility
- **Breaking change**: Users with legacy `devices.toml` will get error message
- Must provide migration guide in README for manual conversion
- Consider providing a standalone migration script in `scripts/` directory

## Success Metrics

- All 35 CLI standards from ADR-20260607001 are implemented and verified
- Legacy migration code is completely removed (0 references to migrate.rs)
- Test suite passes with 100% success rate (no migrate_simple_devices failures)
- Code coverage for new functionality >80%
- Shell completion works for all commands and flags
- TUI mode allows configuration of all config sections
- Man pages are generated and install correctly
- Install/uninstall commands work end-to-end
- User documentation is updated to reflect all changes
- No increase in binary size >20% from new dependencies

## Open Questions

1. Should we provide a standalone migration script in `scripts/` directory for users with legacy configs, or rely solely on documentation?
2. Should TUI mode be optional via feature flag to reduce binary size for users who don't need it?
3. What is the minimum Rust version required for new dependencies (ratatui, etc.)?
4. Should we support Windows terminal for TUI mode, or restrict to Unix-like systems?
5. How should we handle config file initialization when `~/.config/proximityd/` doesn't exist?

## Dependencies

### External Dependencies
- ADR-20260607001: CLI Tool Standards (must conform to all 35 standards)
- Rust crates: ratatui, clap_mangen, indicatif, console (versions TBD)

### Internal Dependencies
- Story 01-004 (Config Model) - must be complete before removing migration logic
- Story 05-002 (status/export CLI) - must integrate with new CLI patterns
- Existing config schema (presence.toml) - no changes required

### Blocked By
- None - this is a new feature that can proceed independently

### Blocking
- Future features that depend on clean config loading
- Documentation updates that assume no legacy migration

## Timeline / Milestones

### Milestone 1: Legacy Migration Removal (Week 1)
- Remove `src/config/migrate.rs`
- Update `src/config/loader.rs` to remove migration logic
- Add error handling for legacy config detection
- Update documentation to remove migration references
- Tests: Verify error messages, ensure no migration code remains

### Milestone 2: Install/Uninstall and Shell Completion (Week 1-2)
- Implement `--install` and `--uninstall` flags
- Generate shell completion scripts for bash/zsh/fish
- Add config file initialization logic
- Tests: Install/uninstall end-to-end, completion script generation

### Milestone 3: TUI Mode (Week 2-3)
- Implement basic TUI framework
- Add config section editors
- Add party/device/identifier management
- Add notifier testing
- Tests: TUI mode with mocked terminal, all config sections editable

### Milestone 4: Man Pages and Pager Integration (Week 3)
- Generate man pages using clap_mangen
- Implement pager auto-detection and integration
- Add `--no-pager` flag
- Tests: Man page generation, pager integration

### Milestone 5: Remaining CLI Standards (Week 3-4)
- Implement dry-run mode
- Add confirmation prompts with `--force`
- Add progress indicators
- Implement standard exit codes
- Add file reference formatting
- Implement terminal size awareness
- Add resource limits
- Tests: Comprehensive test coverage for all new features

### Milestone 6: Documentation and Verification (Week 4)
- Update README.md with all new features
- Update AGENTS.md with new patterns
- Verify all 35 CLI standards are implemented
- Run full test suite
- Performance testing
- Final review and polish

### Target Release
- Include in next minor version release ( vX.Y.0 )
- Breaking change notice in changelog about legacy config removal

---
*Generated from PRD template*