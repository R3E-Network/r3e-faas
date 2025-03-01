// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info, warn};

use crate::sandbox::{ResourceUsage, SandboxConfig};
use crate::security::threat_detection::{ThreatDetectionService, ThreatSeverity};

/// Threat monitor for sandbox execution
pub struct ThreatMonitor {
    /// Threat detection service
    threat_detection: Arc<ThreatDetectionService>,

    /// Sandbox configuration
    config: SandboxConfig,

    /// Function ID
    function_id: String,

    /// User ID
    user_id: String,

    /// Start time
    start_time: Instant,
}

impl ThreatMonitor {
    /// Create a new threat monitor
    pub fn new(
        threat_detection: Arc<ThreatDetectionService>,
        config: SandboxConfig,
        function_id: String,
        user_id: String,
    ) -> Self {
        Self {
            threat_detection,
            config,
            function_id,
            user_id,
            start_time: Instant::now(),
        }
    }

    /// Monitor code for suspicious patterns
    pub fn scan_code(&self, code: &str) {
        // Scan code for suspicious patterns
        let events = self
            .threat_detection
            .scan_code(&self.user_id, &self.function_id, code);

        // Log events
        for event in &events {
            match event.severity {
                ThreatSeverity::Low => {
                    debug!(
                        "Low severity threat detected: {} ({})",
                        event.event_type, event.details
                    );
                }
                ThreatSeverity::Medium => {
                    info!(
                        "Medium severity threat detected: {} ({})",
                        event.event_type, event.details
                    );
                }
                ThreatSeverity::High => {
                    warn!(
                        "High severity threat detected: {} ({})",
                        event.event_type, event.details
                    );
                }
                ThreatSeverity::Critical => {
                    warn!(
                        "Critical severity threat detected: {} ({})",
                        event.event_type, event.details
                    );
                }
            }
        }
    }

    /// Monitor resource usage
    pub fn monitor_resources(&self, resource_usage: &ResourceUsage) {
        // Check CPU usage
        let cpu_usage = resource_usage
            .cpu_usage_percentage
            .load(std::sync::atomic::Ordering::Relaxed) as u8;
        if cpu_usage > self.config.max_cpu_percentage {
            self.threat_detection
                .check_cpu_usage(&self.user_id, &self.function_id, cpu_usage);
        }

        // Check memory usage
        let memory_usage = resource_usage
            .current_memory_usage
            .load(std::sync::atomic::Ordering::Relaxed);
        let memory_percentage =
            (memory_usage as f64 / self.config.max_memory_usage as f64 * 100.0) as u8;
        if memory_percentage > 90 {
            self.threat_detection.check_memory_usage(
                &self.user_id,
                &self.function_id,
                memory_percentage,
            );
        }

        // Check execution time
        let execution_time = self.start_time.elapsed().as_secs();
        if execution_time > self.config.max_execution_time.as_secs() / 2 {
            self.threat_detection.check_execution_time(
                &self.user_id,
                &self.function_id,
                execution_time,
            );
        }
    }

    /// Record execution failure
    pub fn record_failure(&self, error: &str) {
        // Record failed execution
        self.threat_detection
            .record_failed_execution(&self.user_id, &self.function_id);

        // Check for specific error patterns
        if error.contains("shell") || error.contains("exec") || error.contains("spawn") {
            self.threat_detection.record_shell_execution_attempt(
                &self.user_id,
                &self.function_id,
                error,
            );
        } else if error.contains("file") || error.contains("fs") || error.contains("path") {
            self.threat_detection.record_fs_access_violation(
                &self.user_id,
                &self.function_id,
                error,
            );
        } else if error.contains("network") || error.contains("http") || error.contains("fetch") {
            self.threat_detection.record_network_access_violation(
                &self.user_id,
                &self.function_id,
                error,
            );
        }
    }
}
