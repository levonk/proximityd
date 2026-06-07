/// Discovery engine for correlating identifiers and suggesting party/device groupings.
pub mod correlator;
pub mod report;
pub mod runtime;
pub mod tests;

use rusqlite::Connection;
use std::path::Path;

use crate::discovery::correlator::Correlator;
use crate::discovery::report::Suggestion;

/// Discovery engine that computes identifier correlations from signal log data.
pub struct DiscoveryEngine {
    conn: Connection,
}

impl DiscoveryEngine {
    /// Open the signals database at `db_path` for correlation analysis.
    pub fn open(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    /// Discover identifier correlations within the time window.
    ///
    /// # Arguments
    /// * `hours` - Number of hours to look back from now for signal data
    /// * `min_confidence` - Minimum confidence score (0.0 to 1.0) for suggestions
    ///
    /// # Returns
    /// A list of suggestions with confidence scores and rationale
    pub fn discover(
        &self,
        hours: u32,
        min_confidence: f64,
    ) -> Result<Vec<Suggestion>, rusqlite::Error> {
        let correlator = Correlator::new(hours);
        let correlations = correlator.compute(&self.conn)?;
        let suggestions = report::generate_suggestions(correlations, min_confidence);
        Ok(suggestions)
    }
}