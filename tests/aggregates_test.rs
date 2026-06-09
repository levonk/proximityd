//! Integration tests for aggregate computation.

use btnotify::output::{ListAggregate, PartyAggregate, DeviceAggregate, SystemAggregate};

#[test]
fn test_list_aggregate_integration() {
    let items = vec![1, 2, 3, 4, 5];
    let agg = ListAggregate::from_collection(&items);
    
    assert_eq!(agg.total, 5);
    assert_eq!(agg.count, 5);
    assert!(!agg.has_more);
    assert_eq!(agg.count_of_total(), "5 of 5 total");
}

#[test]
fn test_party_aggregate_integration() {
    let agg = PartyAggregate::new(3);
    
    assert_eq!(agg.device_count, 3);
    assert!(agg.devices_present.is_none());
    assert!(agg.device_status_summary.is_none());
}

#[test]
fn test_party_aggregate_with_presence_integration() {
    let agg = PartyAggregate::with_presence(3, 2);
    
    assert_eq!(agg.device_count, 3);
    assert_eq!(agg.devices_present, Some(2));
    assert_eq!(agg.device_status_summary, Some("2/3 present".to_string()));
}

#[test]
fn test_device_aggregate_integration() {
    let agg = DeviceAggregate::new(3);
    
    assert_eq!(agg.identifier_count, 3);
    assert!(agg.identifiers_active.is_none());
    assert!(agg.identifier_status_summary.is_none());
}

#[test]
fn test_device_aggregate_with_activity_integration() {
    let agg = DeviceAggregate::with_activity(3, 3);
    
    assert_eq!(agg.identifier_count, 3);
    assert_eq!(agg.identifiers_active, Some(3));
    assert_eq!(agg.identifier_status_summary, Some("3/3 active".to_string()));
}

#[test]
fn test_system_aggregate_integration() {
    let agg = SystemAggregate::new(5, 10, 2, 4);
    
    assert_eq!(agg.total_parties, 5);
    assert_eq!(agg.total_devices, 10);
    assert_eq!(agg.active_parties, 2);
    assert_eq!(agg.active_devices, 4);
    assert!(agg.status_summary.contains("5 parties"));
    assert!(agg.status_summary.contains("10 devices"));
}

#[test]
fn test_aggregate_serialization() {
    let agg = ListAggregate::from_collection(&vec![1, 2, 3]);
    let json = serde_json::to_string(&agg).unwrap();
    
    assert!(json.contains("\"total\":3"));
    assert!(json.contains("\"count\":3"));
    
    let deserialized: ListAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.total, 3);
    assert_eq!(deserialized.count, 3);
}

#[test]
fn test_party_aggregate_serialization() {
    let agg = PartyAggregate::with_presence(3, 2);
    let json = serde_json::to_string(&agg).unwrap();
    
    assert!(json.contains("\"device_count\":3"));
    assert!(json.contains("\"devices_present\":2"));
    
    let deserialized: PartyAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.device_count, 3);
    assert_eq!(deserialized.devices_present, Some(2));
}

#[test]
fn test_device_aggregate_serialization() {
    let agg = DeviceAggregate::with_activity(3, 2);
    let json = serde_json::to_string(&agg).unwrap();
    
    assert!(json.contains("\"identifier_count\":3"));
    assert!(json.contains("\"identifiers_active\":2"));
    
    let deserialized: DeviceAggregate = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.identifier_count, 3);
    assert_eq!(deserialized.identifiers_active, Some(2));
}

#[test]
fn test_aggregate_efficiency_empty_collection() {
    let items: Vec<i32> = vec![];
    let agg = ListAggregate::from_collection(&items);
    
    assert_eq!(agg.total, 0);
    assert_eq!(agg.count, 0);
    assert!(!agg.has_more);
    assert_eq!(agg.count_of_total(), "0 of 0 total");
}

#[test]
fn test_aggregate_efficiency_large_collection() {
    let items: Vec<i32> = (0..10000).collect();
    let agg = ListAggregate::from_collection(&items);
    
    assert_eq!(agg.total, 10000);
    assert_eq!(agg.count, 10000);
    assert!(!agg.has_more);
}

#[test]
fn test_aggregate_efficiency_paginated() {
    let agg = ListAggregate::paginated(10000, 100, 0);
    
    assert_eq!(agg.total, 10000);
    assert_eq!(agg.count, 100);
    assert!(agg.has_more);
}

#[test]
fn test_aggregate_efficiency_last_page() {
    let agg = ListAggregate::paginated(10000, 100, 9900);
    
    assert_eq!(agg.total, 10000);
    assert_eq!(agg.count, 100);
    assert!(!agg.has_more);
}
