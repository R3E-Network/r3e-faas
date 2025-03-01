// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use crate::{
    middleware::{SecurityHeadersLayer, ValidationLayer},
    routes, 
    service::EndpointService
};

/// Create the application
pub fn create_app(service: Arc<EndpointService>) -> Router {
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Set up security headers
    let security_headers = SecurityHeadersLayer::new();
    
    // Set up validation
    let validation = ValidationLayer::new();
    
    // Build the router
    Router::new()
        // API routes
        .nest("/api", routes::api_routes())
        // User routes
        .nest("/user", routes::user_routes())
        // Service routes
        .nest("/services", routes::service_routes())
        // Auth routes
        .nest("/auth", routes::auth_routes())
        // Add middlewares
        .layer(validation)
        .layer(security_headers)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(service)
}         