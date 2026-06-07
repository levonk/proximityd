use crate::config::presence::{Device, Location, Party};

pub mod gps;
pub mod ip_geo;

/// Location resolution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLocation {
    /// The resolved location.
    pub location: Location,
    /// The source of the resolution (device, party, scanner, or none).
    pub source: LocationSource,
}

/// Source of a resolved location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationSource {
    /// Location came from device-level override.
    Device,
    /// Location came from party-level setting.
    Party,
    /// Location came from scanner-node default.
    Scanner,
    /// No location available.
    None,
}

/// Resolve location for a device using the priority hierarchy:
/// device-level > party-level > scanner-node default > none.
pub fn resolve_location(
    device: &Device,
    party: &Party,
    scanner_location: Option<&Location>,
) -> ResolvedLocation {
    // Priority 1: Device-level override
    if let Some(ref device_location) = device.location {
        return ResolvedLocation {
            location: device_location.clone(),
            source: LocationSource::Device,
        };
    }

    // Priority 2: Party-level location
    if let Some(ref party_location) = party.location {
        return ResolvedLocation {
            location: party_location.clone(),
            source: LocationSource::Party,
        };
    }

    // Priority 3: Scanner-node default
    if let Some(scanner_loc) = scanner_location {
        return ResolvedLocation {
            location: scanner_loc.clone(),
            source: LocationSource::Scanner,
        };
    }

    // Priority 4: No location available
    ResolvedLocation {
        location: Location::default(),
        source: LocationSource::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_location_override_takes_priority() {
        let device = Device {
            name: "Test Device".to_string(),
            location: Some(Location {
                building: Some("Device Building".to_string()),
                floor: Some(3),
                room: Some("Device Room".to_string()),
                zone: Some("Device Zone".to_string()),
            }),
            identifiers: vec![],
        };

        let party = Party {
            name: "Test Party".to_string(),
            location: Some(Location {
                building: Some("Party Building".to_string()),
                floor: Some(1),
                room: Some("Party Room".to_string()),
                zone: Some("Party Zone".to_string()),
            }),
            devices: vec![],
        };

        let scanner_location = Some(Location {
            building: Some("Scanner Building".to_string()),
            floor: Some(2),
            room: Some("Scanner Room".to_string()),
            zone: Some("Scanner Zone".to_string()),
        });

        let resolved = resolve_location(&device, &party, scanner_location.as_ref());
        assert_eq!(resolved.source, LocationSource::Device);
        assert_eq!(
            resolved.location.building,
            Some("Device Building".to_string())
        );
        assert_eq!(resolved.location.floor, Some(3));
    }

    #[test]
    fn test_party_location_fallback_when_device_missing() {
        let device = Device {
            name: "Test Device".to_string(),
            location: None,
            identifiers: vec![],
        };

        let party = Party {
            name: "Test Party".to_string(),
            location: Some(Location {
                building: Some("Party Building".to_string()),
                floor: Some(1),
                room: Some("Party Room".to_string()),
                zone: Some("Party Zone".to_string()),
            }),
            devices: vec![],
        };

        let scanner_location = Some(Location {
            building: Some("Scanner Building".to_string()),
            floor: Some(2),
            room: Some("Scanner Room".to_string()),
            zone: Some("Scanner Zone".to_string()),
        });

        let resolved = resolve_location(&device, &party, scanner_location.as_ref());
        assert_eq!(resolved.source, LocationSource::Party);
        assert_eq!(
            resolved.location.building,
            Some("Party Building".to_string())
        );
        assert_eq!(resolved.location.floor, Some(1));
    }

    #[test]
    fn test_scanner_location_fallback_when_party_missing() {
        let device = Device {
            name: "Test Device".to_string(),
            location: None,
            identifiers: vec![],
        };

        let party = Party {
            name: "Test Party".to_string(),
            location: None,
            devices: vec![],
        };

        let scanner_location = Some(Location {
            building: Some("Scanner Building".to_string()),
            floor: Some(2),
            room: Some("Scanner Room".to_string()),
            zone: Some("Scanner Zone".to_string()),
        });

        let resolved = resolve_location(&device, &party, scanner_location.as_ref());
        assert_eq!(resolved.source, LocationSource::Scanner);
        assert_eq!(
            resolved.location.building,
            Some("Scanner Building".to_string())
        );
        assert_eq!(resolved.location.floor, Some(2));
    }

    #[test]
    fn test_no_location_when_all_missing() {
        let device = Device {
            name: "Test Device".to_string(),
            location: None,
            identifiers: vec![],
        };

        let party = Party {
            name: "Test Party".to_string(),
            location: None,
            devices: vec![],
        };

        let resolved = resolve_location(&device, &party, None);
        assert_eq!(resolved.source, LocationSource::None);
        assert_eq!(resolved.location.building, None);
        assert_eq!(resolved.location.floor, None);
        assert_eq!(resolved.location.room, None);
        assert_eq!(resolved.location.zone, None);
    }

    #[test]
    fn test_partial_location_fields() {
        let device = Device {
            name: "Test Device".to_string(),
            location: Some(Location {
                building: Some("Building".to_string()),
                floor: None,
                room: None,
                zone: None,
            }),
            identifiers: vec![],
        };

        let party = Party {
            name: "Test Party".to_string(),
            location: None,
            devices: vec![],
        };

        let resolved = resolve_location(&device, &party, None);
        assert_eq!(resolved.source, LocationSource::Device);
        assert_eq!(resolved.location.building, Some("Building".to_string()));
        assert_eq!(resolved.location.floor, None);
    }
}