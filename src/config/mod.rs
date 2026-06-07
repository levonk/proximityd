//! Configuration module for proximityd.
//!
//! Handles loading of application configuration (`config.toml`) and
//! device identity mappings (`presence.toml`) with XDG path resolution.

pub mod app;
pub mod devices;
pub mod loader;
pub mod presence;

pub use app::{AppConfig, NotifierConfig};
pub use devices::{DeviceConfig, DevicesConfig};
pub use loader::{load_config, load_devices, load_presence};
pub use presence::{Device, Identifier, IdentifierType, Location, Party, PresenceConfig};
