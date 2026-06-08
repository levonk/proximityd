/// Standard exit codes for proximityd
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERIC_ERROR: i32 = 1;
pub const EXIT_USAGE_ERROR: i32 = 2;
pub const EXIT_NETWORK_ERROR: i32 = 3;
pub const EXIT_VALIDATION_ERROR: i32 = 4;
pub const EXIT_FILE_NOT_FOUND: i32 = 5;
pub const EXIT_PERMISSION_DENIED: i32 = 6;
pub const EXIT_SIGINT: i32 = 130;

#[cfg(test)]
mod tests {
    use super::*;

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
}
