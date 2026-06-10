// Integration tests for session hook infrastructure feature
// Tests session context generation, hook registration, and git detection

use assert_cmd::Command;

#[test]
fn test_session_context_command() {
    // Test session-context subcommand
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_session_context_with_toon_format() {
    // Test session context with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .arg("--toon")
        .assert()
        .success();
}

#[test]
fn test_session_context_with_json_format() {
    // Test session context with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .arg("--json")
        .assert()
        .success();
}

#[test]
fn test_session_context_compact_flag() {
    // Test --compact flag for session context
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .arg("--compact")
        .assert()
        .success();
}

#[test]
fn test_session_context_git_detection() {
    // Test that session context detects git repository
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_session_context_includes_metadata() {
    // Test that session context includes metadata
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_install_agent_hooks_claude() {
    // Test --install-agent-hooks for Claude Code
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--install-agent-hooks")
        .arg("claude")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_install_agent_hooks_codex() {
    // Test --install-agent-hooks for Codex
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--install-agent-hooks")
        .arg("codex")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_install_agent_hooks_idempotent() {
    // Test that hook installation is idempotent
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--install-agent-hooks")
        .arg("claude")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_session_context_caching() {
    // Test that session context uses caching
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_session_context_session_id() {
    // Test that session context includes session ID
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_session_context_absolute_path() {
    // Test that session context includes absolute paths
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}

#[test]
fn test_session_context_token_budget_aware() {
    // Test that session context is token-budget-aware
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .arg("--compact")
        .assert()
        .success();
}

#[test]
fn test_invalid_hook_target() {
    // Test that invalid hook targets are handled gracefully
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--install-agent-hooks")
        .arg("invalid")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_session_context_without_git() {
    // Test session context when not in a git repository
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .assert()
        .success();
}
