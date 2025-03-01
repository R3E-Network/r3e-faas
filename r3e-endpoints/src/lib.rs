// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod config;
pub mod error;
pub mod routes;
pub mod service;
pub mod types;
pub mod utils;

use std::sync::Arc;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::Config;
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

    // Create the router
    let router = create_router(service)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    Ok(router)
}
