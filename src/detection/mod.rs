pub mod bridge;
pub mod debounce;
pub mod engine;

pub use bridge::run_detection_loop;
pub use debounce::DebounceTimer;
pub use engine::DetectionEngine;

#[cfg(test)]
#[path = "simulation.test.rs"]
mod simulation_tests;
