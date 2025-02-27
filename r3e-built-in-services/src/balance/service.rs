// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use crate::balance::storage::BalanceStorage;
use crate::balance::types::{UserBalance, BalanceTransaction, TransactionType};
use crate::gas_bank::GasBankServiceTrait;

/// Balance service trait
#[async_trait]
pub trait BalanceServiceTrait: Send + Sync {
    /// Get user balance
    async fn get_balance(&self, user_id: &str) -> Result<UserBalance, String>;
    
    /// Deposit NEO or GAS
    async fn deposit(&self, user_id: &str, asset_type: &str, amount: u64, tx_hash: &str) -> Result<BalanceTransaction, String>;
    
    /// Withdraw NEO or GAS
    async fn withdraw(&self, user_id: &str, asset_type: &str, amount: u64) -> Result<BalanceTransaction, String>;
    
    /// Charge for function execution
    async fn charge_for_execution(&self, user_id: &str, function_id: &str, gas_amount: u64) -> Result<BalanceTransaction, String>;
    
    /// Get user transactions
    async fn get_transactions(&self, user_id: &str) -> Result<Vec<BalanceTransaction>, String>;
}

/// Balance service implementation
pub struct BalanceService<S: BalanceStorage, G: GasBankServiceTrait> {
    /// Storage
    storage: Arc<S>,
    
    /// Gas bank service
    gas_bank: Arc<G>,
}

impl<S: BalanceStorage, G: GasBankServiceTrait> BalanceService<S, G> {
    /// Create a new balance service
    pub fn new(storage: Arc<S>, gas_bank: Arc<G>) -> Self {
        Self { storage, gas_bank }
    }
}

#[async_trait]
impl<S: BalanceStorage, G: GasBankServiceTrait> BalanceServiceTrait for BalanceService<S, G> {
    async fn get_balance(&self, user_id: &str) -> Result<UserBalance, String> {
        match self.storage.get_balance(user_id).await? {
            Some(balance) => Ok(balance),
            None => {
                // Create new balance if it doesn't exist
                let balance = UserBalance {
                    user_id: user_id.to_string(),
                    neo_balance: 0,
                    gas_balance: 0,
                    updated_at: chrono::Utc::now().timestamp() as u64,
                };
                self.storage.update_balance(balance.clone()).await?;
                Ok(balance)
            }
        }
    }
    
    async fn deposit(&self, user_id: &str, asset_type: &str, amount: u64, tx_hash: &str) -> Result<BalanceTransaction, String> {
        let mut balance = self.get_balance(user_id).await?;
        
        // Update balance based on asset type
        match asset_type.to_lowercase().as_str() {
            "neo" => balance.neo_balance += amount,
            "gas" => balance.gas_balance += amount,
            _ => return Err(format!("Unsupported asset type: {}", asset_type)),
        }
        
        balance.updated_at = chrono::Utc::now().timestamp() as u64;
        self.storage.update_balance(balance).await?;
        
        // Create transaction record
        let transaction = BalanceTransaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Deposit,
            asset_type: asset_type.to_string(),
            amount,
            tx_hash: Some(tx_hash.to_string()),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        self.storage.add_transaction(transaction.clone()).await?;
        
        Ok(transaction)
    }
    
    async fn withdraw(&self, user_id: &str, asset_type: &str, amount: u64) -> Result<BalanceTransaction, String> {
        let mut balance = self.get_balance(user_id).await?;
        
        // Check if user has enough balance
        match asset_type.to_lowercase().as_str() {
            "neo" => {
                if balance.neo_balance < amount {
                    return Err(format!("Insufficient NEO balance: {} < {}", balance.neo_balance, amount));
                }
                balance.neo_balance -= amount;
            },
            "gas" => {
                if balance.gas_balance < amount {
                    return Err(format!("Insufficient GAS balance: {} < {}", balance.gas_balance, amount));
                }
                balance.gas_balance -= amount;
            },
            _ => return Err(format!("Unsupported asset type: {}", asset_type)),
        }
        
        balance.updated_at = chrono::Utc::now().timestamp() as u64;
        self.storage.update_balance(balance).await?;
        
        // Create transaction record
        let transaction = BalanceTransaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::Withdrawal,
            asset_type: asset_type.to_string(),
            amount,
            tx_hash: None, // Will be updated when the withdrawal is processed
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        self.storage.add_transaction(transaction.clone()).await?;
        
        // Process withdrawal through gas bank if it's a GAS withdrawal
        if asset_type.to_lowercase() == "gas" {
            // In a real implementation, this would call the gas bank service to process the withdrawal
            // For now, we'll just log it
            println!("Processing GAS withdrawal for user {}: {} GAS", user_id, amount);
        }
        
        Ok(transaction)
    }
    
    async fn charge_for_execution(&self, user_id: &str, function_id: &str, gas_amount: u64) -> Result<BalanceTransaction, String> {
        let mut balance = self.get_balance(user_id).await?;
        
        // Check if user has enough GAS balance
        if balance.gas_balance < gas_amount {
            return Err(format!("Insufficient GAS balance for function execution: {} < {}", balance.gas_balance, gas_amount));
        }
        
        balance.gas_balance -= gas_amount;
        balance.updated_at = chrono::Utc::now().timestamp() as u64;
        self.storage.update_balance(balance).await?;
        
        // Create transaction record
        let transaction = BalanceTransaction {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            transaction_type: TransactionType::FunctionExecution,
            asset_type: "gas".to_string(),
            amount: gas_amount,
            tx_hash: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        self.storage.add_transaction(transaction.clone()).await?;
        
        Ok(transaction)
    }
    
    async fn get_transactions(&self, user_id: &str) -> Result<Vec<BalanceTransaction>, String> {
        self.storage.get_transactions(user_id).await
    }
}
