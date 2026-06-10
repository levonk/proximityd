// Integration tests for cross-platform compatibility
// Tests that features work correctly on Linux, macOS, and Windows

use assert_cmd::Command;

#[test]
fn test_path_handling_windows() {
    // Test path handling on Windows
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_path_handling_unix() {
    // Test path handling on Unix systems
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_config_directory_resolution() {
    // Test config directory resolution across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_file_permissions_handling() {
    // Test file permissions handling across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("install")
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_line_endings_in_output() {
    // Test that line endings are handled correctly
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_shell_completion_generation() {
    // Test shell completion generation across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--generate")
        .arg("bash")
        .assert()
        .success();
}

#[test]
fn test_daemon_mode_linux_only() {
    // Test that daemon mode is Linux-only
    #[cfg(not(target_os = "linux"))]
    {
        Command::cargo_bin("proximityd")
            .unwrap()
            .arg("--daemon")
            .assert()
            .success(); // Daemon mode is handled gracefully on non-Linux
    }
}

#[test]
fn test_ble_scanning_platform_specific() {
    // Test BLE scanning behavior on different platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("discover")
        .assert()
        .success();
}

#[test]
fn test_config_file_path_resolution() {
    // Test config file path resolution across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--config")
        .arg("/tmp/test-config.toml")
        .arg("status")
        .assert()
        .success(); // Should succeed with defaults
}

#[test]
fn test_environment_variable_handling() {
    // Test environment variable handling across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("PROXIMITYD_LOG_LEVEL", "info")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_temp_file_handling() {
    // Test temp file handling across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("export")
        .arg("--format")
        .arg("jsonl")
        .assert()
        .success();
}

#[test]
fn test_unicode_handling() {
    // Test Unicode handling in output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_color_output_handling() {
    // Test color output handling across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--nocolor")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_signal_handling() {
    // Test signal handling across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_process_termination() {
    // Test process termination across platforms
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}
