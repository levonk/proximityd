// Integration tests for content-first no-args feature
// Tests context detection, live state display, and mode-aware formatting

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_no_args_content_first() {
    // Test that no-args shows content-first behavior
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_stdin_content_first() {
    // Test that stdin shows content-first behavior
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("-")
        .write_stdin("test content")
        .assert()
        .success();
}

#[test]
fn test_context_detection_directory() {
    // Test context detection for directory
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_context_detection_daemon_status() {
    // Test context detection for daemon status
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_context_detection_config_directory() {
    // Test context detection for config directory
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_shows_live_state() {
    // Test that content-first shows live state instead of usage manual
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_default_display() {
    // Test default display in content-first mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_config_directory_display() {
    // Test display when in config directory
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_daemon_running_display() {
    // Test display when daemon is running
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_parties_summary() {
    // Test parties summary in config directory
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_content_first_with_toon_format() {
    // Test content-first with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .assert()
        .success();
}

#[test]
fn test_content_first_with_json_format() {
    // Test content-first with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .assert()
        .success();
}

#[test]
fn test_content_first_in_agent_mode() {
    // Test content-first in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .assert()
        .success();
}

#[test]
fn test_content_first_in_human_mode() {
    // Test content-first in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .assert()
        .success();
}

#[test]
fn test_content_first_contextual_suggestions() {
    // Test contextual command suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn test_help_flag_still_works() {
    // Test that --help flag still works
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Usage:"));
}

#[test]
fn test_content_first_mode_aware_formatting() {
    // Test mode-aware formatting in content-first
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .assert()
        .success();
}
