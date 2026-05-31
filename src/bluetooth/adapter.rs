use crate::bluetooth::types::ScannedDevice;
use std::pin::Pin;

/// Abstraction over platform-specific BLE adapters.
///
/// Implementors yield a stream of [`ScannedDevice`]s during a discovery scan.
/// This trait is object-safe so it can be used behind `dyn` for mocks in tests.
pub trait BluetoothAdapter: Send + Sync {
    /// Start a discovery scan and return a stream of discovered devices.
    ///
    /// The stream completes when the scan finishes or the adapter stops.
    fn scan(&self) -> Pin<Box<dyn futures::Stream<Item = ScannedDevice> + Send>>;
}
