// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EIP-712 Domain Separator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP712Domain {
    /// The name of the signing domain
    pub name: String,
    /// The current version of the signing domain
    pub version: String,
    /// The EIP-155 chain ID
    pub chain_id: u64,
    /// The address of the contract that will verify the signature
    pub verifying_contract: String,
    /// A disambiguating salt for the protocol
    pub salt: Option<String>,
}

/// EIP-712 Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP712Type {
    /// The name of the field
    pub name: String,
    /// The type of the field
    pub r#type: String,
}

/// EIP-712 Typed Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP712TypedData {
    /// The domain separator
    pub domain: EIP712Domain,
    /// The primary type
    pub primary_type: String,
    /// The types
    pub types: HashMap<String, Vec<EIP712Type>>,
    /// The message
    pub message: serde_json::Value,
}

/// Meta transaction message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxMessage {
    pub chain_id: u64,
    pub function: String,
    pub from: String,
    pub to: String,
    pub data: String,
    pub nonce: u64,
    pub deadline: u64,
    pub fee_model: String,
    pub fee_amount: u64,
}

impl MetaTxMessage {
    /// Create a MetaTxMessage from a MetaTxRequest
    pub fn from_request(request: crate::meta_tx::types::MetaTxRequest) -> Self {
        Self {
            chain_id: request.chain_id.unwrap_or(1),
            function: request.function.unwrap_or_default(),
            from: request.sender.clone(),
            to: request.target_address.clone(),
            data: request.tx_data.clone(),
            nonce: request.nonce,
            deadline: request.deadline,
            fee_model: request.fee_model.unwrap_or_default(),
            fee_amount: request.fee_amount,
        }
    }
}
