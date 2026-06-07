use super::*;

#[test]
fn test_health_check_new_is_not_healthy() {
    let health = HealthCheck::new();
    // No scan recorded yet, so not healthy
    assert!(!health.is_healthy());
}

#[test]
fn test_health_check_default() {
    let health: HealthCheck = Default::default();
    assert!(!health.is_healthy());
}

#[test]
fn test_health_check_after_scan() {
    let health = HealthCheck::new();
    health.record_scan();
    assert!(health.is_healthy());
    assert!(health.scan_age().is_some());
}

#[test]
fn test_heartbeat_file_roundtrip() {
    use std::env;
    let path = std::path::PathBuf::from("/tmp/proximityd_test.health");
    env::set_var("PROXIMITYD_HEALTH_FILE", &path);
    write_heartbeat();
    assert!(check_heartbeat_file().is_ok());
    let _ = std::fs::remove_file(&path);
    env::remove_var("PROXIMITYD_HEALTH_FILE");
}
