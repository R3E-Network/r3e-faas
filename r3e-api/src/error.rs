// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// API error types
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("authentication error: {0}")]
    Authentication(String),

    #[error("authorization error: {0}")]
    Authorization(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("service error: {0}")]
    Service(String),

    #[error("server error: {0}")]
    Server(String),

    #[error("external service error: {0}")]
    ExternalService(String),
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Authentication(message) => (StatusCode::UNAUTHORIZED, message),
            ApiError::Authorization(message) => (StatusCode::FORBIDDEN, message),
            ApiError::Validation(message) => (StatusCode::BAD_REQUEST, message),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            ApiError::Database(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::Service(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::Server(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::ExternalService(message) => (StatusCode::BAD_GATEWAY, message),
        };

        let body = Json(ErrorResponse {
            status: status.to_string(),
            message: error_message,
        });

        (status, body).into_response()
    }
}
