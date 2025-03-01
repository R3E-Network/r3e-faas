// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::Error,
    service::EndpointService,
    types::{
        BlockchainType, MessageSigningRequest, MessageSigningResponse, SignatureCurve,
        WalletConnectionRequest, WalletConnectionResponse,
    },
    utils::verify_signature,
};

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (wallet address)
    sub: String,

    /// Blockchain type
    blockchain_type: String,

    /// Connection ID
    connection_id: String,

    /// Issued at
    iat: u64,

    /// Expiration
    exp: u64,
}

/// Connect wallet handler
pub async fn connect(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<WalletConnectionRequest>,
) -> Result<Json<WalletConnectionResponse>, Error> {
    // Verify the signature
    let is_valid = verify_signature(
        &request.blockchain_type,
        &request.signature_curve,
        &request.address,
        &request.message,
        &request.signature,
    )?;

    if !is_valid {
        return Err(Error::Authentication("Invalid signature".to_string()));
    }

    // Generate a connection ID
    let connection_id = Uuid::new_v4().to_string();

    // Create JWT claims
    let now = Utc::now().timestamp() as u64;
    let claims = Claims {
        sub: request.address.clone(),
        blockchain_type: format!("{:?}", request.blockchain_type).to_lowercase(),
        connection_id: connection_id.clone(),
        iat: now,
        exp: now + service.config.jwt_expiration,
    };

    // Create JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(service.config.jwt_secret.as_bytes()),
    )
    .map_err(|e| Error::Internal(format!("Failed to create JWT token: {}", e)))?;

    // Create response
    let response = WalletConnectionResponse {
        connection_id,
        blockchain_type: request.blockchain_type,
        address: request.address,
        token,
        expires_at: claims.exp,
    };

    Ok(Json(response))
}

/// Sign message handler
pub async fn sign_message(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<MessageSigningRequest>,
) -> Result<Json<MessageSigningResponse>, Error> {
    // In a real implementation, this would create a message signing request
    // and return a request ID that the client can use to check the status

    // For this example, we'll return a mock response
    let request_id = Uuid::new_v4().to_string();
    let message_hash = format!("0x{}", hex::encode([0u8; 32]));

    let response = MessageSigningResponse {
        request_id,
        message_hash,
        status: "pending".to_string(),
        timestamp: Utc::now().timestamp() as u64,
    };

    Ok(Json(response))
}

/// Verify signature handler
pub async fn verify_signature(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<VerifySignatureRequest>,
) -> Result<Json<VerifySignatureResponse>, Error> {
    // Verify the signature
    let is_valid = verify_signature(
        &request.blockchain_type,
        &request.signature_curve,
        &request.address,
        &request.message,
        &request.signature,
    )?;

    let response = VerifySignatureResponse {
        is_valid,
        timestamp: Utc::now().timestamp() as u64,
    };

    Ok(Json(response))
}

/// Verify signature request
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignatureRequest {
    /// Blockchain type
    pub blockchain_type: BlockchainType,

    /// Signature curve
    pub signature_curve: SignatureCurve,

    /// Address
    pub address: String,

    /// Message
    pub message: String,

    /// Signature
    pub signature: String,
}

/// Verify signature response
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignatureResponse {
    /// Is valid
    pub is_valid: bool,

    /// Timestamp
    pub timestamp: u64,
}
