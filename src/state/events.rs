/// Events emitted when a device transitions between presence states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceEvent {
    /// Device has been sighted with sufficient RSSI for the required duration.
    Entered {
        /// Human-readable display name.
        name: String,
        /// Bluetooth MAC address.
        mac: String,
    },
    /// Device has not been seen for longer than the exit timeout.
    Exited {
        /// Human-readable display name.
        name: String,
        /// Bluetooth MAC address.
        mac: String,
    },
}

impl PresenceEvent {
    /// Convenience accessor for the MAC address regardless of variant.
    pub fn mac(&self) -> &str {
        match self {
            PresenceEvent::Entered { mac, .. } => mac,
            PresenceEvent::Exited { mac, .. } => mac,
        }
    }

    /// Convenience accessor for the display name regardless of variant.
    pub fn name(&self) -> &str {
        match self {
            PresenceEvent::Entered { name, .. } => name,
            PresenceEvent::Exited { name, .. } => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_entered_fields() {
        let ev = PresenceEvent::Entered {
            name: "Phone".to_string(),
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
        };
        assert_eq!(ev.name(), "Phone");
        assert_eq!(ev.mac(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn event_exited_fields() {
        let ev = PresenceEvent::Exited {
            name: "Watch".to_string(),
            mac: "11:22:33:44:55:66".to_string(),
        };
        assert_eq!(ev.name(), "Watch");
        assert_eq!(ev.mac(), "11:22:33:44:55:66");
    }
}
