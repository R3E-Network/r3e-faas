// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

pub mod auth;
pub mod config;
pub mod error;
pub mod graphql;
pub mod models;
pub mod routes;
pub mod service;
pub mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::config::Config;
use crate::error::ApiError;
use crate::graphql::schema::create_schema;
use crate::routes::{
    auth::auth_routes, functions::function_routes, graphql::graphql_routes, health::health_routes,
    services::service_routes,
};
use crate::service::ApiService;

/// Start the API server
pub async fn start_server(config: Config) -> Result<(), ApiError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the API service
    let api_service = Arc::new(ApiService::new(config.clone()).await?);

    // Create the GraphQL schema
    let schema = create_schema(Arc::clone(&api_service));

    // Create the router
    let app = Router::new()
        .merge(health_routes())
        .merge(auth_routes(Arc::clone(&api_service)))
        .merge(function_routes(Arc::clone(&api_service)))
        .merge(service_routes(Arc::clone(&api_service)))
        .merge(graphql_routes(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(api_service);

    // Start the server
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", config.port))
        .await
        .map_err(|e| ApiError::Server(format!("Failed to bind to port {}: {}", config.port, e)))?;

    tracing::info!("API server listening on http://0.0.0.0:{}", config.port);

    axum::serve(listener, app)
        .await
        .map_err(|e| ApiError::Server(format!("Server error: {}", e)))?;

    Ok(())
}
