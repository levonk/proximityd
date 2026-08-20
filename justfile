# proximityd - Rust CLI Development Commands
# Standard justfile following ADR-20260131001

_log := '
_jv_has() {
  local cat="$1"
  local v="${JUST_LOG:-0}"
  case "$v" in
    1|all) return 0 ;;
    0|"") return 1 ;;
  esac
  v="${v//startend/start,end}"
  echo ",$v," | grep -q ",$cat,"
}
log_info()   { _jv_has info   && echo "$*" || true; }
log_start()  { _jv_has start  && echo "▶ $*" || true; }
log_end()    { _jv_has end    && echo "✔ $*" || true; }
log_status() { _jv_has status && echo "$*" || true; }
log_warn()   { echo "⚠️  $*" >&2; }
log_error()  { echo "❌ $*" >&2; }
log_startend() {
  local msg="$1"; shift
  local rc
  _jv_has start && echo "▶ $msg" || true
  rc=0; "$@" || rc=$?
  _jv_has end && echo "✔ $msg complete" || true
  return $rc
}
'

# Devbox auto-detection: run impl target directly if in devbox,
# re-exec via devbox run if not, or fail with doctor diagnostic.
_devbox target *args:
    #!/usr/bin/env bash
    {{_log}}
    if [ "${DEVBOX_SHELL_ENABLED:-0}" = "1" ]; then
        exec just "{{target}}" {{args}}
    elif command -v devbox >/dev/null 2>&1; then
        exec devbox run -- just "{{target}}" {{args}}
    else
        log_error "devbox not found in PATH."
        log_warn "Running doctor to diagnose environment issues..."
        just doctor 2>/dev/null || true
        exit 1
    fi

# Default recipe
default:
    @just --list

# Normal targets - Developer interface (REQUIRED)
clean:
    @just _devbox clean_impl

dev:
    @just _devbox dev_impl

build:
    @just _devbox build_impl

test:
    @just _devbox test_impl --quiet

lint:
    @just _devbox lint_impl

typecheck:
    @just _devbox typecheck_impl

release:
    @just _devbox release_impl

# Bootstrap recipes (REQUIRED)
bootstrap:
    @just _devbox bootstrap_impl

# Prime recipes (REQUIRED)
prime:
    @just _devbox prime_impl

# Health and diagnostics (REQUIRED)
doctor:
    @just _devbox doctor_impl

# Quality checks (OPTIONAL but RECOMMENDED)
quality:
    @just format-check
    @just lint
    @just test
    @just typecheck

format-check:
    @just _devbox format_check_impl

man:
    @just _devbox man_impl

# Memory and task management targets (NEW)
doc-search:
    @just _devbox doc_search_impl

tasks:
    @just _devbox tasks_impl

task-ready:
    @just _devbox task_ready_impl

task-start:
    @just _devbox task_start_impl

# Language-specific commands for Rust CLI
# Development setup (OPTIONAL)
setup:
    #!/usr/bin/env bash
    {{_log}}
    log_end "Rust CLI development environment ready!"

# Docker recipes
docker-build:
    @just _devbox docker_build_impl

docker-run:
    @just _devbox docker_run_impl

docker-stop:
    @just _devbox docker_stop_impl

# Help target
help:
    echo "🦀 proximityd - Rust CLI Application"
    echo ""
    echo "Standard commands:"
    echo "  just bootstrap    - Initialize the development environment"
    echo "  just build        - Build the project"
    echo "  just test         - Run tests"
    echo "  just lint         - Run linting"
    echo "  just typecheck    - Run type checking"
    echo "  just dev           - Run in development mode"
    echo "  just clean         - Clean build artifacts"
    echo "  just doctor        - Check environment health"
    echo "  just quality       - Run all quality checks"
    echo "  just release       - Full release pipeline"
    echo "  just prime         - Index documentation and update repository"
    echo ""
    echo "Memory & Task Management:"
    echo "  just doc-search    - Search documentation and memory"
    echo "  just tasks         - List current tasks"
    echo "  just task-ready    - Get next available task"
    echo "  just task-start    - Start working on available task"
    echo ""
    echo "Docker commands:"
    echo "  just docker-build  - Build Docker image"
    echo "  just docker-run    - Run with docker-compose"
    echo "  just docker-stop   - Stop docker-compose services"
    echo ""
    echo "Rust-specific commands:"
    echo "  just debug         - Build in debug mode"
    echo "  just install       - Install binary locally"
    echo "  just test-coverage - Run tests with coverage"
    echo "  just format        - Format code"
    echo "  just doc           - Generate documentation"
    echo "  just audit         - Audit dependencies"
    echo ""
    echo "Internal commands (for devbox scripts):"
    echo "  just *_impl        - Internal implementations"

# =============================================================================
# Implementation targets (private)
# =============================================================================

[private]
bootstrap_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Install dependencies and initialize memory management
    log_end "Rust CLI bootstrap complete for proximityd!"

    # Initialize tkr for task management
    if command -v tkr >/dev/null 2>&1; then
        if [ ! -d ".tickets" ]; then
            log_info "Initializing tkr..."
            tkr init || log_warn "tkr init failed"
        else
            log_info "tkr already initialized"
        fi
    fi

    # Create memory directory structure for Obsidian
    log_info "Setting up memory structure..."
    mkdir -p memory/{00-inbox,01-projects,02-decisions,03-patterns,04-learnings,05-references,98-logs,99-daily}

    # Create Obsidian configuration if needed
    if [ ! -d ".obsidian" ]; then
        mkdir -p .obsidian
        echo "# Obsidian configuration" > .obsidian/config.md
    fi

[private]
prime_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Update repository and index documentation
    log_info "Updating repository..."
    git fetch || log_warn "git fetch failed (check remote connectivity)"

    # qmd memory indexing
    if command -v qmd >/dev/null 2>&1; then
        log_info "Indexing documentation with qmd..."
        qmd index docs/ internal-docs/ memory/ README.md || log_warn "qmd indexing failed"
        log_end "qmd indexing complete"
    else
        log_info "Skipping qmd (not installed)"
    fi

    log_end "Rust CLI priming complete"

[private]
doctor_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    # Check Rust CLI environment
    echo "🔍 Checking proximityd development environment..."
    if ! cargo --version >/dev/null 2>&1; then
        echo "❌ Error: cargo not found" >&2
        echo "💡 Suggestion: Ensure Rust toolchain is installed" >&2
        exit 1
    fi
    if ! just --version >/dev/null 2>&1; then
        echo "❌ Error: just not found" >&2
        echo "💡 Suggestion: Ensure just is installed" >&2
        exit 1
    fi
    if [ ! -f Cargo.toml ]; then
        echo "❌ Error: Cargo.toml not found (expected in project root)" >&2
        exit 1
    fi
    echo "✅ OK: Rust toolchain + just + Cargo.toml present"

    # Check for memory management tools
    if command -v qmd >/dev/null 2>&1; then
        echo "✅ OK: qmd available for memory search"
    else
        echo "⚠️  WARNING: qmd not found (install with: cargo install qmd)"
    fi

    if command -v tkr >/dev/null 2>&1; then
        echo "✅ OK: tkr available for task management"
    else
        echo "⚠️  WARNING: tkr not found (install from: https://github.com/levonk/tkr)"
    fi

    # Check for memory structure
    if [ -d memory ]; then
        echo "✅ OK: memory/ directory found"
    else
        echo "⚠️  WARNING: memory/ directory missing (run 'just bootstrap' to create)"
    fi

    echo "🚀 Ready to develop proximityd!"

[private]
format_check_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    log_start "Checking code format"
    cargo fmt -- --check
    log_end "Format check complete"

[private]
man_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    log_start "Generating man pages"
    mkdir -p target/man
    cargo run --bin proximityd -- man > target/man/proximityd.1
    cargo run --bin proximityd -- man status > target/man/proximityd-status.1
    cargo run --bin proximityd -- man export > target/man/proximityd-export.1
    cargo run --bin proximityd -- man discover > target/man/proximityd-discover.1
    cargo run --bin proximityd -- man install > target/man/proximityd-install.1
    cargo run --bin proximityd -- man uninstall > target/man/proximityd-uninstall.1
    cargo run --bin proximityd -- man completion > target/man/proximityd-completion.1
    log_end "Man pages generated in target/man/"

[private]
doc_search_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Search memory and documentation with qmd
    if command -v qmd >/dev/null 2>&1; then
        query="${1:-.}"  # Default to show all if no query
        qmd search "$query" | head -20
    else
        echo "qmd not found - falling back to ripgrep..."
        rg --type md "$query" docs/ internal-docs/ memory/ || true
    fi

[private]
tasks_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # List current tasks
    if command -v tkr >/dev/null 2>&1; then
        tkr list --status=open
    else
        echo "tkr not found"
    fi

[private]
task_ready_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Get next available task
    if command -v tkr >/dev/null 2>&1; then
        tkr ready
    else
        echo "tkr not found"
    fi

[private]
task_start_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Start working on available task
    if command -v tkr >/dev/null 2>&1; then
        task_id=$(tkr ready | head -1 | cut -d' ' -f1)
        if [ -n "$task_id" ]; then
            tkr start "$task_id"
            log_info "Started task: $task_id"
        else
            log_info "No available tasks"
        fi
    else
        echo "tkr not found"
    fi

[private]
clean_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Clean build artifacts
    cargo clean
    log_end "Build artifacts removed"

[private]
build_impl:
    # Build the project in debug mode
    cargo build

[private]
release_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    log_start "Starting release pipeline for proximityd"
    just lint_impl
    just test_impl
    just typecheck_impl
    just build_release_impl
    just man_impl
    log_end "Release complete! Binary available at target/release/proximityd"

[private]
build_release_impl:
    # Build the project in release mode
    cargo build --release

[private]
debug_impl:
    # Build the project in debug mode
    cargo build

[private]
install_impl:
    # Install the binary locally
    cargo install --path .

[private]
lint_impl:
    # Lint the code using clippy
    cargo clippy -- -D warnings

[private]
test_impl *args:
    # Run tests
    cargo test {{args}}

[private]
typecheck_impl:
    # Run type checking (cargo check)
    cargo check

[private]
dev_impl:
    # Run the application in development mode
    cargo run

[private]
run_impl:
    # Run the application with arguments
    cargo run

# Additional Rust-specific targets
[private]
test_coverage_impl:
    # Run tests with coverage
    cargo tarpaulin --out Html

[private]
format_impl:
    # Format code with rustfmt
    cargo fmt

[private]
check_skill_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Check if skill file is stale (outdated)
    if [ -f "SKILL.md" ]; then
        cargo run --bin proximityd -- skill check SKILL.md
    else
        log_info "SKILL.md not found - skipping skill check"
    fi

[private]
doc_impl:
    # Generate documentation
    cargo doc --open

[private]
audit_impl:
    # Audit dependencies
    cargo audit

# Docker recipes
[private]
docker_build_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    ./run.sh build

[private]
docker_run_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    ./run.sh compose-up

[private]
docker_stop_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    ./run.sh compose-down
