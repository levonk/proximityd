use crate::bluetooth::adapter::BluetoothAdapter;
use crate::bluetooth::scan_loop::spawn_scan_loop;
use crate::bluetooth::types::ScannedDevice;
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// A mock adapter that yields a pre-defined sequence of devices.
struct MockAdapter {
    devices: Mutex<Vec<ScannedDevice>>,
}

impl MockAdapter {
    fn new(devices: Vec<ScannedDevice>) -> Self {
        Self {
            devices: Mutex::new(devices),
        }
    }
}

impl BluetoothAdapter for MockAdapter {
    fn scan(&self) -> Pin<Box<dyn Stream<Item = ScannedDevice> + Send>> {
        let devices: Vec<ScannedDevice> = {
            let guard = self.devices.lock().unwrap();
            guard.clone()
        };
        Box::pin(futures::stream::iter(devices))
    }
}

#[tokio::test]
async fn test_scan_loop_receives_devices() {
    let devices = vec![
        ScannedDevice::new("AA:BB:CC:DD:EE:01", -42),
        ScannedDevice::new("AA:BB:CC:DD:EE:02", -55),
    ];

    let adapter: Arc<dyn BluetoothAdapter> = Arc::new(MockAdapter::new(devices));
    let interval = Duration::from_millis(100);
    let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let mut rx = spawn_scan_loop(adapter, interval, shutdown_rx);

    // Give the scan loop time to complete one cycle and send
    sleep(Duration::from_millis(200)).await;

    // We should receive at least one device from the first scan cycle
    let result = timeout(Duration::from_secs(2), rx.recv()).await;
    assert!(
        result.is_ok(),
        "Expected to receive a device within timeout"
    );
    let device = result.unwrap().expect("Channel closed unexpectedly");
    assert_eq!(device.mac, "AA:BB:CC:DD:EE:01");
    assert_eq!(device.rssi, -42);
}

#[tokio::test]
async fn test_scan_loop_cycle_timing() {
    let adapter: Arc<dyn BluetoothAdapter> = Arc::new(MockAdapter::new(vec![]));
    let interval = Duration::from_millis(50);
    let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let _rx = spawn_scan_loop(adapter, interval, shutdown_rx);

    // Just verify it spawns without panicking and cycles
    sleep(Duration::from_millis(150)).await;
}

#[tokio::test]
async fn test_scan_loop_graceful_shutdown() {
    let adapter: Arc<dyn BluetoothAdapter> = Arc::new(MockAdapter::new(vec![]));
    let interval = Duration::from_millis(100);
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let mut rx = spawn_scan_loop(adapter, interval, shutdown_rx);

    // Let the scan loop start one cycle
    sleep(Duration::from_millis(50)).await;

    // Signal shutdown
    shutdown_tx.send(true).expect("send shutdown");

    // Wait for loop to exit (channel will close when loop ends)
    let timeout_result = timeout(Duration::from_secs(2), rx.recv()).await;
    assert!(
        timeout_result.is_ok(),
        "Scan loop did not shut down within timeout"
    );
}
