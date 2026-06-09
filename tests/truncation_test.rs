//! Integration tests for content truncation functionality

use btnotify::output::{truncate_text, TruncationConfig};

#[test]
fn test_truncation_basic() {
    let text = "a".repeat(2000);
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 100);
    assert_eq!(result.total_size, 2000);
}

#[test]
fn test_truncation_disabled() {
    let text = "a".repeat(2000);
    let config = TruncationConfig::disabled();
    let result = truncate_text(&text, &config);
    
    assert!(!result.truncated);
    assert_eq!(result.content.len(), 2000);
    assert_eq!(result.total_size, 2000);
}

#[test]
fn test_truncation_short_text() {
    let text = "Hello, world!";
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    assert!(!result.truncated);
    assert_eq!(result.content, text);
    assert_eq!(result.total_size, text.len());
}

#[test]
fn test_truncation_exact_limit() {
    let text = "a".repeat(100);
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    assert!(!result.truncated);
    assert_eq!(result.content.len(), 100);
}

#[test]
fn test_truncation_with_spaces() {
    let text = "Hello world this is a test string that is quite long and should be truncated";
    let config = TruncationConfig::with_limit(30);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 30);
    // Should break at word boundary
    assert!(!result.content.ends_with(" "));
}

#[test]
fn test_truncation_empty_string() {
    let text = "";
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(text, &config);
    
    assert!(!result.truncated);
    assert_eq!(result.content, "");
    assert_eq!(result.total_size, 0);
}

#[test]
fn test_truncation_single_word() {
    let text = "supercalifragilisticexpialidocious";
    let config = TruncationConfig::with_limit(10);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 10);
}

#[test]
fn test_truncation_help_suggestion() {
    let text = "a".repeat(2000);
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    assert!(result.help_suggestion().is_some());
    let suggestion = result.help_suggestion().unwrap();
    assert!(suggestion.contains("--full"));
    assert!(suggestion.contains("2000"));
}

#[test]
fn test_truncation_no_help_suggestion_when_not_truncated() {
    let text = "Hello";
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(text, &config);
    
    assert!(result.help_suggestion().is_none());
}

#[test]
fn test_truncation_ratio() {
    let text = "a".repeat(2000);
    let config = TruncationConfig::with_limit(1000);
    let result = truncate_text(&text, &config);
    
    let ratio = result.truncation_ratio();
    assert!(ratio > 0.0);
    assert!(ratio < 1.0);
    // Should be approximately 0.5 (1000/2000)
    assert!((ratio - 0.5).abs() < 0.1);
}

#[test]
fn test_truncation_display_format() {
    let text = "a".repeat(2000);
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    let display = format!("{}", result);
    assert!(display.contains("truncated"));
    assert!(display.contains("100"));
    assert!(display.contains("2000"));
}

#[test]
fn test_truncation_config_default() {
    let config = TruncationConfig::default();
    assert!(config.enabled);
    assert_eq!(config.limit, 1000);
}

#[test]
fn test_truncation_config_custom_limit() {
    let config = TruncationConfig::with_limit(500);
    assert!(config.enabled);
    assert_eq!(config.limit, 500);
}

#[test]
fn test_truncation_config_disabled() {
    let config = TruncationConfig::disabled();
    assert!(!config.enabled);
    assert_eq!(config.limit, usize::MAX);
}

#[test]
fn test_truncation_config_with_enabled() {
    let config = TruncationConfig::with_limit(500).with_enabled(false);
    assert!(!config.enabled);
    assert_eq!(config.limit, 500);
}

#[test]
fn test_truncation_multiline_text() {
    let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let config = TruncationConfig::with_limit(20);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 20);
}

#[test]
fn test_truncation_unicode_text() {
    let text = "Hello 世界 🌍 This is a test string with unicode characters";
    let config = TruncationConfig::with_limit(20);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 20);
}

#[test]
fn test_truncation_special_characters() {
    let text = "Hello!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
    let config = TruncationConfig::with_limit(15);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 15);
}

#[test]
fn test_truncation_very_long_single_line() {
    let text = "a".repeat(10000);
    let config = TruncationConfig::with_limit(100);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 100);
    assert_eq!(result.total_size, 10000);
}

#[test]
fn test_truncation_zero_limit() {
    let text = "Hello world";
    let config = TruncationConfig::with_limit(0);
    let result = truncate_text(&text, &config);
    
    // With zero limit, should still return something (empty or minimal)
    assert!(result.content.len() <= 1);
}

#[test]
fn test_truncation_multiple_spaces() {
    let text = "Hello    world    this    is    a    test";
    let config = TruncationConfig::with_limit(20);
    let result = truncate_text(&text, &config);
    
    assert!(result.truncated);
    assert!(result.content.len() <= 20);
}
