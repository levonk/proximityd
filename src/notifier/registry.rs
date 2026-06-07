use anyhow::{Context, Result};
use tracing::info;

use crate::config::{AppConfig, NotifierConfig};

use super::discord::DiscordNotifier;
use super::r#trait::Notifier;
use super::slack::SlackNotifier;
use super::webhook::WebhookNotifier;

#[cfg(feature = "mqtt")]
use super::mqtt::MqttNotifier;

/// Registry that holds all active notifiers built from application config.
pub struct NotifierRegistry {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl NotifierRegistry {
    /// Create an empty registry (useful when notifications are disabled).
    pub fn empty() -> Self {
        Self {
            notifiers: Vec::new(),
        }
    }

    /// Build a registry from the notifier entries in `AppConfig`.
    ///
    /// Each entry with `kind = "discord"` becomes a `DiscordNotifier`.
    /// Each entry with `kind = "slack"` becomes a `SlackNotifier`.
    /// Each entry with `kind = "webhook"` becomes a `WebhookNotifier`.
    /// Each entry with `kind = "mqtt"` becomes a `MqttNotifier` (requires mqtt feature).
    /// Unknown kinds are logged and skipped.
    pub fn from_config(config: &AppConfig) -> Result<Self> {
        let mut notifiers: Vec<Box<dyn Notifier>> = Vec::new();

        for entry in &config.notifiers {
            match entry.kind.as_str() {
                "discord" => {
                    let notifier = build_discord(entry)
                        .context("Failed to build Discord notifier from config")?;
                    notifiers.push(Box::new(notifier));
                }
                "slack" => {
                    let notifier = build_slack(entry)
                        .context("Failed to build Slack notifier from config")?;
                    notifiers.push(Box::new(notifier));
                }
                "webhook" => {
                    let notifier = build_webhook(entry)
                        .context("Failed to build Webhook notifier from config")?;
                    notifiers.push(Box::new(notifier));
                }
                "mqtt" => {
                    #[cfg(feature = "mqtt")]
                    {
                        let notifier = build_mqtt(entry)
                            .context("Failed to build MQTT notifier from config")?;
                        notifiers.push(Box::new(notifier));
                    }
                    #[cfg(not(feature = "mqtt"))]
                    {
                        info!("MQTT notifier requested but mqtt feature is not enabled");
                    }
                }
                other => {
                    info!(kind = %other, "Skipping unknown notifier kind");
                }
            }
        }

        Ok(Self { notifiers })
    }

    /// Dispatch a presence event to every registered notifier.
    ///
    /// Errors from individual notifiers are logged but do not prevent
    /// delivery to the remaining notifiers.
    pub fn dispatch(&self, event: &crate::state::PresenceEvent) {
        for notifier in &self.notifiers {
            if let Err(e) = notifier.notify(event) {
                tracing::error!(error = %e, "Notifier delivery failed");
            }
        }
    }

    /// Number of active notifiers in the registry.
    pub fn len(&self) -> usize {
        self.notifiers.len()
    }

    /// Whether the registry contains any notifiers.
    pub fn is_empty(&self) -> bool {
        self.notifiers.is_empty()
    }
}

fn build_discord(entry: &NotifierConfig) -> Result<DiscordNotifier> {
    let mut notifier = if let Some(token) = &entry.token {
        let channel_id = entry
            .channel_id
            .as_ref()
            .context("discord notifier with bot token requires channel_id")?;
        DiscordNotifier::from_bot_token(token.clone(), channel_id.clone())
    } else {
        DiscordNotifier::from_webhook(entry.webhook_url.clone())
    };

    notifier = notifier.with_timestamp(entry.include_timestamp);
    notifier = notifier.with_mac(entry.include_mac);

    Ok(notifier)
}

fn build_slack(entry: &NotifierConfig) -> Result<SlackNotifier> {
    let mut notifier = SlackNotifier::from_webhook(entry.webhook_url.clone());
    notifier = notifier.with_timestamp(entry.include_timestamp);
    notifier = notifier.with_mac(entry.include_mac);
    Ok(notifier)
}

fn build_webhook(entry: &NotifierConfig) -> Result<WebhookNotifier> {
    let url = if entry.url.is_empty() {
        entry.webhook_url.clone()
    } else {
        entry.url.clone()
    };

    let method = if entry.method.is_empty() {
        "POST".to_string()
    } else {
        entry.method.clone()
    };

    let template = if entry.payload_template.is_empty() {
        "{\"name\": \"{name}\", \"action\": \"{action}\"}".to_string()
    } else {
        entry.payload_template.clone()
    };

    Ok(WebhookNotifier::new(url, method, template))
}

#[cfg(feature = "mqtt")]
fn build_mqtt(entry: &NotifierConfig) -> Result<MqttNotifier> {
    let broker = if entry.broker.is_empty() {
        "localhost".to_string()
    } else {
        entry.broker.clone()
    };

    let topic = if entry.topic.is_empty() {
        "proximityd/presence".to_string()
    } else {
        entry.topic.clone()
    };

    Ok(MqttNotifier::new(broker, entry.port, topic, None::<String>)?)
}

#[cfg(test)]
#[path = "registry.test.rs"]
mod tests;
