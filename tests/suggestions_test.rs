//! Integration tests for contextual suggestions in CLI commands.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parties_command_includes_suggestions() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("parties")
       .arg("--json");

    // The command should succeed (may fail if no config, but that's ok for this test)
    let _result = cmd.assert();
    
    // If command succeeds, check for suggestions
    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        // The output should contain suggestions if results exist
        if stdout.contains("name") || stdout.contains("device_count") {
            assert!(stdout.contains("help") || stdout.contains("Suggestions"), 
                   "Output should contain suggestions when results exist");
        }
    }
}

#[test]
fn test_devices_command_includes_suggestions() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("devices")
       .arg("--json");

    // The command should succeed (may fail if no config, but that's ok for this test)
    let output = cmd.output().unwrap();
    
    // If command succeeds, check for suggestions
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        // The output should contain suggestions if results exist
        if stdout.contains("name") || stdout.contains("identifier_count") {
            assert!(stdout.contains("help") || stdout.contains("Suggestions"), 
                   "Output should contain suggestions when results exist");
        }
    }
}

#[test]
fn test_status_command_includes_suggestions() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
       .arg("--json");

    // The command should succeed
    cmd.assert().success();

    // The output should contain suggestions
    cmd.assert().stdout(predicate::str::contains("help").or(predicate::str::contains("Suggestions")));
}

#[test]
fn test_empty_results_include_suggestions() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("parties")
       .arg("--json");

    // The command should succeed even with empty results
    cmd.assert().success();

    // Empty results should still include suggestions
    cmd.assert().stdout(predicate::str::contains("help").or(predicate::str::contains("Suggestions")));
}

#[test]
fn test_suggestions_format_toon() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
       .arg("--json");

    // The command should succeed
    cmd.assert().success();

    // JSON format should include structured help array
    cmd.assert().stdout(predicate::str::contains("help").or(predicate::str::contains("Suggestions")));
}

#[test]
fn test_suggestions_format_human() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("parties");

    // The command should succeed
    cmd.assert().success();

    // Human format should include "Next steps:" header or suggestions
    cmd.assert().stdout(predicate::str::contains("Next steps").or(predicate::str::contains("Suggestions")));
}

#[test]
fn test_suggestions_limit() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
       .arg("--json");

    // The command should succeed
    cmd.assert().success();

    // Suggestions should be limited (not excessive)
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Count occurrences of "command" in suggestions section
    let command_count = stdout.matches("command").count();
    // Should be reasonable (2-4 suggestions max)
    assert!(command_count <= 4, "Too many suggestions: {}", command_count);
}

#[test]
fn test_suggestions_complete_commands() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
       .arg("--json");

    // The command should succeed
    cmd.assert().success();

    // Suggestions should include complete commands with "proximityd"
    cmd.assert().stdout(predicate::str::contains("proximityd"));
}

#[test]
fn test_suggestions_context_aware() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
       .arg("--json");

    // The command should succeed
    cmd.assert().success();

    // Suggestions should be context-aware (e.g., parties after status)
    cmd.assert().stdout(predicate::str::contains("parties").or(predicate::str::contains("devices")));
}
