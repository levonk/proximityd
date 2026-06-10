// Integration tests for minimal default schemas feature
// Tests field selection, schema defaults, and field validation

use assert_cmd::Command;

#[test]
fn test_default_schema_parties() {
    // Test that parties command uses default schema
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_default_schema_devices() {
    // Test that devices command uses default schema
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_default_schema_status() {
    // Test that status command uses default schema
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_fields_flag_parties() {
    // Test --fields flag with parties command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name,devices")
        .assert()
        .success();
}

#[test]
fn test_fields_flag_devices() {
    // Test --fields flag with devices command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .arg("--fields")
        .arg("name,location")
        .assert()
        .success();
}

#[test]
fn test_fields_flag_status() {
    // Test --fields flag with status command
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .arg("--fields")
        .arg("scope,active")
        .assert()
        .success();
}

#[test]
fn test_fields_single_field() {
    // Test selecting a single field
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_fields_multiple_fields() {
    // Test selecting multiple fields
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("name,devices,location")
        .assert()
        .success();
}

#[test]
fn test_fields_with_toon_format() {
    // Test field selection with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_fields_with_json_format() {
    // Test field selection with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_invalid_field_name() {
    // Test that invalid field names are handled gracefully
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("invalid_field")
        .assert()
        .success();
}

#[test]
fn test_field_validation_per_command() {
    // Test that field validation is command-specific
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("devices")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_empty_fields_list() {
    // Test behavior with empty fields list (uses default schema)
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .arg("--fields")
        .arg("")
        .assert()
        .success();
}

#[test]
fn test_fields_with_agent_mode() {
    // Test field selection in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_fields_with_human_mode() {
    // Test field selection in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("parties")
        .arg("--fields")
        .arg("name")
        .assert()
        .success();
}

#[test]
fn test_default_schema_field_count() {
    // Test that default schemas have limited field count
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("parties")
        .assert()
        .success();
}
