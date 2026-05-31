use crate::bluetooth::adapter::BluetoothAdapter;
use crate::bluetooth::types::ScannedDevice;
use bluez_async::{BluetoothSession, DiscoveredDevice, MacAddress};
use futures::Stream;
use std::pin::Pin;
use tracing::{debug, error, info, warn};

/// BlueZ (Linux) implementation of [`BluetoothAdapter`] using `bluez-async`.
pub struct BlueZAdapter {
    session: BluetoothSession,
}

impl BlueZAdapter {
    /// Create a new BlueZ adapter, connecting to the D-Bus system bus.
    pub async fn new() -> anyhow::Result<Self> {
        info!("Connecting to BlueZ via D-Bus");
        let session = BluetoothSession::new().await?;
        Ok(Self { session })
    }
}

impl BluetoothAdapter for BlueZAdapter {
    fn scan(&self) -> Pin<Box<dyn Stream<Item = ScannedDevice> + Send>> {
        let session = self.session.clone();
        let stream = async_stream::stream! {
            info!("Starting BlueZ BLE discovery scan");
            match session.start_discovery().await {
                Ok(()) => debug!("BlueZ discovery started"),
                Err(e) => {
                    warn!("Failed to start BlueZ discovery: {}", e);
                    return;
                }
            }

            // Poll discovered devices from the session
            // bluez-async exposes discovered devices via events
            // We use a simple polled approach for the scan duration
            loop {
                //TODO: use session.event_stream() or device event polling
                // For now yield nothing and rely on the scan timeout in scan_loop.rs
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        };
        Box::pin(stream)
    }
}

/// Convert a BlueZ [`DiscoveredDevice`] into our [`ScannedDevice`].
fn convert_device(device: &DiscoveredDevice) -> Option<ScannedDevice> {
    let mac = device.mac_address.to_string();
    let rssi = device.rssi?;
    Some(ScannedDevice::new(mac, rssi))
}
