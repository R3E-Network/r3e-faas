// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use crate::balance::types::{UserBalance, BalanceTransaction};

/// Balance storage trait
#[async_trait]
pub trait BalanceStorage: Send + Sync {
    /// Get user balance
    async fn get_balance(&self, user_id: &str) -> Result<Option<UserBalance>, String>;
    
    /// Update user balance
    async fn update_balance(&self, balance: UserBalance) -> Result<(), String>;
    
    /// Add transaction
    async fn add_transaction(&self, transaction: BalanceTransaction) -> Result<(), String>;
    
    /// Get user transactions
    async fn get_transactions(&self, user_id: &str) -> Result<Vec<BalanceTransaction>, String>;
    
    /// Get transaction by ID
    async fn get_transaction(&self, id: &str) -> Result<Option<BalanceTransaction>, String>;
}

/// Memory-based implementation of BalanceStorage
pub struct MemoryBalanceStorage {
    balances: tokio::sync::Mutex<std::collections::HashMap<String, UserBalance>>,
    transactions: tokio::sync::Mutex<std::collections::HashMap<String, BalanceTransaction>>,
}

impl MemoryBalanceStorage {
    /// Create a new memory-based balance storage
    pub fn new() -> Self {
        Self {
            balances: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            transactions: tokio::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl BalanceStorage for MemoryBalanceStorage {
    async fn get_balance(&self, user_id: &str) -> Result<Option<UserBalance>, String> {
        let balances = self.balances.lock().await;
        Ok(balances.get(user_id).cloned())
    }
    
    async fn update_balance(&self, balance: UserBalance) -> Result<(), String> {
        let mut balances = self.balances.lock().await;
        balances.insert(balance.user_id.clone(), balance);
        Ok(())
    }
    
    async fn add_transaction(&self, transaction: BalanceTransaction) -> Result<(), String> {
        let mut transactions = self.transactions.lock().await;
        transactions.insert(transaction.id.clone(), transaction);
        Ok(())
    }
    
    async fn get_transactions(&self, user_id: &str) -> Result<Vec<BalanceTransaction>, String> {
        let transactions = self.transactions.lock().await;
        let user_transactions = transactions
            .values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect();
        Ok(user_transactions)
    }
    
    async fn get_transaction(&self, id: &str) -> Result<Option<BalanceTransaction>, String> {
        let transactions = self.transactions.lock().await;
        Ok(transactions.get(id).cloned())
    }
}
