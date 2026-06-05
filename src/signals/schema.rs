use rusqlite::Connection;

pub const CREATE_SIGNAL_LOG_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS signal_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts TEXT NOT NULL,
    scanner TEXT NOT NULL,
    id_type TEXT NOT NULL,
    id_value TEXT NOT NULL,
    rssi INTEGER,
    party_name TEXT,
    device_name TEXT,
    location_building TEXT,
    location_floor TEXT,
    location_room TEXT,
    location_zone TEXT,
    gps_lat REAL,
    gps_lon REAL,
    public_ip TEXT,
    metadata TEXT
);
"#;

pub const CREATE_SIGNAL_LOG_INDEX_SQL: &str = r#"
CREATE INDEX IF NOT EXISTS idx_signal_log_ts ON signal_log(ts);
CREATE INDEX IF NOT EXISTS idx_signal_log_scanner ON signal_log(scanner);
CREATE INDEX IF NOT EXISTS idx_signal_log_id_value ON signal_log(id_value);
"#;

/// Initialize the signal_log schema and indexes.
pub fn setup(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(CREATE_SIGNAL_LOG_SQL, [])?;
    conn.execute_batch(CREATE_SIGNAL_LOG_INDEX_SQL)?;
    Ok(())
}
