// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod wallet;
pub use wallet::*;

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
    // Log the login attempt with audit trail
    tracing::info!(
        target: "audit",
        "AUDIT: Login attempt - username={}, timestamp={}",
        request.username,
        chrono::Utc::now().timestamp()
    );

    // Validate credentials against the database
    let user = service
        .db_client
        .find_user_by_username(&request.username)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    // Check if user exists
    let user = match user {
        Some(user) => user,
        None => {
            tracing::warn!(
                target: "audit",
                "AUDIT: Login failed - username={}, reason=user_not_found, timestamp={}",
                request.username,
                chrono::Utc::now().timestamp()
            );
            return Err(Error::Authentication("Invalid username or password".into()));
        }
    };

    // Verify password
    let is_valid = verify_password(&request.password, &user.password_hash)
        .map_err(|e| Error::Internal(format!("Password verification error: {}", e)))?;

    if !is_valid {
        tracing::warn!(
            target: "audit",
            "AUDIT: Login failed - username={}, reason=invalid_password, timestamp={}",
            request.username,
            chrono::Utc::now().timestamp()
        );
        return Err(Error::Authentication("Invalid username or password".into()));
    }

    // Create a new session
    let connection_id = Uuid::new_v4().to_string();

    // Generate JWT token
    let token = generate_jwt_token(
        &user.id,
        &user.blockchain_type,
        &connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    // Store the session in the database
    service
        .db_client
        .create_session(&user.id, &connection_id, &token)
        .await
        .map_err(|e| Error::Internal(format!("Failed to create session: {}", e)))?;

    let response = LoginResponse {
        user_id: user.id,
        username: user.username,
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    tracing::info!(
        target: "audit",
        "AUDIT: Login successful - username={}, user_id={}, timestamp={}",
        request.username,
        response.user_id,
        chrono::Utc::now().timestamp()
    );
    Ok(Json(response))
}

/// Register handler
pub async fn register(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, Error> {
    // Log the registration attempt with audit trail
    tracing::info!(
        target: "audit",
        "AUDIT: Registration attempt - username={}, timestamp={}",
        request.username,
        chrono::Utc::now().timestamp()
    );
    // Validate input
    if request.username.len() < 3 || request.username.len() > 30 {
        return Err(Error::Validation(
            "Username must be between 3 and 30 characters".into(),
        ));
    }

    if request.password.len() < 8 {
        return Err(Error::Validation(
            "Password must be at least 8 characters".into(),
        ));
    }

    // Check if the username is already taken
    let existing_user = service
        .db_client
        .find_user_by_username(&request.username)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    if existing_user.is_some() {
        return Err(Error::Validation("Username already taken".into()));
    }

    // Check if the email is already in use
    let existing_email = service
        .db_client
        .find_user_by_email(&request.email)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    if existing_email.is_some() {
        return Err(Error::Validation("Email already in use".into()));
    }

    // Hash the password
    let password_hash = hash_password(&request.password)
        .map_err(|e| Error::Internal(format!("Password hashing error: {}", e)))?;

    // Create the user
    let user_id = Uuid::new_v4().to_string();
    let connection_id = Uuid::new_v4().to_string();

    // Set default blockchain type
    let blockchain_type = crate::types::BlockchainType::NeoN3;

    // Create user in database
    service
        .db_client
        .create_user(
            &user_id,
            &request.username,
            &password_hash,
            &request.email,
            &blockchain_type,
        )
        .await
        .map_err(|e| Error::Internal(format!("Failed to create user: {}", e)))?;

    // Generate JWT token
    let token = generate_jwt_token(
        &user_id,
        &blockchain_type,
        &connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    // Store the session
    service
        .db_client
        .create_session(&user_id, &connection_id, &token)
        .await
        .map_err(|e| Error::Internal(format!("Failed to create session: {}", e)))?;

    let response = RegisterResponse {
        user_id,
        username: request.username,
        token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    tracing::info!(
        target: "audit",
        "AUDIT: Registration successful - username={}, user_id={}, timestamp={}",
        response.username,
        response.user_id,
        chrono::Utc::now().timestamp()
    );
    Ok(Json(response))
}

/// Refresh handler
pub async fn refresh(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, Error> {
    // Log the token refresh attempt with audit trail
    tracing::info!(
        target: "audit",
        "AUDIT: Token refresh attempt - timestamp={}",
        chrono::Utc::now().timestamp()
    );
    // Verify the token
    let claims = match crate::utils::verify_jwt_token(&request.token, &service.config.jwt_secret) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::warn!(
                target: "audit",
                "AUDIT: Token refresh failed - reason=validation_error, error={}, timestamp={}",
                e,
                chrono::Utc::now().timestamp()
            );
            return Err(Error::Authentication("Invalid token".into()));
        }
    };

    // Check if the token is in the database
    let session = service
        .db_client
        .find_session_by_token(&request.token)
        .await
        .map_err(|e| Error::Internal(format!("Database error: {}", e)))?;

    let session = match session {
        Some(session) => session,
        None => {
            tracing::warn!(
                target: "audit",
                "AUDIT: Token refresh failed - reason=token_not_found, timestamp={}",
                chrono::Utc::now().timestamp()
            );
            return Err(Error::Authentication("Invalid token".into()));
        }
    };

    // Check if the session is still valid
    if session.is_expired() {
        tracing::warn!(
            target: "audit",
            "AUDIT: Token refresh failed - reason=session_expired, user_id={}, timestamp={}",
            claims.sub,
            chrono::Utc::now().timestamp()
        );
        return Err(Error::Authentication("Token expired".into()));
    }

    // Generate a new token
    let new_token = generate_jwt_token(
        &claims.sub,
        &claims.blockchain_type,
        &claims.connection_id,
        &service.config.jwt_secret,
        service.config.jwt_expiration,
    )?;

    // Update the session in the database
    service
        .db_client
        .update_session(&session.id, &new_token)
        .await
        .map_err(|e| Error::Internal(format!("Failed to update session: {}", e)))?;

    let response = RefreshResponse {
        token: new_token,
        expires_at: Utc::now().timestamp() as u64 + service.config.jwt_expiration,
    };

    tracing::info!(
        target: "audit",
        "AUDIT: Token refresh successful - user_id={}, timestamp={}",
        claims.sub,
        chrono::Utc::now().timestamp()
    );
    Ok(Json(response))
}

/// Helper function to hash a password
fn hash_password(password: &str) -> Result<String, argon2::Error> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
        Argon2,
    };

    // Generate a salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// Helper function to verify a password
fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}
