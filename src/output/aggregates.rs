//! Pre-computed aggregate counts and derived status fields.
//!
//! This module provides efficient aggregate computation for list outputs,
//! including total counts and lightweight summaries to reduce the need for
//! follow-up queries in agent mode.

use serde::{Deserialize, Serialize};

/// Aggregate metadata for list outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAggregate {
    /// Total number of items in the collection.
    pub total: usize,
    /// Number of items returned in this page/response.
    pub count: usize,
    /// Whether there are more items beyond this page.
    pub has_more: bool,
}

impl ListAggregate {
    /// Create a new aggregate from a collection.
    pub fn from_collection<T>(items: &[T]) -> Self {
        let total = items.len();
        Self {
            total,
            count: total,
            has_more: false,
        }
    }

    /// Create a new aggregate for a paginated response.
    pub fn paginated(total: usize, count: usize, offset: usize) -> Self {
        Self {
            total,
            count,
            has_more: offset + count < total,
        }
    }

    /// Get the "count of total" string representation.
    pub fn count_of_total(&self) -> String {
        format!("{} of {} total", self.count, self.total)
    }
}

/// Derived status fields for party outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyAggregate {
    /// Number of devices configured for this party.
    pub device_count: usize,
    /// Number of devices currently present (if available).
    pub devices_present: Option<usize>,
    /// Device status summary (e.g., "2/3 present").
    pub device_status_summary: Option<String>,
}

impl PartyAggregate {
    /// Create a new party aggregate from device count.
    pub fn new(device_count: usize) -> Self {
        Self {
            device_count,
            devices_present: None,
            device_status_summary: None,
        }
    }

    /// Create a party aggregate with presence information.
    pub fn with_presence(device_count: usize, devices_present: usize) -> Self {
        let device_status_summary = if device_count > 0 {
            Some(format!("{}/{} present", devices_present, device_count))
        } else {
            None
        };

        Self {
            device_count,
            devices_present: Some(devices_present),
            device_status_summary,
        }
    }
}

/// Derived status fields for device outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAggregate {
    /// Number of identifiers associated with this device.
    pub identifier_count: usize,
    /// Number of identifiers currently active (if available).
    pub identifiers_active: Option<usize>,
    /// Identifier status summary (e.g., "3/3 active").
    pub identifier_status_summary: Option<String>,
}

impl DeviceAggregate {
    /// Create a new device aggregate from identifier count.
    pub fn new(identifier_count: usize) -> Self {
        Self {
            identifier_count,
            identifiers_active: None,
            identifier_status_summary: None,
        }
    }

    /// Create a device aggregate with activity information.
    pub fn with_activity(identifier_count: usize, identifiers_active: usize) -> Self {
        let identifier_status_summary = if identifier_count > 0 {
            Some(format!("{}/{} active", identifiers_active, identifier_count))
        } else {
            None
        };

        Self {
            identifier_count,
            identifiers_active: Some(identifiers_active),
            identifier_status_summary,
        }
    }
}

/// System-wide aggregate information for status outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAggregate {
    /// Total number of configured parties.
    pub total_parties: usize,
    /// Total number of configured devices.
    pub total_devices: usize,
    /// Number of parties currently active/present.
    pub active_parties: usize,
    /// Number of devices currently active/present.
    pub active_devices: usize,
    /// System status summary.
    pub status_summary: String,
}

impl SystemAggregate {
    /// Create a new system aggregate.
    pub fn new(
        total_parties: usize,
        total_devices: usize,
        active_parties: usize,
        active_devices: usize,
    ) -> Self {
        let status_summary = if total_parties > 0 || total_devices > 0 {
            format!(
                "{} parties, {} devices ({} active parties, {} active devices)",
                total_parties, total_devices, active_parties, active_devices
            )
        } else {
            "No parties or devices configured".to_string()
        };

        Self {
            total_parties,
            total_devices,
            active_parties,
            active_devices,
            status_summary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_aggregate_from_collection() {
        let items = vec![1, 2, 3, 4, 5];
        let agg = ListAggregate::from_collection(&items);

        assert_eq!(agg.total, 5);
        assert_eq!(agg.count, 5);
        assert!(!agg.has_more);
    }

    #[test]
    fn test_list_aggregate_paginated() {
        let agg = ListAggregate::paginated(100, 10, 0);
        assert_eq!(agg.total, 100);
        assert_eq!(agg.count, 10);
        assert!(agg.has_more);

        let agg = ListAggregate::paginated(100, 10, 90);
        assert_eq!(agg.total, 100);
        assert_eq!(agg.count, 10);
        assert!(!agg.has_more);
    }

    #[test]
    fn test_count_of_total() {
        let agg = ListAggregate::paginated(100, 10, 0);
        assert_eq!(agg.count_of_total(), "10 of 100 total");
    }

    #[test]
    fn test_party_aggregate_new() {
        let agg = PartyAggregate::new(3);
        assert_eq!(agg.device_count, 3);
        assert!(agg.devices_present.is_none());
        assert!(agg.device_status_summary.is_none());
    }

    #[test]
    fn test_party_aggregate_with_presence() {
        let agg = PartyAggregate::with_presence(3, 2);
        assert_eq!(agg.device_count, 3);
        assert_eq!(agg.devices_present, Some(2));
        assert_eq!(agg.device_status_summary, Some("2/3 present".to_string()));
    }

    #[test]
    fn test_party_aggregate_with_presence_zero_devices() {
        let agg = PartyAggregate::with_presence(0, 0);
        assert_eq!(agg.device_count, 0);
        assert_eq!(agg.devices_present, Some(0));
        assert!(agg.device_status_summary.is_none());
    }

    #[test]
    fn test_device_aggregate_new() {
        let agg = DeviceAggregate::new(3);
        assert_eq!(agg.identifier_count, 3);
        assert!(agg.identifiers_active.is_none());
        assert!(agg.identifier_status_summary.is_none());
    }

    #[test]
    fn test_device_aggregate_with_activity() {
        let agg = DeviceAggregate::with_activity(3, 3);
        assert_eq!(agg.identifier_count, 3);
        assert_eq!(agg.identifiers_active, Some(3));
        assert_eq!(agg.identifier_status_summary, Some("3/3 active".to_string()));
    }

    #[test]
    fn test_device_aggregate_with_activity_partial() {
        let agg = DeviceAggregate::with_activity(3, 2);
        assert_eq!(agg.identifier_count, 3);
        assert_eq!(agg.identifiers_active, Some(2));
        assert_eq!(agg.identifier_status_summary, Some("2/3 active".to_string()));
    }

    #[test]
    fn test_device_aggregate_with_activity_zero_identifiers() {
        let agg = DeviceAggregate::with_activity(0, 0);
        assert_eq!(agg.identifier_count, 0);
        assert_eq!(agg.identifiers_active, Some(0));
        assert!(agg.identifier_status_summary.is_none());
    }

    #[test]
    fn test_system_aggregate_new() {
        let agg = SystemAggregate::new(5, 10, 2, 4);
        assert_eq!(agg.total_parties, 5);
        assert_eq!(agg.total_devices, 10);
        assert_eq!(agg.active_parties, 2);
        assert_eq!(agg.active_devices, 4);
        assert!(agg.status_summary.contains("5 parties"));
        assert!(agg.status_summary.contains("10 devices"));
        assert!(agg.status_summary.contains("2 active parties"));
        assert!(agg.status_summary.contains("4 active devices"));
    }

    #[test]
    fn test_system_aggregate_no_config() {
        let agg = SystemAggregate::new(0, 0, 0, 0);
        assert_eq!(agg.total_parties, 0);
        assert_eq!(agg.total_devices, 0);
        assert_eq!(agg.active_parties, 0);
        assert_eq!(agg.active_devices, 0);
        assert_eq!(agg.status_summary, "No parties or devices configured");
    }
}
