use std::time::Instant;

/// The presence state of a tracked device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceState {
    /// Device has crossed the enter threshold and debounce period.
    Entered,
    /// Device has not been seen for longer than the exit timeout.
    Exited,
    /// Device is visible but has not yet satisfied the enter debounce.
    Pending,
}

/// In-memory representation of a device being tracked.
#[derive(Debug, Clone)]
pub struct TrackedDevice {
    /// Bluetooth MAC address (e.g., "AA:BB:CC:DD:EE:FF").
    pub mac: String,
    /// Human-readable display name (may be empty if unmapped).
    pub name: String,
    /// Timestamp of the most recent BLE sighting.
    pub last_seen: Instant,
    /// Last known RSSI in dBm.
    pub rssi: i16,
    /// Current presence state.
    pub state: PresenceState,
}

impl TrackedDevice {
    /// Create a new tracked device with the given MAC and initial RSSI.
    /// `name` is initialised as an empty string; the caller should
    /// populate it from the device mapping config if known.
    pub fn new(mac: impl Into<String>, rssi: i16) -> Self {
        Self {
            mac: mac.into(),
            name: String::new(),
            last_seen: Instant::now(),
            rssi,
            state: PresenceState::Pending,
        }
    }

    /// Update the last-seen timestamp and RSSI.
    pub fn update_rssi(&mut self, rssi: i16) {
        self.last_seen = Instant::now();
        self.rssi = rssi;
    }

    /// Elapsed time since the device was last seen.
    pub fn elapsed_since_seen(&self) -> std::time::Duration {
        self.last_seen.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn tracked_device_new_defaults_to_pending() {
        let dev = TrackedDevice::new("AA:BB:CC:DD:EE:FF", -60);
        assert_eq!(dev.mac, "AA:BB:CC:DD:EE:FF");
        assert_eq!(dev.name, "");
        assert_eq!(dev.rssi, -60);
        assert_eq!(dev.state, PresenceState::Pending);
    }

    #[test]
    fn update_rssi_changes_last_seen_and_rssi() {
        let mut dev = TrackedDevice::new("AA:BB:CC:DD:EE:FF", -60);
        let before = dev.last_seen;
        thread::sleep(Duration::from_millis(10));
        dev.update_rssi(-55);
        assert!(dev.last_seen > before);
        assert_eq!(dev.rssi, -55);
    }

    #[test]
    fn elapsed_since_seen_increases_over_time() {
        let dev = TrackedDevice::new("AA:BB:CC:DD:EE:FF", -60);
        thread::sleep(Duration::from_millis(10));
        assert!(dev.elapsed_since_seen() >= Duration::from_millis(10));
    }

    #[test]
    fn presence_state_equality() {
        assert_eq!(PresenceState::Entered, PresenceState::Entered);
        assert_ne!(PresenceState::Entered, PresenceState::Exited);
    }
}
