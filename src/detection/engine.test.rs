use std::sync::Arc;
use std::time::Duration;

use super::DetectionEngine;
use crate::config::{AppConfig, DeviceConfig, DevicesConfig};
use crate::state::PresenceStateTable;

fn test_config() -> AppConfig {
    AppConfig {
        scan_interval_seconds: 30,
        enter_rssi_threshold_dbm: -70,
        enter_duration_seconds: 1,
        exit_timeout_seconds: 2,
        notifiers: Vec::new(),
        track_unknown: false,
    }
}

fn test_devices() -> DevicesConfig {
    let mut devices = DevicesConfig::default();
    devices.devices.insert(
        "AA:BB:CC:DD:EE:FF".to_string(),
        DeviceConfig {
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
            name: "Test Phone".to_string(),
        },
    );
    devices
}

fn setup() -> (DetectionEngine, Arc<PresenceStateTable>) {
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = DetectionEngine::new(test_config(), test_devices(), state_table.clone());
    (engine, state_table)
}

#[test]
fn evaluate_scan_unknown_device_ignored_when_track_unknown_false() {
    let (engine, table) = setup();
    let event = engine.evaluate_scan("00:00:00:00:00:00", -60);
    assert!(event.is_none());
    assert!(table.get("00:00:00:00:00:00").is_none());
}

#[test]
fn evaluate_scan_tracks_unknown_when_configured() {
    let state_table = Arc::new(PresenceStateTable::new());
    let mut config = test_config();
    config.track_unknown = true;
    let engine = DetectionEngine::new(config, test_devices(), state_table.clone());

    let event = engine.evaluate_scan("00:00:00:00:00:00", -60);
    assert!(event.is_none());
    assert!(state_table.get("00:00:00:00:00:00").is_some());
}

#[test]
fn evaluate_scan_known_device_updates_state_table() {
    let (engine, table) = setup();
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_none());
    let dev = table.get("AA:BB:CC:DD:EE:FF").unwrap();
    assert_eq!(dev.rssi, -60);
}

#[test]
fn evaluate_scan_weak_rssi_does_not_trigger_enter() {
    let (engine, table) = setup();
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -80);
    assert!(event.is_none());
    // Device is tracked in state table even with weak RSSI, but stays Pending.
    assert_eq!(table.get_state("AA:BB:CC:DD:EE:FF"), Some(crate::state::PresenceState::Pending));
}

#[test]
fn evaluate_scan_enter_after_debounce_duration() {
    let (engine, table) = setup();
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_none());

    std::thread::sleep(Duration::from_millis(1100));

    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());
    let ev = event.unwrap();
    assert!(matches!(ev, crate::state::PresenceEvent::Entered { .. }));
    assert_eq!(table.get_state("AA:BB:CC:DD:EE:FF"), Some(crate::state::PresenceState::Entered));
}

#[test]
fn check_exits_after_timeout() {
    let (engine, table) = setup();

    // Enter the device
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_none());
    std::thread::sleep(Duration::from_millis(1100));
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());
    assert_eq!(table.get_state("AA:BB:CC:DD:EE:FF"), Some(crate::state::PresenceState::Entered));

    // Wait for exit timeout
    std::thread::sleep(Duration::from_millis(2100));
    let events = engine.check_exits();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], crate::state::PresenceEvent::Exited { .. }));
    assert_eq!(table.get_state("AA:BB:CC:DD:EE:FF"), Some(crate::state::PresenceState::Exited));
}

#[test]
fn check_exits_does_not_trigger_prematurely() {
    let (engine, table) = setup();

    // Enter the device (two scans with sleep between for debounce)
    engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    std::thread::sleep(Duration::from_millis(1100));
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());

    // Immediately check exits — should not trigger
    let events = engine.check_exits();
    assert!(events.is_empty());
    assert_eq!(table.get_state("AA:BB:CC:DD:EE:FF"), Some(crate::state::PresenceState::Entered));
}

#[test]
fn rapid_flap_does_not_trigger_multiple_enters() {
    let (_engine, _table) = setup();

    // Enter (two scans with sleep between for debounce)
    _engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    std::thread::sleep(Duration::from_millis(1100));
    let event = _engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());

    // Repeated scans while already entered should not re-emit
    let event = _engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_none());
    let event = _engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -55);
    assert!(event.is_none());
}

#[test]
fn re_enter_requires_full_debounce_after_exit() {
    let (engine, _table) = setup();

    // First enter (two scans with sleep between)
    engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    std::thread::sleep(Duration::from_millis(1100));
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());

    // Exit
    std::thread::sleep(Duration::from_millis(2100));
    let events = engine.check_exits();
    assert_eq!(events.len(), 1);

    // Immediate re-scan should not enter
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_none());

    // After full debounce, should enter again
    std::thread::sleep(Duration::from_millis(1100));
    let event = engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert!(event.is_some());
    assert!(matches!(event.unwrap(), crate::state::PresenceEvent::Entered { .. }));
}

#[test]
fn tracked_count_reflects_unique_devices() {
    let state_table = Arc::new(PresenceStateTable::new());
    let mut config = test_config();
    config.track_unknown = true;
    let engine = DetectionEngine::new(config, test_devices(), state_table);

    assert_eq!(engine.tracked_count(), 0);
    engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -60);
    assert_eq!(engine.tracked_count(), 1);
    engine.evaluate_scan("11:22:33:44:55:66", -60);
    assert_eq!(engine.tracked_count(), 2);
    engine.evaluate_scan("AA:BB:CC:DD:EE:FF", -55);
    assert_eq!(engine.tracked_count(), 2);
}
