// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, instrument};

use crate::error::Error;
use crate::service::EndpointService;

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Overall status
    pub status: String,
    
    /// Component statuses
    pub components: HashMap<String, String>,
    
    /// Version
    pub version: String,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Uptime in seconds
    pub uptime_seconds: Option<i64>,
}

/// Create health routes
pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/detailed", get(detailed_health_check))
}

/// Health check handler
#[instrument(skip(service))]
pub async fn health_check(
    State(service): State<Arc<EndpointService>>,
) -> Result<Json<HealthCheckResponse>, Error> {
    info!("Health check requested");
    
    let mut health = HealthCheckResponse {
        status: "ok".to_string(),
        components: HashMap::new(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().timestamp() as u64,
        uptime_seconds: service.get_uptime_seconds(),
    };
    
    // Check database connection
    let db_status = match service.check_database_connection().await {
        Ok(_) => "ok",
        Err(e) => {
            warn!(error = %e, "Database health check failed");
            health.status = "degraded".to_string();
            "error"
        }
    };
    health.components.insert("database".to_string(), db_status.to_string());
    
    // Check worker pool
    let worker_status = match service.check_worker_pool().await {
        Ok(_) => "ok",
        Err(e) => {
            warn!(error = %e, "Worker pool health check failed");
            health.status = "degraded".to_string();
            "error"
        }
    };
    health.components.insert("worker_pool".to_string(), worker_status.to_string());
    
    // Check authentication service
    let auth_status = match service.check_auth_service().await {
        Ok(_) => "ok",
        Err(e) => {
            warn!(error = %e, "Authentication service health check failed");
            health.status = "degraded".to_string();
            "error"
        }
    };
    health.components.insert("auth_service".to_string(), auth_status.to_string());
    
    info!(
        status = %health.status,
        components = ?health.components,
        "Health check completed"
    );
    
    Ok(Json(health))
}

/// Detailed health check handler
#[instrument(skip(service))]
pub async fn detailed_health_check(
    State(service): State<Arc<EndpointService>>,
) -> Result<Json<DetailedHealthResponse>, Error> {
    info!("Detailed health check requested");
    
    // Get basic health check
    let basic_health = health_check(State(service.clone())).await?;
    
    // Get system metrics
    let system_metrics = service.get_system_metrics().await?;
    
    // Create detailed health response
    let detailed_health = DetailedHealthResponse {
        health: basic_health.0,
        system_metrics,
    };
    
    info!("Detailed health check completed");
    
    Ok(Json(detailed_health))
}

/// Detailed health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedHealthResponse {
    /// Basic health information
    pub health: HealthCheckResponse,
    
    /// System metrics
    pub system_metrics: SystemMetrics,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    
    /// Network stats
    pub network: NetworkStats,
    
    /// Function execution stats
    pub function_execution: FunctionExecutionStats,
}

/// Network stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Bytes received
    pub bytes_received: u64,
    
    /// Bytes sent
    pub bytes_sent: u64,
    
    /// Active connections
    pub active_connections: u64,
}

/// Function execution stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionExecutionStats {
    /// Total executions
    pub total_executions: u64,
    
    /// Total errors
    pub total_errors: u64,
    
    /// Average execution time (ms)
    pub avg_execution_time_ms: f64,
    
    /// Average memory usage (MB)
    pub avg_memory_usage_mb: f64,
    
    /// Error rate
    pub error_rate: f64,
}
