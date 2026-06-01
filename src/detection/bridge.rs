use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{debug, info};

use crate::bluetooth::types::ScannedDevice;
use crate::state::PresenceEvent;

use super::engine::DetectionEngine;

/// Run the detection loop, consuming BLE scan results and producing presence events.
///
/// # Arguments
/// * `engine` — The [`DetectionEngine`] to evaluate scan results against.
/// * `mut rx` — Receiver channel for [`ScannedDevice`] sightings from the scan loop.
/// * `exit_check_interval` — How often to poll for exit conditions.
///
/// # Behavior
/// * Each received scan result is fed into `engine.evaluate_scan()`.
/// * Any emitted [`PresenceEvent`] is logged at `info` level.
/// * Every `exit_check_interval`, `engine.check_exits()` is called and results are logged.
/// * If the channel closes, the loop exits cleanly.
pub async fn run_detection_loop(
    engine: Arc<DetectionEngine>,
    mut rx: mpsc::Receiver<ScannedDevice>,
    exit_check_interval: Duration,
) {
    info!(
        "Starting detection loop (exit check every {:?})",
        exit_check_interval
    );

    let mut exit_timer = interval(exit_check_interval);

    loop {
        tokio::select! {
            // Process incoming scan results
            result = rx.recv() => {
                match result {
                    Some(device) => {
                        debug!(mac = %device.mac, rssi = device.rssi, "Evaluating scan result");
                        match engine.evaluate_scan(&device.mac, device.rssi) {
                            Some(PresenceEvent::Entered { ref name, ref mac }) => {
                                info!(mac = %mac, name = %name, "PRESENCE: Device entered");
                            }
                            Some(PresenceEvent::Exited { ref name, ref mac }) => {
                                info!(mac = %mac, name = %name, "PRESENCE: Device exited");
                            }
                            None => {}
                        }
                    }
                    None => {
                        info!("Detection loop shutting down (scan channel closed)");
                        break;
                    }
                }
            }

            // Periodic exit check
            _ = exit_timer.tick() => {
                let events = engine.check_exits();
                for ev in events {
                    match ev {
                        PresenceEvent::Entered { name, mac } => {
                            info!(mac = %mac, name = %name, "PRESENCE: Device entered (exit check)");
                        }
                        PresenceEvent::Exited { name, mac } => {
                            info!(mac = %mac, name = %name, "PRESENCE: Device exited (exit check)");
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, DeviceConfig, DevicesConfig};
    use crate::state::PresenceStateTable;
    use std::time::Duration;

    fn test_config() -> AppConfig {
        AppConfig {
            scan_interval_seconds: 30,
            enter_rssi_threshold_dbm: -70,
            enter_duration_seconds: 1,
            exit_timeout_seconds: 2,
            notifiers: Vec::new(),
            track_unknown: true,
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

    #[tokio::test]
    async fn detection_loop_processes_scan_and_emits_enter() {
        let state_table = Arc::new(PresenceStateTable::new());
        let engine = Arc::new(DetectionEngine::new(
            test_config(),
            test_devices(),
            state_table,
        ));

        let (tx, rx) = mpsc::channel(16);

        // Spawn detection loop
        let handle = tokio::spawn(async move {
            run_detection_loop(engine, rx, Duration::from_secs(1)).await;
        });

        // Send a scan result, wait for debounce, send again
        tx.send(ScannedDevice::new("AA:BB:CC:DD:EE:FF", -60))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1100)).await;
        tx.send(ScannedDevice::new("AA:BB:CC:DD:EE:FF", -60))
            .await
            .unwrap();

        // Give loop time to process
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Drop tx to close channel and exit loop
        drop(tx);
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn detection_loop_exits_when_channel_closes() {
        let state_table = Arc::new(PresenceStateTable::new());
        let engine = Arc::new(DetectionEngine::new(
            test_config(),
            test_devices(),
            state_table,
        ));

        let (_tx, rx) = mpsc::channel(16);

        let handle = tokio::spawn(async move {
            run_detection_loop(engine, rx, Duration::from_secs(1)).await;
        });

        // Close channel immediately
        drop(_tx);
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Should complete without panic
        handle.await.unwrap();
    }
}
