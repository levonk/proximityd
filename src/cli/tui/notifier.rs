use anyhow::{Context, Result};
use std::time::Duration;
use tokio::runtime::Runtime;
use tracing::{info, error};

use crate::config::app::{AppConfig, NotifierConfig};
use crate::state::PresenceEvent;
use crate::notifier::{Notifier, DiscordNotifier, SlackNotifier, WebhookNotifier};

#[cfg(feature = "mqtt")]
use crate::notifier::MqttNotifier;

/// Result of a test notification
#[derive(Debug, Clone)]
pub enum TestResult {
    Success {
        message: String,
        timestamp: String,
    },
    Error {
        message: String,
        details: String,
        timestamp: String,
    },
    Loading,
}

impl TestResult {
    /// Check if the test is currently loading
    pub fn is_loading(&self) -> bool {
        matches!(self, TestResult::Loading)
    }

    /// Check if the test succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, TestResult::Success { .. })
    }

    /// Get the display message
    pub fn display_message(&self) -> String {
        match self {
            TestResult::Success { message, .. } => format!("✓ {}", message),
            TestResult::Error { message, .. } => format!("✗ {}", message),
            TestResult::Loading => "⏳ Sending...".to_string(),
        }
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> String {
        match self {
            TestResult::Success { timestamp, .. } => timestamp.clone(),
            TestResult::Error { timestamp, .. } => timestamp.clone(),
            TestResult::Loading => chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    /// Get detailed error information if available
    pub fn error_details(&self) -> Option<String> {
        match self {
            TestResult::Error { details, .. } => Some(details.clone()),
            _ => None,
        }
    }
}

/// Notifier test manager
pub struct NotifierTestManager {
    /// Config with notifier settings
    config: AppConfig,
    /// Test results for each notifier
    test_results: Vec<TestResult>,
    /// Currently selected notifier index
    selected: usize,
    /// Tokio runtime for async operations
    runtime: Runtime,
}

impl NotifierTestManager {
    /// Create a new notifier test manager
    pub fn new(config: AppConfig) -> Self {
        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        
        Self {
            config,
            test_results: Vec::new(),
            selected: 0,
            runtime,
        }
    }

    /// Get the list of configured notifiers
    pub fn notifier_names(&self) -> Vec<String> {
        self.config.notifiers
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{} ({})", n.kind, i))
            .collect()
    }

    /// Get the number of configured notifiers
    pub fn notifier_count(&self) -> usize {
        self.config.notifiers.len()
    }

    /// Get the selected notifier index
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Set the selected notifier index
    pub fn set_selected(&mut self, index: usize) {
        if index < self.notifier_count() {
            self.selected = index;
        }
    }

    /// Get test result for a specific notifier
    pub fn test_result(&self, index: usize) -> Option<&TestResult> {
        self.test_results.get(index)
    }

    /// Create a sample presence event for testing
    fn create_test_event(&self, _notifier_type: &str) -> PresenceEvent {
        PresenceEvent::Entered {
            name: "Test Device".to_string(),
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
            party_name: Some("Test Party".to_string()),
            source: Some("Test".to_string()),
            id_type: Some("ble_mac".to_string()),
            location: Some("Building A, Floor 1, Room 101".to_string()),
        }
    }

    /// Send a test notification to a specific notifier
    pub fn send_test_notification(&mut self, index: usize) -> Result<()> {
        if index >= self.config.notifiers.len() {
            return Err(anyhow::anyhow!("Invalid notifier index"));
        }

        // Ensure we have enough test results
        while self.test_results.len() <= index {
            self.test_results.push(TestResult::Loading);
        }

        self.test_results[index] = TestResult::Loading;

        let notifier_config = self.config.notifiers[index].clone();
        let test_event = self.create_test_event(&notifier_config.kind);

        let result = self.runtime.block_on(async {
            Self::send_test_notification_async(&notifier_config, &test_event).await
        });

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match result {
            Ok(_) => {
                self.test_results[index] = TestResult::Success {
                    message: format!("Test notification sent to {}", notifier_config.kind),
                    timestamp,
                };
                info!("Test notification sent to {}", notifier_config.kind);
            }
            Err(e) => {
                self.test_results[index] = TestResult::Error {
                    message: format!("Failed to send to {}", notifier_config.kind),
                    details: e.to_string(),
                    timestamp,
                };
                error!("Failed to send test notification to {}: {}", notifier_config.kind, e);
            }
        }

        Ok(())
    }

    /// Send a test notification asynchronously
    async fn send_test_notification_async(
        config: &NotifierConfig,
        event: &PresenceEvent,
    ) -> Result<()> {
        let notifier = Self::build_notifier(config)?;

        // Add timeout to prevent hanging
        tokio::time::timeout(Duration::from_secs(10), async {
            notifier.notify(event)
        })
        .await
        .context("Test notification timed out after 10 seconds")?
    }

    /// Build a notifier from config
    fn build_notifier(config: &NotifierConfig) -> Result<Box<dyn Notifier>> {
        match config.kind.as_str() {
            "discord" => {
                let webhook_url = if !config.webhook_url.is_empty() {
                    &config.webhook_url
                } else if !config.url.is_empty() {
                    &config.url
                } else {
                    return Err(anyhow::anyhow!("Discord notifier requires URL"));
                };
                Ok(Box::new(DiscordNotifier::from_webhook(webhook_url)))
            }
            "slack" => {
                let webhook_url = if !config.webhook_url.is_empty() {
                    &config.webhook_url
                } else if !config.url.is_empty() {
                    &config.url
                } else {
                    return Err(anyhow::anyhow!("Slack notifier requires URL"));
                };
                Ok(Box::new(SlackNotifier::from_webhook(webhook_url)))
            }
            "webhook" => {
                let url = if !config.url.is_empty() {
                    &config.url
                } else if !config.webhook_url.is_empty() {
                    &config.webhook_url
                } else {
                    return Err(anyhow::anyhow!("Webhook notifier requires URL"));
                };
                let method = if !config.method.is_empty() {
                    config.method.as_str()
                } else {
                    "POST"
                };
                let payload_template = config.payload_template.as_str();
                Ok(Box::new(WebhookNotifier::new(url, method, payload_template)))
            }
            "mqtt" => {
                #[cfg(feature = "mqtt")]
                {
                    let broker = if !config.broker.is_empty() {
                        &config.broker
                    } else {
                        return Err(anyhow::anyhow!("MQTT notifier requires broker"));
                    };
                    let port = config.port;
                    let topic = if !config.topic.is_empty() {
                        &config.topic
                    } else {
                        return Err(anyhow::anyhow!("MQTT notifier requires topic"));
                    };
                    Ok(Box::new(MqttNotifier::new(broker, port, topic)))
                }
                #[cfg(not(feature = "mqtt"))]
                {
                    Err(anyhow::anyhow!("MQTT support is not enabled in this build"))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown notifier type: {}", config.kind)),
        }
    }

    /// Clear all test results
    pub fn clear_results(&mut self) {
        self.test_results.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifier_test_manager_creation() {
        let config = AppConfig::default();
        let manager = NotifierTestManager::new(config);
        assert_eq!(manager.notifier_count(), 0);
        assert_eq!(manager.selected(), 0);
    }

    #[test]
    fn test_notifier_names_empty() {
        let config = AppConfig::default();
        let manager = NotifierTestManager::new(config);
        assert!(manager.notifier_names().is_empty());
    }

    #[test]
    fn test_test_result_success() {
        let result = TestResult::Success {
            message: "Test passed".to_string(),
            timestamp: "2024-01-01 12:00:00".to_string(),
        };
        assert!(result.is_success());
        assert!(!result.is_loading());
        assert!(result.display_message().starts_with("✓"));
    }

    #[test]
    fn test_test_result_error() {
        let result = TestResult::Error {
            message: "Test failed".to_string(),
            details: "Connection error".to_string(),
            timestamp: "2024-01-01 12:00:00".to_string(),
        };
        assert!(!result.is_success());
        assert!(!result.is_loading());
        assert!(result.display_message().starts_with("✗"));
        assert_eq!(result.error_details(), Some("Connection error".to_string()));
    }

    #[test]
    fn test_test_result_loading() {
        let result = TestResult::Loading;
        assert!(!result.is_success());
        assert!(result.is_loading());
        assert!(result.display_message().contains("Sending"));
    }

    #[test]
    fn test_clear_results() {
        let config = AppConfig::default();
        let mut manager = NotifierTestManager::new(config);
        manager.test_results.push(TestResult::Loading);
        manager.clear_results();
        assert!(manager.test_results.is_empty());
    }
}
