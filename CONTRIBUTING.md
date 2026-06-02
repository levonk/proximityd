# Contributing to btnotify

Thank you for your interest in contributing!

## Development Setup

This project uses [devbox](https://www.jetify.com/devbox) for reproducible development environments.

```bash
# Install dependencies and enter the shell
devbox shell

# Or run commands directly
devbox run just build-internal
devbox run just test-internal
devbox run just lint-internal
```

## Build & Test

```bash
# Build debug binary
just build-internal

# Run all tests
just test-internal

# Run lints
just lint-internal

# Run type check
just typecheck-internal

# Full quality check
just quality
```

## Project Structure

```
src/
  bluetooth/   # BLE scanning and BlueZ adapter
  config/      # TOML config loading (config.toml, devices.toml)
  detection/   # Enter/exit detection engine with debounce
  notifier/    # Pluggable notifier trait + Discord implementation
  state/       # Presence state tracking
  main.rs      # CLI entry point and daemon mode
  health.rs    # Health check endpoint
```

## Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(module): add new feature
fix(module): resolve bug
docs(module): update documentation
refactor(module): code restructuring
test(module): add or update tests
```

## Pull Request Process

1. Ensure `cargo test`, `cargo clippy`, and `cargo fmt --check` all pass.
2. Update documentation (README, examples) if behavior changes.
3. Keep commits focused and atomic.
