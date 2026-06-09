//! No-args content-first behavior tests

use assert_cmd::Command;
use tempfile::TempDir;
use predicates::str;

#[test]
fn test_noargs_cli_human_mode() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.assert()
        .success()
        .stdout(str::contains("Proximityd presence detection service"))
        .stdout(str::contains("Daemon Status: stopped"))
        .stdout(str::contains("Common Commands:"))
        .stdout(str::contains("--help"));
}

#[test]
fn test_noargs_cli_toon_mode() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--toon")
        .assert()
        .success()
        .stdout(str::contains("context:default"))
        .stdout(str::contains("summary:Proximityd presence detection service"))
        .stdout(str::contains("daemon_status:stopped"))
        .stdout(str::contains("suggestions:"));
}

#[test]
fn test_noargs_cli_json_mode() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--json")
        .assert()
        .success()
        .stdout(str::contains("context"))
        .stdout(str::contains("summary"))
        .stdout(str::contains("suggestions"));
}

#[test]
fn test_noargs_cli_with_help_flag() {
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Usage:"))
        .stdout(str::contains("Options:"))
        .stdout(str::contains("Commands:"));
}

#[test]
fn test_noargs_cli_config_directory() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path();
    
    // Create a presence.toml file to simulate config directory
    let presence_file = config_dir.join("presence.toml");
    std::fs::write(&presence_file, "[[parties]]\nname = \"Test Party\"\n").unwrap();
    
    let mut cmd = Command::cargo_bin("proximityd").unwrap();
    cmd.current_dir(config_dir)
        .assert()
        .success()
        .stdout(str::contains("In config directory"))
        .stdout(str::contains("Configured Parties:"));
}
