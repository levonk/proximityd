use serde::{Deserialize, Serialize};

/// A suggestion for grouping identifiers together based on correlation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Rationale for this suggestion
    pub rationale: String,
    /// Proposed party name (if applicable)
    pub party_name: Option<String>,
    /// Proposed device name (if applicable)
    pub device_name: Option<String>,
    /// Identifiers that should be grouped
    pub identifiers: Vec<IdentifierSuggestion>,
}

/// A single identifier in a suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierSuggestion {
    /// Identifier type (e.g., "ble_mac", "wifi_mac")
    pub id_type: String,
    /// Identifier value
    pub id_value: String,
    /// Notes about this identifier
    pub notes: Option<String>,
}

/// Generate suggestions from correlation data.
///
/// # Arguments
/// * `correlations` - Map of identifier pairs to Jaccard similarity scores
/// * `min_confidence` - Minimum confidence threshold (0.0 to 1.0)
///
/// # Returns
/// A list of suggestions that meet the confidence threshold
pub fn generate_suggestions(
    correlations: std::collections::HashMap<(String, String), f64>,
    min_confidence: f64,
) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Group correlated identifiers into clusters
    let clusters = find_clusters(correlations.clone(), min_confidence);

    for cluster in clusters {
        if cluster.len() < 2 {
            continue; // Skip single-identifier clusters
        }

        let confidence = compute_cluster_confidence(&cluster, &correlations);
        let rationale = format!(
            "Identifiers co-occurred in {} time windows with average similarity {:.2}",
            cluster.len(),
            confidence
        );

        // Parse identifier types from values (simple heuristic)
        let identifiers: Vec<IdentifierSuggestion> = cluster
            .iter()
            .map(|id| IdentifierSuggestion {
                id_type: infer_id_type(id),
                id_value: id.clone(),
                notes: None,
            })
            .collect();

        suggestions.push(Suggestion {
            confidence,
            rationale,
            party_name: None, // User will need to assign
            device_name: None, // User will need to assign
            identifiers,
        });
    }

    // Sort by confidence descending
    suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    suggestions
}

/// Find clusters of correlated identifiers using connected components.
fn find_clusters(
    correlations: std::collections::HashMap<(String, String), f64>,
    min_confidence: f64,
) -> Vec<Vec<String>> {
    let mut graph: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    // Build graph from correlations above threshold
    for ((id1, id2), similarity) in &correlations {
        if similarity >= &min_confidence {
            graph.entry(id1.clone()).or_default().push(id2.clone());
            graph.entry(id2.clone()).or_default().push(id1.clone());
        }
    }

    // Find connected components
    let mut clusters = Vec::new();
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

    for node in graph.keys() {
        if visited.contains(node) {
            continue;
        }

        let mut cluster = Vec::new();
        let mut stack = vec![node.clone()];

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());
            cluster.push(current.clone());

            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        if !cluster.is_empty() {
            clusters.push(cluster);
        }
    }

    clusters
}

/// Compute average confidence for a cluster.
fn compute_cluster_confidence(
    cluster: &[String],
    correlations: &std::collections::HashMap<(String, String), f64>,
) -> f64 {
    if cluster.len() < 2 {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut count = 0;

    for i in 0..cluster.len() {
        for j in (i + 1)..cluster.len() {
            let id1 = &cluster[i];
            let id2 = &cluster[j];

            let key = if id1 < id2 {
                (id1.clone(), id2.clone())
            } else {
                (id2.clone(), id1.clone())
            };

            if let Some(&similarity) = correlations.get(&key) {
                sum += similarity;
                count += 1;
            }
        }
    }

    if count > 0 {
        sum / count as f64
    } else {
        0.0
    }
}

/// Infer identifier type from value (simple heuristic).
pub fn infer_id_type(value: &str) -> String {
    // Check IPv6 first (contains colons but can be variable length)
    if value.parse::<std::net::Ipv6Addr>().is_ok() {
        "ip_v6".to_string()
    } else if value.contains(':') && value.len() == 17 {
        "ble_mac".to_string()
    } else if value.contains(':') && value.len() <= 17 {
        "wifi_mac".to_string()
    } else if value.parse::<std::net::Ipv4Addr>().is_ok() {
        "ip_v4".to_string()
    } else if value.contains('.') {
        "hostname".to_string()
    } else {
        "unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_suggestions_empty() {
        let correlations = std::collections::HashMap::new();
        let suggestions = generate_suggestions(correlations, 0.5);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_infer_id_type() {
        assert_eq!(infer_id_type("AA:BB:CC:DD:EE:FF"), "ble_mac");
        assert_eq!(infer_id_type("AA:BB:CC:DD:EE"), "wifi_mac");
        assert_eq!(infer_id_type("192.168.1.1"), "ip_v4");
        assert_eq!(infer_id_type("::1"), "ip_v6");
        assert_eq!(infer_id_type("example.local"), "hostname");
        assert_eq!(infer_id_type("unknown"), "unknown");
    }
}