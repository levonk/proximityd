//! Tests for output schema and field selection

use btnotify::output::{CommandField, OutputSchema, PartyOutput, DeviceOutput, StatusOutput};
use serde_json;

#[test]
fn test_command_field_for_command() {
    let parties_fields = CommandField::for_command("parties");
    assert_eq!(parties_fields.len(), 3);
    assert!(parties_fields.contains(&CommandField::PartyName));
    assert!(parties_fields.contains(&CommandField::PartyDeviceCount));
    assert!(parties_fields.contains(&CommandField::PartyLocation));

    let devices_fields = CommandField::for_command("devices");
    assert_eq!(devices_fields.len(), 4);
    assert!(devices_fields.contains(&CommandField::DeviceName));
    assert!(devices_fields.contains(&CommandField::DeviceIdentifierCount));
    assert!(devices_fields.contains(&CommandField::DeviceStatus));
    assert!(devices_fields.contains(&CommandField::DeviceLocation));

    let status_fields = CommandField::for_command("status");
    assert_eq!(status_fields.len(), 3);
    assert!(status_fields.contains(&CommandField::DaemonStatus));
    assert!(status_fields.contains(&CommandField::ActiveParties));
    assert!(status_fields.contains(&CommandField::ActiveDevices));
}

#[test]
fn test_command_field_default_fields() {
    let parties_default = CommandField::default_fields("parties");
    assert_eq!(parties_default.len(), 3);

    let devices_default = CommandField::default_fields("devices");
    assert_eq!(devices_default.len(), 3);

    let status_default = CommandField::default_fields("status");
    assert_eq!(status_default.len(), 2);
}

#[test]
fn test_output_schema_new() {
    let schema = OutputSchema::new("parties");
    assert_eq!(schema.command, "parties");
    assert_eq!(schema.field_count(), 3);
    assert!(schema.has_field(CommandField::PartyName));
    assert!(schema.has_field(CommandField::PartyDeviceCount));
    assert!(schema.has_field(CommandField::PartyLocation));
}

#[test]
fn test_output_schema_with_fields() {
    let schema = OutputSchema::with_fields("parties", &["name".to_string(), "device_count".to_string()])
        .expect("Valid fields");
    assert_eq!(schema.field_count(), 2);
    assert!(schema.has_field(CommandField::PartyName));
    assert!(schema.has_field(CommandField::PartyDeviceCount));
    assert!(!schema.has_field(CommandField::PartyLocation));
}

#[test]
fn test_output_schema_with_invalid_field() {
    let result = OutputSchema::with_fields("parties", &["invalid_field".to_string()]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid fields"));
}

#[test]
fn test_output_schema_with_field_not_available_for_command() {
    let result = OutputSchema::with_fields("parties", &["daemon_status".to_string()]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid fields"));
}

#[test]
fn test_output_schema_empty_fields() {
    let result = OutputSchema::with_fields("parties", &[]);
    assert!(result.is_err());
}

#[test]
fn test_party_output_serialization() {
    let party = PartyOutput {
        name: "Alice".to_string(),
        device_count: 2,
        location: Some("Home, Floor 1".to_string()),
    };

    let json = serde_json::to_string(&party).expect("Serialization failed");
    assert!(json.contains("Alice"));
    assert!(json.contains("2"));
    assert!(json.contains("Home"));
}

#[test]
fn test_device_output_serialization() {
    let device = DeviceOutput {
        name: "iPhone".to_string(),
        identifier_count: 2,
        status: "configured".to_string(),
        location: Some("Home".to_string()),
    };

    let json = serde_json::to_string(&device).expect("Serialization failed");
    assert!(json.contains("iPhone"));
    assert!(json.contains("2"));
    assert!(json.contains("configured"));
}

#[test]
fn test_status_output_serialization() {
    let status = StatusOutput {
        daemon_status: "running".to_string(),
        active_parties: 3,
        active_devices: Some(5),
    };

    let json = serde_json::to_string(&status).expect("Serialization failed");
    assert!(json.contains("running"));
    assert!(json.contains("3"));
    assert!(json.contains("5"));
}

#[test]
fn test_status_output_without_active_devices() {
    let status = StatusOutput {
        daemon_status: "running".to_string(),
        active_parties: 3,
        active_devices: None,
    };

    let json = serde_json::to_string(&status).expect("Serialization failed");
    assert!(json.contains("running"));
    assert!(json.contains("3"));
    // When None, JSON serializes as null
    assert!(json.contains("active_devices"));
    assert!(json.contains("null"));
}

#[test]
fn test_command_field_parsing() {
    assert_eq!(CommandField::from_str("name"), Ok(CommandField::PartyName));
    assert_eq!(CommandField::from_str("device_count"), Ok(CommandField::PartyDeviceCount));
    assert_eq!(CommandField::from_str("identifier_count"), Ok(CommandField::DeviceIdentifierCount));
    assert_eq!(CommandField::from_str("daemon_status"), Ok(CommandField::DaemonStatus));
    assert_eq!(CommandField::from_str("active_parties"), Ok(CommandField::ActiveParties));
    assert!(CommandField::from_str("invalid").is_err());
}

#[test]
fn test_command_field_to_string() {
    assert_eq!(CommandField::PartyName.as_str(), "name");
    assert_eq!(CommandField::PartyDeviceCount.as_str(), "device_count");
    assert_eq!(CommandField::DeviceIdentifierCount.as_str(), "identifier_count");
    assert_eq!(CommandField::DaemonStatus.as_str(), "daemon_status");
    assert_eq!(CommandField::ActiveParties.as_str(), "active_parties");
}
