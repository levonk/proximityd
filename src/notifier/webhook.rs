use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;
use tracing::info;

use crate::state::PresenceEvent;

use super::r#trait::Notifier;

/// Notifier that delivers presence events to a generic HTTP webhook.
pub struct WebhookNotifier {
    client: Client,
    url: String,
    method: String,
    payload_template: String,
}

impl WebhookNotifier {
    /// Create a notifier with custom URL, HTTP method, and payload template.
    ///
    /// The payload template supports the following placeholders:
    /// - `{name}` - Device name
    /// - `{mac}` - MAC address
    /// - `{action}` - "entered" or "exited"
    /// - `{party}` - Party name (if available)
    /// - `{source}` - Signal source (if available)
    /// - `{id_type}` - Identifier type (if available)
    /// - `{location}` - Location (if available)
    /// - `{timestamp}` - Current timestamp
    pub fn new(
        url: impl Into<String>,
        method: impl Into<String>,
        payload_template: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("reqwest client build"),
            url: url.into(),
            method: method.into(),
            payload_template: payload_template.into(),
        }
    }

    /// Replace placeholders in the template with actual values.
    fn render_template(&self, event: &PresenceEvent) -> String {
        let (name, mac, action, party_name, source, id_type, location) = match event {
            PresenceEvent::Entered { name, mac, party_name, source, id_type, location } => {
                (name, mac, "entered", party_name, source, id_type, location)
            }
            PresenceEvent::Exited { name, mac, party_name, source, id_type, location } => {
                (name, mac, "exited", party_name, source, id_type, location)
            }
        };

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let mut result = self.payload_template.clone();
        result = result.replace("{name}", name);
        result = result.replace("{mac}", mac);
        result = result.replace("{action}", action);
        result = result.replace("{party}", party_name.as_deref().unwrap_or(""));
        result = result.replace("{source}", source.as_deref().unwrap_or(""));
        result = result.replace("{id_type}", id_type.as_deref().unwrap_or(""));
        result = result.replace("{location}", location.as_deref().unwrap_or(""));
        result = result.replace("{timestamp}", &timestamp);

        result
    }
}

impl Notifier for WebhookNotifier {
    fn notify(&self, event: &PresenceEvent) -> Result<()> {
        let rendered = self.render_template(event);

        // Try to parse as JSON, fall back to plain text
        let body: serde_json::Value = match serde_json::from_str(&rendered) {
            Ok(json) => json,
            Err(_) => json!({ "message": rendered }),
        };

        let method = self.method.to_uppercase();
        let response = match method.as_str() {
            "GET" => self.client.get(&self.url).query(&body).send(),
            "POST" => self.client.post(&self.url).json(&body).send(),
            "PUT" => self.client.put(&self.url).json(&body).send(),
            "PATCH" => self.client.patch(&self.url).json(&body).send(),
            "DELETE" => self.client.delete(&self.url).send(),
            _ => self.client.post(&self.url).json(&body).send(),
        };

        let response = response.context("Webhook request failed")?;

        if response.status().is_success() {
            info!("Webhook notification delivered successfully");
            Ok(())
        } else {
            let status = response.status();
            let response_body = response.text().unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(anyhow::anyhow!("Webhook returned HTTP {}: {}", status, response_body))
                .context("Webhook notification failed")
        }
    }
}

#[cfg(test)]
#[path = "webhook.test.rs"]
mod tests;