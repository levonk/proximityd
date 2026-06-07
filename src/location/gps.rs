use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;

/// GPS coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpsCoordinates {
    /// Latitude in decimal degrees.
    pub lat: f64,
    /// Longitude in decimal degrees.
    pub lon: f64,
}

/// Trait for GPS coordinate sources.
#[async_trait]
pub trait GpsSource: Send + Sync {
    /// Attempt to get GPS coordinates with a timeout.
    /// Returns None if GPS is unavailable or times out.
    async fn get_coordinates(&self, timeout: Duration) -> Result<Option<GpsCoordinates>>;
}

/// Geoclue-based GPS source for Linux.
#[cfg(target_os = "linux")]
pub struct GeoclueGps {
    client: geoclue_zbus::Client,
}

#[cfg(target_os = "linux")]
impl GeoclueGps {
    /// Create a new GeoclueGps instance.
    pub async fn new() -> Result<Self> {
        let connection = zbus::Connection::system()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to system D-Bus: {}", e))?;
        
        let client = geoclue_zbus::Client::new(&connection)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create Geoclue client: {}", e))?;
        
        Ok(Self { client })
    }
}

#[cfg(target_os = "linux")]
#[async_trait]
impl GpsSource for GeoclueGps {
    async fn get_coordinates(&self, timeout: Duration) -> Result<Option<GpsCoordinates>> {
        let client = self.client.clone();
        
        // Use tokio::time::timeout to enforce the timeout
        let result = tokio::time::timeout(timeout, async move {
            // Start the client
            client.start()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start Geoclue client: {}", e))?;
            
            // Get location
            let location = client.location()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get location from Geoclue: {}", e))?;
            
            // Stop the client when done
            let _ = client.stop().await;
            
            Ok::<GpsCoordinates, anyhow::Error>(GpsCoordinates {
                lat: location.latitude(),
                lon: location.longitude(),
            })
        }).await;
        
        match result {
            Ok(Ok(coords)) => Ok(Some(coords)),
            Ok(Err(e)) => {
                // Log the error but don't fail - GPS is optional
                tracing::debug!("Geoclue GPS lookup failed: {}", e);
                Ok(None)
            }
            Err(_) => {
                // Timeout - GPS is optional
                tracing::debug!("Geoclue GPS lookup timed out");
                Ok(None)
            }
        }
    }
}

/// Stub GPS source for non-Linux platforms.
#[cfg(not(target_os = "linux"))]
pub struct StubGps;

#[cfg(not(target_os = "linux"))]
#[async_trait]
impl GpsSource for StubGps {
    async fn get_coordinates(&self, _timeout: Duration) -> Result<Option<GpsCoordinates>> {
        Ok(None)
    }
}

/// Create the appropriate GPS source for the current platform.
pub async fn create_gps_source() -> Result<Box<dyn GpsSource>> {
    #[cfg(target_os = "linux")]
    {
        let geoclue = GeoclueGps::new().await?;
        Ok(Box::new(geoclue))
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Ok(Box::new(StubGps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_geoclue_gps_creation() {
        // This test requires Geoclue to be running
        // In CI environments, this may not be available
        let result = GeoclueGps::new().await;
        match result {
            Ok(_) => println!("Geoclue client created successfully"),
            Err(e) => println!("Geoclue not available (expected in CI): {}", e),
        }
    }

    #[cfg(not(target_os = "linux"))]
    #[tokio::test]
    async fn test_stub_gps_returns_none() {
        let stub = StubGps;
        let coords = stub.get_coordinates(Duration::from_secs(1)).await.unwrap();
        assert!(coords.is_none());
    }

    #[tokio::test]
    async fn test_create_gps_source() {
        let source = create_gps_source().await;
        // Should succeed on all platforms (may be stub on non-Linux)
        assert!(source.is_ok());
    }
}

#[cfg(test)]
#[path = "gps.test.rs"]
mod gps_tests;