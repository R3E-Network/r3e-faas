// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// TODO: The internal modules from Neo3 are no longer accessible, we need to update
// to use the proper public API when available
use serde::{Deserialize, Serialize};

/// Neo Address type (placeholder for now)
pub type Address = String;

/// Neo ScriptHash type (placeholder for now)
pub type ScriptHash = String;

/// Configuration for Neo N3 services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoServicesConfig {
    /// RPC endpoint URL
    pub rpc_url: String,
    /// Network type (mainnet, testnet, private)
    pub network: String,
    /// Gas bank wallet address
    pub gas_bank_address: Option<String>,
    /// Gas bank private key (encrypted)
    pub gas_bank_private_key: Option<String>,
    /// Meta transaction relayer address
    pub meta_tx_relayer_address: Option<String>,
    /// Meta transaction relayer private key (encrypted)
    pub meta_tx_relayer_private_key: Option<String>,
    /// Abstract account factory contract hash
    pub abstract_account_factory: Option<String>,
}

/// Transaction fee model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeModel {
    /// Fixed fee amount
    Fixed(u64),
    /// Percentage of transaction value
    Percentage(f64),
    /// Dynamic fee based on network congestion
    Dynamic,
    /// Free (subsidized by the platform)
    Free,
}

/// Gas bank transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankTransaction {
    /// Transaction hash
    pub hash: String,
    /// Sender address
    pub sender: String,
    /// Recipient address
    pub recipient: String,
    /// Gas amount
    pub gas_amount: u64,
    /// Fee model
    pub fee_model: FeeModel,
    /// Fee amount
    pub fee_amount: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Status
    pub status: String,
}

/// Meta transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTransaction {
    /// Original transaction hash
    pub original_hash: String,
    /// Relayed transaction hash
    pub relayed_hash: Option<String>,
    /// Sender address
    pub sender: String,
    /// Signature
    pub signature: String,
    /// Transaction data
    pub data: String,
    /// Gas amount
    pub gas_amount: u64,
    /// Fee model
    pub fee_model: FeeModel,
    /// Fee amount
    pub fee_amount: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Status
    pub status: String,
}

/// Abstract account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractAccount {
    /// Account address
    pub address: String,
    /// Account owner
    pub owner: String,
    /// Account controllers
    pub controllers: Vec<String>,
    /// Recovery addresses
    pub recovery_addresses: Vec<String>,
    /// Account contract hash
    pub contract_hash: String,
    /// Account creation timestamp
    pub created_at: u64,
    /// Account status
    pub status: String,
}
