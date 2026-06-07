use crate::state::PresenceEvent;

use super::SlackNotifier;

#[test]
fn build_message_entered() {
    let notifier = SlackNotifier::from_webhook("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX");
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    assert_eq!(notifier.build_message(&event), "*Phone has entered the area*");
}

#[test]
fn build_message_exited() {
    let notifier = SlackNotifier::from_webhook("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX");
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    assert_eq!(notifier.build_message(&event), "*Watch has exited the area*");
}

#[test]
fn build_message_with_mac() {
    let notifier = SlackNotifier::from_webhook("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX").with_mac(true);
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let message = notifier.build_message(&event);
    assert!(message.contains("*Phone has entered the area*"));
    assert!(message.contains("MAC: AA:BB:CC:DD:EE:FF"));
}

#[test]
fn build_message_with_timestamp() {
    let notifier = SlackNotifier::from_webhook("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX").with_timestamp(true);
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let message = notifier.build_message(&event);
    assert!(message.contains("*Watch has exited the area*"));
    assert!(message.contains("Timestamp:"));
}

#[test]
fn build_message_with_rich_fields() {
    let notifier = SlackNotifier::from_webhook("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX");
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: Some("John".to_string()),
        source: Some("ble".to_string()),
        id_type: Some("ble_mac".to_string()),
        location: Some("Home/Living Room".to_string()),
    };
    let message = notifier.build_message(&event);
    assert!(message.contains("*Phone has entered the area*"));
    assert!(message.contains("Party: John"));
    assert!(message.contains("Source: ble"));
    assert!(message.contains("ID Type: ble_mac"));
    assert!(message.contains("Location: Home/Living Room"));
}