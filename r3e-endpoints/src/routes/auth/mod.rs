// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Error, service::EndpointService, utils::generate_jwt_token};

/// Login request
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,

    /// Password
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    /// User ID
    pub user_id: String,

    /// Username
    pub username: String,

    /// Token
    pub token: String,

    /// Token expiration
    pub expires_at: u64,
}

/// Register request
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// Username
    pub username: String,

    /// Password
    pub password: String,

    /// Email
    pub email: String,
}

/// Register response
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    /// User ID
    pub user_id: String,

    /// Username
    pub username: String,

    /// Token
    pub token: String,

    /// Token expiration
    pub expires_at: u64,
}

/// Refresh request
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    /// Token
    pub token: String,
}

/// Refresh response
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    /// Token
    pub token: String,

    /// Token expiration
    pub expires_at: u64,
}

/// Login handler
pub async fn login(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    // In a real implementation, this would verify the username and password
    // against a database and return a JWT token if valid

    // For this example, we'll return a mock response
    let user_id = Uuid::new_v4().to_string();
    let connection_id = Uuid::new_v4().to_string();

    // Generate JWT token
    let token = generate_jwt_token(
        &user_id,
        &crate::types::BlockchainType::NeoN3,
        &connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    let response = LoginResponse {
        user_id,
        username: request.username,
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    Ok(Json(response))
}

/// Register handler
pub async fn register(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, Error> {
    // In a real implementation, this would create a new user in the database
    // and return a JWT token

    // For this example, we'll return a mock response
    let user_id = Uuid::new_v4().to_string();
    let connection_id = Uuid::new_v4().to_string();

    // Generate JWT token
    let token = generate_jwt_token(
        &user_id,
        &crate::types::BlockchainType::NeoN3,
        &connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    let response = RegisterResponse {
        user_id,
        username: request.username,
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    Ok(Json(response))
}

/// Refresh handler
pub async fn refresh(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, Error> {
    // In a real implementation, this would verify the token and return a new token
    // if valid

    // For this example, we'll return a mock response
    let claims = crate::utils::verify_jwt_token(&request.token, &service.config.jwt_secret)?;

    // Generate JWT token
    let token = generate_jwt_token(
        &claims.sub,
        &crate::types::BlockchainType::NeoN3,
        &claims.connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    let response = RefreshResponse {
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    Ok(Json(response))
}
