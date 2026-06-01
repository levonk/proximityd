use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use tracing::{info, warn};

use crate::state::PresenceEvent;

use super::r#trait::Notifier;

/// Notifier that delivers presence events to Discord via webhook URL
/// or bot token + channel ID.
pub struct DiscordNotifier {
    client: Client,
    webhook_url: Option<String>,
    bot_token: Option<String>,
    channel_id: Option<String>,
    include_timestamp: bool,
    include_mac: bool,
}

impl DiscordNotifier {
    /// Create a notifier that posts to a Discord webhook URL.
    pub fn from_webhook(url: impl Into<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client build"),
            webhook_url: Some(url.into()),
            bot_token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        }
    }

    /// Create a notifier that posts to a channel using a bot token.
    pub fn from_bot_token(
        token: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client build"),
            webhook_url: None,
            bot_token: Some(token.into()),
            channel_id: Some(channel_id.into()),
            include_timestamp: false,
            include_mac: false,
        }
    }

    /// Enable or disable timestamp inclusion in messages.
    pub fn with_timestamp(mut self, enabled: bool) -> Self {
        self.include_timestamp = enabled;
        self
    }

    /// Enable or disable MAC address inclusion in messages.
    pub fn with_mac(mut self, enabled: bool) -> Self {
        self.include_mac = enabled;
        self
    }

    fn build_content(&self, event: &PresenceEvent) -> String {
        let (name, mac, action) = match event {
            PresenceEvent::Entered { name, mac } => (name, mac, "entered"),
            PresenceEvent::Exited { name, mac } => (name, mac, "exited"),
        };

        let mut parts = vec![format!("{} has {} the area", name, action)];

        if self.include_mac {
            parts.push(format!("MAC: {}", mac));
        }

        if self.include_timestamp {
            parts.push(format!(
                "Timestamp: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ));
        }

        parts.join("\n")
    }

    fn send_with_retry<F>(&self, operation: F) -> Result<()>
    where
        F: Fn() -> reqwest::Result<reqwest::blocking::Response>,
    {
        let mut last_err = None;

        for attempt in 0..3 {
            if attempt > 0 {
                let backoff = Duration::from_secs(2_u64.pow(attempt - 1));
                warn!(
                    attempt = attempt + 1,
                    backoff_ms = backoff.as_millis(),
                    "Discord notification retrying"
                );
                std::thread::sleep(backoff);
            }

            match operation() {
                Ok(resp) => {
                    if resp.status().is_success() {
                        info!("Discord notification delivered successfully");
                        return Ok(());
                    }
                    if resp.status().as_u16() == 429 {
                        warn!("Discord rate limited; will retry");
                    }
                    last_err = Some(anyhow::anyhow!(
                        "Discord returned HTTP {}",
                        resp.status()
                    ));
                }
                Err(e) => {
                    warn!(error = %e, "Discord notification request failed");
                    last_err = Some(e.into());
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("Discord delivery failed")))
            .context("Discord notification failed after 3 retries")
    }
}

impl Notifier for DiscordNotifier {
    fn notify(&self, event: &PresenceEvent) -> Result<()> {
        let content = self.build_content(event);

        if let Some(url) = &self.webhook_url {
            let payload = serde_json::json!({ "content": content });

            return self.send_with_retry(|| {
                self.client
                    .post(url)
                    .json(&payload)
                    .send()
            });
        }

        if let (Some(token), Some(channel_id)) = (&self.bot_token, &self.channel_id) {
            let url = format!(
                "https://discord.com/api/v10/channels/{}/messages",
                channel_id
            );
            let payload = serde_json::json!({ "content": content });

            return self.send_with_retry(|| {
                self.client
                    .post(&url)
                    .header("Authorization", format!("Bot {}", token))
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .send()
            });
        }

        Err(anyhow::anyhow!(
            "DiscordNotifier has no webhook_url or bot_token configured"
        ))
    }
}

#[cfg(test)]
#[path = "discord.test.rs"]
mod tests;
