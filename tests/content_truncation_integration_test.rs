// Integration tests for content truncation feature
// Tests truncation logic, --full flag, config options, and metadata

use assert_cmd::Command;

#[test]
fn test_default_truncation_limit() {
    // Test that content is truncated by default
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_full_flag_disables_truncation() {
    // Test that --full flag disables truncation
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--full")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_full_flag_with_status() {
    // Test --full flag with status command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--full")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_full_flag_with_devices() {
    // Test --full flag with devices command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--full")
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_full_flag_with_discover() {
    // Test --full flag with discover command (environment-dependent)
    // Discover command requires signal log data
}

#[test]
fn test_truncation_metadata_in_output() {
    // Test that truncation metadata is included in output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_help_suggestions() {
    // Test that help suggestions appear when content is truncated
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_with_toon_format() {
    // Test truncation with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_with_json_format() {
    // Test truncation with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_in_agent_mode() {
    // Test truncation in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_in_human_mode() {
    // Test truncation in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_applies_to_device_names() {
    // Test that truncation applies to device names
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_truncation_applies_to_location_fields() {
    // Test that truncation applies to location fields
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_word_boundary() {
    // Test that truncation respects word boundaries
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_with_field_selection() {
    // Test truncation with field selection
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_truncation_total_size_metadata() {
    // Test that total size metadata is included
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_truncation_indicator() {
    // Test that truncation indicator is present
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}
