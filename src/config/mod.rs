//! Configuration module for btnotify.
//!
//! Handles loading of application configuration (`config.toml`) and
//! device identity mappings (`devices.toml`) with XDG path resolution.

pub mod app;
pub mod devices;
pub mod loader;

pub use app::{AppConfig, NotifierConfig};
pub use devices::{DeviceConfig, DevicesConfig};
pub use loader::{load_config, load_devices};
