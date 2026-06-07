use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Health-check threshold: scan must have completed within this window.
const SCAN_TIMEOUT: Duration = Duration::from_secs(120);

/// Health-check threshold: notification must have been delivered within this window.
const NOTIFICATION_TIMEOUT: Duration = Duration::from_secs(600);

/// Default path for the health heartbeat file used by Docker HEALTHCHECK.
fn default_heartbeat_path() -> PathBuf {
    std::env::var("PROXIMITYD_HEALTH_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/proximityd.health"))
}

/// Write a heartbeat timestamp to the health file.
/// Call after each successful scan cycle.
pub fn write_heartbeat() {
    let path = default_heartbeat_path();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let _ = fs::write(&path, now.to_string());
}

/// Check health from the heartbeat file. Returns `Ok(())` if healthy,
/// `Err` with a message if the heartbeat is missing or stale.
pub fn check_heartbeat_file() -> Result<(), String> {
    let path = default_heartbeat_path();
    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read health file {}: {}", path.display(), e))?;
    let timestamp: u64 = contents
        .trim()
        .parse()
        .map_err(|e| format!("Invalid health file contents: {}", e))?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let age = Duration::from_secs(now.saturating_sub(timestamp));
    if age > SCAN_TIMEOUT {
        Err(format!(
            "Health check failed: last scan was {:?} ago (threshold: {:?})",
            age, SCAN_TIMEOUT
        ))
    } else {
        Ok(())
    }
}

/// Memory threshold: warn if RSS exceeds 64 MiB (NFR-1).
const RSS_WARN_THRESHOLD_BYTES: u64 = 64 * 1024 * 1024;

/// Read current RSS from /proc/self/status (Linux only).
/// Returns `None` on non-Linux platforms or if the file cannot be read.
pub fn read_rss_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let contents = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in contents.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                let value = rest.trim();
                // Format: "VmRSS:   1234 kB"
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 2 {
                    let num: u64 = parts[0].parse().ok()?;
                    let unit = parts[1].to_lowercase();
                    return match unit.as_str() {
                        "kb" => Some(num * 1024),
                        "mb" => Some(num * 1024 * 1024),
                        "gb" => Some(num * 1024 * 1024 * 1024),
                        _ => Some(num), // assume bytes if unit unknown
                    };
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Check memory usage and return a warning string if RSS exceeds the threshold.
/// Returns `Ok(())` if under threshold or unable to read.
pub fn check_memory_usage() -> Result<(), String> {
    match read_rss_bytes() {
        Some(rss) if rss > RSS_WARN_THRESHOLD_BYTES => Err(format!(
            "Memory usage ({:.1} MB) exceeds warning threshold ({} MB)",
            rss as f64 / (1024.0 * 1024.0),
            RSS_WARN_THRESHOLD_BYTES / (1024 * 1024)
        )),
        _ => Ok(()),
    }
}

/// Tracks runtime health signals for the daemon.
///
/// Callers should invoke `record_scan()` after each successful scan cycle
/// and `record_notification()` after each successful notification delivery.
/// `is_healthy()` returns `true` when both signals are within their thresholds.
#[derive(Debug, Clone)]
pub struct HealthCheck {
    inner: Arc<Mutex<HealthInner>>,
}

#[derive(Debug)]
struct HealthInner {
    last_scan_time: Option<Instant>,
    last_notification_time: Option<Instant>,
}

impl HealthCheck {
    /// Create a new health check tracker.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HealthInner {
                last_scan_time: None,
                last_notification_time: None,
            })),
        }
    }

    /// Record that a scan cycle completed successfully.
    pub fn record_scan(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.last_scan_time = Some(Instant::now());
        }
    }

    /// Record that a notification was delivered successfully.
    pub fn record_notification(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.last_notification_time = Some(Instant::now());
        }
    }

    /// Return `true` if the daemon is considered healthy.
    ///
    /// Health criteria:
    /// * A scan must have completed within the last 2 minutes.
    /// * If any notification has ever been sent, the last one must have succeeded
    ///   within the last 10 minutes.
    pub fn is_healthy(&self) -> bool {
        let inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

        // Scan health: must have completed a scan within SCAN_TIMEOUT
        let scan_ok = match inner.last_scan_time {
            Some(t) => t.elapsed() <= SCAN_TIMEOUT,
            None => false,
        };

        // Notification health: if we have ever sent one, it must have succeeded
        // within NOTIFICATION_TIMEOUT. If no notification has been sent yet,
        // this criterion is considered satisfied.
        let notification_ok = match inner.last_notification_time {
            Some(t) => t.elapsed() <= NOTIFICATION_TIMEOUT,
            None => true,
        };

        scan_ok && notification_ok
    }

    /// Get the time elapsed since the last successful scan, if any.
    pub fn scan_age(&self) -> Option<Duration> {
        let inner = self.inner.lock().ok()?;
        inner.last_scan_time.map(|t| t.elapsed())
    }

    /// Get the time elapsed since the last successful notification, if any.
    pub fn notification_age(&self) -> Option<Duration> {
        let inner = self.inner.lock().ok()?;
        inner.last_notification_time.map(|t| t.elapsed())
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "health.test.rs"]
mod tests;
