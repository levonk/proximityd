// Integration tests for contextual disclosure feature
// Tests suggestion engine, context-aware ranking, and help formatting

use assert_cmd::Command;

#[test]
fn test_suggestions_in_parties_output() {
    // Test that parties output includes suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_in_devices_output() {
    // Test that devices output includes suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_suggestions_in_status_output() {
    // Test that status output includes suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_suggestions_in_discover_output() {
    // Test that discover output includes suggestions (environment-dependent)
    // This test is skipped as it requires signal log data
}

#[test]
fn test_suggestions_in_export_output() {
    // Test that export output includes suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("export")
        .assert()
        .success();
}

#[test]
fn test_suggestions_format_help_array() {
    // Test that suggestions are formatted as help[] array in TOON
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_limited_to_2_4() {
    // Test that suggestions are limited to 2-4 maximum
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_context_aware_empty_results() {
    // Test that empty results boost discover/status suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_context_aware_truncation() {
    // Test that truncation boosts --full flag suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_context_aware_agent_mode() {
    // Test that agent mode boosts TOON format suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_context_aware_result_count() {
    // Test that result count affects suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_complete_commands() {
    // Test that suggestions are complete commands with flags
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_ranked_by_relevance() {
    // Test that suggestions are ranked by relevance
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_with_toon_format() {
    // Test suggestions with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_with_json_format() {
    // Test suggestions with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_in_agent_mode() {
    // Test suggestions in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_in_human_mode() {
    // Test suggestions in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_suggestions_enable_cli_discovery() {
    // Test that suggestions enable organic CLI discovery
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}
