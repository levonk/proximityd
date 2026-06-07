use crate::config::{AppConfig, NotifierConfig};

use super::NotifierRegistry;

#[test]
fn empty_registry_has_len_zero() {
    let reg = NotifierRegistry::empty();
    assert_eq!(reg.len(), 0);
    assert!(reg.is_empty());
}

#[test]
fn from_config_builds_discord_from_webhook() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "discord".to_string(),
            webhook_url: "https://discord.com/api/webhooks/123".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        }],
        ..Default::default()
    };

    let reg = NotifierRegistry::from_config(&config).expect("valid registry");
    assert_eq!(reg.len(), 1);
}

#[test]
fn from_config_builds_discord_from_bot_token() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "discord".to_string(),
            webhook_url: String::new(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: Some("my-bot-token".to_string()),
            channel_id: Some("123456789".to_string()),
            include_timestamp: true,
            include_mac: true,
        }],
        ..Default::default()
    };

    let reg = NotifierRegistry::from_config(&config).expect("valid registry");
    assert_eq!(reg.len(), 1);
}

#[test]
fn from_config_builds_slack() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "slack".to_string(),
            webhook_url: "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: true,
            include_mac: true,
        }],
        ..Default::default()
    };

    let reg = NotifierRegistry::from_config(&config).expect("valid registry");
    assert_eq!(reg.len(), 1);
}

#[test]
fn from_config_builds_webhook() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "webhook".to_string(),
            url: "https://example.com/webhook".to_string(),
            method: "POST".to_string(),
            payload_template: "{\"name\": \"{name}\", \"action\": \"{action}\"}".to_string(),
            webhook_url: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        }],
        ..Default::default()
    };

    let reg = NotifierRegistry::from_config(&config).expect("valid registry");
    assert_eq!(reg.len(), 1);
}

#[test]
fn from_config_skips_unknown_kind() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "unknown".to_string(),
            webhook_url: "https://example.com/webhook".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        }],
        ..Default::default()
    };

    let reg = NotifierRegistry::from_config(&config).expect("valid registry");
    assert_eq!(reg.len(), 0);
    assert!(reg.is_empty());
}

#[test]
fn from_config_bot_token_missing_channel_id_fails() {
    let config = AppConfig {
        notifiers: vec![NotifierConfig {
            kind: "discord".to_string(),
            webhook_url: String::new(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: Some("my-bot-token".to_string()),
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        }],
        ..Default::default()
    };

    let result = NotifierRegistry::from_config(&config);
    assert!(result.is_err());
}
