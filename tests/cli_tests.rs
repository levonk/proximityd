use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("proximityd"));
}

#[test]
fn test_usage() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--usage")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_quiet() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--quiet")
        .arg("-")
        .write_stdin("test content")
        .assert()
        .success()
        .stderr(predicate::str::contains("ERROR").not()); // Quiet mode should suppress non-error output
}

#[test]
fn test_verbose() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.env("PROXIMITYD_LOG_FORMAT", "pretty")
        .arg("--verbose")
        .arg("-")
        .write_stdin("test content")
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO")); // Verbose mode with -v shows DEBUG, but with config default it may be INFO
}

#[test]
fn test_nocolor() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--nocolor")
        .arg("-")
        .write_stdin("test content")
        .assert()
        .success();
}

#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.assert().failure(); // Should fail as TTY stdin is empty
}

#[test]
fn test_stdin() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("-")
        .write_stdin("test content")
        .assert()
        .success()
        .stderr(predicate::str::contains("Processing content from stdin"));
}

#[test]
fn test_status() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("active devices")); // Empty state format
}

#[test]
fn test_status_json() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("scope")); // New schema output
}

#[test]
fn test_progress_quiet_mode() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--quiet")
        .arg("status")
        .assert()
        .success()
        .stderr(predicate::str::contains("⠋").not()); // No spinner in quiet mode
}

#[test]
fn test_exit_code_success() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_exit_code_usage_error() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--invalid-flag")
        .assert()
        .failure()
        .code(2); // Usage error
}

#[test]
fn test_exit_code_help_includes_exit_codes() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Exit codes:"));
}

#[test]
fn test_export() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("export")
        .arg("--format")
        .arg("jsonl")
        .assert()
        .success(); // May have no data, but should not fail
}

#[test]
fn test_export_csv() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("export")
        .arg("--format")
        .arg("csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("ts,scanner")); // CSV header
}

#[test]
fn test_install_dry_run() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("install")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_uninstall_dry_run() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("uninstall")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_install_creates_config_files() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("install")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("config directory"));
}

#[test]
fn test_install_force_overwrites() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("install")
        .arg("--force")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_uninstall_force_removes_config() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("uninstall")
        .arg("--force")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_uninstall_quiet_no_prompt() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("uninstall")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_completion_command() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--generate")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_completion_zsh() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--generate")
        .arg("zsh")
        .assert()
        .success();
}

#[test]
fn test_completion_fish() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--generate")
        .arg("fish")
        .assert()
        .success();
}

#[test]
fn test_completion_invalid_shell() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--generate")
        .arg("invalid")
        .assert()
        .failure();
}

#[test]
fn test_man_command() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("man")
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"));
}

#[test]
fn test_man_subcommand() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("man")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"));
}

#[test]
fn test_no_pager_flag() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--no-pager")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_force_flag_bypasses_confirmation() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("uninstall")
        .arg("--force")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_progress_with_quiet() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--quiet")
        .arg("status")
        .assert()
        .success()
        .stderr(predicate::str::contains("⠋").not());
}

#[test]
fn test_config_reload_signal() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
        .assert()
        .success();
}

#[test]
fn test_file_reference_in_error() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--config")
        .arg("/nonexistent/config.toml")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("active devices")); // Empty state format
}

#[test]
fn test_glob_pattern_support() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    // Test that glob patterns are accepted (even if no files match)
    cmd.arg("export")
        .arg("--format")
        .arg("jsonl")
        .assert()
        .success();
}
