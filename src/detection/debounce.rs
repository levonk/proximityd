use std::time::{Duration, Instant};

/// Per-device debounce timer tracking first qualified sighting and last sighting.
#[derive(Debug, Clone)]
pub struct DebounceTimer {
    /// When the device was first seen with RSSI above the enter threshold.
    pub first_qualified_seen: Option<Instant>,
    /// When the device was most recently seen (any RSSI).
    pub last_seen: Instant,
    /// Last recorded RSSI value.
    pub last_rssi: i16,
}

impl DebounceTimer {
    /// Create a new timer for a device with the given initial RSSI.
    pub fn new(rssi: i16) -> Self {
        let now = Instant::now();
        Self {
            first_qualified_seen: None,
            last_seen: now,
            last_rssi: rssi,
        }
    }

    /// Record a new sighting. If RSSI meets the threshold, the qualified timer
    /// is started (if not already running).
    pub fn record_sighting(&mut self, rssi: i16, threshold: i16) {
        self.last_seen = Instant::now();
        self.last_rssi = rssi;

        if rssi >= threshold && self.first_qualified_seen.is_none() {
            self.first_qualified_seen = Some(self.last_seen);
        }
    }

    /// Reset the enter debounce so re-entry requires the full duration again.
    pub fn reset_enter(&mut self) {
        self.first_qualified_seen = None;
    }

    /// Elapsed time since the first qualified sighting, if any.
    pub fn enter_elapsed(&self) -> Option<Duration> {
        self.first_qualified_seen.map(|t| t.elapsed())
    }

    /// Elapsed time since the most recent sighting.
    pub fn last_seen_elapsed(&self) -> Duration {
        self.last_seen.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn new_timer_has_no_qualified_time() {
        let timer = DebounceTimer::new(-50);
        assert!(timer.first_qualified_seen.is_none());
        assert_eq!(timer.last_rssi, -50);
    }

    #[test]
    fn record_qualified_sighting_starts_timer() {
        let mut timer = DebounceTimer::new(-80);
        timer.record_sighting(-60, -70);
        assert!(timer.first_qualified_seen.is_some());
        assert_eq!(timer.last_rssi, -60);
    }

    #[test]
    fn record_weak_sighting_does_not_start_timer() {
        let mut timer = DebounceTimer::new(-60);
        timer.record_sighting(-80, -70);
        assert!(timer.first_qualified_seen.is_none());
        assert_eq!(timer.last_rssi, -80);
    }

    #[test]
    fn reset_enter_clears_qualified_timer() {
        let mut timer = DebounceTimer::new(-80);
        timer.record_sighting(-60, -70);
        assert!(timer.first_qualified_seen.is_some());
        timer.reset_enter();
        assert!(timer.first_qualified_seen.is_none());
    }

    #[test]
    fn enter_elapsed_increases_over_time() {
        let mut timer = DebounceTimer::new(-80);
        timer.record_sighting(-60, -70);
        thread::sleep(Duration::from_millis(10));
        let elapsed = timer.enter_elapsed().unwrap();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn last_seen_elapsed_increases_over_time() {
        let timer = DebounceTimer::new(-50);
        thread::sleep(Duration::from_millis(10));
        assert!(timer.last_seen_elapsed() >= Duration::from_millis(10));
    }
}
