// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::auth::key_rotation::{ApiKey, KeyRotationService};
use crate::error::Error;
use crate::service::EndpointService;

/// Create API key request
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    /// User ID
    pub user_id: String,
}

/// Create API key response
#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    /// API key ID
    pub key_id: String,

    /// API key value
    pub key_value: String,

    /// API key metadata
    pub key_metadata: ApiKey,
}

/// Rotate API key request
#[derive(Debug, Deserialize)]
pub struct RotateApiKeyRequest {
    /// User ID
    pub user_id: String,
}

/// Rotate API key response
#[derive(Debug, Serialize)]
pub struct RotateApiKeyResponse {
    /// New API key ID
    pub key_id: String,

    /// New API key value
    pub key_value: String,

    /// New API key metadata
    pub key_metadata: ApiKey,
}

/// Revoke API key request
#[derive(Debug, Deserialize)]
pub struct RevokeApiKeyRequest {
    /// User ID
    pub user_id: String,
}

/// List API keys response
#[derive(Debug, Serialize)]
pub struct ListApiKeysResponse {
    /// API keys
    pub keys: Vec<ApiKey>,
}

/// Create a new API key
pub async fn create_api_key(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, Error> {
    // Get the key rotation service
    let key_rotation_service = service.key_rotation_service();

    // Create a new API key
    let (key_value, key_metadata) = key_rotation_service.create_key(&request.user_id).await?;

    // Return the API key
    Ok(Json(CreateApiKeyResponse {
        key_id: key_metadata.id.clone(),
        key_value,
        key_metadata,
    }))
}

/// Rotate an API key
pub async fn rotate_api_key(
    State(service): State<Arc<EndpointService>>,
    Path(key_id): Path<String>,
    Json(request): Json<RotateApiKeyRequest>,
) -> Result<Json<RotateApiKeyResponse>, Error> {
    // Get the key rotation service
    let key_rotation_service = service.key_rotation_service();

    // Rotate the API key
    let (key_value, key_metadata) = key_rotation_service
        .rotate_key(&key_id, &request.user_id)
        .await?;

    // Return the new API key
    Ok(Json(RotateApiKeyResponse {
        key_id: key_metadata.id.clone(),
        key_value,
        key_metadata,
    }))
}

/// Revoke an API key
pub async fn revoke_api_key(
    State(service): State<Arc<EndpointService>>,
    Path(key_id): Path<String>,
    Json(request): Json<RevokeApiKeyRequest>,
) -> Result<StatusCode, Error> {
    // Get the key rotation service
    let key_rotation_service = service.key_rotation_service();

    // Revoke the API key
    key_rotation_service
        .revoke_key(&key_id, &request.user_id)
        .await?;

    // Return success
    Ok(StatusCode::NO_CONTENT)
}

/// List API keys for a user
pub async fn list_api_keys(
    State(service): State<Arc<EndpointService>>,
    Path(user_id): Path<String>,
) -> Result<Json<ListApiKeysResponse>, Error> {
    // Get the key rotation service
    let key_rotation_service = service.key_rotation_service();

    // Get the user's API keys
    let keys = key_rotation_service.get_user_keys(&user_id);

    // Return the API keys
    Ok(Json(ListApiKeysResponse { keys }))
}
