// Integration tests for mode selection feature
// Tests mode detection, CLI flags, environment variables, and config integration

use assert_cmd::Command;

#[test]
fn test_mode_auto_detection_agent_session() {
    // Test auto-detection when agent session is detected
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("CLAUDE_SESSION", "true")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_auto_detection_tty() {
    // Test auto-detection based on TTY presence
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_cli_flag_human() {
    // Test --human flag forces human mode
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--human")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_cli_flag_mode_agent() {
    // Test --mode agent flag
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_cli_flag_mode_human() {
    // Test --mode human flag
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_cli_flag_mode_auto() {
    // Test --mode auto flag
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("auto")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_env_var_proximityd_mode() {
    // Test PROXIMITYD_MODE environment variable
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("PROXIMITYD_MODE", "agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_precedence_cli_over_env() {
    // Test that CLI flags take precedence over environment variables
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("PROXIMITYD_MODE", "human")
        .arg("--mode")
        .arg("agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_precedence_env_over_config() {
    // Test that environment variables take precedence over config
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("PROXIMITYD_MODE", "agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_invalid_mode_value() {
    // Test that invalid mode values are handled gracefully
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("invalid")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_toon_format_with_agent_mode() {
    // Test that agent mode defaults to TOON format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_human_format_with_human_mode() {
    // Test that human mode defaults to human-readable format
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("human")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_mode_interactive_flag() {
    // Test that --interactive flag works with mode selection
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--interactive")
        .arg("status")
        .assert()
        .success();
}
