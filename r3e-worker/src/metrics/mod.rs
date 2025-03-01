// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Function metrics
#[derive(Debug)]
pub struct FunctionMetrics {
    /// Function ID
    pub function_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Execution count
    pub execution_count: AtomicU64,
    
    /// Error count
    pub error_count: AtomicU64,
    
    /// Execution time sum (ms)
    pub execution_time_sum: AtomicU64,
    
    /// Execution time max (ms)
    pub execution_time_max: AtomicU64,
    
    /// Memory usage sum (MB)
    pub memory_usage_sum: AtomicU64,
    
    /// Memory usage max (MB)
    pub memory_usage_max: AtomicU64,
    
    /// Last execution time
    pub last_execution: RwLock<DateTime<Utc>>,
}

impl FunctionMetrics {
    /// Create new function metrics
    pub fn new(function_id: &str, user_id: &str) -> Self {
        Self {
            function_id: function_id.to_string(),
            user_id: user_id.to_string(),
            execution_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            execution_time_sum: AtomicU64::new(0),
            execution_time_max: AtomicU64::new(0),
            memory_usage_sum: AtomicU64::new(0),
            memory_usage_max: AtomicU64::new(0),
            last_execution: RwLock::new(Utc::now()),
        }
    }
    
    /// Record function execution
    pub fn record_execution(&self, execution_time_ms: u64, memory_usage_mb: u64, success: bool) {
        // Update execution count
        self.execution_count.fetch_add(1, Ordering::Relaxed);
        
        // Update error count if execution failed
        if !success {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update execution time metrics
        self.execution_time_sum.fetch_add(execution_time_ms, Ordering::Relaxed);
        self.update_max(&self.execution_time_max, execution_time_ms);
        
        // Update memory usage metrics
        self.memory_usage_sum.fetch_add(memory_usage_mb, Ordering::Relaxed);
        self.update_max(&self.memory_usage_max, memory_usage_mb);
        
        // Update last execution time
        let mut last_execution = self.last_execution.write().unwrap();
        *last_execution = Utc::now();
    }
    
    /// Update maximum value
    fn update_max(&self, max: &AtomicU64, value: u64) {
        let mut current = max.load(Ordering::Relaxed);
        while value > current {
            match max.compare_exchange_weak(
                current,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(new_current) => current = new_current,
            }
        }
    }
    
    /// Get average execution time
    pub fn avg_execution_time(&self) -> f64 {
        let count = self.execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let sum = self.execution_time_sum.load(Ordering::Relaxed);
        sum as f64 / count as f64
    }
    
    /// Get average memory usage
    pub fn avg_memory_usage(&self) -> f64 {
        let count = self.execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let sum = self.memory_usage_sum.load(Ordering::Relaxed);
        sum as f64 / count as f64
    }
    
    /// Get error rate
    pub fn error_rate(&self) -> f64 {
        let count = self.execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let errors = self.error_count.load(Ordering::Relaxed);
        errors as f64 / count as f64
    }
    
    /// Get metrics summary
    pub fn get_summary(&self) -> FunctionMetricsSummary {
        FunctionMetricsSummary {
            function_id: self.function_id.clone(),
            user_id: self.user_id.clone(),
            execution_count: self.execution_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            avg_execution_time_ms: self.avg_execution_time(),
            max_execution_time_ms: self.execution_time_max.load(Ordering::Relaxed),
            avg_memory_usage_mb: self.avg_memory_usage(),
            max_memory_usage_mb: self.memory_usage_max.load(Ordering::Relaxed),
            error_rate: self.error_rate(),
            last_execution: *self.last_execution.read().unwrap(),
        }
    }
}

/// Function metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetricsSummary {
    /// Function ID
    pub function_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Execution count
    pub execution_count: u64,
    
    /// Error count
    pub error_count: u64,
    
    /// Average execution time (ms)
    pub avg_execution_time_ms: f64,
    
    /// Maximum execution time (ms)
    pub max_execution_time_ms: u64,
    
    /// Average memory usage (MB)
    pub avg_memory_usage_mb: f64,
    
    /// Maximum memory usage (MB)
    pub max_memory_usage_mb: u64,
    
    /// Error rate
    pub error_rate: f64,
    
    /// Last execution time
    pub last_execution: DateTime<Utc>,
}

/// Metrics manager
#[derive(Debug)]
pub struct MetricsManager {
    /// Function metrics
    function_metrics: RwLock<HashMap<String, Arc<FunctionMetrics>>>,
    
    /// System metrics
    system_metrics: RwLock<SystemMetrics>,
    
    /// Alert manager
    alert_manager: Arc<AlertManager>,
}

impl MetricsManager {
    /// Create a new metrics manager
    pub fn new() -> Self {
        Self {
            function_metrics: RwLock::new(HashMap::new()),
            system_metrics: RwLock::new(SystemMetrics::new()),
            alert_manager: Arc::new(AlertManager::new()),
        }
    }
    
    /// Get function metrics
    pub fn get_function_metrics(&self, function_id: &str) -> Option<Arc<FunctionMetrics>> {
        let metrics = self.function_metrics.read().unwrap();
        metrics.get(function_id).cloned()
    }
    
    /// Get or create function metrics
    pub fn get_or_create_function_metrics(&self, function_id: &str, user_id: &str) -> Arc<FunctionMetrics> {
        let mut metrics = self.function_metrics.write().unwrap();
        
        metrics
            .entry(function_id.to_string())
            .or_insert_with(|| Arc::new(FunctionMetrics::new(function_id, user_id)))
            .clone()
    }
    
    /// Record function execution
    #[instrument(skip(self))]
    pub fn record_function_execution(
        &self,
        function_id: &str,
        user_id: &str,
        execution_time_ms: u64,
        memory_usage_mb: u64,
        success: bool,
    ) {
        // Get or create function metrics
        let metrics = self.get_or_create_function_metrics(function_id, user_id);
        
        // Record execution
        metrics.record_execution(execution_time_ms, memory_usage_mb, success);
        
        // Update system metrics
        let mut system_metrics = self.system_metrics.write().unwrap();
        system_metrics.record_function_execution(execution_time_ms, memory_usage_mb, success);
        
        // Check for alerts
        if !success {
            self.alert_manager.check_function_error_alert(function_id, user_id);
        }
        
        if execution_time_ms > 5000 {
            self.alert_manager.check_function_performance_alert(function_id, user_id, execution_time_ms);
        }
        
        if memory_usage_mb > 500 {
            self.alert_manager.check_function_memory_alert(function_id, user_id, memory_usage_mb);
        }
        
        // Log metrics
        info!(
            function_id = %function_id,
            user_id = %user_id,
            execution_time_ms = %execution_time_ms,
            memory_usage_mb = %memory_usage_mb,
            success = %success,
            "Function execution recorded"
        );
    }
    
    /// Get all function metrics
    pub fn get_all_function_metrics(&self) -> Vec<FunctionMetricsSummary> {
        let metrics = self.function_metrics.read().unwrap();
        
        metrics
            .values()
            .map(|m| m.get_summary())
            .collect()
    }
    
    /// Get system metrics
    pub fn get_system_metrics(&self) -> SystemMetricsSummary {
        let metrics = self.system_metrics.read().unwrap();
        metrics.get_summary()
    }
    
    /// Get alert manager
    pub fn get_alert_manager(&self) -> Arc<AlertManager> {
        self.alert_manager.clone()
    }
}

/// System metrics
#[derive(Debug)]
pub struct SystemMetrics {
    /// Total execution count
    pub total_execution_count: AtomicU64,
    
    /// Total error count
    pub total_error_count: AtomicU64,
    
    /// Total execution time (ms)
    pub total_execution_time_ms: AtomicU64,
    
    /// Total memory usage (MB)
    pub total_memory_usage_mb: AtomicU64,
    
    /// Start time
    pub start_time: DateTime<Utc>,
    
    /// Last update time
    pub last_update: RwLock<DateTime<Utc>>,
}

impl SystemMetrics {
    /// Create new system metrics
    pub fn new() -> Self {
        Self {
            total_execution_count: AtomicU64::new(0),
            total_error_count: AtomicU64::new(0),
            total_execution_time_ms: AtomicU64::new(0),
            total_memory_usage_mb: AtomicU64::new(0),
            start_time: Utc::now(),
            last_update: RwLock::new(Utc::now()),
        }
    }
    
    /// Record function execution
    pub fn record_function_execution(
        &mut self,
        execution_time_ms: u64,
        memory_usage_mb: u64,
        success: bool,
    ) {
        // Update execution count
        self.total_execution_count.fetch_add(1, Ordering::Relaxed);
        
        // Update error count if execution failed
        if !success {
            self.total_error_count.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update execution time
        self.total_execution_time_ms.fetch_add(execution_time_ms, Ordering::Relaxed);
        
        // Update memory usage
        self.total_memory_usage_mb.fetch_add(memory_usage_mb, Ordering::Relaxed);
        
        // Update last update time
        let mut last_update = self.last_update.write().unwrap();
        *last_update = Utc::now();
    }
    
    /// Get average execution time
    pub fn avg_execution_time(&self) -> f64 {
        let count = self.total_execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let sum = self.total_execution_time_ms.load(Ordering::Relaxed);
        sum as f64 / count as f64
    }
    
    /// Get average memory usage
    pub fn avg_memory_usage(&self) -> f64 {
        let count = self.total_execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let sum = self.total_memory_usage_mb.load(Ordering::Relaxed);
        sum as f64 / count as f64
    }
    
    /// Get error rate
    pub fn error_rate(&self) -> f64 {
        let count = self.total_execution_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0.0;
        }
        
        let errors = self.total_error_count.load(Ordering::Relaxed);
        errors as f64 / count as f64
    }
    
    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> i64 {
        (Utc::now() - self.start_time).num_seconds()
    }
    
    /// Get metrics summary
    pub fn get_summary(&self) -> SystemMetricsSummary {
        SystemMetricsSummary {
            total_execution_count: self.total_execution_count.load(Ordering::Relaxed),
            total_error_count: self.total_error_count.load(Ordering::Relaxed),
            avg_execution_time_ms: self.avg_execution_time(),
            avg_memory_usage_mb: self.avg_memory_usage(),
            error_rate: self.error_rate(),
            uptime_seconds: self.uptime_seconds(),
            start_time: self.start_time,
            last_update: *self.last_update.read().unwrap(),
        }
    }
}

/// System metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsSummary {
    /// Total execution count
    pub total_execution_count: u64,
    
    /// Total error count
    pub total_error_count: u64,
    
    /// Average execution time (ms)
    pub avg_execution_time_ms: f64,
    
    /// Average memory usage (MB)
    pub avg_memory_usage_mb: f64,
    
    /// Error rate
    pub error_rate: f64,
    
    /// Uptime in seconds
    pub uptime_seconds: i64,
    
    /// Start time
    pub start_time: DateTime<Utc>,
    
    /// Last update time
    pub last_update: DateTime<Utc>,
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info
    Info,
    
    /// Warning
    Warning,
    
    /// Error
    Error,
    
    /// Critical
    Critical,
}

/// Alert type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    /// Function error
    FunctionError,
    
    /// Function performance
    FunctionPerformance,
    
    /// Function memory
    FunctionMemory,
    
    /// System error rate
    SystemErrorRate,
    
    /// System performance
    SystemPerformance,
    
    /// System memory
    SystemMemory,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    
    /// Alert type
    pub alert_type: AlertType,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Alert message
    pub message: String,
    
    /// Function ID (if applicable)
    pub function_id: Option<String>,
    
    /// User ID (if applicable)
    pub user_id: Option<String>,
    
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Alert resolved
    pub resolved: bool,
    
    /// Alert resolved timestamp
    pub resolved_timestamp: Option<DateTime<Utc>>,
}

impl Alert {
    /// Create a new alert
    pub fn new(
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        function_id: Option<String>,
        user_id: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            alert_type,
            severity,
            message,
            function_id,
            user_id,
            timestamp: Utc::now(),
            resolved: false,
            resolved_timestamp: None,
        }
    }
    
    /// Resolve the alert
    pub fn resolve(&mut self) {
        self.resolved = true;
        self.resolved_timestamp = Some(Utc::now());
    }
}

/// Alert manager
#[derive(Debug)]
pub struct AlertManager {
    /// Alerts
    alerts: RwLock<Vec<Alert>>,
    
    /// Alert handlers
    alert_handlers: RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new() -> Self {
        Self {
            alerts: RwLock::new(Vec::new()),
            alert_handlers: RwLock::new(Vec::new()),
        }
    }
    
    /// Add an alert handler
    pub fn add_alert_handler<H>(&self, handler: H)
    where
        H: AlertHandler + Send + Sync + 'static,
    {
        let mut handlers = self.alert_handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }
    
    /// Create and handle an alert
    pub fn create_alert(
        &self,
        alert_type: AlertType,
        severity: AlertSeverity,
        message: String,
        function_id: Option<String>,
        user_id: Option<String>,
    ) -> Alert {
        // Create the alert
        let alert = Alert::new(alert_type, severity, message, function_id, user_id);
        
        // Add the alert to the list
        let mut alerts = self.alerts.write().unwrap();
        alerts.push(alert.clone());
        
        // Handle the alert
        let handlers = self.alert_handlers.read().unwrap();
        for handler in handlers.iter() {
            handler.handle_alert(&alert);
        }
        
        // Log the alert
        match severity {
            AlertSeverity::Info => {
                info!(
                    alert_id = %alert.id,
                    alert_type = ?alert.alert_type,
                    function_id = ?alert.function_id,
                    user_id = ?alert.user_id,
                    "Alert created: {}",
                    alert.message
                );
            }
            AlertSeverity::Warning => {
                warn!(
                    alert_id = %alert.id,
                    alert_type = ?alert.alert_type,
                    function_id = ?alert.function_id,
                    user_id = ?alert.user_id,
                    "Alert created: {}",
                    alert.message
                );
            }
            AlertSeverity::Error | AlertSeverity::Critical => {
                error!(
                    alert_id = %alert.id,
                    alert_type = ?alert.alert_type,
                    function_id = ?alert.function_id,
                    user_id = ?alert.user_id,
                    "Alert created: {}",
                    alert.message
                );
            }
        }
        
        alert
    }
    
    /// Get all alerts
    pub fn get_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().unwrap();
        alerts.clone()
    }
    
    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().unwrap();
        alerts.iter().filter(|a| !a.resolved).cloned().collect()
    }
    
    /// Resolve an alert
    pub fn resolve_alert(&self, alert_id: &str) -> Option<Alert> {
        let mut alerts = self.alerts.write().unwrap();
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolve();
            Some(alert.clone())
        } else {
            None
        }
    }
    
    /// Check for function error alert
    pub fn check_function_error_alert(&self, function_id: &str, user_id: &str) {
        self.create_alert(
            AlertType::FunctionError,
            AlertSeverity::Warning,
            format!("Function {} execution failed", function_id),
            Some(function_id.to_string()),
            Some(user_id.to_string()),
        );
    }
    
    /// Check for function performance alert
    pub fn check_function_performance_alert(&self, function_id: &str, user_id: &str, execution_time_ms: u64) {
        self.create_alert(
            AlertType::FunctionPerformance,
            AlertSeverity::Warning,
            format!("Function {} execution time is high: {}ms", function_id, execution_time_ms),
            Some(function_id.to_string()),
            Some(user_id.to_string()),
        );
    }
    
    /// Check for function memory alert
    pub fn check_function_memory_alert(&self, function_id: &str, user_id: &str, memory_usage_mb: u64) {
        self.create_alert(
            AlertType::FunctionMemory,
            AlertSeverity::Warning,
            format!("Function {} memory usage is high: {}MB", function_id, memory_usage_mb),
            Some(function_id.to_string()),
            Some(user_id.to_string()),
        );
    }
}

/// Alert handler trait
pub trait AlertHandler {
    /// Handle an alert
    fn handle_alert(&self, alert: &Alert);
}

/// Log alert handler
pub struct LogAlertHandler;

impl AlertHandler for LogAlertHandler {
    fn handle_alert(&self, alert: &Alert) {
        match alert.severity {
            AlertSeverity::Info => {
                info!("Alert: {}", alert.message);
            }
            AlertSeverity::Warning => {
                warn!("Alert: {}", alert.message);
            }
            AlertSeverity::Error | AlertSeverity::Critical => {
                error!("Alert: {}", alert.message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_metrics() {
        let metrics = FunctionMetrics::new("test-function", "test-user");
        
        // Record successful execution
        metrics.record_execution(100, 50, true);
        
        assert_eq!(metrics.execution_count.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.error_count.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.execution_time_sum.load(Ordering::Relaxed), 100);
        assert_eq!(metrics.execution_time_max.load(Ordering::Relaxed), 100);
        assert_eq!(metrics.memory_usage_sum.load(Ordering::Relaxed), 50);
        assert_eq!(metrics.memory_usage_max.load(Ordering::Relaxed), 50);
        
        // Record failed execution
        metrics.record_execution(200, 100, false);
        
        assert_eq!(metrics.execution_count.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.error_count.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.execution_time_sum.load(Ordering::Relaxed), 300);
        assert_eq!(metrics.execution_time_max.load(Ordering::Relaxed), 200);
        assert_eq!(metrics.memory_usage_sum.load(Ordering::Relaxed), 150);
        assert_eq!(metrics.memory_usage_max.load(Ordering::Relaxed), 100);
        
        // Check derived metrics
        assert_eq!(metrics.avg_execution_time(), 150.0);
        assert_eq!(metrics.avg_memory_usage(), 75.0);
        assert_eq!(metrics.error_rate(), 0.5);
    }
    
    #[test]
    fn test_metrics_manager() {
        let manager = MetricsManager::new();
        
        // Record function executions
        manager.record_function_execution("test-function-1", "test-user-1", 100, 50, true);
        manager.record_function_execution("test-function-1", "test-user-1", 200, 100, false);
        manager.record_function_execution("test-function-2", "test-user-2", 150, 75, true);
        
        // Check function metrics
        let metrics_1 = manager.get_function_metrics("test-function-1").unwrap();
        assert_eq!(metrics_1.execution_count.load(Ordering::Relaxed), 2);
        assert_eq!(metrics_1.error_count.load(Ordering::Relaxed), 1);
        
        let metrics_2 = manager.get_function_metrics("test-function-2").unwrap();
        assert_eq!(metrics_2.execution_count.load(Ordering::Relaxed), 1);
        assert_eq!(metrics_2.error_count.load(Ordering::Relaxed), 0);
        
        // Check system metrics
        let system_metrics = manager.get_system_metrics();
        assert_eq!(system_metrics.total_execution_count, 3);
        assert_eq!(system_metrics.total_error_count, 1);
        assert_eq!(system_metrics.avg_execution_time_ms, 150.0);
        assert_eq!(system_metrics.avg_memory_usage_mb, 75.0);
        assert_eq!(system_metrics.error_rate, 1.0 / 3.0);
    }
    
    #[test]
    fn test_alert_manager() {
        let manager = AlertManager::new();
        
        // Create alerts
        let alert_1 = manager.create_alert(
            AlertType::FunctionError,
            AlertSeverity::Warning,
            "Test alert 1".to_string(),
            Some("test-function-1".to_string()),
            Some("test-user-1".to_string()),
        );
        
        let alert_2 = manager.create_alert(
            AlertType::SystemPerformance,
            AlertSeverity::Error,
            "Test alert 2".to_string(),
            None,
            None,
        );
        
        // Check alerts
        let alerts = manager.get_alerts();
        assert_eq!(alerts.len(), 2);
        
        // Resolve an alert
        let resolved = manager.resolve_alert(&alert_1.id).unwrap();
        assert!(resolved.resolved);
        assert!(resolved.resolved_timestamp.is_some());
        
        // Check active alerts
        let active_alerts = manager.get_active_alerts();
        assert_eq!(active_alerts.len(), 1);
        assert_eq!(active_alerts[0].id, alert_2.id);
    }
}
