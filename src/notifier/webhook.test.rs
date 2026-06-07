use crate::state::PresenceEvent;

use super::WebhookNotifier;

#[test]
fn render_template_basic() {
    let notifier = WebhookNotifier::new(
        "https://example.com/webhook",
        "POST",
        "{\"name\": \"{name}\", \"action\": \"{action}\"}",
    );
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let rendered = notifier.render_template(&event);
    assert_eq!(rendered, "{\"name\": \"Phone\", \"action\": \"entered\"}");
}

#[test]
fn render_template_with_all_fields() {
    let notifier = WebhookNotifier::new(
        "https://example.com/webhook",
        "POST",
        "{\"name\": \"{name}\", \"mac\": \"{mac}\", \"action\": \"{action}\", \"party\": \"{party}\", \"source\": \"{source}\", \"id_type\": \"{id_type}\", \"location\": \"{location}\"}",
    );
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: Some("John".to_string()),
        source: Some("ble".to_string()),
        id_type: Some("ble_mac".to_string()),
        location: Some("Home/Living Room".to_string()),
    };
    let rendered = notifier.render_template(&event);
    assert!(rendered.contains("\"name\": \"Phone\""));
    assert!(rendered.contains("\"mac\": \"AA:BB:CC:DD:EE:FF\""));
    assert!(rendered.contains("\"action\": \"entered\""));
    assert!(rendered.contains("\"party\": \"John\""));
    assert!(rendered.contains("\"source\": \"ble\""));
    assert!(rendered.contains("\"id_type\": \"ble_mac\""));
    assert!(rendered.contains("\"location\": \"Home/Living Room\""));
}

#[test]
fn render_template_with_timestamp() {
    let notifier = WebhookNotifier::new(
        "https://example.com/webhook",
        "POST",
        "{\"name\": \"{name}\", \"timestamp\": \"{timestamp}\"}",
    );
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let rendered = notifier.render_template(&event);
    assert!(rendered.contains("\"name\": \"Watch\""));
    assert!(rendered.contains("\"timestamp\":"));
}

#[test]
fn render_template_plain_text() {
    let notifier = WebhookNotifier::new(
        "https://example.com/webhook",
        "POST",
        "Device {name} has {action}",
    );
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let rendered = notifier.render_template(&event);
    assert_eq!(rendered, "Device Phone has entered");
}

#[test]
fn render_template_empty_optional_fields() {
    let notifier = WebhookNotifier::new(
        "https://example.com/webhook",
        "POST",
        "{\"party\": \"{party}\", \"source\": \"{source}\"}",
    );
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        party_name: None,
        source: None,
        id_type: None,
        location: None,
    };
    let rendered = notifier.render_template(&event);
    assert_eq!(rendered, "{\"party\": \"\", \"source\": \"\"}");
}