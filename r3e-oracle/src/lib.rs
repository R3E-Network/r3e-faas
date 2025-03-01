// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

pub mod auth;
pub mod provider;
pub mod service;
pub mod types;

/// Oracle service error types
#[derive(Debug, Error)]
pub enum OracleError {
    #[error("authentication error: {0}")]
    Authentication(String),

    #[error("authorization error: {0}")]
    Authorization(String),

    #[error("rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("provider error: {0}")]
    Provider(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("timeout error: {0}")]
    Timeout(String),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Oracle request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleRequestStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Oracle request type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleRequestType {
    Price,
    Random,
    Weather,
    Sports,
    Custom,
}

/// Oracle request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRequest {
    /// Unique request ID
    pub id: String,

    /// Request type
    pub request_type: OracleRequestType,

    /// Request data (JSON string)
    pub data: String,

    /// Callback URL (optional)
    pub callback_url: Option<String>,

    /// Requester ID
    pub requester_id: String,

    /// Request timestamp
    pub timestamp: u64,

    /// Request status
    pub status: OracleRequestStatus,
}

/// Oracle response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResponse {
    /// Request ID
    pub request_id: String,

    /// Response data (JSON string)
    pub data: String,

    /// Response status code
    pub status_code: u32,

    /// Response timestamp
    pub timestamp: u64,

    /// Error message (if any)
    pub error: Option<String>,
}

/// Oracle service trait
#[async_trait::async_trait]
pub trait OracleService: Send + Sync {
    /// Submit a new oracle request
    async fn submit_request(&self, request: OracleRequest) -> Result<String, OracleError>;

    /// Get the status of an oracle request
    async fn get_request_status(
        &self,
        request_id: &str,
    ) -> Result<OracleRequestStatus, OracleError>;

    /// Get the response for a completed oracle request
    async fn get_response(&self, request_id: &str) -> Result<OracleResponse, OracleError>;

    /// Cancel an oracle request
    async fn cancel_request(&self, request_id: &str) -> Result<bool, OracleError>;
}

/// Oracle provider trait
#[async_trait::async_trait]
pub trait OracleProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Get the provider description
    fn description(&self) -> &str;

    /// Get the supported request types
    fn supported_types(&self) -> Vec<OracleRequestType>;

    /// Process an oracle request
    async fn process_request(&self, request: &OracleRequest)
        -> Result<OracleResponse, OracleError>;
}
