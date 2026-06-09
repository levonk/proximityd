//! Content truncation for large text fields.
//!
//! This module provides truncation logic to reduce token consumption while
//! maintaining useful previews of content. Truncation includes metadata
//! about the original size and can be disabled via the --full flag.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Default truncation limit in characters.
pub const DEFAULT_TRUNCATION_LIMIT: usize = 1000;

/// Truncation configuration.
#[derive(Debug, Clone, Copy)]
pub struct TruncationConfig {
    /// Maximum characters before truncation.
    pub limit: usize,
    /// Whether truncation is enabled.
    pub enabled: bool,
}

impl Default for TruncationConfig {
    fn default() -> Self {
        Self {
            limit: DEFAULT_TRUNCATION_LIMIT,
            enabled: true,
        }
    }
}

impl TruncationConfig {
    /// Create a new truncation config with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a config with a custom limit.
    pub fn with_limit(limit: usize) -> Self {
        Self {
            limit,
            enabled: true,
        }
    }

    /// Disable truncation (full output).
    pub fn disabled() -> Self {
        Self {
            limit: usize::MAX,
            enabled: false,
        }
    }

    /// Enable truncation with a custom limit.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Truncation result with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncatedText {
    /// The (possibly truncated) text content.
    pub content: String,
    /// Total size of the original text.
    pub total_size: usize,
    /// Whether the content was truncated.
    pub truncated: bool,
}

impl TruncatedText {
    /// Create a non-truncated text result.
    pub fn full(content: String) -> Self {
        let total_size = content.len();
        Self {
            content,
            total_size,
            truncated: false,
        }
    }

    /// Create a truncated text result.
    pub fn truncated(content: String, total_size: usize) -> Self {
        Self {
            content,
            total_size,
            truncated: true,
        }
    }

    /// Get the truncation ratio (0.0 = no truncation, 1.0 = completely truncated).
    pub fn truncation_ratio(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            1.0 - (self.content.len() as f64 / self.total_size as f64)
        }
    }

    /// Get a help suggestion for viewing full content.
    pub fn help_suggestion(&self) -> Option<String> {
        if self.truncated {
            Some(format!(
                "Content truncated ({} chars total, showing {}). Use --full flag to disable truncation.",
                self.total_size,
                self.content.len()
            ))
        } else {
            None
        }
    }
}

impl fmt::Display for TruncatedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)?;
        if self.truncated {
            write!(f, " [truncated: {}/{} chars]", self.content.len(), self.total_size)?;
        }
        Ok(())
    }
}

/// Truncate text according to the given configuration.
pub fn truncate_text(text: &str, config: &TruncationConfig) -> TruncatedText {
    if !config.enabled || text.len() <= config.limit {
        return TruncatedText::full(text.to_string());
    }

    // Truncate at the limit, but try to break at a word boundary
    let limit = config.limit;
    let truncated = if let Some(space_pos) = text[..limit].rfind(' ') {
        &text[..space_pos]
    } else {
        &text[..limit]
    };

    TruncatedText::truncated(truncated.to_string(), text.len())
}

/// Truncate text with a default configuration.
pub fn truncate(text: &str) -> TruncatedText {
    truncate_text(text, &TruncationConfig::default())
}

/// Truncate text with a custom limit.
pub fn truncate_with_limit(text: &str, limit: usize) -> TruncatedText {
    truncate_text(text, &TruncationConfig::with_limit(limit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_short_text() {
        let text = "Hello, world!";
        let result = truncate(text);
        assert!(!result.truncated);
        assert_eq!(result.content, text);
        assert_eq!(result.total_size, text.len());
    }

    #[test]
    fn test_truncate_long_text() {
        let text = "a".repeat(2000);
        let result = truncate(&text);
        assert!(result.truncated);
        assert!(result.content.len() <= DEFAULT_TRUNCATION_LIMIT);
        assert_eq!(result.total_size, 2000);
    }

    #[test]
    fn test_truncate_with_custom_limit() {
        let text = "a".repeat(500);
        let result = truncate_with_limit(&text, 100);
        assert!(result.truncated);
        assert!(result.content.len() <= 100);
    }

    #[test]
    fn test_truncate_disabled() {
        let text = "a".repeat(2000);
        let config = TruncationConfig::disabled();
        let result = truncate_text(&text, &config);
        assert!(!result.truncated);
        assert_eq!(result.content.len(), 2000);
    }

    #[test]
    fn test_truncate_word_boundary() {
        let text = "Hello world this is a test string that is quite long";
        let result = truncate_with_limit(text, 20);
        assert!(result.truncated);
        // Should break at word boundary (at space), not in the middle of a word
        // With limit 20, we get "Hello world this is" (18 chars) or "Hello world this" (15 chars)
        assert!(result.content.len() <= 20);
        // Should not end with a partial word
        assert!(!result.content.ends_with(" th") && !result.content.ends_with(" te"));
    }

    #[test]
    fn test_truncation_ratio() {
        let text = "a".repeat(2000);
        let result = truncate(&text);
        let ratio = result.truncation_ratio();
        assert!(ratio > 0.0);
        assert!(ratio < 1.0);
    }

    #[test]
    fn test_help_suggestion() {
        let text = "a".repeat(2000);
        let result = truncate(&text);
        assert!(result.help_suggestion().is_some());

        let short_text = "Hello";
        let short_result = truncate(short_text);
        assert!(short_result.help_suggestion().is_none());
    }

    #[test]
    fn test_display() {
        let text = "a".repeat(2000);
        let result = truncate(&text);
        let display = format!("{}", result);
        assert!(display.contains("truncated"));
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let result = truncate(text);
        assert!(!result.truncated);
        assert_eq!(result.content, "");
        assert_eq!(result.total_size, 0);
    }

    #[test]
    fn test_exact_limit() {
        let text = "a".repeat(DEFAULT_TRUNCATION_LIMIT);
        let result = truncate(&text);
        assert!(!result.truncated);
        assert_eq!(result.content.len(), DEFAULT_TRUNCATION_LIMIT);
    }
}
