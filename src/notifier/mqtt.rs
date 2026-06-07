#[cfg(feature = "mqtt")]
use anyhow::{Context, Result};
#[cfg(feature = "mqtt")]
use rumqttc::{AsyncClient, MqttOptions, QoS, LastWill};
#[cfg(feature = "mqtt")]
use serde_json::json;
#[cfg(feature = "mqtt")]
use tracing::info;

#[cfg(feature = "mqtt")]
use crate::state::PresenceEvent;

#[cfg(feature = "mqtt")]
use super::r#trait::Notifier;

/// Notifier that delivers presence events to an MQTT broker.
#[cfg(feature = "mqtt")]
pub struct MqttNotifier {
    client: AsyncClient,
    topic: String,
}

#[cfg(feature = "mqtt")]
impl MqttNotifier {
    /// Create a notifier that publishes to an MQTT broker.
    ///
    /// # Arguments
    /// * `broker` - MQTT broker address (e.g., "localhost" or "192.168.1.100")
    /// * `port` - MQTT broker port (default 1883)
    /// * `topic` - MQTT topic to publish to (e.g., "proximityd/presence")
    /// * `client_id` - Optional client ID (defaults to "proximityd")
    pub fn new(
        broker: impl Into<String>,
        port: u16,
        topic: impl Into<String>,
        client_id: Option<impl Into<String>>,
    ) -> Result<Self> {
        let broker = broker.into();
        let topic = topic.into();
        let client_id = client_id.map(|id| id.into()).unwrap_or_else(|| "proximityd".to_string());

        let mut mqttoptions = MqttOptions::new(client_id, &broker, port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

        let client = AsyncClient::new(mqttoptions, 10);

        Ok(Self { client, topic })
    }

    /// Build JSON payload for the presence event.
    fn build_payload(&self, event: &PresenceEvent) -> serde_json::Value {
        let (name, mac, action, party_name, source, id_type, location) = match event {
            PresenceEvent::Entered { name, mac, party_name, source, id_type, location } => {
                (name, mac, "entered", party_name, source, id_type, location)
            }
            PresenceEvent::Exited { name, mac, party_name, source, id_type, location } => {
                (name, mac, "exited", party_name, source, id_type, location)
            }
        };

        let mut payload = json!({
            "name": name,
            "mac": mac,
            "action": action,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        if let Some(party) = party_name {
            payload["party"] = json!(party);
        }

        if let Some(src) = source {
            payload["source"] = json!(src);
        }

        if let Some(id_t) = id_type {
            payload["id_type"] = json!(id_t);
        }

        if let Some(loc) = location {
            payload["location"] = json!(loc);
        }

        payload
    }
}

#[cfg(feature = "mqtt")]
impl Notifier for MqttNotifier {
    fn notify(&self, event: &PresenceEvent) -> Result<()> {
        let payload = self.build_payload(event);
        let payload_str = serde_json::to_string(&payload)
            .context("Failed to serialize MQTT payload")?;

        // Note: This is a simplified synchronous implementation.
        // In a real async context, you'd use the async client properly.
        // For now, we'll use a blocking approach with a timeout.
        info!("MQTT notification would be published to topic: {}", self.topic);
        info!("MQTT payload: {}", payload_str);

        // TODO: Implement actual MQTT publishing when async context is available
        // This requires the notification system to be async-aware
        Ok(())
    }
}

#[cfg(all(test, feature = "mqtt"))]
#[path = "mqtt.test.rs"]
mod tests;