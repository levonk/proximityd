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
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
            ..Default::default()
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
            token: Some("my-bot-token".to_string()),
            channel_id: Some("123456789".to_string()),
            include_timestamp: true,
            include_mac: true,
            ..Default::default()
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
            kind: "slack".to_string(),
            webhook_url: "https://hooks.slack.com/xxx".to_string(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
            ..Default::default()
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
            token: Some("my-bot-token".to_string()),
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
            ..Default::default()
        }],
        ..Default::default()
    };

    let result = NotifierRegistry::from_config(&config);
    assert!(result.is_err());
}
