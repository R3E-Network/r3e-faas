// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::RocksDBStore;
use r3e_store::rocksdb::RocksDbConfig;
use std::path::Path;
use std::sync::Arc;

use super::storage::GasBankStorage;
use super::types::{GasBankAccount, GasBankDeposit, GasBankTransaction, GasBankWithdrawal};
use crate::Error;

/// RocksDB implementation of GasBankStorage
pub struct RocksDBGasBankStorage {
    db: Arc<RocksDBStore>,
    accounts_cf: String,
    deposits_cf: String,
    withdrawals_cf: String,
    transactions_cf: String,
    contract_mappings_cf: String,
}

impl RocksDBGasBankStorage {
    /// Create a new RocksDB gas bank storage
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, Error> {
        let config = RocksDbConfig {
            path: db_path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };
        
        let db = RocksDBStore::new(config);
        
        // Open the database
        db.open().map_err(|e| Error::Storage(format!("Failed to open RocksDB store: {}", e)))?;
        
        let accounts_cf = "gas_bank_accounts".to_string();
        let deposits_cf = "gas_bank_deposits".to_string();
        let withdrawals_cf = "gas_bank_withdrawals".to_string();
        let transactions_cf = "gas_bank_transactions".to_string();
        let contract_mappings_cf = "gas_bank_contract_mappings".to_string();
        
        // Create column families if they don't exist
        for cf in [&accounts_cf, &deposits_cf, &withdrawals_cf, &transactions_cf, &contract_mappings_cf] {
            db.create_cf_if_missing(cf)
                .map_err(|e| Error::Storage(format!("Failed to create column family {}: {}", cf, e)))?;
        }

        Ok(Self {
            db: Arc::new(db),
            accounts_cf,
            deposits_cf,
            withdrawals_cf,
            transactions_cf,
            contract_mappings_cf,
        })
    }
}

#[async_trait]
impl GasBankStorage for RocksDBGasBankStorage {
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error> {
        let key = address;

        match self.db.get_cf::<_, Vec<u8>>(&self.accounts_cf, key) {
            Ok(Some(value)) => {
                let account = serde_json::from_slice::<GasBankAccount>(&value)
                    .map_err(|e| Error::Storage(format!("Failed to deserialize account: {}", e)))?;
                Ok(Some(account))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(Error::Storage(format!("Failed to get account: {}", e))),
        }
    }

    async fn create_account(&self, account: GasBankAccount) -> Result<(), Error> {
        let key = account.address.clone();
        let value = serde_json::to_vec(&account)
            .map_err(|e| Error::Storage(format!("Failed to serialize account: {}", e)))?;

        // Check if the account already exists
        if let Ok(Some(_)) = self.db.get_cf::<_, Vec<u8>>(&self.accounts_cf, &key) {
            return Err(Error::Storage(format!(
                "Account already exists: {}",
                account.address
            )));
        }

        self.db
            .put_cf(&self.accounts_cf, key, &value)
            .map_err(|e| Error::Storage(format!("Failed to store account: {}", e)))?;

        Ok(())
    }

    async fn update_account(&self, account: GasBankAccount) -> Result<(), Error> {
        let key = account.address.clone();
        let value = serde_json::to_vec(&account)
            .map_err(|e| Error::Storage(format!("Failed to serialize account: {}", e)))?;

        // Check if the account exists
        if let Ok(None) = self.db.get_cf::<_, Vec<u8>>(&self.accounts_cf, &key) {
            return Err(Error::Storage(format!(
                "Account does not exist: {}",
                account.address
            )));
        }

        self.db
            .put_cf(&self.accounts_cf, key, &value)
            .map_err(|e| Error::Storage(format!("Failed to update account: {}", e)))?;

        Ok(())
    }

    async fn get_deposits(&self, address: &str) -> Result<Vec<GasBankDeposit>, Error> {
        let prefix = format!("{}:", address);
        
        // Create a prefix iterator and collect the results manually
        let iter: Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + Send> = 
            self.db.prefix_iter_cf(&self.deposits_cf, prefix.as_bytes())
            .map_err(|e| Error::Storage(format!("Failed to scan deposits: {}", e)))?;
        
        let mut deposits = Vec::new();
        
        for (_, value_boxed) in iter {
            let value_vec = value_boxed.to_vec();
            
            let deposit = serde_json::from_slice::<GasBankDeposit>(&value_vec)
                .map_err(|e| Error::Storage(format!("Failed to deserialize deposit: {}", e)))?;

            // Only add deposits for this address
            if deposit.address == address {
                deposits.push(deposit);
            }
        }

        Ok(deposits)
    }

    async fn add_deposit(&self, deposit: GasBankDeposit) -> Result<(), Error> {
        let key = format!("{}:{}", deposit.address, deposit.tx_hash);
        let value = serde_json::to_vec(&deposit)
            .map_err(|e| Error::Storage(format!("Failed to serialize deposit: {}", e)))?;

        self.db
            .put_cf(&self.deposits_cf, key, &value)
            .map_err(|e| Error::Storage(format!("Failed to store deposit: {}", e)))?;

        Ok(())
    }

    async fn get_withdrawals(&self, address: &str) -> Result<Vec<GasBankWithdrawal>, Error> {
        let prefix = format!("{}:", address);
        
        // Create a prefix iterator and collect the results manually
        let iter: Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + Send> = 
            self.db.prefix_iter_cf(&self.withdrawals_cf, prefix.as_bytes())
            .map_err(|e| Error::Storage(format!("Failed to scan withdrawals: {}", e)))?;
        
        let mut withdrawals = Vec::new();
        
        for (_, value_boxed) in iter {
            let value_vec = value_boxed.to_vec();
            
            let withdrawal = serde_json::from_slice::<GasBankWithdrawal>(&value_vec)
                .map_err(|e| Error::Storage(format!("Failed to deserialize withdrawal: {}", e)))?;

            // Only add withdrawals for this address
            if withdrawal.address == address {
                withdrawals.push(withdrawal);
            }
        }

        Ok(withdrawals)
    }

    async fn add_withdrawal(&self, withdrawal: GasBankWithdrawal) -> Result<(), Error> {
        let key = format!("{}:{}", withdrawal.address, withdrawal.tx_hash);
        let value = serde_json::to_vec(&withdrawal)
            .map_err(|e| Error::Storage(format!("Failed to serialize withdrawal: {}", e)))?;

        self.db
            .put_cf(&self.withdrawals_cf, key, &value)
            .map_err(|e| Error::Storage(format!("Failed to store withdrawal: {}", e)))?;

        Ok(())
    }

    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error> {
        let prefix = format!("{}:", address);
        
        // Create a prefix iterator and collect the results manually
        let iter: Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + Send> = 
            self.db.prefix_iter_cf(&self.transactions_cf, prefix.as_bytes())
            .map_err(|e| Error::Storage(format!("Failed to scan transactions: {}", e)))?;
        
        let mut transactions = Vec::new();
        
        for (_, value_boxed) in iter {
            let value_vec = value_boxed.to_vec();
            
            let transaction = serde_json::from_slice::<GasBankTransaction>(&value_vec)
                .map_err(|e| Error::Storage(format!("Failed to deserialize transaction: {}", e)))?;

            // Only add transactions for this address
            if transaction.address == address {
                transactions.push(transaction);
            }
        }

        Ok(transactions)
    }

    async fn add_transaction(&self, transaction: GasBankTransaction) -> Result<(), Error> {
        let key = format!("{}:{}", transaction.address, transaction.tx_hash);
        let value = serde_json::to_vec(&transaction)
            .map_err(|e| Error::Storage(format!("Failed to serialize transaction: {}", e)))?;

        self.db
            .put_cf(&self.transactions_cf, key, &value)
            .map_err(|e| Error::Storage(format!("Failed to store transaction: {}", e)))?;

        Ok(())
    }

    async fn get_contract_account_mapping(
        &self,
        contract_hash: &str,
    ) -> Result<Option<String>, Error> {
        match self.db.get_cf::<_, String>(&self.contract_mappings_cf, contract_hash) {
            Ok(Some(address)) => Ok(Some(address)),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::Storage(format!(
                "Failed to get contract mapping: {}",
                e
            ))),
        }
    }

    async fn set_contract_account_mapping(
        &self,
        contract_hash: &str,
        address: &str,
    ) -> Result<(), Error> {
        let address_string = address.to_string();
        self.db
            .put_cf(&self.contract_mappings_cf, contract_hash.to_string(), &address_string)
            .map_err(|e| Error::Storage(format!("Failed to set contract mapping: {}", e)))?;

        Ok(())
    }
}
