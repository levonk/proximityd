use rusqlite::{params, Connection};
use std::path::Path;
use std::time::Duration;
use tracing::{info, warn};

use crate::location::gps::create_gps_source;
use crate::location::ip_geo::fetch_public_ip;
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

    /// Insert a raw signal row with GPS and IP geolocation (async version).
    /// GPS and IP are fetched with timeouts before acquiring the database lock,
    /// ensuring they never block scanning or other database operations.
    pub async fn log_with_location(
        &self,
        raw: &RawSignal,
        gps_timeout: Duration,
        ip_timeout: Duration,
    ) -> Result<(), rusqlite::Error> {
        // Fetch GPS and IP first, before any database operations
        // This ensures network timeouts don't block the database lock
        let (gps_result, ip_result) = tokio::join!(
            async {
                match create_gps_source().await {
                    Ok(source) => source.get_coordinates(gps_timeout).await,
                    Err(e) => {
                        warn!(error = %e, "Failed to create GPS source");
                        Ok(None)
                    }
                }
            },
            fetch_public_ip(ip_timeout)
        );
        
        let (gps_lat, gps_lon) = match gps_result {
            Ok(Some(coords)) => (Some(coords.lat), Some(coords.lon)),
            _ => (None, None),
        };
        
        let public_ip = ip_result.ok().flatten();
        
        // Now acquire database connection and write
        let ts = chrono::Utc::now().to_rfc3339();
        
        match self.conn.execute(
            r#"INSERT INTO signal_log
               (ts, scanner, id_type, id_value, rssi, gps_lat, gps_lon, public_ip, metadata)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#,
            params![
                ts,
                &raw.scanner,
                &raw.id_type,
                &raw.id_value,
                raw.rssi,
                gps_lat,
                gps_lon,
                public_ip.as_deref(),
                raw.metadata.as_ref(),
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!(error = %e, "failed to insert signal_log row with location; scanning continues");
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
