use rusqlite::{params, Connection};
use std::path::Path;
use tracing::{info, warn};

use crate::signals::schema;
use crate::signals::types::RawSignal;

/// Inserts raw signal sightings into an SQLite `signal_log` table.
pub struct SignalLogger {
    conn: Connection,
}

impl SignalLogger {
    /// Open (or create) the signals database at `db_path`, run schema setup,
    /// and optionally prune rows older than `max_log_age_days`.
    pub fn open(
        db_path: &Path,
        max_log_age_days: Option<u32>,
    ) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        schema::setup(&conn)?;
        let this = Self { conn };
        if let Some(days) = max_log_age_days {
            if let Err(e) = this.prune(days) {
                warn!(error = %e, "signal_log auto-prune failed; continuing without prune");
            }
        }
        Ok(this)
    }

    /// Insert a raw signal row. Party/device/location fields are left NULL
    /// to be resolved later by the detection pipeline.
    pub fn log(&self, raw: &RawSignal) -> Result<(), rusqlite::Error> {
        let ts = chrono::Utc::now().to_rfc3339();
        match self.conn.execute(
            r#"INSERT INTO signal_log
               (ts, scanner, id_type, id_value, rssi, metadata)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
            params![
                ts,
                &raw.scanner,
                &raw.id_type,
                &raw.id_value,
                raw.rssi,
                raw.metadata.as_ref(),
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!(error = %e, "failed to insert signal_log row; scanning continues");
                Err(e)
            }
        }
    }

    /// Delete rows older than `max_age_days`. Returns the number of rows removed.
    pub fn prune(&self, max_age_days: u32) -> Result<usize, rusqlite::Error> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(i64::from(max_age_days));
        let count = self.conn.execute(
            "DELETE FROM signal_log WHERE ts < ?1",
            params![cutoff.to_rfc3339()],
        )?;
        info!(pruned = count, max_age_days, "signal_log auto-prune complete");
        Ok(count)
    }
}
