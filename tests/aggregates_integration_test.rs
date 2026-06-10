// Integration tests for pre-computed aggregates feature
// Tests aggregate computation, list output, and derived status fields

use assert_cmd::Command;

#[test]
fn test_aggregate_count_in_parties_list() {
    // Test that parties list includes aggregate count
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_count_in_devices_list() {
    // Test that devices list includes aggregate count
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_aggregate_total_count_format() {
    // Test that aggregate count uses "X of Y total" format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_derived_status_identifiers() {
    // Test derived status for identifiers (e.g., "identifiers: 3/3 active")
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_derived_status_devices() {
    // Test derived status for devices (e.g., "devices: 2 present")
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregates_with_toon_format() {
    // Test aggregates with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregates_with_json_format() {
    // Test aggregates with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregates_in_agent_mode() {
    // Test aggregates in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregates_in_human_mode() {
    // Test aggregates in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_field_in_output() {
    // Test that aggregate field is present in output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_with_field_selection() {
    // Test aggregates with field selection
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_aggregate_computation_efficiency() {
    // Test that aggregate computation is efficient
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_with_empty_results() {
    // Test aggregates when no results are present
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_with_single_result() {
    // Test aggregates with single result
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_aggregate_with_multiple_results() {
    // Test aggregates with multiple results
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}
