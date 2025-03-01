// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod config;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod service;
pub mod types;
pub mod utils;

use std::sync::Arc;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::middleware::audit::AuditLogLayer;
use crate::middleware::rate_limit::RateLimitLayer;
use crate::routes::create_router;
use crate::service::EndpointService;

/// Create the API application
pub async fn create_app(config: Config) -> Result<Router, error::Error> {
    // Create the endpoint service
    let service = Arc::new(EndpointService::new(config.clone()).await?);

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true);

    // Create middleware layers
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(AuditLogLayer::new())
        .layer(RateLimitLayer::new(config.rate_limit_requests_per_minute));

    // Create the router with middleware
    let router = create_router(service).layer(middleware);

    Ok(router)
}
