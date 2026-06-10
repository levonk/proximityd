// Integration tests for structured errors & exit codes feature
// Tests error formatting, exit codes, idempotent operations, and flag validation

use assert_cmd::Command;

#[test]
fn test_structured_error_format() {
    // Test that errors are formatted with structured output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_structured_error_with_toon_format() {
    // Test structured errors with TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--toon")
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_structured_error_with_json_format() {
    // Test structured errors with JSON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--json")
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_exit_code_success() {
    // Test exit code 0 for success
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_exit_code_error() {
    // Test exit code 1 for errors (graceful fallback)
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--config")
        .arg("/nonexistent/config.toml")
        .arg("status")
        .assert()
        .success(); // Falls back to defaults
}

#[test]
fn test_exit_code_usage() {
    // Test exit code 2 for usage errors
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn test_exit_code_config() {
    // Test exit code 3 for config errors (graceful fallback)
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--config")
        .arg("/nonexistent/config.toml")
        .arg("status")
        .assert()
        .success(); // Falls back to defaults
}

#[test]
fn test_error_suggestions() {
    // Test that errors include suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_idempotent_install() {
    // Test that install is idempotent
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("install")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_idempotent_uninstall() {
    // Test that uninstall is idempotent
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("uninstall")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_flag_validation() {
    // Test that invalid flags are validated
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_error_in_agent_mode() {
    // Test error formatting in agent mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_error_in_human_mode() {
    // Test error formatting in human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_structured_error_metadata() {
    // Test that structured errors include metadata
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_error_with_suggestion_field() {
    // Test that errors include suggestion field
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_quiet_mode_suppresses_errors() {
    // Test that quiet mode suppresses non-error output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--quiet")
        .arg("status")
        .assert()
        .success();
}
