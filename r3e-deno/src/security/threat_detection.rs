// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};
use regex::Regex;
use lazy_static::lazy_static;

/// Threat detection configuration
#[derive(Debug, Clone)]
pub struct ThreatDetectionConfig {
    /// Maximum number of failed executions before triggering an alert
    pub max_failed_executions: u32,
    
    /// Time window for failed executions (in seconds)
    pub failed_execution_window: u64,
    
    /// Maximum CPU usage percentage that triggers an alert
    pub max_cpu_usage_threshold: u8,
    
    /// Maximum memory usage percentage that triggers an alert
    pub max_memory_usage_threshold: u8,
    
    /// Maximum execution time that triggers an alert (in seconds)
    pub max_execution_time_threshold: u64,
    
    /// Enable detection of suspicious code patterns
    pub detect_suspicious_patterns: bool,
    
    /// Enable detection of network scanning
    pub detect_network_scanning: bool,
    
    /// Enable detection of crypto mining
    pub detect_crypto_mining: bool,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            max_failed_executions: 5,
            failed_execution_window: 60, // 1 minute
            max_cpu_usage_threshold: 90,
            max_memory_usage_threshold: 90,
            max_execution_time_threshold: 30, // 30 seconds
            detect_suspicious_patterns: true,
            detect_network_scanning: true,
            detect_crypto_mining: true,
        }
    }
}

/// Threat severity level
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    /// Low severity
    Low,
    
    /// Medium severity
    Medium,
    
    /// High severity
    High,
    
    /// Critical severity
    Critical,
}

/// Threat event type
#[derive(Debug, Clone, PartialEq)]
pub enum ThreatEventType {
    /// Too many failed executions
    TooManyFailedExecutions,
    
    /// High CPU usage
    HighCpuUsage,
    
    /// High memory usage
    HighMemoryUsage,
    
    /// Long execution time
    LongExecutionTime,
    
    /// Suspicious code pattern
    SuspiciousCodePattern,
    
    /// Network scanning
    NetworkScanning,
    
    /// Crypto mining
    CryptoMining,
    
    /// Shell execution attempt
    ShellExecutionAttempt,
    
    /// File system access violation
    FileSystemAccessViolation,
    
    /// Unauthorized network access
    UnauthorizedNetworkAccess,
}

/// Threat detection event
#[derive(Debug, Clone)]
pub struct ThreatEvent {
    /// Event type
    pub event_type: ThreatEventType,
    
    /// Function ID
    pub function_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Details
    pub details: String,
    
    /// Severity
    pub severity: ThreatSeverity,
}

/// Failed execution record
#[derive(Debug, Clone)]
struct FailedExecutionRecord {
    /// User ID
    user_id: String,
    
    /// Function ID
    function_id: String,
    
    /// Timestamps of failed executions
    timestamps: Vec<u64>,
}

/// Threat detection service
pub struct ThreatDetectionService {
    /// Configuration
    config: ThreatDetectionConfig,
    
    /// Failed executions
    failed_executions: Arc<Mutex<HashMap<String, FailedExecutionRecord>>>,
    
    /// Event handlers
    event_handlers: Vec<Box<dyn Fn(&ThreatEvent) + Send + Sync>>,
    
    /// Suspicious code patterns
    suspicious_patterns: Vec<Regex>,
    
    /// Network scanning patterns
    network_scanning_patterns: Vec<Regex>,
    
    /// Crypto mining patterns
    crypto_mining_patterns: Vec<Regex>,
}

/// Get default suspicious code patterns
fn get_suspicious_patterns() -> Vec<&'static str> {
    vec![
        "eval", "Function", "new Function", "process.binding", "child_process",
        "require", "exec", "spawn", "fork", "Deno.core", "Deno.internal",
        "Deno.permissions", "__proto__", "constructor.constructor", "Object.constructor"
    ]
}

/// Get default network scanning patterns
fn get_network_scanning_patterns() -> Vec<&'static str> {
    vec![
        "for", "fetch", ".map", ".forEach", "ping", "traceroute", "nmap"
    ]
}

/// Get default crypto mining patterns
fn get_crypto_mining_patterns() -> Vec<&'static str> {
    vec![
        "CryptoNight", "hashPow", "miner.start", "mining.start", "cryptonight",
        "minero", "coinhive", "jsecoin", "webmining", "deepminer", "deepMiner",
        "coinlab", "cryptoloot", "crypto-loot", "cryptaloot", "webmine", "webminer"
    ]
}

impl ThreatDetectionService {
    /// Create a new threat detection service
    pub fn new(config: ThreatDetectionConfig) -> Self {
        // Compile regex patterns
        let suspicious_patterns = if config.detect_suspicious_patterns {
            get_suspicious_patterns().iter()
                .map(|p| Regex::new(p).unwrap())
                .collect()
        } else {
            Vec::new()
        };
        
        let network_scanning_patterns = if config.detect_network_scanning {
            get_network_scanning_patterns().iter()
                .map(|p| Regex::new(p).unwrap())
                .collect()
        } else {
            Vec::new()
        };
        
        let crypto_mining_patterns = if config.detect_crypto_mining {
            get_crypto_mining_patterns().iter()
                .map(|p| Regex::new(p).unwrap())
                .collect()
        } else {
            Vec::new()
        };
        
        Self {
            config,
            failed_executions: Arc::new(Mutex::new(HashMap::new())),
            event_handlers: Vec::new(),
            suspicious_patterns,
            network_scanning_patterns,
            crypto_mining_patterns,
        }
    }
    
    /// Add an event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&ThreatEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }
    
    /// Record a failed execution
    pub fn record_failed_execution(&self, user_id: &str, function_id: &str) {
        let key = format!("{}:{}", user_id, function_id);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut failed_executions = self.failed_executions.lock().unwrap();
        
        // Get or create the record
        let record = failed_executions
            .entry(key.clone())
            .or_insert_with(|| FailedExecutionRecord {
                user_id: user_id.to_string(),
                function_id: function_id.to_string(),
                timestamps: Vec::new(),
            });
        
        // Add the timestamp
        record.timestamps.push(now);
        
        // Remove old timestamps
        let window_start = now.saturating_sub(self.config.failed_execution_window);
        record.timestamps.retain(|&t| t >= window_start);
        
        // Check if we've exceeded the threshold
        if record.timestamps.len() as u32 >= self.config.max_failed_executions {
            // Create a threat event
            let event = ThreatEvent {
                event_type: ThreatEventType::TooManyFailedExecutions,
                function_id: function_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: now,
                details: format!(
                    "Too many failed executions: {} in the last {} seconds",
                    record.timestamps.len(),
                    self.config.failed_execution_window
                ),
                severity: ThreatSeverity::Medium,
            };
            
            // Trigger event handlers
            self.trigger_event(&event);
        }
    }
    
    /// Check CPU usage
    pub fn check_cpu_usage(&self, user_id: &str, function_id: &str, cpu_usage: u8) {
        if cpu_usage >= self.config.max_cpu_usage_threshold {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Create a threat event
            let event = ThreatEvent {
                event_type: ThreatEventType::HighCpuUsage,
                function_id: function_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: now,
                details: format!(
                    "High CPU usage: {}% (threshold: {}%)",
                    cpu_usage,
                    self.config.max_cpu_usage_threshold
                ),
                severity: if cpu_usage >= 95 {
                    ThreatSeverity::High
                } else {
                    ThreatSeverity::Medium
                },
            };
            
            // Trigger event handlers
            self.trigger_event(&event);
        }
    }
    
    /// Check memory usage
    pub fn check_memory_usage(&self, user_id: &str, function_id: &str, memory_usage: u8) {
        if memory_usage >= self.config.max_memory_usage_threshold {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Create a threat event
            let event = ThreatEvent {
                event_type: ThreatEventType::HighMemoryUsage,
                function_id: function_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: now,
                details: format!(
                    "High memory usage: {}% (threshold: {}%)",
                    memory_usage,
                    self.config.max_memory_usage_threshold
                ),
                severity: if memory_usage >= 95 {
                    ThreatSeverity::High
                } else {
                    ThreatSeverity::Medium
                },
            };
            
            // Trigger event handlers
            self.trigger_event(&event);
        }
    }
    
    /// Check execution time
    pub fn check_execution_time(&self, user_id: &str, function_id: &str, execution_time_secs: u64) {
        if execution_time_secs >= self.config.max_execution_time_threshold {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Create a threat event
            let event = ThreatEvent {
                event_type: ThreatEventType::LongExecutionTime,
                function_id: function_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: now,
                details: format!(
                    "Long execution time: {}s (threshold: {}s)",
                    execution_time_secs,
                    self.config.max_execution_time_threshold
                ),
                severity: if execution_time_secs >= self.config.max_execution_time_threshold * 2 {
                    ThreatSeverity::High
                } else {
                    ThreatSeverity::Medium
                },
            };
            
            // Trigger event handlers
            self.trigger_event(&event);
        }
    }
    
    /// Scan code for suspicious patterns
    pub fn scan_code(&self, user_id: &str, function_id: &str, code: &str) -> Vec<ThreatEvent> {
        let mut events = Vec::new();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check for suspicious patterns
        if self.config.detect_suspicious_patterns {
            for pattern in &self.suspicious_patterns {
                if let Some(captures) = pattern.captures(code) {
                    let matched = captures.get(0).map_or("", |m| m.as_str());
                    
                    // Create a threat event
                    let event = ThreatEvent {
                        event_type: ThreatEventType::SuspiciousCodePattern,
                        function_id: function_id.to_string(),
                        user_id: user_id.to_string(),
                        timestamp: now,
                        details: format!(
                            "Suspicious code pattern detected: {}",
                            matched
                        ),
                        severity: ThreatSeverity::High,
                    };
                    
                    events.push(event.clone());
                    
                    // Trigger event handlers
                    self.trigger_event(&event);
                }
            }
        }
        
        // Check for network scanning patterns
        if self.config.detect_network_scanning {
            for pattern in &self.network_scanning_patterns {
                if let Some(captures) = pattern.captures(code) {
                    let matched = captures.get(0).map_or("", |m| m.as_str());
                    
                    // Create a threat event
                    let event = ThreatEvent {
                        event_type: ThreatEventType::NetworkScanning,
                        function_id: function_id.to_string(),
                        user_id: user_id.to_string(),
                        timestamp: now,
                        details: format!(
                            "Network scanning pattern detected: {}",
                            matched
                        ),
                        severity: ThreatSeverity::High,
                    };
                    
                    events.push(event.clone());
                    
                    // Trigger event handlers
                    self.trigger_event(&event);
                }
            }
        }
        
        // Check for crypto mining patterns
        if self.config.detect_crypto_mining {
            for pattern in &self.crypto_mining_patterns {
                if let Some(captures) = pattern.captures(code) {
                    let matched = captures.get(0).map_or("", |m| m.as_str());
                    
                    // Create a threat event
                    let event = ThreatEvent {
                        event_type: ThreatEventType::CryptoMining,
                        function_id: function_id.to_string(),
                        user_id: user_id.to_string(),
                        timestamp: now,
                        details: format!(
                            "Crypto mining pattern detected: {}",
                            matched
                        ),
                        severity: ThreatSeverity::Critical,
                    };
                    
                    events.push(event.clone());
                    
                    // Trigger event handlers
                    self.trigger_event(&event);
                }
            }
        }
        
        events
    }
    
    /// Record a shell execution attempt
    pub fn record_shell_execution_attempt(&self, user_id: &str, function_id: &str, command: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create a threat event
        let event = ThreatEvent {
            event_type: ThreatEventType::ShellExecutionAttempt,
            function_id: function_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: now,
            details: format!(
                "Shell execution attempt: {}",
                command
            ),
            severity: ThreatSeverity::Critical,
        };
        
        // Trigger event handlers
        self.trigger_event(&event);
    }
    
    /// Record a file system access violation
    pub fn record_fs_access_violation(&self, user_id: &str, function_id: &str, path: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create a threat event
        let event = ThreatEvent {
            event_type: ThreatEventType::FileSystemAccessViolation,
            function_id: function_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: now,
            details: format!(
                "File system access violation: {}",
                path
            ),
            severity: ThreatSeverity::High,
        };
        
        // Trigger event handlers
        self.trigger_event(&event);
    }
    
    /// Record an unauthorized network access
    pub fn record_network_access_violation(&self, user_id: &str, function_id: &str, url: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create a threat event
        let event = ThreatEvent {
            event_type: ThreatEventType::UnauthorizedNetworkAccess,
            function_id: function_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: now,
            details: format!(
                "Unauthorized network access: {}",
                url
            ),
            severity: ThreatSeverity::High,
        };
        
        // Trigger event handlers
        self.trigger_event(&event);
    }
    
    /// Trigger event handlers
    fn trigger_event(&self, event: &ThreatEvent) {
        for handler in &self.event_handlers {
            handler(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_suspicious_code_detection() {
        let config = ThreatDetectionConfig::default();
        let service = ThreatDetectionService::new(config);
        
        let code = r#"
        function malicious() {
            eval("alert('hacked')");
            const evil = new Function("return process.binding('os')");
            require('child_process').exec('rm -rf /');
        }
        "#;
        
        let events = service.scan_code("user1", "func1", code);
        
        // Should detect eval, new Function, and child_process.exec
        assert_eq!(events.len(), 3);
        
        // Check event types
        assert!(events.iter().all(|e| e.event_type == ThreatEventType::SuspiciousCodePattern));
        
        // Check severity
        assert!(events.iter().all(|e| e.severity == ThreatSeverity::High));
    }
    
    #[test]
    fn test_network_scanning_detection() {
        let config = ThreatDetectionConfig::default();
        let service = ThreatDetectionService::new(config);
        
        let code = r#"
        async function scanNetwork() {
            const ips = [];
            for (let i = 1; i < 255; i++) {
                ips.push(`192.168.1.${i}`);
            }
            
            for (const ip of ips) {
                fetch(`http://${ip}`);
            }
            
            // Alternative approach
            ips.map(ip => fetch(`http://${ip}`));
        }
        "#;
        
        let events = service.scan_code("user1", "func1", code);
        
        // Should detect for loop with fetch and map with fetch
        assert_eq!(events.len(), 2);
        
        // Check event types
        assert!(events.iter().all(|e| e.event_type == ThreatEventType::NetworkScanning));
        
        // Check severity
        assert!(events.iter().all(|e| e.severity == ThreatSeverity::High));
    }
    
    #[test]
    fn test_crypto_mining_detection() {
        let config = ThreatDetectionConfig::default();
        let service = ThreatDetectionService::new(config);
        
        let code = r#"
        function startMining() {
            const miner = new CoinHive.Anonymous('SITE_KEY');
            miner.start();
            
            // Alternative miners
            const cryptonight = new CryptoNight();
            const webminer = new WebMiner();
        }
        "#;
        
        let events = service.scan_code("user1", "func1", code);
        
        // Should detect multiple crypto mining patterns
        assert!(events.len() >= 3);
        
        // Check event types
        assert!(events.iter().all(|e| e.event_type == ThreatEventType::CryptoMining));
        
        // Check severity
        assert!(events.iter().all(|e| e.severity == ThreatSeverity::Critical));
    }
    
    #[test]
    fn test_failed_executions() {
        let config = ThreatDetectionConfig {
            max_failed_executions: 3,
            failed_execution_window: 60,
            ..ThreatDetectionConfig::default()
        };
        
        let service = ThreatDetectionService::new(config);
        
        // Add an event handler to capture events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        
        let mut service_with_handler = service;
        service_with_handler.add_event_handler(move |event| {
            let mut events = events_clone.lock().unwrap();
            events.push(event.clone());
        });
        
        // Record failed executions
        service_with_handler.record_failed_execution("user1", "func1");
        service_with_handler.record_failed_execution("user1", "func1");
        
        // Should not trigger an event yet
        assert_eq!(events.lock().unwrap().len(), 0);
        
        // Record one more failed execution to exceed the threshold
        service_with_handler.record_failed_execution("user1", "func1");
        
        // Should trigger an event
        assert_eq!(events.lock().unwrap().len(), 1);
        
        // Check event type
        let event = &events.lock().unwrap()[0];
        assert_eq!(event.event_type, ThreatEventType::TooManyFailedExecutions);
        assert_eq!(event.user_id, "user1");
        assert_eq!(event.function_id, "func1");
    }
}
