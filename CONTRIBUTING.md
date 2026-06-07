# Contributing to proximityd

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

## Licensing

By submitting a contribution to this repository, you agree to the following:

1. You retain the copyright to your contribution.
2. You grant the project maintainers a **perpetual, irrevocable, worldwide, non-exclusive, royalty-free, sublicensable license** to:
   - use, reproduce, modify, and distribute your contribution;
   - prepare derivative works from your contribution;
   - sublicense your contribution under any terms whatsoever (including proprietary, open-source, or copyleft licenses).
3. You confirm that you have the legal right to make this contribution (you authored the code, or your employer has authorized the contribution).

### Why this matters for dual-licensing

`proximityd` is released under AGPL-3.0 *and* a commercial license. In order to offer the commercial license, the project must have the right to distribute your contribution under non-AGPL terms. This broad grant is the standard mechanism used by MongoDB, Elastic, and every other dual-license project.

You are **not** assigning your copyright away — you still own it. You are simply granting the project the flexibility to license the combined work under terms that may differ from AGPL. Your contribution remains available to the community under AGPL-3.0 in every release.

If you cannot agree to these terms, please do not submit a pull request. If you have questions about the licensing implications for your specific situation, open a discussion before contributing.

## Pull Request Process

1. Ensure `cargo test`, `cargo clippy`, and `cargo fmt --check` all pass.
2. Update documentation (README, examples) if behavior changes.
3. Keep commits focused and atomic.
