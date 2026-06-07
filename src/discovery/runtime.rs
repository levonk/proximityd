use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Runtime suggestion loader and resolver.
///
/// Loads `suggestions.toml` and provides identifier-to-party resolution
/// for high-confidence suggestions above the configured threshold.
#[derive(Debug, Clone)]
pub struct SuggestionRuntime {
    /// Mapping from identifier value to (party_name, device_name, confidence)
    identifier_map: HashMap<String, (String, Option<String>, f64)>,
    /// Whether suggestions are enabled
    enabled: bool,
    /// Confidence threshold for auto-promotion
    threshold: f64,
}

/// Suggestion loaded from suggestions.toml.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuggestionToml {
    pub confidence: f64,
    pub rationale: String,
    pub party: PartySuggestion,
}

/// Party suggestion from suggestions.toml.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartySuggestion {
    pub name: String,
    pub devices: Vec<DeviceSuggestion>,
}

/// Device suggestion from suggestions.toml.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceSuggestion {
    pub name: String,
    pub identifiers: Vec<IdentifierSuggestionToml>,
}

/// Identifier suggestion from suggestions.toml.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdentifierSuggestionToml {
    #[serde(rename = "type")]
    pub id_type: String,
    pub value: String,
}

/// Top-level suggestions.toml structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
struct SuggestionsFile {
    suggestions: Vec<SuggestionToml>,
}

impl SuggestionRuntime {
    /// Load suggestions from the given path.
    ///
    /// # Arguments
    /// * `suggestions_path` - Path to suggestions.toml file
    /// * `enabled` - Whether suggestion runtime is enabled
    /// * `threshold` - Confidence threshold for auto-promotion (0.0 to 1.0)
    ///
    /// # Returns
    /// Ok(SuggestionRuntime) if file exists and parses, or Ok(None) if file doesn't exist
    pub fn load(
        suggestions_path: &Path,
        enabled: bool,
        threshold: f64,
    ) -> Result<Option<Self>> {
        if !enabled {
            return Ok(None);
        }

        if !suggestions_path.exists() {
            // Not an error - suggestions file may not exist yet
            return Ok(None);
        }

        let content = fs::read_to_string(suggestions_path)
            .with_context(|| format!("Failed to read suggestions file: {:?}", suggestions_path))?;

        let suggestions_file: SuggestionsFile = toml::from_str(&content)
            .with_context(|| "Failed to parse suggestions.toml")?;

        let mut identifier_map = HashMap::new();

        for suggestion in &suggestions_file.suggestions {
            // Only load suggestions above threshold
            if suggestion.confidence < threshold {
                continue;
            }

            let party_name = suggestion.party.name.clone();

            for device in &suggestion.party.devices {
                let device_name = Some(device.name.clone());
                for identifier in &device.identifiers {
                    identifier_map.insert(
                        identifier.value.clone(),
                        (party_name.clone(), device_name.clone(), suggestion.confidence),
                    );
                }
            }
        }

        Ok(Some(Self {
            identifier_map,
            enabled,
            threshold,
        }))
    }

    /// Resolve an identifier to its party and device using suggestions.
    ///
    /// # Arguments
    /// * `identifier_value` - The identifier value to resolve (e.g., MAC address, IP)
    ///
    /// # Returns
    /// Some((party_name, device_name, confidence)) if found above threshold, None otherwise
    pub fn resolve(&self, identifier_value: &str) -> Option<&(String, Option<String>, f64)> {
        self.identifier_map.get(identifier_value)
    }

    /// Check if suggestion runtime is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the confidence threshold.
    pub fn threshold(&self) -> f64 {
        self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_suggestions_disabled() {
        let temp_file = NamedTempFile::new().unwrap();
        let runtime = SuggestionRuntime::load(temp_file.path(), false, 0.95).unwrap();
        assert!(runtime.is_none());
    }

    #[test]
    fn test_load_suggestions_file_not_found() {
        let runtime = SuggestionRuntime::load(Path::new("/nonexistent/suggestions.toml"), true, 0.95)
            .unwrap();
        assert!(runtime.is_none());
    }

    #[test]
    fn test_load_and_resolve_suggestions() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
[[suggestions]]
confidence = 0.97
rationale = "Co-occurrence within 5-minute window"

  [suggestions.party]
  name = "Test Party"

    [[suggestions.party.devices]]
    name = "Device A"

      [[suggestions.party.devices.identifiers]]
      type = "ble_mac"
      value = "aa:bb:cc:dd:ee:ff"

      [[suggestions.party.devices.identifiers]]
      type = "wifi_mac"
      value = "11:22:33:44:55:66"

[[suggestions]]
confidence = 0.80
rationale = "Low confidence suggestion"

  [suggestions.party]
  name = "Low Confidence Party"

    [[suggestions.party.devices]]
    name = "Device B"

      [[suggestions.party.devices.identifiers]]
      type = "ip_v4"
      value = "192.168.1.50"
"#;
        temp_file.write_all(toml_content.as_bytes()).unwrap();

        let runtime = SuggestionRuntime::load(temp_file.path(), true, 0.95)
            .unwrap()
            .expect("Runtime should be loaded");

        assert!(runtime.is_enabled());
        assert_eq!(runtime.threshold(), 0.95);

        // High-confidence identifier should resolve
        let result = runtime.resolve("aa:bb:cc:dd:ee:ff");
        assert!(result.is_some());
        let (party_name, device_name, confidence) = result.unwrap();
        assert_eq!(party_name, "Test Party");
        assert_eq!(device_name, &Some("Device A".to_string()));
        assert_eq!(*confidence, 0.97);

        // Another high-confidence identifier should resolve
        let result = runtime.resolve("11:22:33:44:55:66");
        assert!(result.is_some());

        // Low-confidence identifier should NOT resolve (below threshold)
        let result = runtime.resolve("192.168.1.50");
        assert!(result.is_none());

        // Unknown identifier should not resolve
        let result = runtime.resolve("unknown:identifier");
        assert!(result.is_none());
    }

    #[test]
    fn test_load_with_lower_threshold() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
[[suggestions]]
confidence = 0.80
rationale = "Lower confidence"

  [suggestions.party]
  name = "Test Party"

    [[suggestions.party.devices]]
    name = "Device A"

      [[suggestions.party.devices.identifiers]]
      type = "ip_v4"
      value = "192.168.1.50"
"#;
        temp_file.write_all(toml_content.as_bytes()).unwrap();

        let runtime = SuggestionRuntime::load(temp_file.path(), true, 0.75)
            .unwrap()
            .expect("Runtime should be loaded");

        // With lower threshold, this should resolve
        let result = runtime.resolve("192.168.1.50");
        assert!(result.is_some());
    }
}