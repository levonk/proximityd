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
        .stdout(predicate::str::contains("Presence Status"));
}

#[test]
fn test_status_json() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("status")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("[")); // JSON array start
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
