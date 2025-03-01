// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

mod auth;
mod health;
mod meta_tx;
mod services;
mod wallet;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::service::EndpointService;

/// Create the router
pub fn create_router(service: Arc<EndpointService>) -> Router {
    // Create the router
    Router::new()
        // Health routes
        .route("/health", get(health::health_check))
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh))
        // Wallet routes
        .route("/wallet/connect", post(wallet::connect))
        .route("/wallet/sign", post(wallet::sign_message))
        .route("/wallet/verify", post(wallet::verify_signature))
        // Meta transaction routes
        .route("/meta-tx/submit", post(meta_tx::submit))
        .route("/meta-tx/status/:id", get(meta_tx::get_status))
        .route("/meta-tx/transaction/:id", get(meta_tx::get_transaction))
        .route("/meta-tx/nonce/:address", get(meta_tx::get_next_nonce))
        // Service routes
        .route("/services", get(services::list_services))
        .route("/services/:id", get(services::get_service))
        .route("/services/:id/invoke", post(services::invoke_service))
        // Add the service state
        .with_state(service)
}

pub fn auth_routes() -> Router<Arc<EndpointService>> {
    Router::new()
        // Wallet authentication routes
        .route("/wallet/connect", post(auth::connect_wallet))
        .route("/wallet/authenticate", post(auth::authenticate_wallet))
        // Token refresh route
        .route("/refresh", post(auth::refresh))
}
