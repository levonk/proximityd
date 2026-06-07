/// Unit tests for the discovery module.
///
/// Tests for the correlation engine and suggestion generation.

#[cfg(test)]
mod tests {
    use super::super::{correlator::Correlator, report::{generate_suggestions, infer_id_type}};
    use rusqlite::Connection;
    use tempfile::NamedTempFile;
    use std::collections::HashMap;

    /// Create a temporary signal log with synthetic data for testing.
    fn create_test_signal_log() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // Create the signal_log table
        conn.execute(
            r#"
            CREATE TABLE signal_log (
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
            "#,
            [],
        )
        .unwrap();

        // Insert synthetic signal data
        // Simulate two devices that co-occur frequently (should have high correlation)
        let base_time = chrono::Utc::now() - chrono::Duration::hours(1);

        for i in 0..10 {
            let ts = base_time + chrono::Duration::minutes(i * 5);
            let ts_str = ts.to_rfc3339();

            // Device A appears at every time window
            conn.execute(
                r#"INSERT INTO signal_log (ts, scanner, id_type, id_value) VALUES (?1, ?2, ?3, ?4)"#,
                [ts_str.clone(), "ble".to_string(), "ble_mac".to_string(), "AA:BB:CC:DD:EE:FF".to_string()],
            )
            .unwrap();

            // Device B appears in the same time windows (high correlation)
            conn.execute(
                r#"INSERT INTO signal_log (ts, scanner, id_type, id_value) VALUES (?1, ?2, ?3, ?4)"#,
                [ts_str.clone(), "ble".to_string(), "ble_mac".to_string(), "11:22:33:44:55:66".to_string()],
            )
            .unwrap();

            // Device C appears in different time windows (low correlation)
            if i % 3 == 0 {
                let ts_c = ts + chrono::Duration::minutes(2);
                conn.execute(
                    r#"INSERT INTO signal_log (ts, scanner, id_type, id_value) VALUES (?1, ?2, ?3, ?4)"#,
                    [ts_c.to_rfc3339(), "wifi".to_string(), "wifi_mac".to_string(), "AA:BB:CC:DD:EE:00".to_string()],
                )
                .unwrap();
            }
        }

        temp_file
    }

    #[test]
    fn test_correlator_compute() {
        let temp_file = create_test_signal_log();
        let conn = Connection::open(temp_file.path()).unwrap();
        let correlator = Correlator::new(24); // 24 hours

        let correlations = correlator.compute(&conn).unwrap();

        // Should find correlations between the devices
        assert!(!correlations.is_empty(), "Should find correlations");

        // The pair (AA:BB:CC:DD:EE:FF, 11:22:33:44:55:66) should have high correlation
        let key1 = ("AA:BB:CC:DD:EE:FF".to_string(), "11:22:33:44:55:66".to_string());
        let key2 = ("11:22:33:44:55:66".to_string(), "AA:BB:CC:DD:EE:FF".to_string());

        let similarity = correlations.get(&key1).or_else(|| correlations.get(&key2));
        assert!(
            similarity.is_some(),
            "Should find correlation between co-occurring devices"
        );

        if let Some(&sim) = similarity {
            assert!(
                sim > 0.5,
                "Co-occurring devices should have similarity > 0.5, got {}",
                sim
            );
        }
    }

    #[test]
    fn test_generate_suggestions() {
        let mut correlations = HashMap::new();

        // Add some high-confidence correlations
        correlations.insert(
            ("AA:BB:CC:DD:EE:FF".to_string(), "11:22:33:44:55:66".to_string()),
            0.9,
        );
        correlations.insert(
            ("11:22:33:44:55:66".to_string(), "AA:BB:CC:DD:EE:FF".to_string()),
            0.9,
        );

        // Add a low-confidence correlation
        correlations.insert(
            ("AA:BB:CC:DD:EE:FF".to_string(), "AA:BB:CC:DD:EE:00".to_string()),
            0.2,
        );

        let suggestions = generate_suggestions(correlations, 0.5);

        // Should only include high-confidence suggestions
        assert_eq!(suggestions.len(), 1, "Should have one suggestion above threshold");

        let suggestion = &suggestions[0];
        assert!(
            suggestion.confidence >= 0.5,
            "Suggestion confidence should be >= threshold"
        );
        assert!(
            suggestion.identifiers.len() >= 2,
            "Suggestion should include at least 2 identifiers"
        );
    }

    #[test]
    fn test_generate_suggestions_empty() {
        let correlations = HashMap::new();
        let suggestions = generate_suggestions(correlations, 0.5);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_infer_id_type_ble_mac() {
        assert_eq!(infer_id_type("AA:BB:CC:DD:EE:FF"), "ble_mac");
    }

    #[test]
    fn test_infer_id_type_wifi_mac() {
        assert_eq!(infer_id_type("AA:BB:CC:DD:EE"), "wifi_mac");
    }

    #[test]
    fn test_infer_id_type_ipv4() {
        assert_eq!(infer_id_type("192.168.1.1"), "ip_v4");
    }

    #[test]
    fn test_infer_id_type_ipv6() {
        assert_eq!(infer_id_type("::1"), "ip_v6");
        assert_eq!(infer_id_type("fe80::1"), "ip_v6");
    }

    #[test]
    fn test_infer_id_type_hostname() {
        assert_eq!(infer_id_type("example.local"), "hostname");
        assert_eq!(infer_id_type("my-device.home"), "hostname");
    }

    #[test]
    fn test_infer_id_type_unknown() {
        assert_eq!(infer_id_type("unknown"), "unknown");
    }
}