pub mod discord;
pub mod registry;
pub mod r#trait;
pub mod slack;
pub mod webhook;

#[cfg(feature = "mqtt")]
pub mod mqtt;

pub use discord::DiscordNotifier;
pub use r#trait::Notifier;
pub use registry::NotifierRegistry;
pub use slack::SlackNotifier;
pub use webhook::WebhookNotifier;

#[cfg(feature = "mqtt")]
pub use mqtt::MqttNotifier;
