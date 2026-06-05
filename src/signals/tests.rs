use std::path::PathBuf;

use crate::signals::logger::SignalLogger;
use crate::signals::schema;
use crate::signals::types::RawSignal;
use rusqlite::Connection;
use tempfile::NamedTempFile;

fn temp_db_path() -> PathBuf {
    NamedTempFile::new().unwrap().into_temp_path().to_path_buf()
}

#[test]
fn schema_setup_creates_tables() {
    let path = temp_db_path();
    let conn = Connection::open(&path).unwrap();
    schema::setup(&conn).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='signal_log'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn logger_inserts_row() {
    let path = temp_db_path();
    let logger = SignalLogger::open(&path, None).unwrap();

    let raw = RawSignal {
        scanner: "ble".into(),
        id_type: "mac".into(),
        id_value: "AA:BB:CC:DD:EE:FF".into(),
        rssi: Some(-72),
        metadata: Some(r#"{"tx_power":4}"#.into()),
    };
    logger.log(&raw).unwrap();

    let conn = Connection::open(&path).unwrap();
    let (scanner, id_type, id_value, rssi): (String, String, String, Option<i32>) = conn
        .query_row(
            "SELECT scanner, id_type, id_value, rssi FROM signal_log LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .unwrap();
    assert_eq!(scanner, "ble");
    assert_eq!(id_type, "mac");
    assert_eq!(id_value, "AA:BB:CC:DD:EE:FF");
    assert_eq!(rssi, Some(-72));
}

#[test]
fn prune_deletes_old_rows() {
    let path = temp_db_path();
    let conn = Connection::open(&path).unwrap();
    schema::setup(&conn).unwrap();

    // Insert an old row manually
    conn.execute(
        "INSERT INTO signal_log (ts, scanner, id_type, id_value)
         VALUES ('2020-01-01T00:00:00Z', 'ble', 'mac', '00:00:00:00:00:00')",
        [],
    )
    .unwrap();

    let logger = SignalLogger::open(&path, None).unwrap();
    let pruned = logger.prune(1).unwrap();
    assert_eq!(pruned, 1);

    let count: i64 = conn
        .query_row(
            "SELECT count(*) FROM signal_log",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn prune_leaves_recent_rows() {
    let path = temp_db_path();
    let logger = SignalLogger::open(&path, None).unwrap();

    let raw = RawSignal {
        scanner: "wifi_arp".into(),
        id_type: "ip".into(),
        id_value: "192.168.1.42".into(),
        rssi: None,
        metadata: None,
    };
    logger.log(&raw).unwrap();

    let pruned = logger.prune(7).unwrap();
    assert_eq!(pruned, 0);

    let conn = Connection::open(&path).unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT count(*) FROM signal_log",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
}
