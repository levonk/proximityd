use super::*;
use std::time::Duration;

#[cfg(test)]
mod mock_tests {
    use super::*;
    use async_trait::async_trait;

    /// Mock GPS source for testing.
    struct MockGpsSource {
        should_return_none: bool,
        should_timeout: bool,
        coordinates: Option<GpsCoordinates>,
    }

    #[async_trait]
    impl GpsSource for MockGpsSource {
        async fn get_coordinates(&self, timeout: Duration) -> Result<Option<GpsCoordinates>> {
            if self.should_timeout {
                tokio::time::sleep(timeout + Duration::from_millis(100)).await;
                return Ok(None);
            }

            if self.should_return_none {
                return Ok(None);
            }

            Ok(self.coordinates)
        }
    }

    #[tokio::test]
    async fn test_mock_gps_returns_coordinates() {
        let mock = MockGpsSource {
            should_return_none: false,
            should_timeout: false,
            coordinates: Some(GpsCoordinates {
                lat: 37.7749,
                lon: -122.4194,
            }),
        };

        let result = mock.get_coordinates(Duration::from_secs(5)).await.unwrap();
        assert!(result.is_some());
        let coords = result.unwrap();
        assert_eq!(coords.lat, 37.7749);
        assert_eq!(coords.lon, -122.4194);
    }

    #[tokio::test]
    async fn test_mock_gps_returns_none() {
        let mock = MockGpsSource {
            should_return_none: true,
            should_timeout: false,
            coordinates: None,
        };

        let result = mock.get_coordinates(Duration::from_secs(5)).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_mock_gps_times_out() {
        let mock = MockGpsSource {
            should_return_none: false,
            should_timeout: true,
            coordinates: Some(GpsCoordinates {
                lat: 37.7749,
                lon: -122.4194,
            }),
        };

        let result = mock.get_coordinates(Duration::from_millis(10)).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_gps_coordinates_equality() {
        let coords1 = GpsCoordinates {
            lat: 37.7749,
            lon: -122.4194,
        };
        let coords2 = GpsCoordinates {
            lat: 37.7749,
            lon: -122.4194,
        };
        let coords3 = GpsCoordinates {
            lat: 40.7128,
            lon: -74.0060,
        };

        assert_eq!(coords1, coords2);
        assert_ne!(coords1, coords3);
    }

    #[tokio::test]
    async fn test_gps_coordinates_copy() {
        let coords1 = GpsCoordinates {
            lat: 37.7749,
            lon: -122.4194,
        };
        let coords2 = coords1; // Should implement Copy

        assert_eq!(coords1.lat, coords2.lat);
        assert_eq!(coords1.lon, coords2.lon);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_gps_source() {
        let result = create_gps_source().await;
        // Should succeed on all platforms (may be stub on non-Linux)
        assert!(result.is_ok());
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_geoclue_gps_real() {
        // This test requires Geoclue to be running
        // In CI environments, this may not be available
        let result = GeoclueGps::new().await;
        match result {
            Ok(geoclue) => {
                // Try to get coordinates with a short timeout
                let coords = geoclue.get_coordinates(Duration::from_secs(2)).await;
                match coords {
                    Ok(Some(c)) => {
                        println!("Got GPS coordinates: {}, {}", c.lat, c.lon);
                        assert!(c.lat >= -90.0 && c.lat <= 90.0);
                        assert!(c.lon >= -180.0 && c.lon <= 180.0);
                    }
                    Ok(None) => {
                        println!("Geoclue available but no location (expected without GPS hardware)");
                    }
                    Err(e) => {
                        println!("Geoclue location fetch failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Geoclue not available (expected in CI): {}", e);
            }
        }
    }
}