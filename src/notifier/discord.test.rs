use crate::state::PresenceEvent;

use super::DiscordNotifier;

#[test]
fn build_content_entered() {
    let notifier = DiscordNotifier::from_webhook("https://example.com");
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
    };
    assert_eq!(notifier.build_content(&event), "Phone has entered the area");
}

#[test]
fn build_content_exited() {
    let notifier = DiscordNotifier::from_webhook("https://example.com");
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
    };
    assert_eq!(notifier.build_content(&event), "Watch has exited the area");
}

#[test]
fn build_content_with_mac() {
    let notifier = DiscordNotifier::from_webhook("https://example.com").with_mac(true);
    let event = PresenceEvent::Entered {
        name: "Phone".to_string(),
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
    };
    let content = notifier.build_content(&event);
    assert!(content.contains("Phone has entered the area"));
    assert!(content.contains("MAC: AA:BB:CC:DD:EE:FF"));
}

#[test]
fn build_content_with_timestamp() {
    let notifier = DiscordNotifier::from_webhook("https://example.com").with_timestamp(true);
    let event = PresenceEvent::Exited {
        name: "Watch".to_string(),
        mac: "11:22:33:44:55:66".to_string(),
    };
    let content = notifier.build_content(&event);
    assert!(content.contains("Watch has exited the area"));
    assert!(content.contains("Timestamp:"));
}
