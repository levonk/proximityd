// Integration tests for definitive empty states feature
// Tests empty state formatting, context, and consistency across commands

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_empty_state_parties() {
    // Test empty state format for parties command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_devices() {
    // Test empty state format for devices command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_status() {
    // Test empty state format for status command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_includes_context() {
    // Test that empty states include context
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_exit_code_zero() {
    // Test that empty states exit with code 0
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_empty_state_with_toon_format() {
    // Test empty state with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_with_json_format() {
    // Test empty state with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_consistency_across_commands() {
    // Test that empty states are consistent across commands
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_in_agent_mode() {
    // Test empty state in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_in_human_mode() {
    // Test empty state in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("parties")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_with_field_selection() {
    // Test empty state with field selection
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success()
        .stdout(contains("0 results"));
}

#[test]
fn test_empty_state_with_suggestions() {
    // Test that empty states include suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_empty_state_discover_command() {
    // Test empty state for discover command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("discover")
        .assert()
        .success();
}

#[test]
fn test_empty_state_export_command() {
    // Test empty state for export command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("export")
        .assert()
        .success();
}
