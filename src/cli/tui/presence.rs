#![allow(clippy::unnecessary_cast)]
use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use crate::config::presence::{Party, Device, Identifier, IdentifierType, Location, PresenceConfig};
use crate::config::loader;
use std::fs;
use toml;

/// Presence management state for TUI
pub struct PresenceManager {
    /// Loaded presence config
    pub config: PresenceConfig,
    /// Currently selected party index
    pub selected_party: Option<usize>,
    /// Currently selected device index (within selected party)
    pub selected_device: Option<usize>,
    /// Currently selected identifier index (within selected device)
    pub selected_identifier: Option<usize>,
    /// Editor for adding/editing entities
    pub editor: Option<PresenceEditor>,
    /// Confirmation dialog state
    pub confirmation: Option<ConfirmationDialog>,
    /// Save message
    pub save_message: Option<String>,
}

/// Editor for presence entities
pub enum PresenceEditor {
    Party(PartyEditor),
    Device(DeviceEditor),
    Identifier(IdentifierEditor),
}

/// Editor for party fields
pub struct PartyEditor {
    pub name: String,
    pub location: Location,
    pub editing_field: PartyField,
    pub dirty: bool,
}

/// Editor for device fields
pub struct DeviceEditor {
    pub name: String,
    pub location: Location,
    pub editing_field: DeviceField,
    pub dirty: bool,
}

/// Editor for identifier fields
pub struct IdentifierEditor {
    pub name: String,
    pub id_type: IdentifierType,
    pub value: String,
    pub editing_field: IdentifierField,
    pub dirty: bool,
}

/// Fields in party editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyField {
    Name,
    Building,
    Floor,
    Room,
    Zone,
}

/// Fields in device editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceField {
    Name,
    Building,
    Floor,
    Room,
    Zone,
}

/// Fields in identifier editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierField {
    Name,
    Type,
    Value,
}

/// Confirmation dialog state
pub struct ConfirmationDialog {
    pub message: String,
    pub confirmed: bool,
    pub callback: Box<dyn Fn(&mut PresenceManager) + Send>,
}

impl PresenceManager {
    /// Create a new presence manager by loading presence.toml
    pub fn new() -> Result<Self> {
        let config_dir = loader::resolve_config_dir();
        let presence_path = config_dir.join("presence.toml");

        let config = if presence_path.exists() {
            let content = fs::read_to_string(&presence_path)
                .context(format!("Failed to read {}", presence_path.display()))?;
            toml::from_str(&content)
                .context("Failed to parse presence.toml")?
        } else {
            PresenceConfig::default()
        };

        Ok(PresenceManager {
            config,
            selected_party: None,
            selected_device: None,
            selected_identifier: None,
            editor: None,
            confirmation: None,
            save_message: None,
        })
    }

    /// Get the current party count
    pub fn party_count(&self) -> usize {
        self.config.parties.len()
    }

    /// Get the device count for the selected party
    pub fn device_count(&self) -> usize {
        if let Some(party_idx) = self.selected_party {
            if let Some(party) = self.config.parties.get(party_idx) {
                return party.devices.len();
            }
        }
        0
    }

    /// Get the identifier count for the selected device
    pub fn identifier_count(&self) -> usize {
        if let Some(party_idx) = self.selected_party {
            if let Some(device_idx) = self.selected_device {
                if let Some(party) = self.config.parties.get(party_idx) {
                    if let Some(device) = party.devices.get(device_idx) {
                        return device.identifiers.len();
                    }
                }
            }
        }
        0
    }

    /// Add a new party
    pub fn add_party(&mut self) {
        let new_party = Party {
            name: "New Party".to_string(),
            location: None,
            devices: vec![],
        };
        self.config.parties.push(new_party);
        self.selected_party = Some(self.config.parties.len() - 1);
        self.selected_device = None;
        self.selected_identifier = None;
    }

    /// Delete the selected party
    pub fn delete_party(&mut self) {
        if let Some(idx) = self.selected_party {
            self.config.parties.remove(idx);
            self.selected_party = None;
            self.selected_device = None;
            self.selected_identifier = None;
        }
    }

    /// Add a new device to the selected party
    pub fn add_device(&mut self) {
        if let Some(party_idx) = self.selected_party {
            if let Some(party) = self.config.parties.get_mut(party_idx) {
                let new_device = Device {
                    name: "New Device".to_string(),
                    location: None,
                    identifiers: vec![],
                };
                party.devices.push(new_device);
                self.selected_device = Some(party.devices.len() - 1);
                self.selected_identifier = None;
            }
        }
    }

    /// Delete the selected device
    pub fn delete_device(&mut self) {
        if let Some(party_idx) = self.selected_party {
            if let Some(device_idx) = self.selected_device {
                if let Some(party) = self.config.parties.get_mut(party_idx) {
                    party.devices.remove(device_idx);
                    self.selected_device = None;
                    self.selected_identifier = None;
                }
            }
        }
    }

    /// Add a new identifier to the selected device
    pub fn add_identifier(&mut self) {
        if let Some(party_idx) = self.selected_party {
            if let Some(device_idx) = self.selected_device {
                if let Some(party) = self.config.parties.get_mut(party_idx) {
                    if let Some(device) = party.devices.get_mut(device_idx) {
                        let new_identifier = Identifier {
                            name: "New Identifier".to_string(),
                            id_type: IdentifierType::BleMac,
                            value: "".to_string(),
                        };
                        device.identifiers.push(new_identifier);
                        self.selected_identifier = Some(device.identifiers.len() - 1);
                    }
                }
            }
        }
    }

    /// Delete the selected identifier
    pub fn delete_identifier(&mut self) {
        if let Some(party_idx) = self.selected_party {
            if let Some(device_idx) = self.selected_device {
                if let Some(identifier_idx) = self.selected_identifier {
                    if let Some(party) = self.config.parties.get_mut(party_idx) {
                        if let Some(device) = party.devices.get_mut(device_idx) {
                            device.identifiers.remove(identifier_idx);
                            self.selected_identifier = None;
                        }
                    }
                }
            }
        }
    }

    /// Save the presence config to file
    pub fn save(&mut self) -> Result<()> {
        let config_dir = loader::resolve_config_dir();
        let presence_path = config_dir.join("presence.toml");

        let toml_string = toml::to_string_pretty(&self.config)
            .context("Failed to serialize presence config")?;

        fs::write(&presence_path, toml_string)
            .context(format!("Failed to write {}", presence_path.display()))?;

        self.save_message = Some("Presence config saved successfully".to_string());
        Ok(())
    }

    /// Validate an identifier value based on its type
    pub fn validate_identifier(id_type: &IdentifierType, value: &str) -> Result<()> {
        match id_type {
            IdentifierType::BleMac | IdentifierType::WifiMac => {
                // MAC address format: aa:bb:cc:dd:ee:ff
                if !regex::Regex::new(r"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$")?.is_match(value) {
                    anyhow::bail!("Invalid MAC address format. Expected format: aa:bb:cc:dd:ee:ff");
                }
            }
            IdentifierType::IpV4 => {
                // IPv4 address format
                if !regex::Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$")?.is_match(value) {
                    anyhow::bail!("Invalid IPv4 address format");
                }
                // Validate each octet
                for octet in value.split('.') {
                    let _num: u8 = octet.parse()
                        .map_err(|_| anyhow::anyhow!("Invalid IPv4 octet: {}", octet))?;
                }
            }
            IdentifierType::IpV6 => {
                // Basic IPv6 validation
                if !regex::Regex::new(r"^([0-9a-fA-F]{0,4}:){2,7}[0-9a-fA-F]{0,4}$")?.is_match(value) {
                    anyhow::bail!("Invalid IPv6 address format");
                }
            }
            IdentifierType::Hostname => {
                if value.is_empty() {
                    anyhow::bail!("Hostname cannot be empty");
                }
                if value.len() > 253 {
                    anyhow::bail!("Hostname too long (max 253 characters)");
                }
            }
            IdentifierType::CardId | IdentifierType::DoorSensor => {
                if value.is_empty() {
                    anyhow::bail!("Identifier value cannot be empty");
                }
            }
        }
        Ok(())
    }

    /// Get the selected party
    pub fn get_selected_party(&self) -> Option<&Party> {
        self.selected_party.and_then(|idx| self.config.parties.get(idx))
    }

    /// Get the selected device
    pub fn get_selected_device(&self) -> Option<&Device> {
        self.selected_party.and_then(|party_idx| {
            self.config.parties.get(party_idx).and_then(|party| {
                self.selected_device.and_then(|device_idx| party.devices.get(device_idx))
            })
        })
    }

    /// Get the selected identifier
    pub fn get_selected_identifier(&self) -> Option<&Identifier> {
        self.selected_party.and_then(|party_idx| {
            self.config.parties.get(party_idx).and_then(|party| {
                self.selected_device.and_then(|device_idx| {
                    party.devices.get(device_idx).and_then(|device| {
                        self.selected_identifier.and_then(|id_idx| device.identifiers.get(id_idx))
                    })
                })
            })
        })
    }

    /// Update the selected party with new values
    pub fn update_selected_party<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Party),
    {
        if let Some(party_idx) = self.selected_party {
            if let Some(party) = self.config.parties.get_mut(party_idx) {
                updater(party);
                return Ok(());
            }
        }
        anyhow::bail!("No party selected");
    }

    /// Update the selected device with new values
    pub fn update_selected_device<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Device),
    {
        if let Some(party_idx) = self.selected_party {
            if let Some(party) = self.config.parties.get_mut(party_idx) {
                if let Some(device_idx) = self.selected_device {
                    if let Some(device) = party.devices.get_mut(device_idx) {
                        updater(device);
                        return Ok(());
                    }
                }
            }
        }
        anyhow::bail!("No device selected");
    }

    /// Update the selected identifier with new values
    pub fn update_selected_identifier<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Identifier),
    {
        if let Some(party_idx) = self.selected_party {
            if let Some(party) = self.config.parties.get_mut(party_idx) {
                if let Some(device_idx) = self.selected_device {
                    if let Some(device) = party.devices.get_mut(device_idx) {
                        if let Some(id_idx) = self.selected_identifier {
                            if let Some(identifier) = device.identifiers.get_mut(id_idx) {
                                updater(identifier);
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        anyhow::bail!("No identifier selected");
    }
}

impl PartyEditor {
    /// Create a new party editor from a party
    pub fn from_party(party: &Party) -> Self {
        PartyEditor {
            name: party.name.clone(),
            location: party.location.clone().unwrap_or_default(),
            editing_field: PartyField::Name,
            dirty: false,
        }
    }

    /// Apply editor values to a party
    pub fn apply_to_party(&self, party: &mut Party) {
        party.name = self.name.clone();
        if self.location != Location::default() {
            party.location = Some(self.location.clone());
        } else {
            party.location = None;
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.editing_field = match self.editing_field {
                    PartyField::Name => PartyField::Name,
                    PartyField::Building => PartyField::Name,
                    PartyField::Floor => PartyField::Building,
                    PartyField::Room => PartyField::Floor,
                    PartyField::Zone => PartyField::Room,
                };
            }
            KeyCode::Down => {
                self.editing_field = match self.editing_field {
                    PartyField::Name => PartyField::Building,
                    PartyField::Building => PartyField::Floor,
                    PartyField::Floor => PartyField::Room,
                    PartyField::Room => PartyField::Zone,
                    PartyField::Zone => PartyField::Zone,
                };
            }
            KeyCode::Char(c) => {
                self.dirty = true;
                match self.editing_field {
                    PartyField::Name => self.name.push(c),
                    PartyField::Building => {
                        if let Some(ref mut b) = self.location.building {
                            b.push(c);
                        } else {
                            self.location.building = Some(c.to_string());
                        }
                    }
                    PartyField::Floor => {
                        if c.is_ascii_digit() {
                            if let Some(ref mut f) = self.location.floor {
                                *f = *f * 10 + c.to_digit(10).unwrap() as u32;
                            } else {
                                self.location.floor = Some(c.to_digit(10).unwrap() as u32);
                            }
                        }
                    }
                    PartyField::Room => {
                        if let Some(ref mut r) = self.location.room {
                            r.push(c);
                        } else {
                            self.location.room = Some(c.to_string());
                        }
                    }
                    PartyField::Zone => {
                        if let Some(ref mut z) = self.location.zone {
                            z.push(c);
                        } else {
                            self.location.zone = Some(c.to_string());
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                self.dirty = true;
                match self.editing_field {
                    PartyField::Name => { self.name.pop(); }
                    PartyField::Building => {
                        if let Some(ref mut b) = self.location.building {
                            b.pop();
                            if b.is_empty() {
                                self.location.building = None;
                            }
                        }
                    }
                    PartyField::Floor => {
                        if let Some(ref mut f) = self.location.floor {
                            *f /= 10;
                            if *f == 0 {
                                self.location.floor = None;
                            }
                        }
                    }
                    PartyField::Room => {
                        if let Some(ref mut r) = self.location.room {
                            r.pop();
                            if r.is_empty() {
                                self.location.room = None;
                            }
                        }
                    }
                    PartyField::Zone => {
                        if let Some(ref mut z) = self.location.zone {
                            z.pop();
                            if z.is_empty() {
                                self.location.zone = None;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl DeviceEditor {
    /// Create a new device editor from a device
    pub fn from_device(device: &Device) -> Self {
        DeviceEditor {
            name: device.name.clone(),
            location: device.location.clone().unwrap_or_default(),
            editing_field: DeviceField::Name,
            dirty: false,
        }
    }

    /// Apply editor values to a device
    pub fn apply_to_device(&self, device: &mut Device) {
        device.name = self.name.clone();
        if self.location != Location::default() {
            device.location = Some(self.location.clone());
        } else {
            device.location = None;
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.editing_field = match self.editing_field {
                    DeviceField::Name => DeviceField::Name,
                    DeviceField::Building => DeviceField::Name,
                    DeviceField::Floor => DeviceField::Building,
                    DeviceField::Room => DeviceField::Floor,
                    DeviceField::Zone => DeviceField::Room,
                };
            }
            KeyCode::Down => {
                self.editing_field = match self.editing_field {
                    DeviceField::Name => DeviceField::Building,
                    DeviceField::Building => DeviceField::Floor,
                    DeviceField::Floor => DeviceField::Room,
                    DeviceField::Room => DeviceField::Zone,
                    DeviceField::Zone => DeviceField::Zone,
                };
            }
            KeyCode::Char(c) => {
                self.dirty = true;
                match self.editing_field {
                    DeviceField::Name => self.name.push(c),
                    DeviceField::Building => {
                        if let Some(ref mut b) = self.location.building {
                            b.push(c);
                        } else {
                            self.location.building = Some(c.to_string());
                        }
                    }
                    DeviceField::Floor => {
                        if c.is_ascii_digit() {
                            if let Some(ref mut f) = self.location.floor {
                                *f = *f * 10 + c.to_digit(10).unwrap() as u32;
                            } else {
                                self.location.floor = Some(c.to_digit(10).unwrap() as u32);
                            }
                        }
                    }
                    DeviceField::Room => {
                        if let Some(ref mut r) = self.location.room {
                            r.push(c);
                        } else {
                            self.location.room = Some(c.to_string());
                        }
                    }
                    DeviceField::Zone => {
                        if let Some(ref mut z) = self.location.zone {
                            z.push(c);
                        } else {
                            self.location.zone = Some(c.to_string());
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                self.dirty = true;
                match self.editing_field {
                    DeviceField::Name => { self.name.pop(); }
                    DeviceField::Building => {
                        if let Some(ref mut b) = self.location.building {
                            b.pop();
                            if b.is_empty() {
                                self.location.building = None;
                            }
                        }
                    }
                    DeviceField::Floor => {
                        if let Some(ref mut f) = self.location.floor {
                            *f /= 10;
                            if *f == 0 {
                                self.location.floor = None;
                            }
                        }
                    }
                    DeviceField::Room => {
                        if let Some(ref mut r) = self.location.room {
                            r.pop();
                            if r.is_empty() {
                                self.location.room = None;
                            }
                        }
                    }
                    DeviceField::Zone => {
                        if let Some(ref mut z) = self.location.zone {
                            z.pop();
                            if z.is_empty() {
                                self.location.zone = None;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl IdentifierEditor {
    /// Create a new identifier editor from an identifier
    pub fn from_identifier(identifier: &Identifier) -> Self {
        IdentifierEditor {
            name: identifier.name.clone(),
            id_type: identifier.id_type.clone(),
            value: identifier.value.clone(),
            editing_field: IdentifierField::Name,
            dirty: false,
        }
    }

    /// Apply editor values to an identifier
    pub fn apply_to_identifier(&self, identifier: &mut Identifier) {
        identifier.name = self.name.clone();
        identifier.id_type = self.id_type.clone();
        identifier.value = self.value.clone();
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.editing_field = match self.editing_field {
                    IdentifierField::Name => IdentifierField::Name,
                    IdentifierField::Type => IdentifierField::Name,
                    IdentifierField::Value => IdentifierField::Type,
                };
            }
            KeyCode::Down => {
                self.editing_field = match self.editing_field {
                    IdentifierField::Name => IdentifierField::Type,
                    IdentifierField::Type => IdentifierField::Value,
                    IdentifierField::Value => IdentifierField::Value,
                };
            }
            KeyCode::Char(c) => {
                self.dirty = true;
                match self.editing_field {
                    IdentifierField::Name => self.name.push(c),
                    IdentifierField::Value => self.value.push(c),
                    IdentifierField::Type => {
                        // Cycle through identifier types
                        self.id_type = match self.id_type {
                            IdentifierType::BleMac => IdentifierType::WifiMac,
                            IdentifierType::WifiMac => IdentifierType::IpV4,
                            IdentifierType::IpV4 => IdentifierType::IpV6,
                            IdentifierType::IpV6 => IdentifierType::Hostname,
                            IdentifierType::Hostname => IdentifierType::CardId,
                            IdentifierType::CardId => IdentifierType::DoorSensor,
                            IdentifierType::DoorSensor => IdentifierType::BleMac,
                        };
                    }
                }
            }
            KeyCode::Backspace => {
                self.dirty = true;
                match self.editing_field {
                    IdentifierField::Name => { self.name.pop(); }
                    IdentifierField::Value => { self.value.pop(); }
                    IdentifierField::Type => {}
                }
            }
            _ => {}
        }
    }

    /// Validate the current identifier
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Identifier name cannot be empty");
        }
        PresenceManager::validate_identifier(&self.id_type, &self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_presence_manager_creation() {
        let _temp_dir = TempDir::new().unwrap();
        let manager = PresenceManager::new().unwrap();
        // Just check that it creates successfully
        assert!(manager.party_count() >= 0);
    }

    #[test]
    fn test_add_party() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        let initial_count = manager.party_count();
        manager.add_party();
        assert_eq!(manager.party_count(), initial_count + 1);
        assert_eq!(manager.selected_party, Some(initial_count));
    }

    #[test]
    fn test_delete_party() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        manager.add_party();
        let initial_count = manager.party_count();
        manager.delete_party();
        assert_eq!(manager.party_count(), initial_count - 1);
        assert_eq!(manager.selected_party, None);
    }

    #[test]
    fn test_add_device() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        manager.add_party();
        manager.add_device();
        assert_eq!(manager.device_count(), 1);
        assert_eq!(manager.selected_device, Some(0));
    }

    #[test]
    fn test_delete_device() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        manager.add_party();
        manager.add_device();
        manager.delete_device();
        assert_eq!(manager.device_count(), 0);
        assert_eq!(manager.selected_device, None);
    }

    #[test]
    fn test_add_identifier() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        manager.add_party();
        manager.add_device();
        manager.add_identifier();
        assert_eq!(manager.identifier_count(), 1);
        assert_eq!(manager.selected_identifier, Some(0));
    }

    #[test]
    fn test_delete_identifier() {
        let _temp_dir = TempDir::new().unwrap();
        let mut manager = PresenceManager::new().unwrap();
        manager.add_party();
        manager.add_device();
        manager.add_identifier();
        manager.delete_identifier();
        assert_eq!(manager.identifier_count(), 0);
        assert_eq!(manager.selected_identifier, None);
    }

    #[test]
    fn test_validate_mac_address() {
        assert!(PresenceManager::validate_identifier(&IdentifierType::BleMac, "aa:bb:cc:dd:ee:ff").is_ok());
        assert!(PresenceManager::validate_identifier(&IdentifierType::BleMac, "invalid").is_err());
    }

    #[test]
    fn test_validate_ipv4() {
        assert!(PresenceManager::validate_identifier(&IdentifierType::IpV4, "192.168.1.1").is_ok());
        assert!(PresenceManager::validate_identifier(&IdentifierType::IpV4, "invalid").is_err());
    }

    #[test]
    fn test_validate_hostname() {
        assert!(PresenceManager::validate_identifier(&IdentifierType::Hostname, "my-device").is_ok());
        assert!(PresenceManager::validate_identifier(&IdentifierType::Hostname, "").is_err());
    }

    #[test]
    fn test_party_editor_from_party() {
        let party = Party {
            name: "Test Party".to_string(),
            location: Some(Location {
                building: Some("Home".to_string()),
                floor: Some(1),
                room: Some("Living Room".to_string()),
                zone: None,
            }),
            devices: vec![],
        };
        let editor = PartyEditor::from_party(&party);
        assert_eq!(editor.name, "Test Party");
        assert_eq!(editor.location.building, Some("Home".to_string()));
    }

    #[test]
    fn test_device_editor_from_device() {
        let device = Device {
            name: "Test Device".to_string(),
            location: Some(Location {
                building: Some("Office".to_string()),
                floor: Some(2),
                room: Some("Desk".to_string()),
                zone: None,
            }),
            identifiers: vec![],
        };
        let editor = DeviceEditor::from_device(&device);
        assert_eq!(editor.name, "Test Device");
        assert_eq!(editor.location.building, Some("Office".to_string()));
    }

    #[test]
    fn test_identifier_editor_from_identifier() {
        let identifier = Identifier {
            name: "BLE MAC".to_string(),
            id_type: IdentifierType::BleMac,
            value: "aa:bb:cc:dd:ee:ff".to_string(),
        };
        let editor = IdentifierEditor::from_identifier(&identifier);
        assert_eq!(editor.name, "BLE MAC");
        assert_eq!(editor.id_type, IdentifierType::BleMac);
        assert_eq!(editor.value, "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn test_identifier_editor_validation() {
        let mut editor = IdentifierEditor {
            name: "Test".to_string(),
            id_type: IdentifierType::BleMac,
            value: "aa:bb:cc:dd:ee:ff".to_string(),
            editing_field: IdentifierField::Name,
            dirty: false,
        };
        assert!(editor.validate().is_ok());

        editor.value = "invalid".to_string();
        assert!(editor.validate().is_err());
    }
}
