// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::rocksdb::RocksDBStore;
use std::path::Path;
use std::sync::Arc;

use crate::balance::storage::BalanceStorage;
use crate::balance::types::{BalanceTransaction, UserBalance};

/// RocksDB implementation of BalanceStorage
pub struct RocksDBBalanceStorage {
    db: Arc<RocksDBStore>,
    balances_cf: String,
    transactions_cf: String,
}

impl RocksDBBalanceStorage {
    /// Create a new RocksDB balance storage
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, String> {
        let db = RocksDBStore::new(db_path)
            .map_err(|e| format!("Failed to create RocksDB store: {}", e))?;

        let balances_cf = "balances".to_string();
        let transactions_cf = "transactions".to_string();

        Ok(Self {
            db: Arc::new(db),
            balances_cf,
            transactions_cf,
        })
    }
}

#[async_trait]
impl BalanceStorage for RocksDBBalanceStorage {
    async fn get_balance(&self, user_id: &str) -> Result<Option<UserBalance>, String> {
        let key = user_id.as_bytes();

        match self.db.get(&self.balances_cf, key) {
            Ok(value) => match serde_json::from_slice::<UserBalance>(&value) {
                Ok(balance) => Ok(Some(balance)),
                Err(e) => Err(format!("Failed to deserialize balance: {}", e)),
            },
            Err(r3e_store::GetError::NoSuchKey) => Ok(None),
            Err(e) => Err(format!("Failed to get balance: {}", e)),
        }
    }

    async fn update_balance(&self, balance: UserBalance) -> Result<(), String> {
        let key = balance.user_id.as_bytes();
        let value = serde_json::to_vec(&balance)
            .map_err(|e| format!("Failed to serialize balance: {}", e))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.balances_cf, input)
            .map_err(|e| format!("Failed to update balance: {}", e))
    }

    async fn add_transaction(&self, transaction: BalanceTransaction) -> Result<(), String> {
        let key = transaction.id.as_bytes();
        let value = serde_json::to_vec(&transaction)
            .map_err(|e| format!("Failed to serialize transaction: {}", e))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: true,
        };

        self.db
            .put(&self.transactions_cf, input)
            .map_err(|e| format!("Failed to add transaction: {}", e))
    }

    async fn get_transactions(&self, user_id: &str) -> Result<Vec<BalanceTransaction>, String> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.transactions_cf, input)
            .map_err(|e| format!("Failed to scan transactions: {}", e))?;

        let mut transactions = Vec::new();

        for (_, value) in output.kvs {
            let transaction = serde_json::from_slice::<BalanceTransaction>(&value)
                .map_err(|e| format!("Failed to deserialize transaction: {}", e))?;

            if transaction.user_id == user_id {
                transactions.push(transaction);
            }
        }

        Ok(transactions)
    }

    async fn get_transaction(&self, id: &str) -> Result<Option<BalanceTransaction>, String> {
        let key = id.as_bytes();

        match self.db.get(&self.transactions_cf, key) {
            Ok(value) => match serde_json::from_slice::<BalanceTransaction>(&value) {
                Ok(transaction) => Ok(Some(transaction)),
                Err(e) => Err(format!("Failed to deserialize transaction: {}", e)),
            },
            Err(r3e_store::GetError::NoSuchKey) => Ok(None),
            Err(e) => Err(format!("Failed to get transaction: {}", e)),
        }
    }
}
