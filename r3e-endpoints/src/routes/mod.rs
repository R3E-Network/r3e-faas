// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

mod auth;
mod health;
mod meta_tx;
mod services;
mod wallet;

use std::sync::Arc;

use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::middleware::KeyRotationLayer;
use crate::service::EndpointService;

/// Create the router
pub fn create_router(service: Arc<EndpointService>) -> Router {
    // Create the key rotation layer
    let key_rotation_layer = KeyRotationLayer::new(service.key_rotation_service());
    
    // Create the router
    Router::new()
        // Health routes
        .route("/health", get(health::health_check))
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh))
        // API key routes
        .route("/auth/api-keys", post(auth::api_keys::create_api_key))
        .route("/auth/api-keys/:key_id", post(auth::api_keys::rotate_api_key))
        .route("/auth/api-keys/:key_id", delete(auth::api_keys::revoke_api_key))
        .route("/auth/api-keys/user/:user_id", get(auth::api_keys::list_api_keys))
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
        // Add the key rotation middleware
        .layer(key_rotation_layer)
}

pub fn auth_routes() -> Router<Arc<EndpointService>> {
    Router::new()
        // Wallet authentication routes 
        .route("/wallet/connect", post(auth::connect_wallet))
        .route("/wallet/authenticate", post(auth::authenticate_wallet))
        // Token refresh route
        .route("/refresh", post(auth::refresh))
        // API key routes
        .route("/api-keys", post(auth::api_keys::create_api_key))
        .route("/api-keys/:key_id", post(auth::api_keys::rotate_api_key))
        .route("/api-keys/:key_id", delete(auth::api_keys::revoke_api_key))
        .route("/api-keys/user/:user_id", get(auth::api_keys::list_api_keys))
}
