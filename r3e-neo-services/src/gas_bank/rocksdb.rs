// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::rocksdb::RocksDBStore;
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
        let db = RocksDBStore::new(db_path)
            .map_err(|e| Error::Storage(format!("Failed to create RocksDB store: {}", e)))?;

        let accounts_cf = "gas_bank_accounts".to_string();
        let deposits_cf = "gas_bank_deposits".to_string();
        let withdrawals_cf = "gas_bank_withdrawals".to_string();
        let transactions_cf = "gas_bank_transactions".to_string();
        let contract_mappings_cf = "gas_bank_contract_mappings".to_string();

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
        let key = address.as_bytes();

        match self.db.get(&self.accounts_cf, key) {
            Ok(value) => match serde_json::from_slice::<GasBankAccount>(&value) {
                Ok(account) => Ok(Some(account)),
                Err(e) => Err(Error::Storage(format!(
                    "Failed to deserialize account: {}",
                    e
                ))),
            },
            Err(r3e_store::GetError::NoSuchKey) => Ok(None),
            Err(e) => Err(Error::Storage(format!("Failed to get account: {}", e))),
        }
    }

    async fn create_account(&self, account: GasBankAccount) -> Result<(), Error> {
        let key = account.address.as_bytes();
        let value = serde_json::to_vec(&account)
            .map_err(|e| Error::Storage(format!("Failed to serialize account: {}", e)))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: true,
        };

        match self.db.put(&self.accounts_cf, input) {
            Ok(_) => Ok(()),
            Err(r3e_store::PutError::AlreadyExists) => Err(Error::InvalidParameter(format!(
                "Account already exists for address: {}",
                account.address
            ))),
            Err(e) => Err(Error::Storage(format!("Failed to create account: {}", e))),
        }
    }

    async fn update_account(&self, account: GasBankAccount) -> Result<(), Error> {
        // Check if account exists
        if self.get_account(&account.address).await?.is_none() {
            return Err(Error::NotFound(format!(
                "Account not found for address: {}",
                account.address
            )));
        }

        let key = account.address.as_bytes();
        let value = serde_json::to_vec(&account)
            .map_err(|e| Error::Storage(format!("Failed to serialize account: {}", e)))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.accounts_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to update account: {}", e)))?;

        Ok(())
    }

    async fn get_deposits(&self, address: &str) -> Result<Vec<GasBankDeposit>, Error> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.deposits_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to scan deposits: {}", e)))?;

        let mut deposits = Vec::new();

        for (_, value) in output.kvs {
            let deposit = serde_json::from_slice::<GasBankDeposit>(&value)
                .map_err(|e| Error::Storage(format!("Failed to deserialize deposit: {}", e)))?;

            if deposit.address == address {
                deposits.push(deposit);
            }
        }

        Ok(deposits)
    }

    async fn add_deposit(&self, deposit: GasBankDeposit) -> Result<(), Error> {
        let key = deposit.tx_hash.as_bytes();
        let value = serde_json::to_vec(&deposit)
            .map_err(|e| Error::Storage(format!("Failed to serialize deposit: {}", e)))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: true,
        };

        self.db
            .put(&self.deposits_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to add deposit: {}", e)))?;

        Ok(())
    }

    async fn get_withdrawals(&self, address: &str) -> Result<Vec<GasBankWithdrawal>, Error> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.withdrawals_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to scan withdrawals: {}", e)))?;

        let mut withdrawals = Vec::new();

        for (_, value) in output.kvs {
            let withdrawal = serde_json::from_slice::<GasBankWithdrawal>(&value)
                .map_err(|e| Error::Storage(format!("Failed to deserialize withdrawal: {}", e)))?;

            if withdrawal.address == address {
                withdrawals.push(withdrawal);
            }
        }

        Ok(withdrawals)
    }

    async fn add_withdrawal(&self, withdrawal: GasBankWithdrawal) -> Result<(), Error> {
        let key = withdrawal.tx_hash.as_bytes();
        let value = serde_json::to_vec(&withdrawal)
            .map_err(|e| Error::Storage(format!("Failed to serialize withdrawal: {}", e)))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: true,
        };

        self.db
            .put(&self.withdrawals_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to add withdrawal: {}", e)))?;

        Ok(())
    }

    async fn get_transactions(&self, address: &str) -> Result<Vec<GasBankTransaction>, Error> {
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
            .map_err(|e| Error::Storage(format!("Failed to scan transactions: {}", e)))?;

        let mut transactions = Vec::new();

        for (_, value) in output.kvs {
            let transaction = serde_json::from_slice::<GasBankTransaction>(&value)
                .map_err(|e| Error::Storage(format!("Failed to deserialize transaction: {}", e)))?;

            if transaction.address == address {
                transactions.push(transaction);
            }
        }

        Ok(transactions)
    }

    async fn add_transaction(&self, transaction: GasBankTransaction) -> Result<(), Error> {
        let key = transaction.tx_hash.as_bytes();
        let value = serde_json::to_vec(&transaction)
            .map_err(|e| Error::Storage(format!("Failed to serialize transaction: {}", e)))?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: true,
        };

        self.db
            .put(&self.transactions_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to add transaction: {}", e)))?;

        Ok(())
    }

    async fn get_contract_account_mapping(
        &self,
        contract_hash: &str,
    ) -> Result<Option<String>, Error> {
        let key = contract_hash.as_bytes();

        match self.db.get(&self.contract_mappings_cf, key) {
            Ok(value) => {
                let address = String::from_utf8(value)
                    .map_err(|e| Error::Storage(format!("Failed to deserialize address: {}", e)))?;
                Ok(Some(address))
            }
            Err(r3e_store::GetError::NoSuchKey) => Ok(None),
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
        let key = contract_hash.as_bytes();
        let value = address.as_bytes();

        let input = r3e_store::PutInput {
            key,
            value,
            if_not_exists: false, // Allow overwriting existing mappings
        };

        self.db
            .put(&self.contract_mappings_cf, input)
            .map_err(|e| Error::Storage(format!("Failed to set contract mapping: {}", e)))?;

        Ok(())
    }
}
