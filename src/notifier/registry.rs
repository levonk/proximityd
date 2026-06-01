use anyhow::{Context, Result};
use tracing::info;

use crate::config::{AppConfig, NotifierConfig};

use super::discord::DiscordNotifier;
use super::r#trait::Notifier;

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
        DiscordNotifier::from_webhook(entry.target.clone())
    };

    notifier = notifier.with_timestamp(entry.include_timestamp);
    notifier = notifier.with_mac(entry.include_mac);

    Ok(notifier)
}

#[cfg(test)]
#[path = "registry.test.rs"]
mod tests;
