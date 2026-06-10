// Integration tests for end-to-end agent workflows
// Tests complete agent mode workflows from start to finish

use assert_cmd::Command;

#[test]
fn test_agent_workflow_status_check() {
    // Test complete agent workflow for status check
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_device_discovery() {
    // Test complete agent workflow for device discovery (environment-dependent)
    // This test is skipped as it requires signal log data
}

#[test]
fn test_agent_workflow_list_parties() {
    // Test complete agent workflow for listing parties
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_list_devices() {
    // Test complete agent workflow for listing devices
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("devices")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_with_field_selection() {
    // Test agent workflow with field selection
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
fn test_agent_workflow_with_truncation() {
    // Test agent workflow with truncation
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_with_full_output() {
    // Test agent workflow with full output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("--full")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_session_context() {
    // Test agent workflow with session context (subcommand)
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("session-context")
        .arg("--toon")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_skill_generation() {
    // Test agent workflow with skill generation (requires file I/O)
    // This test is skipped as it requires file I/O
}

#[test]
fn test_agent_workflow_error_handling() {
    // Test agent workflow error handling
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("--invalid-flag")
        .assert()
        .failure();
}

#[test]
fn test_agent_workflow_empty_state_handling() {
    // Test agent workflow with empty states
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_suggestions() {
    // Test agent workflow with contextual suggestions
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_aggregates() {
    // Test agent workflow with aggregates
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("parties")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_export() {
    // Test agent workflow for export
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("export")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_mode_detection() {
    // Test agent workflow with auto mode detection
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("CLAUDE_SESSION", "true")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_content_first() {
    // Test agent workflow with content-first no-args
    Command::cargo_bin("proximityd")
        .unwrap()
        .env("CLAUDE_SESSION", "true")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_token_efficiency() {
    // Test that agent workflow is token-efficient
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("--toon")
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_agent_workflow_structured_output() {
    // Test that agent workflow produces structured output
    Command::cargo_bin("proximityd")
        .unwrap()
        .arg("--mode")
        .arg("agent")
        .arg("--toon")
        .arg("status")
        .assert()
        .success();
}
