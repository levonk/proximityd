//! Empty state formatting for definitive empty results.
//!
//! This module provides consistent empty state formatting across all commands,
//! ensuring agents can distinguish between empty results and errors.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Empty state context information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyContext {
    /// The command that produced the empty result.
    pub command: String,
    /// Filter criteria applied (if any).
    pub filters: Option<Vec<String>>,
    /// Scope of the query (e.g., "all parties", "active devices").
    pub scope: String,
}

impl EmptyContext {
    /// Create a new empty context.
    pub fn new(command: &str, scope: &str) -> Self {
        Self {
            command: command.to_string(),
            filters: None,
            scope: scope.to_string(),
        }
    }

    /// Add filter criteria to the context.
    pub fn with_filters(mut self, filters: Vec<String>) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// Empty state message formatter.
pub struct EmptyFormatter {
    /// Whether to include context in the message.
    include_context: bool,
}

impl EmptyFormatter {
    /// Create a new empty formatter.
    pub fn new(include_context: bool) -> Self {
        Self { include_context }
    }

    /// Format an empty state message for human-readable output.
    pub fn format_human(&self, context: &EmptyContext) -> String {
        let mut message = format!("0 results: {}\n", context.scope);

        if self.include_context {
            if let Some(filters) = &context.filters {
                if !filters.is_empty() {
                    message.push_str("Filters applied:\n");
                    for filter in filters {
                        message.push_str(&format!("  - {}\n", filter));
                    }
                }
            }
        }

        message
    }

    /// Format an empty state message for TOON output.
    pub fn format_toon(&self, context: &EmptyContext) -> String {
        let mut output = String::from("count: 0\n");
        output.push_str(&format!("scope: {}\n", context.scope));

        if self.include_context {
            if let Some(filters) = &context.filters {
                if !filters.is_empty() {
                    output.push_str("filters: ");
                    let filter_str = filters.join(",");
                    output.push_str(&format!("{}\n", filter_str));
                }
            }
        }

        output
    }

    /// Format an empty state message for JSON output.
    pub fn format_json(&self, context: &EmptyContext) -> Result<String> {
        let output = serde_json::to_string_pretty(context)?;
        Ok(output)
    }
}

impl Default for EmptyFormatter {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context_creation() {
        let context = EmptyContext::new("parties", "all configured parties");
        assert_eq!(context.command, "parties");
        assert_eq!(context.scope, "all configured parties");
        assert!(context.filters.is_none());
    }

    #[test]
    fn test_empty_context_with_filters() {
        let context = EmptyContext::new("devices", "configured devices")
            .with_filters(vec!["location:office".to_string(), "status:active".to_string()]);
        assert_eq!(context.filters.unwrap().len(), 2);
    }

    #[test]
    fn test_format_human_without_context() {
        let formatter = EmptyFormatter::new(false);
        let context = EmptyContext::new("parties", "all configured parties");
        let output = formatter.format_human(&context);
        assert!(output.starts_with("0 results:"));
        assert!(!output.contains("Filters"));
    }

    #[test]
    fn test_format_human_with_context() {
        let formatter = EmptyFormatter::new(true);
        let context = EmptyContext::new("devices", "configured devices")
            .with_filters(vec!["location:office".to_string()]);
        let output = formatter.format_human(&context);
        assert!(output.contains("Filters applied:"));
        assert!(output.contains("location:office"));
    }

    #[test]
    fn test_format_toon() {
        let formatter = EmptyFormatter::new(true);
        let context = EmptyContext::new("status", "active devices");
        let output = formatter.format_toon(&context);
        assert!(output.contains("count: 0"));
        assert!(output.contains("scope: active devices"));
    }

    #[test]
    fn test_format_json() {
        let formatter = EmptyFormatter::new(true);
        let context = EmptyContext::new("parties", "all parties");
        let output = formatter.format_json(&context).unwrap();
        assert!(output.contains("\"command\""));
        assert!(output.contains("\"scope\""));
    }
}
