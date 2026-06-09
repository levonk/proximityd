//! Output schema definitions and field selection logic.
//!
//! This module provides minimal default schemas for command outputs to reduce
//! token consumption in agent mode, while allowing explicit field selection via
//! the --fields flag for human mode or detailed analysis.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;

/// Available output fields for different command types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandField {
    // Party fields
    PartyName,
    PartyDeviceCount,
    PartyLocation,
    
    // Device fields
    DeviceName,
    DeviceIdentifierCount,
    DeviceStatus,
    DeviceLocation,
    
    // Status fields
    DaemonStatus,
    ActiveParties,
    ActiveDevices,
    
    // Discover fields
    CorrelationId,
    ConfidenceScore,
    DiscoveredAt,
    
    // Export fields
    Timestamp,
    SignalType,
    SignalValue,
}

impl CommandField {
    /// Get all available fields for a given command type.
    pub fn for_command(command: &str) -> Vec<CommandField> {
        match command {
            "parties" => vec![
                CommandField::PartyName,
                CommandField::PartyDeviceCount,
                CommandField::PartyLocation,
            ],
            "devices" => vec![
                CommandField::DeviceName,
                CommandField::DeviceIdentifierCount,
                CommandField::DeviceStatus,
                CommandField::DeviceLocation,
            ],
            "status" => vec![
                CommandField::DaemonStatus,
                CommandField::ActiveParties,
                CommandField::ActiveDevices,
            ],
            "discover" => vec![
                CommandField::CorrelationId,
                CommandField::ConfidenceScore,
                CommandField::DiscoveredAt,
            ],
            "export" => vec![
                CommandField::Timestamp,
                CommandField::SignalType,
                CommandField::SignalValue,
            ],
            _ => vec![],
        }
    }
    
    /// Get the default minimal fields for a command (3-4 fields max).
    pub fn default_fields(command: &str) -> Vec<CommandField> {
        match command {
            "parties" => vec![
                CommandField::PartyName,
                CommandField::PartyDeviceCount,
                CommandField::PartyLocation,
            ],
            "devices" => vec![
                CommandField::DeviceName,
                CommandField::DeviceIdentifierCount,
                CommandField::DeviceStatus,
            ],
            "status" => vec![
                CommandField::DaemonStatus,
                CommandField::ActiveParties,
            ],
            "discover" => vec![
                CommandField::CorrelationId,
                CommandField::ConfidenceScore,
            ],
            "export" => vec![
                CommandField::Timestamp,
                CommandField::SignalType,
            ],
            _ => vec![],
        }
    }
    
    /// Convert field to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandField::PartyName => "name",
            CommandField::PartyDeviceCount => "device_count",
            CommandField::PartyLocation => "location",
            CommandField::DeviceName => "name",
            CommandField::DeviceIdentifierCount => "identifier_count",
            CommandField::DeviceStatus => "status",
            CommandField::DeviceLocation => "location",
            CommandField::DaemonStatus => "daemon_status",
            CommandField::ActiveParties => "active_parties",
            CommandField::ActiveDevices => "active_devices",
            CommandField::CorrelationId => "correlation_id",
            CommandField::ConfidenceScore => "confidence_score",
            CommandField::DiscoveredAt => "discovered_at",
            CommandField::Timestamp => "timestamp",
            CommandField::SignalType => "signal_type",
            CommandField::SignalValue => "signal_value",
        }
    }
}

impl FromStr for CommandField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(CommandField::PartyName), // Shared by party/device
            "device_count" => Ok(CommandField::PartyDeviceCount),
            "location" => Ok(CommandField::PartyLocation), // Shared by party/device
            "identifier_count" => Ok(CommandField::DeviceIdentifierCount),
            "status" => Ok(CommandField::DeviceStatus),
            "daemon_status" => Ok(CommandField::DaemonStatus),
            "active_parties" => Ok(CommandField::ActiveParties),
            "active_devices" => Ok(CommandField::ActiveDevices),
            "correlation_id" => Ok(CommandField::CorrelationId),
            "confidence_score" => Ok(CommandField::ConfidenceScore),
            "discovered_at" => Ok(CommandField::DiscoveredAt),
            "timestamp" => Ok(CommandField::Timestamp),
            "signal_type" => Ok(CommandField::SignalType),
            "signal_value" => Ok(CommandField::SignalValue),
            _ => Err(format!("Invalid field name: {}", s)),
        }
    }
}

/// Schema configuration for output formatting.
#[derive(Debug, Clone)]
pub struct OutputSchema {
    /// Selected fields for output.
    pub fields: Vec<CommandField>,
    /// Command type this schema applies to.
    pub command: String,
}

impl OutputSchema {
    /// Create a new schema with default fields for the given command.
    pub fn new(command: &str) -> Self {
        Self {
            fields: CommandField::default_fields(command),
            command: command.to_string(),
        }
    }
    
    /// Create a schema with explicit field selection.
    pub fn with_fields(command: &str, field_names: &[String]) -> Result<Self> {
        let available_fields = CommandField::for_command(command);
        let available_set: HashSet<_> = available_fields.iter().collect();
        
        let mut fields = Vec::new();
        let mut invalid_fields = Vec::new();
        
        for field_name in field_names {
            if let Ok(field) = CommandField::from_str(field_name) {
                // Check if field is valid for this command
                if available_set.contains(&field) {
                    fields.push(field);
                } else {
                    invalid_fields.push(field_name.clone());
                }
            } else {
                invalid_fields.push(field_name.clone());
            }
        }
        
        if !invalid_fields.is_empty() {
            bail!(
                "Invalid fields for command '{}': {}. Available fields: {}",
                command,
                invalid_fields.join(", "),
                available_fields.iter().map(|f| f.as_str()).collect::<Vec<_>>().join(", ")
            );
        }
        
        if fields.is_empty() {
            bail!("At least one valid field must be specified");
        }
        
        Ok(Self {
            fields,
            command: command.to_string(),
        })
    }
    
    /// Check if a field is included in this schema.
    pub fn has_field(&self, field: CommandField) -> bool {
        self.fields.contains(&field)
    }
    
    /// Get the number of fields in this schema.
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

/// A minimal representation of party data for output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyOutput {
    pub name: String,
    pub device_count: usize,
    pub location: Option<String>,
}

/// A minimal representation of device data for output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOutput {
    pub name: String,
    pub identifier_count: usize,
    pub status: String,
    pub location: Option<String>,
}

/// A minimal representation of status data for output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOutput {
    pub daemon_status: String,
    pub active_parties: usize,
    pub active_devices: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_fields_parties() {
        let fields = CommandField::default_fields("parties");
        assert_eq!(fields.len(), 3);
        assert!(fields.contains(&CommandField::PartyName));
        assert!(fields.contains(&CommandField::PartyDeviceCount));
        assert!(fields.contains(&CommandField::PartyLocation));
    }
    
    #[test]
    fn test_default_fields_devices() {
        let fields = CommandField::default_fields("devices");
        assert_eq!(fields.len(), 3);
        assert!(fields.contains(&CommandField::DeviceName));
        assert!(fields.contains(&CommandField::DeviceIdentifierCount));
        assert!(fields.contains(&CommandField::DeviceStatus));
    }
    
    #[test]
    fn test_default_fields_status() {
        let fields = CommandField::default_fields("status");
        assert_eq!(fields.len(), 2);
        assert!(fields.contains(&CommandField::DaemonStatus));
        assert!(fields.contains(&CommandField::ActiveParties));
    }
    
    #[test]
    fn test_schema_with_valid_fields() {
        let schema = OutputSchema::with_fields("parties", &["name".to_string(), "device_count".to_string()])
            .expect("Valid fields");
        assert_eq!(schema.field_count(), 2);
        assert!(schema.has_field(CommandField::PartyName));
        assert!(schema.has_field(CommandField::PartyDeviceCount));
    }
    
    #[test]
    fn test_schema_with_invalid_fields() {
        let result = OutputSchema::with_fields("parties", &["invalid_field".to_string()]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_schema_field_not_available_for_command() {
        let result = OutputSchema::with_fields("parties", &["daemon_status".to_string()]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_field_parsing() {
        assert_eq!(CommandField::from_str("name"), Ok(CommandField::PartyName));
        assert_eq!(CommandField::from_str("device_count"), Ok(CommandField::PartyDeviceCount));
        assert!(CommandField::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_field_to_string() {
        assert_eq!(CommandField::PartyName.as_str(), "name");
        assert_eq!(CommandField::PartyDeviceCount.as_str(), "device_count");
    }
}
