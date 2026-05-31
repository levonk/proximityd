//! Integration test: BLE scan cycle timing (NFR-1).
//!
//! This test requires a Bluetooth adapter and BlueZ on Linux.
//! Run with: `cargo test --test ble_scan_test -- --ignored`

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[cfg(target_os = "linux")]
#[tokio::test]
#[ignore = "requires Bluetooth adapter and BlueZ D-Bus"]
async fn scan_cycle_completes_within_5_seconds() {
    use btnotify::bluetooth::bluez::BlueZAdapter;
    use btnotify::bluetooth::scan_loop::spawn_scan_loop;

    let adapter = Arc::new(
        BlueZAdapter::new()
            .await
            .expect("Failed to connect to BlueZ — is D-Bus running?"),
    );

    let interval = Duration::from_secs(10);
    let mut rx = spawn_scan_loop(adapter, interval);

    let start = Instant::now();

    // Wait for at least one device or timeout after 5 seconds
    let result = timeout(Duration::from_secs(5), rx.recv()).await;

    let elapsed = start.elapsed();

    // Even if no devices are found, the scan cycle itself should complete within 5s
    assert!(
        elapsed < Duration::from_secs(5),
        "Scan cycle took {:?}, exceeding 5-second NFR-1 limit",
        elapsed
    );

    // If a device was found, verify its structure
    if let Ok(Some(device)) = result {
        assert!(
            !device.mac.is_empty(),
            "Discovered device must have a MAC address"
        );
        assert!(device.rssi <= 0, "RSSI should be negative or zero dBm");
    }
}

#[cfg(not(target_os = "linux"))]
#[tokio::test]
#[ignore = "BlueZ integration test only runnable on Linux"]
async fn scan_cycle_skipped_on_non_linux() {}
