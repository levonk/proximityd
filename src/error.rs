use std::path::Path;

/// Standard exit codes for proximityd
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERIC_ERROR: i32 = 1;
pub const EXIT_USAGE_ERROR: i32 = 2;
pub const EXIT_NETWORK_ERROR: i32 = 3;
pub const EXIT_VALIDATION_ERROR: i32 = 4;
pub const EXIT_FILE_NOT_FOUND: i32 = 5;
pub const EXIT_PERMISSION_DENIED: i32 = 6;
pub const EXIT_SIGINT: i32 = 130;

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
}
