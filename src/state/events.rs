/// Events emitted when a device transitions between presence states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceEvent {
    /// Device has been sighted with sufficient RSSI for the required duration.
    Entered {
        /// Human-readable display name.
        name: String,
        /// Bluetooth MAC address.
        mac: String,
        /// Party name (if available).
        party_name: Option<String>,
        /// Signal source (e.g., "ble", "wifi_arp", "ping_sweep", "mdns").
        source: Option<String>,
        /// Identifier type (e.g., "ble_mac", "wifi_mac", "ip_v4", "hostname").
        id_type: Option<String>,
        /// Location context (building, floor, room, zone).
        location: Option<String>,
    },
    /// Device has not been seen for longer than the exit timeout.
    Exited {
        /// Human-readable display name.
        name: String,
        /// Bluetooth MAC address.
        mac: String,
        /// Party name (if available).
        party_name: Option<String>,
        /// Signal source (e.g., "ble", "wifi_arp", "ping_sweep", "mdns").
        source: Option<String>,
        /// Identifier type (e.g., "ble_mac", "wifi_mac", "ip_v4", "hostname").
        id_type: Option<String>,
        /// Location context (building, floor, room, zone).
        location: Option<String>,
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

    /// Convenience accessor for the party name regardless of variant.
    pub fn party_name(&self) -> Option<&str> {
        match self {
            PresenceEvent::Entered { party_name, .. } => party_name.as_deref(),
            PresenceEvent::Exited { party_name, .. } => party_name.as_deref(),
        }
    }

    /// Convenience accessor for the signal source regardless of variant.
    pub fn source(&self) -> Option<&str> {
        match self {
            PresenceEvent::Entered { source, .. } => source.as_deref(),
            PresenceEvent::Exited { source, .. } => source.as_deref(),
        }
    }

    /// Convenience accessor for the identifier type regardless of variant.
    pub fn id_type(&self) -> Option<&str> {
        match self {
            PresenceEvent::Entered { id_type, .. } => id_type.as_deref(),
            PresenceEvent::Exited { id_type, .. } => id_type.as_deref(),
        }
    }

    /// Convenience accessor for the location regardless of variant.
    pub fn location(&self) -> Option<&str> {
        match self {
            PresenceEvent::Entered { location, .. } => location.as_deref(),
            PresenceEvent::Exited { location, .. } => location.as_deref(),
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
            party_name: Some("John".to_string()),
            source: Some("ble".to_string()),
            id_type: Some("ble_mac".to_string()),
            location: Some("Home/Living Room".to_string()),
        };
        assert_eq!(ev.name(), "Phone");
        assert_eq!(ev.mac(), "AA:BB:CC:DD:EE:FF");
        assert_eq!(ev.party_name(), Some("John"));
        assert_eq!(ev.source(), Some("ble"));
        assert_eq!(ev.id_type(), Some("ble_mac"));
        assert_eq!(ev.location(), Some("Home/Living Room"));
    }

    #[test]
    fn event_exited_fields() {
        let ev = PresenceEvent::Exited {
            name: "Watch".to_string(),
            mac: "11:22:33:44:55:66".to_string(),
            party_name: None,
            source: Some("wifi_arp".to_string()),
            id_type: Some("wifi_mac".to_string()),
            location: None,
        };
        assert_eq!(ev.name(), "Watch");
        assert_eq!(ev.mac(), "11:22:33:44:55:66");
        assert_eq!(ev.party_name(), None);
        assert_eq!(ev.source(), Some("wifi_arp"));
        assert_eq!(ev.id_type(), Some("wifi_mac"));
        assert_eq!(ev.location(), None);
    }

    #[test]
    fn event_backward_compatibility() {
        // Test that events can still be created with minimal fields
        let ev = PresenceEvent::Entered {
            name: "Phone".to_string(),
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
            party_name: None,
            source: None,
            id_type: None,
            location: None,
        };
        assert_eq!(ev.name(), "Phone");
        assert_eq!(ev.mac(), "AA:BB:CC:DD:EE:FF");
        assert!(ev.party_name().is_none());
        assert!(ev.source().is_none());
        assert!(ev.id_type().is_none());
        assert!(ev.location().is_none());
    }
}
