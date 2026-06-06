//! Async scan loop that drives one or more [`Scanner`] implementations.
//!
//! Replaces the legacy `src/bluetooth/scan_loop.rs` which was tied to the
//! `BluetoothAdapter` trait and `bluez-async`.

use crate::scanner::types::RawSignal;
use crate::scanner::Scanner;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{debug, info, warn};

/// Duration to wait before retrying after a scanner failure.
const RECOVERY_INTERVAL: Duration = Duration::from_secs(30);

/// Run a continuous scan loop, yielding discovered signals via an async channel.
///
/// # Arguments
/// * `scanner` — The [`Scanner`] implementation to poll.
/// * `scan_interval` — Duration between successive scan cycles.
/// * `tx` — Sender channel for [`RawSignal`] results.
/// * `shutdown` — Watch receiver for graceful shutdown signal.
///
/// # Behavior
/// * Calls `scanner.scan()` each cycle, collects results, sends them through `tx`.
/// * Waits `scan_interval` before starting the next scan.
/// * If the scanner returns an error, retries every 30 seconds.
/// * Responds to shutdown signal by exiting cleanly after the current cycle.
pub async fn run_scan_loop(
    scanner: Arc<dyn Scanner>,
    scan_interval: Duration,
    tx: mpsc::Sender<RawSignal>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    info!(
        scanner = %scanner.name(),
        "Starting scan loop with interval {:?}",
        scan_interval
    );

    loop {
        if *shutdown.borrow() {
            info!(scanner = %scanner.name(), "Scan loop received shutdown signal, exiting");
            return;
        }

        debug!(scanner = %scanner.name(), "Beginning new scan cycle");

        match scanner.scan().await {
            Ok(signals) => {
                debug!(
                    scanner = %scanner.name(),
                    count = signals.len(),
                    "Scan cycle complete"
                );

                for signal in signals {
                    if let Err(e) = tx.send(signal).await {
                        warn!(scanner = %scanner.name(), "Scan loop channel closed: {}", e);
                        return;
                    }
                }

                crate::health::write_heartbeat();
                if let Err(msg) = crate::health::check_memory_usage() {
                    warn!("{}", msg);
                }

                tokio::select! {
                    _ = sleep(scan_interval) => {},
                    _ = shutdown.changed() => {
                        info!(scanner = %scanner.name(), "Scan loop received shutdown signal during sleep, exiting");
                        return;
                    }
                }
            }
            Err(e) => {
                warn!(
                    scanner = %scanner.name(),
                    error = %e,
                    "Scanner failed, retrying in {:?}",
                    RECOVERY_INTERVAL
                );

                tokio::select! {
                    _ = sleep(RECOVERY_INTERVAL) => {},
                    _ = shutdown.changed() => {
                        info!(scanner = %scanner.name(), "Scan loop received shutdown signal during recovery, exiting");
                        return;
                    }
                }
            }
        }
    }
}

/// Create a new scan loop task and return the receiver channel.
pub fn spawn_scan_loop(
    scanner: Arc<dyn Scanner>,
    scan_interval: Duration,
    shutdown: tokio::sync::watch::Receiver<bool>,
) -> mpsc::Receiver<RawSignal> {
    let (tx, rx) = mpsc::channel(256);
    tokio::spawn(async move {
        run_scan_loop(scanner, scan_interval, tx, shutdown).await;
    });
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::types::{IdType, RawSignal};
    use anyhow::Result;

    struct MockScanner {
        name: &'static str,
        signals: Vec<RawSignal>,
    }

    #[async_trait::async_trait]
    impl Scanner for MockScanner {
        async fn scan(&self) -> Result<Vec<RawSignal>> {
            Ok(self.signals.clone())
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }

    #[tokio::test]
    async fn test_scan_loop_receives_signals() {
        let signals = vec![
            RawSignal::new(IdType::BleMac, "AA:BB:CC:DD:EE:01", "mock").with_rssi(-42),
            RawSignal::new(IdType::BleMac, "AA:BB:CC:DD:EE:02", "mock").with_rssi(-55),
        ];

        let scanner: Arc<dyn Scanner> = Arc::new(MockScanner {
            name: "mock",
            signals,
        });
        let interval = Duration::from_millis(100);
        let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let mut rx = spawn_scan_loop(scanner, interval, shutdown_rx);

        // Give the scan loop time to complete one cycle and send
        sleep(Duration::from_millis(200)).await;

        // We should receive at least one signal from the first scan cycle
        let result = tokio::time::timeout(Duration::from_secs(2), rx.recv()).await;
        assert!(result.is_ok(), "Expected to receive a signal within timeout");
        let signal = result.unwrap().expect("Channel closed unexpectedly");
        assert_eq!(signal.id_value, "AA:BB:CC:DD:EE:01");
        assert_eq!(signal.rssi, Some(-42));
    }

    #[tokio::test]
    async fn test_scan_loop_cycle_timing() {
        let scanner: Arc<dyn Scanner> = Arc::new(MockScanner {
            name: "mock",
            signals: vec![],
        });
        let interval = Duration::from_millis(50);
        let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let _rx = spawn_scan_loop(scanner, interval, shutdown_rx);

        // Just verify it spawns without panicking and cycles
        sleep(Duration::from_millis(150)).await;
    }

    #[tokio::test]
    async fn test_scan_loop_graceful_shutdown() {
        let scanner: Arc<dyn Scanner> = Arc::new(MockScanner {
            name: "mock",
            signals: vec![],
        });
        let interval = Duration::from_millis(100);
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let mut rx = spawn_scan_loop(scanner, interval, shutdown_rx);

        // Let the scan loop start one cycle
        sleep(Duration::from_millis(50)).await;

        // Signal shutdown
        shutdown_tx.send(true).expect("send shutdown");

        // Wait for loop to exit (channel will close when loop ends)
        let timeout_result = tokio::time::timeout(Duration::from_secs(2), rx.recv()).await;
        assert!(
            timeout_result.is_ok(),
            "Scan loop did not shut down within timeout"
        );
    }
}
