use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::config::{AppConfig, DevicesConfig};
use crate::discovery::runtime::SuggestionRuntime;
use crate::signals::{RawSignal, SignalLogger};
use crate::state::{PresenceEvent, PresenceState, PresenceStateTable};

use super::debounce::DebounceTimer;

/// Core detection engine that evaluates BLE scan results against configured
/// thresholds and manages per-device debounce timers.
pub struct DetectionEngine {
    config: AppConfig,
    devices: DevicesConfig,
    state_table: Arc<PresenceStateTable>,
    timers: RwLock<HashMap<String, DebounceTimer>>,
    signal_logger: Option<Mutex<SignalLogger>>,
    suggestion_runtime: Option<SuggestionRuntime>,
}

impl DetectionEngine {
    /// Create a new detection engine.
    pub fn new(
        config: AppConfig,
        devices: DevicesConfig,
        state_table: Arc<PresenceStateTable>,
    ) -> Self {
        Self {
            config,
            devices,
            state_table,
            timers: RwLock::new(HashMap::new()),
            signal_logger: None,
            suggestion_runtime: None,
        }
    }

    /// Attach a signal logger (stub wiring for 01-001; full wiring in 01-003/01-004).
    pub fn with_signal_logger(mut self, logger: SignalLogger) -> Self {
        self.signal_logger = Some(Mutex::new(logger));
        self
    }

    /// Attach a suggestion runtime for auto-discovery fallback.
    pub fn with_suggestion_runtime(mut self, runtime: SuggestionRuntime) -> Self {
        self.suggestion_runtime = Some(runtime);
        self
    }

    /// Resolve a device name for a given MAC/identifier.
    ///
    /// First checks the normal devices config, then falls back to suggestions
    /// if enabled and the identifier is found above the confidence threshold.
    fn resolve_device_name(&self, identifier: &str) -> String {
        // First try normal config lookup
        if let Some(device) = self.devices.get(identifier) {
            return device.name.clone();
        }

        // Fall back to suggestions if enabled
        if let Some(ref runtime) = self.suggestion_runtime {
            if let Some((party_name, device_name, confidence)) = runtime.resolve(identifier) {
                warn!(
                    identifier = %identifier,
                    party = %party_name,
                    device = ?device_name,
                    confidence = %confidence,
                    "Using suggestion-based mapping (auto-discovery)"
                );
                // Return device name if available, otherwise party name
                return device_name.clone().unwrap_or_else(|| party_name.clone());
            }
        }

        // Fallback to the identifier itself
        identifier.to_string()
    }

    /// Evaluate a single scan result and return a presence event if a state
    /// transition occurs.
    ///
    /// # Arguments
    /// * `mac` — Bluetooth MAC address of the discovered device.
    /// * `rssi` — Received Signal Strength Indicator in dBm.
    ///
    /// # Returns
    /// `Some(PresenceEvent)` if the device enters or exits, otherwise `None`.
    pub fn evaluate_scan(&self, mac: impl Into<String>, rssi: i16) -> Option<PresenceEvent> {
        let mac = mac.into();

        // Stub wiring: log every raw signal sighting before evaluation.
        // Full wiring (configurable scanner source, etc.) comes in 01-003/01-004.
        if let Some(ref logger) = self.signal_logger {
            let raw = RawSignal {
                scanner: "ble".into(),
                id_type: "mac".into(),
                id_value: mac.clone(),
                rssi: Some(i32::from(rssi)),
                metadata: None,
            };
            if let Ok(guard) = logger.lock() {
                let _ = guard.log(&raw);
            }
        }

        // Ignore unknown devices unless tracking is enabled.
        let is_known = self.devices.get(&mac).is_some();
        #[allow(deprecated)]
        if !is_known && !self.config.track_unknown {
            debug!(mac = %mac, "Ignoring unknown device (track_unknown=false)");
            return None;
        }

        // Update the state table with the latest RSSI.
        self.state_table.update(&mac, rssi);

        let mut timers = self.timers.write().expect("lock poisoned");
        let timer = timers
            .entry(mac.clone())
            .or_insert_with(|| DebounceTimer::new(rssi));

        #[allow(deprecated)]
        timer.record_sighting(rssi, self.config.enter_rssi_threshold_dbm);

        let current_state = self.state_table.get_state(&mac);

        // Check for enter transition.
        #[allow(deprecated)]
        if rssi >= self.config.enter_rssi_threshold_dbm {
            if let Some(elapsed) = timer.enter_elapsed() {
                #[allow(deprecated)]
                if elapsed >= Duration::from_secs(self.config.enter_duration_seconds)
                    && current_state != Some(PresenceState::Entered)
                {
                    let name = self.resolve_device_name(&mac);

                    drop(timers);
                    self.state_table.set_state(&mac, PresenceState::Entered);
                    info!(mac = %mac, name = %name, "Device entered");
                    return Some(PresenceEvent::Entered {
                        name,
                        mac,
                        party_name: None,
                        source: None,
                        id_type: None,
                        location: None,
                    });
                }
            }
        }

        None
    }

    /// Check all tracked devices for exit conditions and return any exit events.
    pub fn check_exits(&self) -> Vec<PresenceEvent> {
        let mut events = Vec::new();
        #[allow(deprecated)]
        let timeout = Duration::from_secs(self.config.exit_timeout_seconds);

        // Collect the MACs of devices that may have exited.
        let candidates: Vec<String> = {
            let present = self.state_table.list_present();
            present
                .into_iter()
                .filter(|d| d.state == PresenceState::Entered)
                .filter(|d| {
                    let timers = self.timers.read().expect("lock poisoned");
                    if let Some(timer) = timers.get(&d.mac) {
                        timer.last_seen_elapsed() >= timeout
                    } else {
                        false
                    }
                })
                .map(|d| d.mac)
                .collect()
        };

        for mac in candidates {
            let name = self.resolve_device_name(&mac);

            self.state_table.set_state(&mac, PresenceState::Exited);

            {
                let mut timers = self.timers.write().expect("lock poisoned");
                if let Some(t) = timers.get_mut(&mac) {
                    t.reset_enter();
                }
            }

            info!(mac = %mac, name = %name, "Device exited");
            events.push(PresenceEvent::Exited {
                name,
                mac,
                party_name: None,
                source: None,
                id_type: None,
                location: None,
            });
        }

        events
    }

    /// Check if a specific device is currently considered present (Entered).
    pub fn is_present(&self, mac: &str) -> bool {
        self.state_table
            .get_state(mac)
            .map(|s| s == PresenceState::Entered)
            .unwrap_or(false)
    }

    /// Total number of devices currently tracked by the engine.
    pub fn tracked_count(&self) -> usize {
        self.timers.read().expect("lock poisoned").len()
    }
}

#[cfg(test)]
#[path = "engine.test.rs"]
mod tests;
