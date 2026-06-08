use anyhow::Result;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::{error, info};

/// Memory limit tracker
pub struct MemoryLimit {
    max_bytes: Option<u64>,
    current_bytes: AtomicU64,
}

impl MemoryLimit {
    /// Create a new memory limit tracker
    pub fn new(max_bytes: Option<u64>) -> Self {
        Self {
            max_bytes,
            current_bytes: AtomicU64::new(0),
        }
    }

    /// Check if adding more bytes would exceed the limit
    pub fn check(&self, additional_bytes: u64) -> Result<()> {
        if let Some(max) = self.max_bytes {
            let current = self.current_bytes.load(Ordering::Relaxed);
            if current + additional_bytes > max {
                error!(
                    "Memory limit exceeded: {} bytes + {} bytes > {} bytes limit",
                    current, additional_bytes, max
                );
                anyhow::bail!("Memory limit exceeded: {} bytes", max);
            }
        }
        Ok(())
    }

    /// Add bytes to the current usage
    pub fn add(&self, bytes: u64) {
        self.current_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Get current memory usage
    pub fn current(&self) -> u64 {
        self.current_bytes.load(Ordering::Relaxed)
    }

    /// Check if a limit is set
    pub fn is_enabled(&self) -> bool {
        self.max_bytes.is_some()
    }
}

/// CPU limit configuration
#[derive(Debug, Clone)]
pub struct CpuLimit {
    max_cores: Option<usize>,
}

impl CpuLimit {
    /// Create a new CPU limit configuration
    pub fn new(max_cores: Option<usize>) -> Self {
        Self { max_cores }
    }

    /// Get the maximum number of cores to use
    pub fn max_cores(&self) -> Option<usize> {
        self.max_cores
    }

    /// Apply CPU limit by returning the effective parallelism
    pub fn effective_parallelism(&self, default: usize) -> usize {
        match self.max_cores {
            Some(max) => {
                let effective = default.min(max);
                if effective < default {
                    info!("CPU limit applied: {} cores (default: {})", effective, default);
                }
                effective
            }
            None => default,
        }
    }

    /// Check if a limit is set
    pub fn is_enabled(&self) -> bool {
        self.max_cores.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_limit_check_within_limit() {
        let limit = MemoryLimit::new(Some(1000));
        assert!(limit.check(500).is_ok());
    }

    #[test]
    fn test_memory_limit_check_exceeds_limit() {
        let limit = MemoryLimit::new(Some(1000));
        assert!(limit.check(1500).is_err());
    }

    #[test]
    fn test_memory_limit_no_limit() {
        let limit = MemoryLimit::new(None);
        assert!(limit.check(u64::MAX).is_ok());
    }

    #[test]
    fn test_memory_limit_add() {
        let limit = MemoryLimit::new(Some(1000));
        limit.add(500);
        assert_eq!(limit.current(), 500);
        limit.add(300);
        assert_eq!(limit.current(), 800);
    }

    #[test]
    fn test_memory_limit_is_enabled() {
        let limit_with = MemoryLimit::new(Some(1000));
        assert!(limit_with.is_enabled());

        let limit_without = MemoryLimit::new(None);
        assert!(!limit_without.is_enabled());
    }

    #[test]
    fn test_cpu_limit_effective_parallelism() {
        let limit = CpuLimit::new(Some(2));
        assert_eq!(limit.effective_parallelism(8), 2);
    }

    #[test]
    fn test_cpu_limit_no_limit() {
        let limit = CpuLimit::new(None);
        assert_eq!(limit.effective_parallelism(8), 8);
    }

    #[test]
    fn test_cpu_limit_is_enabled() {
        let limit_with = CpuLimit::new(Some(2));
        assert!(limit_with.is_enabled());

        let limit_without = CpuLimit::new(None);
        assert!(!limit_without.is_enabled());
    }
}
