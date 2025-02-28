// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::Error,
    service::EndpointService,
    types::{MetaTransactionRequest, MetaTransactionResponse},
    utils::verify_jwt_token,
};

/// Submit meta transaction handler
pub async fn submit(
    State(service): State<Arc<EndpointService>>,
    Json(request): Json<MetaTransactionRequest>,
) -> Result<Json<MetaTransactionResponse>, Error> {
    // Convert to r3e-neo-services MetaTxRequest
    let meta_tx_request = r3e_neo_services::meta_tx::types::MetaTxRequest {
        tx_data: request.tx_data.clone(),
        sender: request.sender.clone(),
        signature: request.signature.clone(),
        nonce: request.nonce,
        deadline: request.deadline,
        blockchain_type: match request.blockchain_type {
            crate::types::BlockchainType::NeoN3 => r3e_neo_services::meta_tx::types::BlockchainType::NeoN3,
            crate::types::BlockchainType::Ethereum => r3e_neo_services::meta_tx::types::BlockchainType::Ethereum,
        },
        target_contract: request.target_contract.clone(),
        signature_curve: match request.signature_curve {
            crate::types::SignatureCurve::Secp256r1 => r3e_neo_services::meta_tx::types::SignatureCurve::Secp256r1,
            crate::types::SignatureCurve::Secp256k1 => r3e_neo_services::meta_tx::types::SignatureCurve::Secp256k1,
        },
        fee_model: r3e_neo_services::types::FeeModel::Percentage(1.0),
        fee_amount: 0,
        timestamp: request.timestamp,
    };
    
    // Submit the meta transaction
    let response = service.meta_tx_service.submit(meta_tx_request).await
        .map_err(|e| Error::Blockchain(format!("Failed to submit meta transaction: {}", e)))?;
    
    // Convert to API response
    let api_response = MetaTransactionResponse {
        request_id: response.request_id,
        original_hash: response.original_hash,
        relayed_hash: response.relayed_hash,
        status: response.status,
        error: response.error,
        timestamp: response.timestamp,
    };
    
    Ok(Json(api_response))
}

/// Get meta transaction status handler
pub async fn get_status(
    State(service): State<Arc<EndpointService>>,
    Path(id): Path<String>,
) -> Result<Json<String>, Error> {
    // Get the meta transaction status
    let status = service.meta_tx_service.get_status(&id).await
        .map_err(|e| Error::Blockchain(format!("Failed to get meta transaction status: {}", e)))?;
    
    Ok(Json(status.to_string()))
}

/// Get meta transaction handler
pub async fn get_transaction(
    State(service): State<Arc<EndpointService>>,
    Path(id): Path<String>,
) -> Result<Json<r3e_neo_services::meta_tx::types::MetaTxRecord>, Error> {
    // Get the meta transaction
    let record = service.meta_tx_service.get_transaction(&id).await
        .map_err(|e| Error::Blockchain(format!("Failed to get meta transaction: {}", e)))?
        .ok_or_else(|| Error::NotFound(format!("Meta transaction not found: {}", id)))?;
    
    Ok(Json(record))
}

/// Get next nonce handler
pub async fn get_next_nonce(
    State(service): State<Arc<EndpointService>>,
    Path(address): Path<String>,
) -> Result<Json<u64>, Error> {
    // Get the next nonce
    let nonce = service.meta_tx_service.get_next_nonce(&address).await
        .map_err(|e| Error::Blockchain(format!("Failed to get next nonce: {}", e)))?;
    
    Ok(Json(nonce))
}
