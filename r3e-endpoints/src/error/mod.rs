// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// API error
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Blockchain error
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,

    /// Error code
    pub code: String,

    /// Error details
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            Error::Configuration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CONFIGURATION_ERROR"),
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            Error::Authentication(_) => (StatusCode::UNAUTHORIZED, "AUTHENTICATION_ERROR"),
            Error::Authorization(_) => (StatusCode::FORBIDDEN, "AUTHORIZATION_ERROR"),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Error::Network(_) => (StatusCode::BAD_GATEWAY, "NETWORK_ERROR"),
            Error::Blockchain(_) => (StatusCode::BAD_GATEWAY, "BLOCKCHAIN_ERROR"),
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = ErrorResponse {
            message: self.to_string(),
            code: code.to_string(),
            details: None,
        };

        let body = serde_json::to_string(&body).unwrap_or_else(|_| {
            r#"{"message":"Failed to serialize error response","code":"INTERNAL_ERROR"}"#
                .to_string()
        });

        (status, body).into_response()
    }
}
