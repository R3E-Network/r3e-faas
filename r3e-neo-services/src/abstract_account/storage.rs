// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use super::types::{AbstractAccount, AccountOperationRecord, AccountStatus, OperationStatus};
use crate::Error;
use async_trait::async_trait;
use std::sync::Arc;

/// Abstract account storage trait
#[async_trait]
pub trait AbstractAccountStorage: Send + Sync {
    /// Get abstract account by address
    async fn get_account(&self, address: &str) -> Result<Option<AbstractAccount>, Error>;

    /// Get abstract accounts by owner
    async fn get_accounts_by_owner(&self, owner: &str) -> Result<Vec<AbstractAccount>, Error>;

    /// Create abstract account
    async fn create_account(&self, account: AbstractAccount) -> Result<(), Error>;

    /// Update abstract account
    async fn update_account(&self, account: AbstractAccount) -> Result<(), Error>;

    /// Get account operation record by ID
    async fn get_operation_record(
        &self,
        request_id: &str,
    ) -> Result<Option<AccountOperationRecord>, Error>;

    /// Get account operation records by account address
    async fn get_operation_records_by_account(
        &self,
        address: &str,
    ) -> Result<Vec<AccountOperationRecord>, Error>;

    /// Create account operation record
    async fn create_operation_record(&self, record: AccountOperationRecord) -> Result<(), Error>;

    /// Update account operation record
    async fn update_operation_record(&self, record: AccountOperationRecord) -> Result<(), Error>;

    /// Get next nonce for account
    async fn get_next_nonce(&self, address: &str) -> Result<u64, Error>;
}

/// In-memory abstract account storage implementation
pub struct InMemoryAbstractAccountStorage {
    accounts: tokio::sync::RwLock<Vec<AbstractAccount>>,
    operation_records: tokio::sync::RwLock<Vec<AccountOperationRecord>>,
}

impl InMemoryAbstractAccountStorage {
    /// Create a new in-memory abstract account storage
    pub fn new() -> Self {
        Self {
            accounts: tokio::sync::RwLock::new(Vec::new()),
            operation_records: tokio::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl AbstractAccountStorage for InMemoryAbstractAccountStorage {
    async fn get_account(&self, address: &str) -> Result<Option<AbstractAccount>, Error> {
        let accounts = self.accounts.read().await;
        Ok(accounts.iter().find(|a| a.address == address).cloned())
    }

    async fn get_accounts_by_owner(&self, owner: &str) -> Result<Vec<AbstractAccount>, Error> {
        let accounts = self.accounts.read().await;
        Ok(accounts
            .iter()
            .filter(|a| a.owner == owner)
            .cloned()
            .collect())
    }

    async fn create_account(&self, account: AbstractAccount) -> Result<(), Error> {
        let mut accounts = self.accounts.write().await;
        if accounts.iter().any(|a| a.address == account.address) {
            return Err(Error::InvalidParameter(format!(
                "Account already exists with address: {}",
                account.address
            )));
        }
        accounts.push(account);
        Ok(())
    }

    async fn update_account(&self, account: AbstractAccount) -> Result<(), Error> {
        let mut accounts = self.accounts.write().await;
        if let Some(index) = accounts.iter().position(|a| a.address == account.address) {
            accounts[index] = account;
            Ok(())
        } else {
            Err(Error::NotFound(format!(
                "Account not found with address: {}",
                account.address
            )))
        }
    }

    async fn get_operation_record(
        &self,
        request_id: &str,
    ) -> Result<Option<AccountOperationRecord>, Error> {
        let records = self.operation_records.read().await;
        Ok(records.iter().find(|r| r.request_id == request_id).cloned())
    }

    async fn get_operation_records_by_account(
        &self,
        address: &str,
    ) -> Result<Vec<AccountOperationRecord>, Error> {
        let records = self.operation_records.read().await;
        Ok(records
            .iter()
            .filter(|r| r.account_address == address)
            .cloned()
            .collect())
    }

    async fn create_operation_record(&self, record: AccountOperationRecord) -> Result<(), Error> {
        let mut records = self.operation_records.write().await;
        if records.iter().any(|r| r.request_id == record.request_id) {
            return Err(Error::InvalidParameter(format!(
                "Operation record already exists with ID: {}",
                record.request_id
            )));
        }
        records.push(record);
        Ok(())
    }

    async fn update_operation_record(&self, record: AccountOperationRecord) -> Result<(), Error> {
        let mut records = self.operation_records.write().await;
        if let Some(index) = records
            .iter()
            .position(|r| r.request_id == record.request_id)
        {
            records[index] = record;
            Ok(())
        } else {
            Err(Error::NotFound(format!(
                "Operation record not found with ID: {}",
                record.request_id
            )))
        }
    }

    async fn get_next_nonce(&self, address: &str) -> Result<u64, Error> {
        let records = self.operation_records.read().await;
        let max_nonce = records
            .iter()
            .filter(|r| r.account_address == address)
            .map(|r| r.request.nonce)
            .max()
            .unwrap_or(0);
        Ok(max_nonce + 1)
    }
}
