// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Chain error: {0}")]
    Chain(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
}

/// Supported blockchain networks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    /// Neo N3
    NeoN3,

    /// Ethereum
    Ethereum,

    /// Binance Smart Chain
    BinanceSmartChain,

    /// Polygon
    Polygon,

    /// Solana
    Solana,
}

impl std::fmt::Display for BlockchainNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainNetwork::NeoN3 => write!(f, "neo_n3"),
            BlockchainNetwork::Ethereum => write!(f, "ethereum"),
            BlockchainNetwork::BinanceSmartChain => write!(f, "binance_smart_chain"),
            BlockchainNetwork::Polygon => write!(f, "polygon"),
            BlockchainNetwork::Solana => write!(f, "solana"),
        }
    }
}

/// Bridge transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeTransactionStatus {
    /// Transaction is pending
    Pending,

    /// Transaction is in progress
    InProgress,

    /// Transaction is completed
    Completed,

    /// Transaction failed
    Failed,
}

/// Bridge transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeTransactionType {
    /// Token transfer
    TokenTransfer,

    /// Asset wrapping
    AssetWrapping,

    /// Message passing
    MessagePassing,
}

/// Bridge transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    /// Transaction ID
    pub id: String,

    /// Transaction type
    pub transaction_type: BridgeTransactionType,

    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Source transaction hash
    pub source_tx_hash: Option<String>,

    /// Destination transaction hash
    pub destination_tx_hash: Option<String>,

    /// Transaction status
    pub status: BridgeTransactionStatus,

    /// Transaction data
    pub data: serde_json::Value,

    /// Error message (if any)
    pub error: Option<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last updated timestamp
    pub updated_at: u64,
}

/// Token bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBridge {
    /// Bridge ID
    pub id: String,

    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Source token address
    pub source_token: String,

    /// Destination token address
    pub destination_token: String,

    /// Bridge fee percentage
    pub fee_percentage: f64,

    /// Minimum transfer amount
    pub min_amount: u64,

    /// Maximum transfer amount
    pub max_amount: Option<u64>,

    /// Is the bridge enabled?
    pub enabled: bool,
}

/// Message bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBridge {
    /// Bridge ID
    pub id: String,

    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Source contract address
    pub source_contract: String,

    /// Destination contract address
    pub destination_contract: String,

    /// Message fee
    pub fee: u64,

    /// Maximum message size in bytes
    pub max_message_size: u32,

    /// Is the bridge enabled?
    pub enabled: bool,
}

/// Asset wrapper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetWrapper {
    /// Wrapper ID
    pub id: String,

    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Source asset address
    pub source_asset: String,

    /// Wrapped asset address
    pub wrapped_asset: String,

    /// Wrapping fee percentage
    pub fee_percentage: f64,

    /// Minimum wrap amount
    pub min_amount: u64,

    /// Maximum wrap amount
    pub max_amount: Option<u64>,

    /// Is the wrapper enabled?
    pub enabled: bool,
}

/// Token transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransferRequest {
    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Token address on source chain
    pub token_address: String,

    /// Amount to transfer
    pub amount: u64,

    /// Recipient address on destination chain
    pub recipient: String,
}

/// Asset wrapping request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetWrappingRequest {
    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Asset address on source chain
    pub asset_address: String,

    /// Amount to wrap
    pub amount: u64,

    /// Recipient address on destination chain
    pub recipient: String,
}

/// Message passing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePassingRequest {
    /// Source blockchain
    pub from_chain: BlockchainNetwork,

    /// Destination blockchain
    pub to_chain: BlockchainNetwork,

    /// Source contract address
    pub source_contract: String,

    /// Destination contract address
    pub destination_contract: String,

    /// Message data
    pub message: Vec<u8>,
}
