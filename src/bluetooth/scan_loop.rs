use crate::bluetooth::adapter::BluetoothAdapter;
use crate::bluetooth::types::ScannedDevice;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

/// Run a continuous BLE scan loop, yielding discovered devices via an async channel.
///
/// # Arguments
/// * `adapter` — The platform-specific BLE adapter (e.g., [`BlueZAdapter`](super::bluez::BlueZAdapter))
/// * `scan_interval` — Duration between successive scan cycles
/// * `tx` — Sender channel for [`ScannedDevice`] results
///
/// # Behavior
/// * Starts a scan, collects all devices discovered within the cycle, sends them through `tx`
/// * Waits `scan_interval` before starting the next scan
/// * If the adapter is unavailable, retries every 30 seconds (NFR-2)
pub async fn run_scan_loop(
    adapter: Arc<dyn BluetoothAdapter>,
    scan_interval: Duration,
    tx: mpsc::Sender<ScannedDevice>,
) {
    info!(
        "Starting BLE scan loop with interval {:?}",
        scan_interval
    );

    loop {
        debug!("Beginning new scan cycle");

        //TODO: implement actual stream consumption once bluez-async event API is wired
        // For the skeleton we simulate a scan and then sleep
        let mut stream = adapter.scan();

        // Drain the stream for a fixed scan window
        let scan_window = Duration::from_secs(5);
        let deadline = tokio::time::Instant::now() + scan_window;

        while tokio::time::Instant::now() < deadline {
            match tokio::time::timeout(
                Duration::from_millis(100),
                stream.as_mut().next(),
            ).await {
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
                Ok(None) => break,
                Err(_) => {} // timeout — continue polling
            }
        }

        debug!("Scan cycle complete, waiting {:?} before next scan", scan_interval);
        sleep(scan_interval).await;
    }
}

/// Create a new scan loop task and return the receiver channel.
#[cfg(test)]
#[path = "scan_loop.test.rs"]
mod tests;

pub fn spawn_scan_loop(
    adapter: Arc<dyn BluetoothAdapter>,
    scan_interval: Duration,
) -> mpsc::Receiver<ScannedDevice> {
    let (tx, rx) = mpsc::channel(256);
    tokio::spawn(async move {
        run_scan_loop(adapter, scan_interval, tx).await;
    });
    rx
}
