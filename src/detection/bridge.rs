use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{debug, info};

use crate::notifier::NotifierRegistry;
use crate::scanner::types::RawSignal;
use crate::state::PresenceEvent;

use super::engine::DetectionEngine;

/// Run the detection loop, consuming BLE scan results and producing presence events.
///
/// # Arguments
/// * `engine` — The [`DetectionEngine`] to evaluate scan results against.
/// * `mut rx` — Receiver channel for [`RawSignal`] sightings from the scan loop.
/// * `exit_check_interval` — How often to poll for exit conditions.
/// * `notifiers` — Optional [`NotifierRegistry`] to dispatch presence events.
/// * `mut shutdown` — Watch receiver for graceful shutdown signal.
///
/// # Behavior
/// * Each received scan result is fed into `engine.evaluate_scan()`.
/// * Any emitted [`PresenceEvent`] is logged at `info` level and dispatched to notifiers.
/// * Every `exit_check_interval`, `engine.check_exits()` is called and results are logged.
/// * If the channel closes or shutdown is signalled, the loop exits cleanly.
pub async fn run_detection_loop(
    engine: Arc<DetectionEngine>,
    mut rx: mpsc::Receiver<RawSignal>,
    exit_check_interval: Duration,
    notifiers: Option<Arc<NotifierRegistry>>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    info!(
        "Starting detection loop (exit check every {:?})",
        exit_check_interval
    );

    let mut exit_timer = interval(exit_check_interval);

    loop {
        tokio::select! {
            // Graceful shutdown
            _ = shutdown.changed() => {
                info!("Detection loop received shutdown signal, exiting cleanly");
                break;
            }

            // Process incoming scan results
            result = rx.recv() => {
                match result {
                    Some(signal) => {
                        debug!(
                            id = %signal.id_value,
                            id_type = %signal.id_type,
                            rssi = ?signal.rssi,
                            scanner = %signal.scanner_name,
                            "Evaluating scan result"
                        );
                        let rssi = signal.rssi.unwrap_or(i16::MIN);
                        match engine.evaluate_scan(&signal.id_value, rssi) {
                            Some(ref ev @ PresenceEvent::Entered { ref name, ref mac }) => {
                                info!(mac = %mac, name = %name, "PRESENCE: Device entered");
                                dispatch_notifiers(notifiers.as_ref(), ev);
                            }
                            Some(ref ev @ PresenceEvent::Exited { ref name, ref mac }) => {
                                info!(mac = %mac, name = %name, "PRESENCE: Device exited");
                                dispatch_notifiers(notifiers.as_ref(), ev);
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
                for ev in &events {
                    match ev {
                        PresenceEvent::Entered { name, mac } => {
                            info!(mac = %mac, name = %name, "PRESENCE: Device entered (exit check)");
                        }
                        PresenceEvent::Exited { name, mac } => {
                            info!(mac = %mac, name = %name, "PRESENCE: Device exited (exit check)");
                        }
                    }
                }
                for ev in events {
                    dispatch_notifiers(notifiers.as_ref(), &ev);
                }
            }
        }
    }
}

fn dispatch_notifiers(notifiers: Option<&Arc<NotifierRegistry>>, event: &PresenceEvent) {
    if let Some(registry) = notifiers {
        let registry = Arc::clone(registry);
        let event = event.clone();
        tokio::task::spawn_blocking(move || {
            registry.dispatch(&event);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, DeviceConfig, DevicesConfig};
    use crate::state::PresenceStateTable;
    use std::time::Duration;

    #[allow(deprecated)]
    fn test_config() -> AppConfig {
        AppConfig {
            enter_rssi_threshold_dbm: -70,
            enter_duration_seconds: 1,
            exit_timeout_seconds: 2,
            notifiers: Vec::new(),
            track_unknown: true,
            general: Default::default(),
            privacy: Default::default(),
            scanner: Default::default(),
            detection: Default::default(),
            discovery: Default::default(),
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

        // Spawn detection loop with dummy shutdown channel
        let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let handle = tokio::spawn(async move {
            run_detection_loop(engine, rx, Duration::from_secs(1), None, shutdown_rx).await;
        });

        // Send a scan result, wait for debounce, send again
        tx.send(RawSignal::new(
                crate::scanner::types::IdType::BleMac,
                "AA:BB:CC:DD:EE:FF",
                "ble",
            ).with_rssi(-60))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1100)).await;
        tx.send(RawSignal::new(
                crate::scanner::types::IdType::BleMac,
                "AA:BB:CC:DD:EE:FF",
                "ble",
            ).with_rssi(-60))
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

        let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let handle = tokio::spawn(async move {
            run_detection_loop(engine, rx, Duration::from_secs(1), None, shutdown_rx).await;
        });

        // Close channel immediately
        drop(_tx);
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Should complete without panic
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn detection_loop_graceful_shutdown() {
        let state_table = Arc::new(PresenceStateTable::new());
        let engine = Arc::new(DetectionEngine::new(
            test_config(),
            test_devices(),
            state_table,
        ));

        let (_tx, rx) = mpsc::channel(16);

        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let handle = tokio::spawn(async move {
            run_detection_loop(engine, rx, Duration::from_secs(1), None, shutdown_rx).await;
        });

        // Give loop time to start
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Signal shutdown
        shutdown_tx.send(true).expect("send shutdown");

        // Should complete within timeout
        tokio::time::timeout(Duration::from_secs(2), handle)
            .await
            .expect("detection loop did not shut down within timeout")
            .unwrap();
    }
}
