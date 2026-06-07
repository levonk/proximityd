use crate::bluetooth::adapter::BluetoothAdapter;
use crate::bluetooth::types::ScannedDevice;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

/// Duration to wait before retrying after an adapter failure.
const RECOVERY_INTERVAL: Duration = Duration::from_secs(30);

/// Run a continuous BLE scan loop, yielding discovered devices via an async channel.
///
/// # Arguments
/// * `adapter` — The platform-specific BLE adapter (e.g., [`BlueZAdapter`](super::bluez::BlueZAdapter))
/// * `scan_interval` — Duration between successive scan cycles
/// * `tx` — Sender channel for [`ScannedDevice`] results
/// * `shutdown` — Watch receiver for graceful shutdown signal
///
/// # Behavior
/// * Starts a scan, collects all devices discovered within the cycle, sends them through `tx`
/// * Waits `scan_interval` before starting the next scan
/// * If the adapter is unavailable, retries every 30 seconds (NFR-2)
/// * Responds to shutdown signal by exiting cleanly after the current cycle
pub async fn run_scan_loop(
    adapter: Arc<dyn BluetoothAdapter>,
    scan_interval: Duration,
    tx: mpsc::Sender<ScannedDevice>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    info!("Starting BLE scan loop with interval {:?}", scan_interval);

    loop {
        if *shutdown.borrow() {
            info!("Scan loop received shutdown signal, exiting");
            return;
        }

        debug!("Beginning new scan cycle");

        let mut stream = adapter.scan();

        // Drain the stream for a fixed scan window
        let scan_window = Duration::from_secs(5);
        let deadline = tokio::time::Instant::now() + scan_window;
        let mut stream_ended_early = false;

        while tokio::time::Instant::now() < deadline {
            match tokio::time::timeout(Duration::from_millis(100), stream.as_mut().next()).await {
                Ok(Some(device)) => {
                    debug!(
                        mac = %device.mac,
                        rssi = device.rssi,
                        "Discovered BLE device"
                    );
                    if let Err(e) = tx.send(device).await {
                        warn!("Scan loop channel closed: {}", e);
                        return;
                    }
                }
                Ok(None) => {
                    stream_ended_early = true;
                    break;
                }
                Err(_) => {} // timeout — continue polling
            }
        }

        if stream_ended_early {
            error!(
                "Bluetooth adapter stream ended early (adapter may be unavailable). Retrying in {:?}",
                RECOVERY_INTERVAL
            );
            tokio::select! {
                _ = sleep(RECOVERY_INTERVAL) => {},
                _ = shutdown.changed() => {
                    info!("Scan loop received shutdown signal during recovery, exiting");
                    return;
                }
            }
        } else {
            debug!(
                "Scan cycle complete, waiting {:?} before next scan",
                scan_interval
            );
            crate::health::write_heartbeat();
            if let Err(msg) = crate::health::check_memory_usage() {
                warn!("{}", msg);
            }
            tokio::select! {
                _ = sleep(scan_interval) => {},
                _ = shutdown.changed() => {
                    info!("Scan loop received shutdown signal during sleep, exiting");
                    return;
                }
            }
        }
    }
}

/// Create a new scan loop task and return the receiver channel.
#[cfg(test)]
#[path = "scan_loop.test.rs"]
mod tests;

pub fn spawn_scan_loop(
    adapter: Arc<dyn BluetoothAdapter>,
    scan_interval: Duration,
    shutdown: tokio::sync::watch::Receiver<bool>,
) -> mpsc::Receiver<ScannedDevice> {
    let (tx, rx) = mpsc::channel(256);
    tokio::spawn(async move {
        run_scan_loop(adapter, scan_interval, tx, shutdown).await;
    });
    rx
}
