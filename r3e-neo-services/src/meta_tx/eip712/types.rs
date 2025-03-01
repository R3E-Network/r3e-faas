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
    pub message: HashMap<String, serde_json::Value>,
}

/// Meta Transaction EIP-712 Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTxMessage {
    /// The sender address
    pub from: String,
    /// The target contract address
    pub to: String,
    /// The transaction data
    pub data: String,
    /// The nonce
    pub nonce: u64,
    /// The deadline timestamp
    pub deadline: u64,
    /// The fee model
    pub fee_model: String,
    /// The fee amount
    pub fee_amount: u64,
}

impl MetaTxMessage {
    /// Create a new MetaTxMessage from a MetaTxRequest
    pub fn from_request(request: &crate::meta_tx::types::MetaTxRequest) -> Self {
        let target_contract = request.target_contract.clone().unwrap_or_default();

        Self {
            from: request.sender.clone(),
            to: target_contract,
            data: request.tx_data.clone(),
            nonce: request.nonce,
            deadline: request.deadline,
            fee_model: request.fee_model.clone(),
            fee_amount: request.fee_amount,
        }
    }
}
