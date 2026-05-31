//! Bluetooth Low Energy (BLE) scanning module.
//!
//! Provides a platform-agnostic trait ([`BluetoothAdapter`]) for BLE discovery,
//! with a BlueZ (Linux) implementation behind a `#[cfg(target_os = "linux")]` gate.

pub mod adapter;
pub mod scan_loop;
pub mod types;

#[cfg(target_os = "linux")]
pub mod bluez;

// Re-exports for convenience
pub use adapter::BluetoothAdapter;
pub use scan_loop::{run_scan_loop, spawn_scan_loop};
pub use types::ScannedDevice;

#[cfg(target_os = "linux")]
pub use bluez::BlueZAdapter;
