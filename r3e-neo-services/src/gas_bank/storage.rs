// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use crate::Error;
use super::types::{GasBankAccount, GasBankDeposit, GasBankWithdrawal, GasBankTransaction};

/// Gas bank storage trait
#[async_trait]
pub trait GasBankStorage: Send + Sync {
    /// Get gas bank account
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error>;
    
    /// Create gas bank account
    async fn create_account(&self, account: GasBankAccount) -> Result<(), Error>;
    
    /// Update gas bank account
    async fn update_account(&self, account: GasBankAccount) -> Result<(), Error>;
    
    /// Get gas bank deposits
    async fn get_deposits(&self, address: &str) -> Result<Vec<GasBankDeposit>, Error>;
    
    /// Add gas bank deposit
    async fn add_deposit(&self, deposit: GasBankDeposit) -> Result<(), Error>;
    
    /// Get gas bank withdrawals
    async fn get_withdrawals(&self, address: &str) -> Result<Vec<GasBankWithdrawal>, Error>;
    
    /// Add gas bank withdrawal
    async fn add_withdrawal(&self, withdrawal: GasBankWithdrawal) -> Result<(), Error>;
    
    /// Get gas bank transactions
    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error>;
    
    /// Add gas bank transaction
    async fn add_transaction(&self, transaction: GasBankTransaction) -> Result<(), Error>;
}

/// In-memory gas bank storage implementation
pub struct InMemoryGasBankStorage {
    accounts: tokio::sync::RwLock<Vec<GasBankAccount>>,
    deposits: tokio::sync::RwLock<Vec<GasBankDeposit>>,
    withdrawals: tokio::sync::RwLock<Vec<GasBankWithdrawal>>,
    transactions: tokio::sync::RwLock<Vec<GasBankTransaction>>,
}

impl InMemoryGasBankStorage {
    /// Create a new in-memory gas bank storage
    pub fn new() -> Self {
        Self {
            accounts: tokio::sync::RwLock::new(Vec::new()),
            deposits: tokio::sync::RwLock::new(Vec::new()),
            withdrawals: tokio::sync::RwLock::new(Vec::new()),
            transactions: tokio::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl GasBankStorage for InMemoryGasBankStorage {
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error> {
        let accounts = self.accounts.read().await;
        Ok(accounts.iter().find(|a| a.address == address).cloned())
    }
    
    async fn create_account(&self, account: GasBankAccount) -> Result<(), Error> {
        let mut accounts = self.accounts.write().await;
        if accounts.iter().any(|a| a.address == account.address) {
            return Err(Error::InvalidParameter(format!("Account already exists for address: {}", account.address)));
        }
        accounts.push(account);
        Ok(())
    }
    
    async fn update_account(&self, account: GasBankAccount) -> Result<(), Error> {
        let mut accounts = self.accounts.write().await;
        if let Some(index) = accounts.iter().position(|a| a.address == account.address) {
            accounts[index] = account;
            Ok(())
        } else {
            Err(Error::NotFound(format!("Account not found for address: {}", account.address)))
        }
    }
    
    async fn get_deposits(&self, address: &str) -> Result<Vec<GasBankDeposit>, Error> {
        let deposits = self.deposits.read().await;
        Ok(deposits.iter().filter(|d| d.address == address).cloned().collect())
    }
    
    async fn add_deposit(&self, deposit: GasBankDeposit) -> Result<(), Error> {
        let mut deposits = self.deposits.write().await;
        deposits.push(deposit);
        Ok(())
    }
    
    async fn get_withdrawals(&self, address: &str) -> Result<Vec<GasBankWithdrawal>, Error> {
        let withdrawals = self.withdrawals.read().await;
        Ok(withdrawals.iter().filter(|w| w.address == address).cloned().collect())
    }
    
    async fn add_withdrawal(&self, withdrawal: GasBankWithdrawal) -> Result<(), Error> {
        let mut withdrawals = self.withdrawals.write().await;
        withdrawals.push(withdrawal);
        Ok(())
    }
    
    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error> {
        let transactions = self.transactions.read().await;
        Ok(transactions.iter().filter(|t| t.address == address).cloned().collect())
    }
    
    async fn add_transaction(&self, transaction: GasBankTransaction) -> Result<(), Error> {
        let mut transactions = self.transactions.write().await;
        transactions.push(transaction);
        Ok(())
    }
}
