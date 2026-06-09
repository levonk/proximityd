# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### CLI Standards Compliance (Phase 01-06)
- **Install/Uninstall Functionality**: Added `proximityd install` and `proximityd uninstall` commands for easy setup and cleanup
- **Shell Completion**: Added shell completion generation for bash, zsh, and fish via `proximityd completion <shell>`
- **Config Initialization**: Automatic config file creation on first run with `proximityd install` or `--init-config` flag
- **TUI Mode**: Added interactive terminal UI via `--interactive` flag for configuration management
- **Man Pages**: Added man page generation via `proximityd man [command]` and `--man` flag
- **Pager Integration**: Automatic paging for long output with `--no-pager` flag to disable
- **Dry-Run Mode**: Added `--dry-run` flag to preview operations without making changes
- **Confirmation Prompts**: Added confirmation prompts for destructive operations with `--force` flag to bypass
- **Progress Indicators**: Added progress bars and spinners for long-running operations with `--quiet` flag to suppress
- **Standard Exit Codes**: Updated exit codes to follow standard conventions (0=success, 1=error, 2=usage, 3=config, 4=permission, 5=network)
- **File Reference Formatting**: Added VSCode-compatible file reference formatting in error messages
- **Terminal Size Awareness**: Added terminal size detection for responsive output formatting
- **Resource Limits**: Added `--max-memory` and `--max-cpu` flags for Discover and Export commands
- **Input Globbing**: Added glob pattern support for file arguments with stdin input via `-`

#### TUI Features (Phase 04)
- **Config Section Editors**: TUI editors for all config sections (General, Privacy, Scanner, Detection, Discovery, Notifier)
- **Party/Device Management**: TUI screens for managing parties, devices, and identifiers
- **Notifier Testing**: TUI screen for testing configured notifiers with sample events

#### Multi-Signal Detection (Generic Presence Notify Feature)
- **WiFi ARP Scanner**: Cross-platform ARP table parsing with SNMP fallback
- **Ping Sweep Scanner**: ICMP ping sweep with fping support
- **mDNS Scanner**: Hostname discovery via avahi-browse (Linux) and dns-sd (macOS)
- **Signal Log**: SQLite-based logging of all scanner detections with location metadata
- **Discovery Engine**: Offline correlation analysis using Jaccard similarity
- **Suggestion Runtime**: Runtime device identification based on discovery suggestions
- **Hierarchical Location Model**: Building/floor/room/zone location hierarchy
- **GPS/IP Geolocation**: GPS and IP geolocation logging with non-blocking fetch

#### Notifiers
- **Slack Notifier**: Webhook-based Slack notifications with rich formatting
- **Webhook Notifier**: Generic HTTP webhook with configurable method and payload template
- **MQTT Notifier**: MQTT broker integration with configurable topic (requires `mqtt` feature)

### Changed

#### Breaking Changes
- **Legacy Config Migration Removed**: The legacy `devices.toml` format is no longer supported. Users must manually migrate to the new `presence.toml` format. See README.md for migration guide.
- **Exit Codes Updated**: Exit codes now follow standard conventions (0=success, 1=error, 2=usage, 3=config, 4=permission, 5=network). Scripts relying on specific exit codes may need updates.

#### Configuration
- **Config Structure**: Reorganized config.toml into nested sections (general, privacy, scanner, detection, discovery, notifiers)
- **Presence Model**: New hierarchical identity model with parties, devices, and identifiers replacing flat device mapping
- **Scanner Configuration**: Scanner configuration moved to `[scanner.<type>]` sections with enabled flags

### Removed

- **Legacy Migration Logic**: Removed automatic migration from `devices.toml` to `presence.toml`
- **Deprecated Config Fields**: Removed legacy config fields that were marked as deprecated

### Fixed

- **Test Ordering**: Fixed pre-existing test ordering issue in `config::migrate::tests::migrate_simple_devices`

### Security

- **Config Validation**: Enhanced config file validation with clear error messages
- **Credential Handling**: No hardcoded credentials; all sensitive data via environment variables

### Documentation

- **README Update**: Comprehensive documentation of new CLI features, TUI mode, and configuration
- **AGENTS Update**: Updated agent guidelines with new CLI patterns and considerations
- **Migration Guide**: Added step-by-step migration guide from legacy `devices.toml` format
- **Verification Checklist**: Created CLI standards verification checklist documenting compliance with all 35 standards from ADR-20260607001

## [0.1.0] - 2026-05-27

### Added

- Initial release of proximityd
- BLE device scanning via btleplug
- Basic presence detection with enter/exit notifications
- Discord webhook notifier
- TOML-based configuration
- Structured logging with tracing
- Docker support with multi-arch builds
