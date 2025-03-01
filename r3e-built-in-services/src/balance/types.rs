// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};

/// User balance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalance {
    /// User ID
    pub user_id: String,

    /// NEO balance
    pub neo_balance: u64,

    /// GAS balance
    pub gas_balance: u64,

    /// Last updated timestamp
    pub updated_at: u64,
}

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Deposit
    Deposit,

    /// Withdrawal
    Withdrawal,

    /// Function execution fee
    FunctionExecution,
}

/// Balance transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceTransaction {
    /// Transaction ID
    pub id: String,

    /// User ID
    pub user_id: String,

    /// Transaction type
    pub transaction_type: TransactionType,

    /// Asset type (NEO or GAS)
    pub asset_type: String,

    /// Amount
    pub amount: u64,

    /// Transaction hash (for blockchain transactions)
    pub tx_hash: Option<String>,

    /// Timestamp
    pub timestamp: u64,
}
