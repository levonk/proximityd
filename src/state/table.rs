use std::collections::HashMap;
use std::sync::RwLock;

use tracing::{debug, info};

use super::types::{PresenceState, TrackedDevice};

/// Thread-safe in-memory table tracking the presence state of BLE devices.
pub struct PresenceStateTable {
    inner: RwLock<HashMap<String, TrackedDevice>>,
}

impl PresenceStateTable {
    /// Create a new empty state table.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// Update the RSSI and last-seen timestamp for a device.
    /// Creates a new entry if the MAC is not yet known.
    pub fn update(&self, mac: impl Into<String>, rssi: i16) {
        let mac = mac.into();
        let mut guard = self.inner.write().expect("lock poisoned");
        if let Some(dev) = guard.get_mut(&mac) {
            dev.update_rssi(rssi);
            debug!(mac = %mac, rssi = rssi, "Updated existing device in state table");
        } else {
            let dev = TrackedDevice::new(&mac, rssi);
            guard.insert(mac.clone(), dev);
            debug!(mac = %mac, rssi = rssi, "Inserted new device into state table");
        }
    }

    /// Get the current [`PresenceState`] for a device, if known.
    pub fn get_state(&self, mac: &str) -> Option<PresenceState> {
        let guard = self.inner.read().expect("lock poisoned");
        guard.get(mac).map(|d| d.state)
    }

    /// Get a clone of the full [`TrackedDevice`] for a MAC, if known.
    pub fn get(&self, mac: &str) -> Option<TrackedDevice> {
        let guard = self.inner.read().expect("lock poisoned");
        guard.get(mac).cloned()
    }

    /// Set the presence state for a device, logging transitions.
    pub fn set_state(&self, mac: &str, new_state: PresenceState) {
        let mut guard = self.inner.write().expect("lock poisoned");
        if let Some(dev) = guard.get_mut(mac) {
            let old_state = dev.state;
            if old_state != new_state {
                match new_state {
                    PresenceState::Entered => {
                        info!(mac = %mac, name = %dev.name, "Device entered");
                    }
                    PresenceState::Exited => {
                        info!(mac = %mac, name = %dev.name, "Device exited");
                    }
                    PresenceState::Pending => {
                        debug!(mac = %mac, name = %dev.name, "Device state set to Pending");
                    }
                }
            }
            dev.state = new_state;
        }
    }

    /// Return a snapshot of all devices currently considered "present"
    /// (`Entered` or `Pending`).
    pub fn list_present(&self) -> Vec<TrackedDevice> {
        let guard = self.inner.read().expect("lock poisoned");
        guard
            .values()
            .filter(|d| matches!(d.state, PresenceState::Entered | PresenceState::Pending))
            .cloned()
            .collect()
    }

    /// Return a snapshot of all devices currently considered "exited".
    pub fn list_exited(&self) -> Vec<TrackedDevice> {
        let guard = self.inner.read().expect("lock poisoned");
        guard
            .values()
            .filter(|d| matches!(d.state, PresenceState::Exited))
            .cloned()
            .collect()
    }

    /// Total number of tracked devices.
    pub fn len(&self) -> usize {
        let guard = self.inner.read().expect("lock poisoned");
        guard.len()
    }

    /// True if no devices are tracked.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for PresenceStateTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn update_creates_new_device() {
        let table = PresenceStateTable::new();
        table.update("AA:BB:CC:DD:EE:FF", -55);
        assert_eq!(table.len(), 1);
        let dev = table.get("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(dev.rssi, -55);
        assert_eq!(dev.state, PresenceState::Pending);
    }

    #[test]
    fn update_refreshes_existing_device() {
        let table = PresenceStateTable::new();
        table.update("AA:BB:CC:DD:EE:FF", -55);
        table.update("AA:BB:CC:DD:EE:FF", -50);
        let dev = table.get("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(dev.rssi, -50);
    }

    #[test]
    fn get_state_returns_none_for_unknown_mac() {
        let table = PresenceStateTable::new();
        assert!(table.get_state("00:00:00:00:00:00").is_none());
    }

    #[test]
    fn list_present_filters_correctly() {
        let table = PresenceStateTable::new();
        table.update("AA:BB:CC:DD:EE:FF", -55);
        table.set_state("AA:BB:CC:DD:EE:FF", PresenceState::Entered);
        table.update("11:22:33:44:55:66", -60);
        // 11:22:33:44:55:66 stays Pending
        table.update("FF:FF:FF:FF:FF:FF", -80);
        table.set_state("FF:FF:FF:FF:FF:FF", PresenceState::Exited);

        let present = table.list_present();
        assert_eq!(present.len(), 2);
        assert!(present.iter().any(|d| d.mac == "AA:BB:CC:DD:EE:FF"));
        assert!(present.iter().any(|d| d.mac == "11:22:33:44:55:66"));
    }

    #[test]
    fn list_exited_filters_correctly() {
        let table = PresenceStateTable::new();
        table.update("AA:BB:CC:DD:EE:FF", -55);
        table.set_state("AA:BB:CC:DD:EE:FF", PresenceState::Exited);
        table.update("11:22:33:44:55:66", -60);
        table.set_state("11:22:33:44:55:66", PresenceState::Entered);

        let exited = table.list_exited();
        assert_eq!(exited.len(), 1);
        assert_eq!(exited[0].mac, "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn concurrent_updates_are_safe() {
        let table = std::sync::Arc::new(PresenceStateTable::new());
        let mut handles = vec![];

        for i in 0..10 {
            let t = table.clone();
            handles.push(thread::spawn(move || {
                t.update("AA:BB:CC:DD:EE:FF", -50 - i);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(table.len(), 1);
    }
}
