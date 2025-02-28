// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::Json;
use serde::{Deserialize, Serialize};

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Status
    pub status: String,
    
    /// Version
    pub version: String,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Health check handler
pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}
