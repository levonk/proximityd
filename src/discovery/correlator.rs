use chrono::DurationRound;
use rusqlite::{params, Connection};
use std::collections::{HashMap, HashSet};

/// Correlator that computes Jaccard similarity between identifiers based on co-occurrence.
pub struct Correlator {
    hours: u32,
}

impl Correlator {
    /// Create a new correlator with a time window.
    pub fn new(hours: u32) -> Self {
        Self { hours }
    }

    /// Compute identifier correlations from the signal log.
    ///
    /// Returns a map of identifier pairs to their Jaccard similarity scores.
    pub fn compute(&self, conn: &Connection) -> Result<HashMap<(String, String), f64>, rusqlite::Error> {
        let cutoff = chrono::Utc::now() - chrono::Duration::hours(i64::from(self.hours));
        let cutoff_ts = cutoff.to_rfc3339();

        // Query all signals within the time window
        let mut stmt = conn.prepare(
            r#"
            SELECT id_value, ts
            FROM signal_log
            WHERE ts >= ?1
            ORDER BY ts
            "#,
        )?;

        let rows = stmt.query_map(params![cutoff_ts], |row| {
            Ok((
                row.get::<_, String>(0)?, // id_value
                row.get::<_, String>(1)?, // ts
            ))
        })?;

        // Group signals by time windows (default 5 minutes)
        let window_duration = chrono::Duration::minutes(5);
        let mut time_windows: HashMap<String, HashSet<String>> = HashMap::new();

        for row in rows {
            let (id_value, ts) = row?;
            let timestamp = chrono::DateTime::parse_from_rfc3339(&ts)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                .with_timezone(&chrono::Utc);

            // Round timestamp to 5-minute window
            let window_key = format!(
                "{}",
                timestamp
                    .duration_round(window_duration)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?
                    .to_rfc3339()
            );

            time_windows
                .entry(window_key)
                .or_insert_with(HashSet::new)
                .insert(id_value);
        }

        // Compute Jaccard similarity for all identifier pairs
        let mut correlations: HashMap<(String, String), f64> = HashMap::new();
        let identifiers: Vec<&String> = time_windows
            .values()
            .flat_map(|set| set.iter())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        for i in 0..identifiers.len() {
            for j in (i + 1)..identifiers.len() {
                let id1 = identifiers[i];
                let id2 = identifiers[j];

                let (intersection, union) = self.compute_jaccard(&time_windows, id1, id2);
                if union > 0 {
                    let similarity = intersection as f64 / union as f64;
                    correlations.insert((id1.clone(), id2.clone()), similarity);
                }
            }
        }

        Ok(correlations)
    }

    /// Compute intersection and union sizes for two identifiers across time windows.
    fn compute_jaccard(
        &self,
        time_windows: &HashMap<String, HashSet<String>>,
        id1: &str,
        id2: &str,
    ) -> (usize, usize) {
        let mut intersection = 0;
        let mut union = 0;

        for window in time_windows.values() {
            let has_id1 = window.contains(id1);
            let has_id2 = window.contains(id2);

            if has_id1 && has_id2 {
                intersection += 1;
                union += 1;
            } else if has_id1 || has_id2 {
                union += 1;
            }
        }

        (intersection, union)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlator_creation() {
        let correlator = Correlator::new(5);
        assert_eq!(correlator.hours, 5);
    }
}