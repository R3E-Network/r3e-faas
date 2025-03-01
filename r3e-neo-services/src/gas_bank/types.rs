// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::types::FeeModel;
use serde::{Deserialize, Serialize};

/// Gas bank account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankAccount {
    /// User address
    pub address: String,
    /// Gas balance
    pub balance: u64,
    /// Fee model
    pub fee_model: FeeModel,
    /// Credit limit
    pub credit_limit: u64,
    /// Used credit
    pub used_credit: u64,
    /// Last updated timestamp
    pub updated_at: u64,
    /// Status
    pub status: String,
}

/// Gas bank deposit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankDeposit {
    /// Transaction hash
    pub tx_hash: String,
    /// User address
    pub address: String,
    /// Gas amount
    pub amount: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Status
    pub status: String,
}

/// Gas bank withdrawal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankWithdrawal {
    /// Transaction hash
    pub tx_hash: String,
    /// User address
    pub address: String,
    /// Gas amount
    pub amount: u64,
    /// Fee amount
    pub fee: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Status
    pub status: String,
}

/// Gas bank transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankTransaction {
    /// Transaction hash
    pub tx_hash: String,
    /// User address
    pub address: String,
    /// Transaction type
    pub tx_type: String,
    /// Gas amount
    pub amount: u64,
    /// Fee amount
    pub fee: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Status
    pub status: String,
}
