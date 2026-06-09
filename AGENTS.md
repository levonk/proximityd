# Agent Guidelines: proximityd

## Project Overview

**proximityd** is a Rust CLI application for generic presence detection. It scans for nearby Bluetooth devices, maps MAC addresses to human-readable labels via TOML config, and sends enter/exit notifications via pluggable notifiers (Discord is the primary implementation).

- **Repository**: https://github.com/levonk/proximityd
- **Language**: Rust 2021 Edition
- **Runtime**: Tokio async (daemon mode)
- **Platform constraint**: Daemon mode requires **Linux + BlueZ**. Builds on macOS but daemon is gated behind `#[cfg(target_os = "linux")]`.

---

## Build System & Commands

### Environment
- **devbox** manages the environment. Always use `devbox run` for automated/AI agent operations.
- `direnv` is configured via `.envrc` for automatic activation.
- Standard pattern: `direnv → devbox → just (*-internal) → cargo`

### For AI Agents (always use `devbox run`)
```bash
# Build
devbox run just build-internal

# Test
devbox run just test-internal

# Lint (clippy with -D warnings)
devbox run just lint-internal

# Type check
devbox run just typecheck-internal

# Full release pipeline
devbox run just release-internal

# Docker
devbox run just docker-build-internal
devbox run just docker-run-internal
devbox run just docker-stop-internal
```

### For Human Developers
```bash
just build       # → devbox run build → just build-internal
just test
just lint
just typecheck
just doctor      # Health check
just quality     # lint + test + typecheck
```

### Install/Uninstall
```bash
# Install proximityd (generates shell completions and initializes config)
proximityd install

# Uninstall proximityd (removes completions, prompts for config removal)
proximityd uninstall

# Force uninstall without prompts
proximityd uninstall --force

# Generate shell completion scripts manually
proximityd --generate bash  # Output to stdout
proximityd --generate zsh
proximityd --generate fish
```

### New CLI Features for Agents

Agents should be aware of the following new CLI features:

- **Install/Uninstall**: Use `proximityd install` to initialize config files and shell completions. Use `--dry-run` to preview changes.
- **TUI Mode**: The `--interactive` flag launches a terminal-based UI for configuration. Agents should avoid TUI mode as it requires interactive terminal input.
- **Man Pages**: Use `proximityd man [subcommand]` to view documentation in pager.
- **Pager Integration**: Use `--no-pager` to disable pager when scripting output.
- **Dry-Run Mode**: Use `--dry-run` to preview daemon operations without making changes.
- **Confirmation Prompts**: Use `--force` to bypass confirmation prompts in scripts.
- **Progress Indicators**: Use `--quiet` to suppress progress bars and spinners in scripts.
- **Standard Exit Codes**: Exit codes follow standard conventions (0=success, 1=error, 2=usage, 3=config, 4=permission, 5=network).

### TUI Mode Considerations for Agents

The TUI mode (`--interactive`) is designed for human interaction and should not be used by agents:
- It requires interactive terminal input (keyboard navigation)
- It may not work in non-TTY environments
- For automated configuration, use direct config file editing instead

### Internal Targets (called by devbox scripts, not directly by agents)
| Target | Command |
|--------|---------|
| `build-internal` | `cargo build` |
| `test-internal` | `cargo test` |
| `lint-internal` | `cargo clippy -- -D warnings` |
| `typecheck-internal` | `cargo check` |
| `build-release-internal` | `cargo build --release` |
| `format-internal` | `cargo fmt` |
| `clean-internal` | `cargo clean` |

---

## Project Structure

```
src/
  main.rs           # CLI entry point (clap), daemon mode, logging init
  lib.rs            # Public module re-exports
  cli/              # CLI utilities (install/uninstall)
    install.rs      # Install/uninstall functionality
  config/           # TOML config loading
    app.rs          # AppConfig struct (scan intervals, thresholds, notifiers)
    devices.rs      # DevicesConfig (MAC → name mapping)
    loader.rs       # File resolution (~/.config/proximityd/ or env override)
  detection/        # Presence detection logic
    engine.rs       # Core detection engine
    bridge.rs       # Bridges scan events → detection state
    debounce.rs     # Enter/exit debounce logic
  notifier/         # Pluggable notifications
    trait.rs        # Notifier trait
    discord.rs      # Discord webhook/bot notifier
    registry.rs     # Notifier registry from config
  state/            # Presence state tracking
    table.rs        # PresenceStateTable (who is present/absent)
    events.rs       # Event types (Enter, Exit)
    types.rs        # State machine types
tests/
  cli_tests.rs      # CLI integration tests (assert_cmd)
  ble_scan_test.rs  # BLE scan integration tests
```

---

## Common Mistakes & Pitfalls

### 1. Do NOT forget `devbox run` for automated operations
- The normal `just` targets delegate to `devbox run <script>`, which then calls `just *-internal`.
- If you run `cargo test` or `cargo build` directly without ensuring the devbox environment is active, you may miss dependencies or environment variables.
- **AI agent rule**: Always prefix with `devbox run just <target>-internal`.

### 2. Daemon mode is Linux-only
- `run_daemon` is gated behind `#[cfg(target_os = "linux")]`.
- On non-Linux, `--daemon` exits with an error: `"Daemon mode requires a BLE adapter; only Linux (BlueZ) is currently supported"`.
- Do not attempt to refactor away the `cfg` gate or make daemon cross-platform without explicit user direction.

### 3. BlueZ is a Linux-only dependency
- `bluez-async` is under `[target.'cfg(target_os = "linux")'.dependencies]`.
- Building on macOS succeeds because the dependency is conditional, but BLE functionality is unavailable.

### 4. Config files are runtime-required
- Daemon mode loads `config.toml` and `presence.toml` at startup.
- Default config dir: `~/.config/proximityd/` (or `PROXIMITYD_CONFIG_DIR` override).
- Example configs are in repo root: `config.example.toml`, `presence.example.toml`.
- Run `proximityd install` to initialize config files and shell completions.

### 5. Logging is config-aware but CLI/env takes precedence
- Log level resolution order: `PROXIMITYD_LOG_LEVEL` env → CLI `-q`/`-v` flags → `config.toml` → default `INFO`.
- `PROXIMITYD_LOG_FORMAT` accepts `json` or `pretty`; auto-detects based on TTY if unset.

### 6. Docker requires host network + D-Bus
- The container needs `--network host` and `/var/run/dbus` mounted for BlueZ access.
- The Dockerfile drops to non-root user `proximityd` (UID/GID 1000).

### 7. Test structure
- Unit tests are co-located with source (`*.test.rs` files, e.g. `scan_loop.test.rs`).
- Integration tests are in `tests/` directory.
- Tests use `assert_cmd`, `predicates`, and `tempfile`.

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI parsing (derive macro) |
| `tokio` | Async runtime |
| `bluez-async` | BLE scanning (Linux only) |
| `serde` + `toml` | Config deserialization |
| `tracing` + `tracing-subscriber` | Structured logging |
| `reqwest` | HTTP client (Discord webhook) |
| `anyhow` / `thiserror` | Error handling |

---

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `PROXIMITYD_CONFIG_DIR` | Directory for `config.toml` and `presence.toml` |
| `PROXIMITYD_CONFIG` | Override config file path (CLI `--config`) |
| `PROXIMITYD_DISCORD_WEBHOOK` | Discord webhook URL override |
| `PROXIMITYD_LOG_LEVEL` | Override log level |
| `PROXIMITYD_LOG_FORMAT` | `json` or `pretty` |
| `NO_COLOR` | Disable ANSI colors |
| `RUST_LOG` | Standard tracing env filter |
| `RUST_BACKTRACE` | Backtrace setting (devbox default: `1`) |

---

## Docker Quick Reference

```bash
# Build image
./run.sh build

# Multi-arch build
./run.sh buildx

# Compose
./run.sh compose-up
./run.sh compose-down
```

Dockerfile stages:
- **Builder**: `rust:1-slim` + `libbluetooth-dev` + `libdbus-1-dev`
- **Runtime**: `debian:bookworm-slim` + `libbluetooth3` + `dbus`, runs as `proximityd` user

---

## Commit & Quality Gates

- Use [Conventional Commits](https://www.conventionalcommits.org/): `feat(module):`, `fix(module):`, `docs(module):`, etc.
- Before committing, run `just quality` (wraps `cargo fmt --check`, `cargo clippy`, `cargo test`, and `cargo check`).
