// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use NeoRust::prelude::{Wallet, Transaction, TransactionBuilder, RpcClient};
use crate::Error;
use crate::types::FeeModel;
use super::storage::GasBankStorage;
use super::types::{GasBankAccount, GasBankDeposit, GasBankWithdrawal, GasBankTransaction};
use std::sync::Arc;
use log::{debug, info, warn, error};

/// Gas bank service trait
#[async_trait]
pub trait GasBankServiceTrait: Send + Sync {
    /// Get gas bank account
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error>;
    
    /// Create gas bank account
    async fn create_account(&self, address: &str, fee_model: FeeModel, credit_limit: u64) -> Result<GasBankAccount, Error>;
    
    /// Deposit gas to account
    async fn deposit(&self, tx_hash: &str, address: &str, amount: u64) -> Result<GasBankDeposit, Error>;
    
    /// Withdraw gas from account
    async fn withdraw(&self, address: &str, amount: u64) -> Result<GasBankWithdrawal, Error>;
    
    /// Pay gas for transaction
    async fn pay_gas_for_transaction(&self, tx_hash: &str, address: &str, amount: u64) -> Result<GasBankTransaction, Error>;
    
    /// Get gas price
    async fn get_gas_price(&self) -> Result<u64, Error>;
    
    /// Estimate gas for transaction
    async fn estimate_gas(&self, tx_data: &[u8]) -> Result<u64, Error>;
    
    /// Get account balance
    async fn get_balance(&self, address: &str) -> Result<u64, Error>;
    
    /// Get account transactions
    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error>;
    
    /// Get gas bank account for contract
    async fn get_account_for_contract(&self, contract_hash: &str) -> Result<Option<GasBankAccount>, Error>;
    
    /// Set gas bank account for contract
    async fn set_account_for_contract(&self, contract_hash: &str, address: &str) -> Result<(), Error>;
    
    /// Pay for Ethereum meta transaction
    async fn pay_for_ethereum_meta_tx(&self, tx_hash: &str, contract_hash: &str, amount: u64) -> Result<GasBankTransaction, Error>;
}

/// Gas bank service implementation
pub struct GasBankService<S: GasBankStorage> {
    /// Storage
    storage: Arc<S>,
    /// RPC client
    rpc_client: Arc<RpcClient>,
    /// Gas bank wallet
    wallet: Arc<Wallet>,
    /// Network type
    network: String,
    /// Default fee model
    default_fee_model: FeeModel,
    /// Default credit limit
    default_credit_limit: u64,
}

impl<S: GasBankStorage> GasBankService<S> {
    /// Create a new gas bank service
    pub fn new(
        storage: Arc<S>,
        rpc_client: Arc<RpcClient>,
        wallet: Arc<Wallet>,
        network: String,
        default_fee_model: FeeModel,
        default_credit_limit: u64,
    ) -> Self {
        Self {
            storage,
            rpc_client,
            wallet,
            network,
            default_fee_model,
            default_credit_limit,
        }
    }
    
    /// Calculate fee for amount
    fn calculate_fee(&self, amount: u64, fee_model: &FeeModel) -> u64 {
        match fee_model {
            FeeModel::Fixed(fee) => *fee,
            FeeModel::Percentage(percentage) => ((amount as f64) * percentage / 100.0) as u64,
            FeeModel::Dynamic => {
                // Calculate dynamic fee based on network congestion
                let gas_price = self.get_gas_price().await.unwrap_or(1000);
                let network_usage = self.rpc_client.get_network_usage().await.unwrap_or(50);
                let congestion_multiplier = 1.0 + (network_usage as f64 / 100.0);
                ((amount as f64) * 0.01 * congestion_multiplier) as u64
            },
            FeeModel::Free => 0,
        }
    }
    
    /// Create a gas transfer transaction
    async fn create_gas_transfer_transaction(&self, to: &str, amount: u64) -> Result<Transaction, Error> {
        let tx = TransactionBuilder::new()
            .script(vec![]) // This would be the actual script for transferring GAS
            .gas_limit(2100000)
            .build();
        
        Ok(tx)
    }
    
    /// Send transaction
    async fn send_transaction(&self, tx: Transaction) -> Result<String, Error> {
        // Sign the transaction with the gas bank wallet
        let signed_tx = self.wallet.sign_transaction(tx).await
            .map_err(|e| Error::Transaction(format!("Failed to sign transaction: {}", e)))?;
        
        // Send the transaction to the network
        let tx_hash = self.rpc_client.send_raw_transaction(signed_tx).await
            .map_err(|e| Error::Transaction(format!("Failed to send transaction: {}", e)))?;
            
        Ok(tx_hash)
    }
}

#[async_trait]
impl<S: GasBankStorage> GasBankServiceTrait for GasBankService<S> {
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error> {
        self.storage.get_account(address).await
    }
    
    async fn create_account(&self, address: &str, fee_model: FeeModel, credit_limit: u64) -> Result<GasBankAccount, Error> {
        // Check if account already exists
        if let Some(_) = self.storage.get_account(address).await? {
            return Err(Error::InvalidParameter(format!("Account already exists for address: {}", address)));
        }
        
        // Create new account
        let account = GasBankAccount {
            address: address.to_string(),
            balance: 0,
            fee_model,
            credit_limit,
            used_credit: 0,
            updated_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        };
        
        // Store account
        self.storage.create_account(account.clone()).await?;
        
        Ok(account)
    }
    
    async fn deposit(&self, tx_hash: &str, address: &str, amount: u64) -> Result<GasBankDeposit, Error> {
        // Get account
        let mut account = match self.storage.get_account(address).await? {
            Some(account) => account,
            None => {
                // Create account with default settings if it doesn't exist
                self.create_account(address, self.default_fee_model.clone(), self.default_credit_limit).await?
            }
        };
        
        // Update account balance
        account.balance += amount;
        account.updated_at = chrono::Utc::now().timestamp() as u64;
        
        // Store updated account
        self.storage.update_account(account).await?;
        
        // Create deposit record
        let deposit = GasBankDeposit {
            tx_hash: tx_hash.to_string(),
            address: address.to_string(),
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        };
        
        // Store deposit
        self.storage.add_deposit(deposit.clone()).await?;
        
        Ok(deposit)
    }
    
    async fn withdraw(&self, address: &str, amount: u64) -> Result<GasBankWithdrawal, Error> {
        // Get account
        let mut account = match self.storage.get_account(address).await? {
            Some(account) => account,
            None => return Err(Error::NotFound(format!("Account not found for address: {}", address))),
        };
        
        // Calculate fee
        let fee = self.calculate_fee(amount, &account.fee_model);
        
        // Check if account has enough balance
        if account.balance < amount + fee {
            return Err(Error::InsufficientFunds(format!("Insufficient funds for withdrawal: {} < {}", account.balance, amount + fee)));
        }
        
        // Create and send transaction
        let tx = self.create_gas_transfer_transaction(address, amount).await?;
        let tx_hash = self.send_transaction(tx).await?;
        
        // Update account balance
        account.balance -= amount + fee;
        account.updated_at = chrono::Utc::now().timestamp() as u64;
        
        // Store updated account
        self.storage.update_account(account).await?;
        
        // Create withdrawal record
        let withdrawal = GasBankWithdrawal {
            tx_hash,
            address: address.to_string(),
            amount,
            fee,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        };
        
        // Store withdrawal
        self.storage.add_withdrawal(withdrawal.clone()).await?;
        
        // Create transaction record
        let transaction = GasBankTransaction {
            tx_hash: withdrawal.tx_hash.clone(),
            address: address.to_string(),
            tx_type: "withdrawal".to_string(),
            amount,
            fee,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        };
        
        // Store transaction
        self.storage.add_transaction(transaction).await?;
        
        Ok(withdrawal)
    }
    
    async fn pay_gas_for_transaction(&self, tx_hash: &str, address: &str, amount: u64) -> Result<GasBankTransaction, Error> {
        // Get account
        let mut account = match self.storage.get_account(address).await? {
            Some(account) => account,
            None => return Err(Error::NotFound(format!("Account not found for address: {}", address))),
        };
        
        // Calculate fee
        let fee = self.calculate_fee(amount, &account.fee_model);
        
        // Check if account has enough balance or credit
        let total_cost = amount + fee;
        let available_funds = account.balance + (account.credit_limit - account.used_credit);
        
        if available_funds < total_cost {
            return Err(Error::InsufficientFunds(format!("Insufficient funds for gas payment: {} < {}", available_funds, total_cost)));
        }
        
        // Use credit if needed
        if account.balance < total_cost {
            let credit_needed = total_cost - account.balance;
            account.used_credit += credit_needed;
            account.balance = 0;
        } else {
            account.balance -= total_cost;
        }
        
        account.updated_at = chrono::Utc::now().timestamp() as u64;
        
        // Store updated account
        self.storage.update_account(account).await?;
        
        // Create transaction record
        let transaction = GasBankTransaction {
            tx_hash: tx_hash.to_string(),
            address: address.to_string(),
            tx_type: "gas_payment".to_string(),
            amount,
            fee,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        };
        
        // Store transaction
        self.storage.add_transaction(transaction.clone()).await?;
        
        Ok(transaction)
    }
    
    async fn get_gas_price(&self) -> Result<u64, Error> {
        // Query the network for current gas price
        self.rpc_client.get_gas_price().await
            .map_err(|e| Error::Network(format!("Failed to get gas price: {}", e)))
    }
    
    async fn estimate_gas(&self, tx_data: &[u8]) -> Result<u64, Error> {
        // Create an unsigned transaction with the data
        let tx = TransactionBuilder::new()
            .script(tx_data.to_vec())
            .build();
            
        // Estimate gas using RPC client
        self.rpc_client.estimate_gas(tx).await
            .map_err(|e| Error::Transaction(format!("Failed to estimate gas: {}", e)))
    }
    
    async fn get_balance(&self, address: &str) -> Result<u64, Error> {
        // Get account
        let account = match self.storage.get_account(address).await? {
            Some(account) => account,
            None => return Err(Error::NotFound(format!("Account not found for address: {}", address))),
        };
        
        Ok(account.balance)
    }
    
    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error> {
        self.storage.get_transactions(address).await
    }
    
    async fn get_account_for_contract(&self, contract_hash: &str) -> Result<Option<GasBankAccount>, Error> {
        // Retrieve the mapping from storage
        
        // Check if we have a mapping for this contract
        let address = match self.storage.get_contract_account_mapping(contract_hash).await? {
            Some(address) => address,
            None => return Ok(None),
        };
        
        // Get the account for this address
        self.get_account(&address).await
    }
    
    async fn set_account_for_contract(&self, contract_hash: &str, address: &str) -> Result<(), Error> {
        // Store the mapping in storage
        
        // Check if the account exists
        if let None = self.get_account(address).await? {
            return Err(Error::NotFound(format!("Account not found for address: {}", address)));
        }
        
        // Store the mapping
        self.storage.set_contract_account_mapping(contract_hash, address).await?;
        
        info!("Set Gas Bank account {} for contract {}", address, contract_hash);
        
        Ok(())
    }
    
    async fn pay_for_ethereum_meta_tx(&self, tx_hash: &str, contract_hash: &str, amount: u64) -> Result<GasBankTransaction, Error> {
        // Get the Gas Bank account for this contract
        let account_address = match self.get_account_for_contract(contract_hash).await? {
            Some(account) => account.address,
            None => return Err(Error::NotFound(format!("No Gas Bank account found for contract: {}", contract_hash))),
        };
        
        // Pay for the transaction using the contract's Gas Bank account
        let transaction = self.pay_gas_for_transaction(tx_hash, &account_address, amount).await?;
        
        info!("Paid for Ethereum meta transaction {} using Gas Bank account {} for contract {}", 
              tx_hash, account_address, contract_hash);
        
        Ok(transaction)
    }
}
