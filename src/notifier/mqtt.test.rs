use crate::state::PresenceEvent;

use super::MqttNotifier;

#[test]
fn build_payload_basic() {
    let notifier = MqttNotifier::new("localhost", 1883, "proximityd/presence", None).unwrap();
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let payload = notifier.build_payload(&event);
    assert_eq!(payload["name"], "Phone");
    assert_eq!(payload["mac"], "AA:BB:CC:DD:EE:FF");
    assert_eq!(payload["action"], "entered");
    assert!(payload["timestamp"].is_string());
    assert!(payload.get("party").is_none());
    assert!(payload.get("source").is_none());
}

#[test]
fn build_payload_with_rich_fields() {
    let notifier = MqttNotifier::new("localhost", 1883, "proximityd/presence", None).unwrap();
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: Some("John".to_string()),
        source: Some("ble".to_string()),
        id_type: Some("ble_mac".to_string()),
        location: Some("Home/Living Room".to_string()),
    };
    let payload = notifier.build_payload(&event);
    assert_eq!(payload["name"], "Phone");
    assert_eq!(payload["mac"], "AA:BB:CC:DD:EE:FF");
    assert_eq!(payload["action"], "entered");
    assert_eq!(payload["party"], "John");
    assert_eq!(payload["source"], "ble");
    assert_eq!(payload["id_type"], "ble_mac");
    assert_eq!(payload["location"], "Home/Living Room");
}

#[test]
fn build_payload_exited() {
    let notifier = MqttNotifier::new("localhost", 1883, "proximityd/presence", None).unwrap();
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let payload = notifier.build_payload(&event);
    assert_eq!(payload["name"], "Watch");
    assert_eq!(payload["mac"], "11:22:33:44:55:66");
    assert_eq!(payload["action"], "exited");
}

#[test]
fn mqtt_notifier_custom_client_id() {
    let notifier = MqttNotifier::new(
        "localhost",
        1883,
        "proximityd/presence",
        Some("custom_client"),
    )
    .unwrap();
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let payload = notifier.build_payload(&event);
    assert_eq!(payload["name"], "Phone");
}