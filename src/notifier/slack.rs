use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use tracing::info;

use crate::state::PresenceEvent;

use super::r#trait::Notifier;

/// Notifier that delivers presence events to Slack via webhook URL.
pub struct SlackNotifier {
    client: Client,
    webhook_url: String,
    include_timestamp: bool,
    include_mac: bool,
}

impl SlackNotifier {
    /// Create a notifier that posts to a Slack webhook URL.
    pub fn from_webhook(url: impl Into<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client build"),
            webhook_url: url.into(),
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

    fn build_message(&self, event: &PresenceEvent) -> String {
        let (name, mac, action, party_name, source, id_type, location) = match event {
            PresenceEvent::Entered { name, mac, party_name, source, id_type, location } => {
                (name, mac, "entered", party_name, source, id_type, location)
            }
            PresenceEvent::Exited { name, mac, party_name, source, id_type, location } => {
                (name, mac, "exited", party_name, source, id_type, location)
            }
        };

        let mut parts = vec![format!("*{} has {} the area*", name, action)];

        if let Some(party) = party_name {
            parts.push(format!("Party: {}", party));
        }

        if let Some(src) = source {
            parts.push(format!("Source: {}", src));
        }

        if let Some(id_t) = id_type {
            parts.push(format!("ID Type: {}", id_t));
        }

        if self.include_mac {
            parts.push(format!("MAC: {}", mac));
        }

        if let Some(loc) = location {
            parts.push(format!("Location: {}", loc));
        }

        if self.include_timestamp {
            parts.push(format!(
                "Timestamp: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ));
        }

        parts.join("\n")
    }
}

impl Notifier for SlackNotifier {
    fn notify(&self, event: &PresenceEvent) -> Result<()> {
        let text = self.build_message(event);

        let payload = serde_json::json!({
            "text": text
        });

        let response = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .context("Slack webhook request failed")?;

        if response.status().is_success() {
            info!("Slack notification delivered successfully");
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(anyhow::anyhow!("Slack returned HTTP {}: {}", status, body))
                .context("Slack notification failed")
        }
    }
}

#[cfg(test)]
#[path = "slack.test.rs"]
mod tests;