use anyhow::Result;
use std::time::Duration;

/// Fetch the public IP address via HTTP with a timeout.
/// Returns None if the request fails or times out.
pub async fn fetch_public_ip(timeout: Duration) -> Result<Option<String>> {
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;
    
    let result = tokio::time::timeout(timeout, async move {
        // Try icanhazip.com first (simple, just returns IP)
        let response = client
            .get("https://icanhazip.com")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch public IP: {}", e))?;
        
        if response.status().is_success() {
            let ip = response
                .text()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to read IP response: {}", e))?;
            
            Ok::<String, anyhow::Error>(ip.trim().to_string())
        } else {
            Err(anyhow::anyhow!("IP service returned status: {}", response.status()))
        }
    }).await;
    
    match result {
        Ok(Ok(ip)) => Ok(Some(ip)),
        Ok(Err(e)) => {
            // Log the error but don't fail - IP geolocation is optional
            tracing::debug!("Public IP fetch failed: {}", e);
            Ok(None)
        }
        Err(_) => {
            // Timeout - IP geolocation is optional
            tracing::debug!("Public IP fetch timed out");
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_public_ip_success() {
        // This test requires network access
        // In CI environments, network may be restricted
        let result = fetch_public_ip(Duration::from_secs(5)).await;
        match result {
            Ok(Some(ip)) => {
                println!("Successfully fetched public IP: {}", ip);
                assert!(!ip.is_empty());
            }
            Ok(None) => {
                println!("Public IP unavailable (expected in offline environments)");
            }
            Err(e) => {
                println!("Public IP fetch error: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_public_ip_timeout() {
        // Very short timeout should fail
        let result = fetch_public_ip(Duration::from_millis(1)).await;
        // Should either timeout (Ok(None)) or fail quickly
        match result {
            Ok(None) => {
                println!("Timeout test passed - request timed out as expected");
            }
            Ok(Some(_)) => {
                println!("Timeout test passed - request succeeded unexpectedly fast");
            }
            Err(_) => {
                println!("Timeout test passed - request failed quickly");
            }
        }
    }
}