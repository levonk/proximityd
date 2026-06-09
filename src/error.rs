use std::path::Path;
use serde::{Deserialize, Serialize};

/// Standard exit codes for proximityd
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERIC_ERROR: i32 = 1;
pub const EXIT_USAGE_ERROR: i32 = 2;
pub const EXIT_NETWORK_ERROR: i32 = 3;
pub const EXIT_VALIDATION_ERROR: i32 = 4;
pub const EXIT_FILE_NOT_FOUND: i32 = 5;
pub const EXIT_PERMISSION_DENIED: i32 = 6;
pub const EXIT_SIGINT: i32 = 130;

/// Structured error type for agent-friendly error reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredError {
    pub error_type: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub exit_code: i32,
}

impl StructuredError {
    /// Create a new structured error
    pub fn new(error_type: impl Into<String>, message: impl Into<String>, exit_code: i32) -> Self {
        Self {
            error_type: error_type.into(),
            message: message.into(),
            suggestion: None,
            exit_code,
        }
    }

    /// Add an actionable suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Format as JSON for stdout
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Format as TOON for stdout
    pub fn to_toon(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("error_type: {}\n", self.error_type));
        output.push_str(&format!("message: {}\n", self.message));
        if let Some(ref suggestion) = self.suggestion {
            output.push_str(&format!("suggestion: {}\n", suggestion));
        }
        output.push_str(&format!("exit_code: {}\n", self.exit_code));
        output
    }
}

/// Common error types
pub mod error_types {
    pub const CONFIG_ALREADY_EXISTS: &str = "config_already_exists";
    pub const CONFIG_NOT_FOUND: &str = "config_not_found";
    pub const INVALID_CONFIG: &str = "invalid_config";
    pub const PERMISSION_DENIED: &str = "permission_denied";
    pub const FILE_NOT_FOUND: &str = "file_not_found";
    pub const NETWORK_ERROR: &str = "network_error";
    pub const VALIDATION_ERROR: &str = "validation_error";
    pub const USAGE_ERROR: &str = "usage_error";
    pub const GENERIC_ERROR: &str = "generic_error";
}

/// Format an error message with file reference
pub fn format_error_with_file(message: &str, file: &Path, line: Option<u32>, column: Option<u32>) -> String {
    use crate::cli::format_file_reference_simple;
    format!("{} ({})", message, format_file_reference_simple(file, line, column))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_exit_code_constants() {
        assert_eq!(EXIT_SUCCESS, 0);
        assert_eq!(EXIT_GENERIC_ERROR, 1);
        assert_eq!(EXIT_USAGE_ERROR, 2);
        assert_eq!(EXIT_NETWORK_ERROR, 3);
        assert_eq!(EXIT_VALIDATION_ERROR, 4);
        assert_eq!(EXIT_FILE_NOT_FOUND, 5);
        assert_eq!(EXIT_PERMISSION_DENIED, 6);
        assert_eq!(EXIT_SIGINT, 130);
    }

    #[test]
    fn test_format_error_with_file() {
        let path = Path::new("/tmp/test.rs");
        let result = format_error_with_file("Config error", path, Some(10), Some(5));
        assert!(result.contains("Config error"));
        assert!(result.contains("test.rs:10:5"));
    }

    #[test]
    fn test_structured_error_creation() {
        let error = StructuredError::new("test_type", "test message", 1);
        assert_eq!(error.error_type, "test_type");
        assert_eq!(error.message, "test message");
        assert_eq!(error.exit_code, 1);
        assert!(error.suggestion.is_none());
    }

    #[test]
    fn test_structured_error_with_suggestion() {
        let error = StructuredError::new("test_type", "test message", 1)
            .with_suggestion("Try this instead");
        assert_eq!(error.suggestion, Some("Try this instead".to_string()));
    }

    #[test]
    fn test_structured_error_to_json() {
        let error = StructuredError::new("test_type", "test message", 1)
            .with_suggestion("Try this instead");
        let json = error.to_json().unwrap();
        assert!(json.contains("test_type"));
        assert!(json.contains("test message"));
        assert!(json.contains("Try this instead"));
        assert!(json.contains("1"));
    }

    #[test]
    fn test_structured_error_to_toon() {
        let error = StructuredError::new("test_type", "test message", 1)
            .with_suggestion("Try this instead");
        let toon = error.to_toon();
        assert!(toon.contains("error_type: test_type"));
        assert!(toon.contains("message: test message"));
        assert!(toon.contains("suggestion: Try this instead"));
        assert!(toon.contains("exit_code: 1"));
    }

    #[test]
    fn test_error_types_module() {
        assert_eq!(error_types::CONFIG_ALREADY_EXISTS, "config_already_exists");
        assert_eq!(error_types::CONFIG_NOT_FOUND, "config_not_found");
        assert_eq!(error_types::INVALID_CONFIG, "invalid_config");
        assert_eq!(error_types::PERMISSION_DENIED, "permission_denied");
        assert_eq!(error_types::FILE_NOT_FOUND, "file_not_found");
        assert_eq!(error_types::NETWORK_ERROR, "network_error");
        assert_eq!(error_types::VALIDATION_ERROR, "validation_error");
        assert_eq!(error_types::USAGE_ERROR, "usage_error");
        assert_eq!(error_types::GENERIC_ERROR, "generic_error");
    }
}
