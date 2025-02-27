// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Status
    pub status: String,
    
    /// Version
    pub version: String,
}

/// Health check handler
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Health check routes
pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
