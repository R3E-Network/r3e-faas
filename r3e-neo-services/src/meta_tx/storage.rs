// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use super::types::{MetaTxRecord, MetaTxRequest, MetaTxResponse, MetaTxStatus};
use crate::Error;
use async_trait::async_trait;
use std::sync::Arc;

/// Meta transaction storage trait
#[async_trait]
pub trait MetaTxStorage: Send + Sync {
    /// Get meta transaction record by ID
    async fn get_record(&self, request_id: &str) -> Result<Option<MetaTxRecord>, Error>;

    /// Get meta transaction records by sender
    async fn get_records_by_sender(&self, sender: &str) -> Result<Vec<MetaTxRecord>, Error>;

    /// Create meta transaction record
    async fn create_record(&self, record: MetaTxRecord) -> Result<(), Error>;

    /// Update meta transaction record
    async fn update_record(&self, record: MetaTxRecord) -> Result<(), Error>;

    /// Get meta transaction nonce for sender
    async fn get_nonce(&self, sender: &str) -> Result<u64, Error>;
}

/// In-memory meta transaction storage implementation
pub struct InMemoryMetaTxStorage {
    records: tokio::sync::RwLock<Vec<MetaTxRecord>>,
}

impl InMemoryMetaTxStorage {
    /// Create a new in-memory meta transaction storage
    pub fn new() -> Self {
        Self {
            records: tokio::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl MetaTxStorage for InMemoryMetaTxStorage {
    async fn get_record(&self, request_id: &str) -> Result<Option<MetaTxRecord>, Error> {
        let records = self.records.read().await;
        Ok(records.iter().find(|r| r.request_id == request_id).cloned())
    }

    async fn get_records_by_sender(&self, sender: &str) -> Result<Vec<MetaTxRecord>, Error> {
        let records = self.records.read().await;
        Ok(records
            .iter()
            .filter(|r| r.request.sender == sender)
            .cloned()
            .collect())
    }

    async fn create_record(&self, record: MetaTxRecord) -> Result<(), Error> {
        let mut records = self.records.write().await;
        if records.iter().any(|r| r.request_id == record.request_id) {
            return Err(Error::InvalidParameter(format!(
                "Record already exists with ID: {}",
                record.request_id
            )));
        }
        records.push(record);
        Ok(())
    }

    async fn update_record(&self, record: MetaTxRecord) -> Result<(), Error> {
        let mut records = self.records.write().await;
        if let Some(index) = records
            .iter()
            .position(|r| r.request_id == record.request_id)
        {
            records[index] = record;
            Ok(())
        } else {
            Err(Error::NotFound(format!(
                "Record not found with ID: {}",
                record.request_id
            )))
        }
    }

    async fn get_nonce(&self, sender: &str) -> Result<u64, Error> {
        let records = self.records.read().await;
        let max_nonce = records
            .iter()
            .filter(|r| r.request.sender == sender)
            .map(|r| r.request.nonce)
            .max()
            .unwrap_or(0);
        Ok(max_nonce + 1)
    }
}
