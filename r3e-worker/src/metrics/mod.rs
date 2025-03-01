// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// Worker metrics
pub struct WorkerMetrics {
    /// Active functions
    active_functions: AtomicUsize,
    
    /// Total functions executed
    total_functions: AtomicUsize,
    
    /// Total execution time in milliseconds
    total_execution_time_ms: AtomicUsize,
}

impl WorkerMetrics {
    /// Create new worker metrics
    pub fn new() -> Self {
        Self {
            active_functions: AtomicUsize::new(0),
            total_functions: AtomicUsize::new(0),
            total_execution_time_ms: AtomicUsize::new(0),
        }
    }
    
    /// Increment active functions
    pub fn increment_active_functions(&self) {
        self.active_functions.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Decrement active functions
    pub fn decrement_active_functions(&self) {
        self.active_functions.fetch_sub(1, Ordering::SeqCst);
    }
    
    /// Increment total functions
    pub fn increment_total_functions(&self) {
        self.total_functions.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Record execution time
    pub fn record_execution_time(&self, duration: Duration) {
        let ms = duration.as_millis() as usize;
        self.total_execution_time_ms.fetch_add(ms, Ordering::SeqCst);
    }
    
    /// Get active functions
    pub fn active_functions(&self) -> usize {
        self.active_functions.load(Ordering::SeqCst)
    }
    
    /// Get total functions
    pub fn total_functions(&self) -> usize {
        self.total_functions.load(Ordering::SeqCst)
    }
    
    /// Get average execution time
    pub fn average_execution_time(&self) -> Option<Duration> {
        let total = self.total_functions.load(Ordering::SeqCst);
        if total == 0 {
            return None;
        }
        
        let total_ms = self.total_execution_time_ms.load(Ordering::SeqCst);
        let avg_ms = total_ms / total;
        
        Some(Duration::from_millis(avg_ms as u64))
    }
}
