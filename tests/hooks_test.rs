//! Integration tests for session hooks

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_session_context_command() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "session-context", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cwd"))
        .stdout(predicate::str::contains("metadata"));
}

#[test]
fn test_session_context_toon_format() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "session-context", "--format", "toon"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cwd:"));
}

#[test]
fn test_session_context_compact_mode() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "session-context", "--format", "json", "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cwd"));
}

#[test]
fn test_session_context_invalid_format() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "session-context", "--format", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported format"));
}

#[test]
fn test_install_agent_hooks_claude() {
    let temp_dir = TempDir::new().unwrap();
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).unwrap();
    
    // Set HOME to temp directory for this test
    std::env::set_var("HOME", temp_dir.path());
    
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "install-agent-hooks", "--claude"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Claude Code"));
    
    // Verify settings file was created
    let settings_path = claude_dir.join("settings.json");
    assert!(settings_path.exists());
    
    // Verify hook was registered
    let content = fs::read_to_string(&settings_path).unwrap();
    assert!(content.contains("session-start"));
}

#[test]
fn test_install_agent_hooks_codex() {
    let temp_dir = TempDir::new().unwrap();
    let codex_dir = temp_dir.path().join(".codex");
    fs::create_dir_all(&codex_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "install-agent-hooks", "--codex"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Codex"));
    
    // Verify hooks file was created
    let hooks_path = codex_dir.join("hooks.json");
    assert!(hooks_path.exists());
    
    // Verify hook was registered
    let content = fs::read_to_string(&hooks_path).unwrap();
    assert!(content.contains("session-start"));
}

#[test]
fn test_install_agent_hooks_all() {
    let temp_dir = TempDir::new().unwrap();
    let claude_dir = temp_dir.path().join(".claude");
    let codex_dir = temp_dir.path().join(".codex");
    fs::create_dir_all(&claude_dir).unwrap();
    fs::create_dir_all(&codex_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "install-agent-hooks", "--all"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Claude Code"))
        .stdout(predicate::str::contains("Codex"));
}

#[test]
fn test_install_agent_hooks_idempotent() {
    let temp_dir = TempDir::new().unwrap();
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).unwrap();
    
    // First installation
    let mut cmd1 = Command::cargo_bin("proximityd").unwrap();
    cmd1.args(["hooks", "install-agent-hooks", "--claude"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
    
    // Second installation (should be idempotent)
    let mut cmd2 = Command::cargo_bin("proximityd").unwrap();
    cmd2.args(["hooks", "install-agent-hooks", "--claude"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
    
    // Settings file should still exist and be valid
    let settings_path = claude_dir.join("settings.json");
    assert!(settings_path.exists());
    let content = fs::read_to_string(&settings_path).unwrap();
    assert!(content.contains("session-start"));
}

#[test]
fn test_install_agent_hooks_no_flags() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.args(["hooks", "install-agent-hooks"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No hooks installed"));
}
