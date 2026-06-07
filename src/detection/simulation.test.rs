//! Time-accelerated simulation test for zero-false-positive validation.
//!
//! This test exercises realistic RSSI fluctuation patterns that a BLE device
//! might exhibit inside a home environment.  The simulation is short-duration
//! (seconds rather than days) but covers the same behavioral classes:
//!
//! 1.  Device arrives — strong signal, qualified enter after debounce.
//! 2.  Device sits idle — signal fluctuates above and below threshold.
//! 3.  Device briefly out of range — signal lost for < exit_timeout.
//! 4.  Device actually departs — signal lost for > exit_timeout.
//!
//! Acceptance: exactly one Enter event, exactly one Exit event, zero false
//! transitions.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::config::{AppConfig, DeviceConfig, DevicesConfig};
use crate::detection::engine::DetectionEngine;
use crate::state::PresenceEvent;
use crate::state::PresenceStateTable;

const KNOWN_MAC: &str = "AA:BB:CC:DD:EE:FF";
const KNOWN_NAME: &str = "Levon's Phone";

fn sim_config() -> AppConfig {
    AppConfig {
        enter_rssi_threshold_dbm: -70,
        enter_duration_seconds: 2,
        exit_timeout_seconds: 3,
        notifiers: Vec::new(),
        track_unknown: false,
        general: Default::default(),
        privacy: Default::default(),
        scanner: Default::default(),
        detection: Default::default(),
        discovery: Default::default(),
    }
}

fn sim_devices() -> DevicesConfig {
    let mut devices = DevicesConfig::default();
    devices.devices.insert(
        KNOWN_MAC.to_string(),
        DeviceConfig {
            mac: KNOWN_MAC.to_string(),
            name: KNOWN_NAME.to_string(),
        },
    );
    devices
}

/// Simulate a stream of (rssi, delay_ms_after) scan readings.
fn simulate_scans(engine: &DetectionEngine, readings: &[(i16, u64)]) -> Vec<PresenceEvent> {
    let mut events = Vec::new();
    for &(rssi, delay_ms) in readings {
        if let Some(ev) = engine.evaluate_scan(KNOWN_MAC, rssi) {
            events.push(ev);
        }
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
    events
}

/// Collect any exit events from the engine.
fn collect_exits(engine: &DetectionEngine) -> Vec<PresenceEvent> {
    engine.check_exits()
}

#[test]
fn simulation_zero_false_positives_realistic_home() {
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = DetectionEngine::new(sim_config(), sim_devices(), state_table);

    // ── Phase 1: Device arrives (strong signal, qualified) ──
    // Two qualified scans with 2.2s between them = enter_duration satisfied.
    let mut all_events = simulate_scans(
        &engine,
        &[
            (-55, 2200), // scan 1, wait > enter_duration
            (-58, 100),  // scan 2, should trigger Enter
        ],
    );

    // Should have exactly one Enter event.
    assert_eq!(
        all_events.len(),
        1,
        "Expected exactly one Enter after arrival, got {:?}",
        all_events
    );
    assert!(
        matches!(&all_events[0], PresenceEvent::Entered { mac, name, .. } if mac == KNOWN_MAC && name == KNOWN_NAME),
        "First event should be Entered, got {:?}",
        all_events[0]
    );

    // ── Phase 2: Signal fluctuation (threshold crossings, no false exits) ──
    // RSSI bounces above and below threshold while device is still in the house.
    let fluctuation_events = simulate_scans(
        &engine,
        &[
            (-60, 500), // strong
            (-72, 500), // below enter threshold, still in range
            (-65, 500), // above again
            (-75, 500), // below threshold, still in range
            (-62, 500), // above
        ],
    );

    assert_eq!(
        fluctuation_events.len(),
        0,
        "Expected zero events during signal fluctuation, got {:?}",
        fluctuation_events
    );

    // ── Phase 3: Brief signal loss (device in basement, still present) ──
    // No scans for 2 seconds (less than exit_timeout of 3).
    thread::sleep(Duration::from_millis(2000));
    let brief_loss_events = simulate_scans(
        &engine,
        &[
            (-80, 0), // weak scan after brief absence, should not exit yet
        ],
    );

    // Check exits — no exit should trigger because last seen was 2s ago (< 3s).
    let exits_after_brief_loss = collect_exits(&engine);
    assert_eq!(
        brief_loss_events.len(),
        0,
        "Expected zero events after brief signal loss, got {:?}",
        brief_loss_events
    );
    assert_eq!(
        exits_after_brief_loss.len(),
        0,
        "Expected zero exits after brief signal loss, got {:?}",
        exits_after_brief_loss
    );

    // ── Phase 4: Device actually departs ──
    // No scans for 3.2 seconds (> exit_timeout of 3).
    thread::sleep(Duration::from_millis(3200));
    let exits_after_departure = collect_exits(&engine);

    assert_eq!(
        exits_after_departure.len(),
        1,
        "Expected exactly one Exit after departure, got {:?}",
        exits_after_departure
    );
    assert!(
        matches!(&exits_after_departure[0], PresenceEvent::Exited { mac, name, .. } if mac == KNOWN_MAC && name == KNOWN_NAME),
        "Exit event should match device, got {:?}",
        exits_after_departure[0]
    );

    // Collect final event list.
    all_events.extend(exits_after_departure);

    // ── Final tally ──
    let enter_count = all_events
        .iter()
        .filter(|e| matches!(e, PresenceEvent::Entered { .. }))
        .count();
    let exit_count = all_events
        .iter()
        .filter(|e| matches!(e, PresenceEvent::Exited { .. }))
        .count();

    assert_eq!(
        enter_count, 1,
        "Expected exactly 1 Enter event in full simulation, got {}",
        enter_count
    );
    assert_eq!(
        exit_count, 1,
        "Expected exactly 1 Exit event in full simulation, got {}",
        exit_count
    );
    assert_eq!(
        all_events.len(),
        2,
        "Expected exactly 2 total events (1 Enter + 1 Exit), got {}",
        all_events.len()
    );
}

#[test]
fn simulation_device_at_exact_threshold_boundary() {
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = DetectionEngine::new(sim_config(), sim_devices(), state_table);

    // RSSI exactly at threshold = -70. This should count as qualified.
    let mut events = simulate_scans(&engine, &[(-70, 2200), (-70, 100)]);

    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], PresenceEvent::Entered { .. }));

    // Departure
    thread::sleep(Duration::from_millis(3200));
    let exits = collect_exits(&engine);
    assert_eq!(exits.len(), 1);

    events.extend(exits);
    assert_eq!(events.len(), 2);
}

#[test]
fn simulation_rapid_flapping_around_threshold() {
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = DetectionEngine::new(sim_config(), sim_devices(), state_table);

    // Device arrives
    let mut events = simulate_scans(&engine, &[(-55, 2200), (-55, 100)]);
    assert_eq!(events.len(), 1);

    // Rapidly alternate above/below threshold — should not trigger any events
    let flap_events = simulate_scans(
        &engine,
        &[
            (-60, 100),
            (-72, 100),
            (-68, 100),
            (-71, 100),
            (-69, 100),
            (-73, 100),
            (-67, 100),
            (-74, 100),
            (-66, 100),
            (-72, 100),
        ],
    );
    assert_eq!(
        flap_events.len(),
        0,
        "Rapid flap should produce zero events"
    );

    // No false exits either
    let exits = collect_exits(&engine);
    assert_eq!(
        exits.len(),
        0,
        "Rapid flap should not trigger premature exit"
    );

    // Proper departure
    thread::sleep(Duration::from_millis(3200));
    let exits = collect_exits(&engine);
    events.extend(exits);
    assert_eq!(events.len(), 2);
}

#[test]
fn simulation_multiple_brief_dropouts_still_present() {
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = DetectionEngine::new(sim_config(), sim_devices(), state_table);

    // Device arrives
    let mut events = simulate_scans(&engine, &[(-55, 2200), (-55, 100)]);
    assert_eq!(events.len(), 1);

    // Multiple short dropouts (each 2s, less than exit_timeout 3s)
    // Between dropouts we get a quick scan to reset the timer
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(2000));
        let ev = engine.evaluate_scan(KNOWN_MAC, -60);
        assert!(
            ev.is_none(),
            "Should not trigger event during brief dropout"
        );
        let exits = collect_exits(&engine);
        assert_eq!(exits.len(), 0, "Brief dropout should not cause exit");
    }

    // Actual departure
    thread::sleep(Duration::from_millis(3200));
    let exits = collect_exits(&engine);
    events.extend(exits);

    assert_eq!(
        events.len(),
        2,
        "Expected 1 Enter + 1 Exit across multiple brief dropouts"
    );
}
