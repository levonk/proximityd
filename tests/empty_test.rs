//! Empty state formatting tests.
//!
//! Tests for definitive empty state formatting across all commands.

use btnotify::output::{EmptyContext, EmptyFormatter};

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

#[test]
fn test_empty_formatter_default() {
    let formatter = EmptyFormatter::default();
    let context = EmptyContext::new("status", "active devices");
    let output = formatter.format_human(&context);
    assert!(output.contains("0 results:"));
}
