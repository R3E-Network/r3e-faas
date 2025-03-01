// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use r3e_store::rocksdb::RocksDBStore;
use std::path::Path;
use std::sync::Arc;

use crate::auto_contract::storage::AutoContractStorage;
use crate::auto_contract::types::{AutoContract, AutoContractError, AutoContractExecution};

/// RocksDB implementation of AutoContractStorage
pub struct RocksDBAutoContractStorage {
    db: Arc<RocksDBStore>,
    contracts_cf: String,
    executions_cf: String,
}

impl RocksDBAutoContractStorage {
    /// Create a new RocksDB auto contract storage
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, AutoContractError> {
        let db = RocksDBStore::new(db_path).map_err(|e| {
            AutoContractError::Storage(format!("Failed to create RocksDB store: {}", e))
        })?;

        let contracts_cf = "auto_contracts".to_string();
        let executions_cf = "auto_contract_executions".to_string();

        Ok(Self {
            db: Arc::new(db),
            contracts_cf,
            executions_cf,
        })
    }
}

#[async_trait]
impl AutoContractStorage for RocksDBAutoContractStorage {
    async fn store_contract(&self, contract: AutoContract) -> Result<(), AutoContractError> {
        let key = contract.id.as_bytes();
        let value = serde_json::to_vec(&contract).map_err(|e| {
            AutoContractError::Storage(format!("Failed to serialize contract: {}", e))
        })?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.contracts_cf, input)
            .map_err(|e| AutoContractError::Storage(format!("Failed to store contract: {}", e)))?;

        Ok(())
    }

    async fn get_contract(&self, id: &str) -> Result<AutoContract, AutoContractError> {
        let key = id.as_bytes();

        match self.db.get(&self.contracts_cf, key) {
            Ok(value) => {
                let contract = serde_json::from_slice::<AutoContract>(&value).map_err(|e| {
                    AutoContractError::Storage(format!("Failed to deserialize contract: {}", e))
                })?;
                Ok(contract)
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(AutoContractError::NotFound(format!(
                "Contract not found: {}",
                id
            ))),
            Err(e) => Err(AutoContractError::Storage(format!(
                "Failed to get contract: {}",
                e
            ))),
        }
    }

    async fn delete_contract(&self, id: &str) -> Result<(), AutoContractError> {
        let key = id.as_bytes();

        // Check if contract exists
        match self.db.get(&self.contracts_cf, key) {
            Ok(_) => {
                self.db.delete(&self.contracts_cf, key).map_err(|e| {
                    AutoContractError::Storage(format!("Failed to delete contract: {}", e))
                })?;
                Ok(())
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(AutoContractError::NotFound(format!(
                "Contract not found: {}",
                id
            ))),
            Err(e) => Err(AutoContractError::Storage(format!(
                "Failed to get contract: {}",
                e
            ))),
        }
    }

    async fn list_user_contracts(
        &self,
        user_id: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.contracts_cf, input)
            .map_err(|e| AutoContractError::Storage(format!("Failed to scan contracts: {}", e)))?;

        let mut contracts = Vec::new();

        for (_, value) in output.kvs {
            let contract = serde_json::from_slice::<AutoContract>(&value).map_err(|e| {
                AutoContractError::Storage(format!("Failed to deserialize contract: {}", e))
            })?;

            if contract.user_id == user_id {
                contracts.push(contract);
            }
        }

        Ok(contracts)
    }

    async fn list_contracts_by_trigger(
        &self,
        trigger_type: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.contracts_cf, input)
            .map_err(|e| AutoContractError::Storage(format!("Failed to scan contracts: {}", e)))?;

        let mut contracts = Vec::new();

        for (_, value) in output.kvs {
            let contract = serde_json::from_slice::<AutoContract>(&value).map_err(|e| {
                AutoContractError::Storage(format!("Failed to deserialize contract: {}", e))
            })?;

            if contract.trigger.trigger_type.to_string() == trigger_type {
                contracts.push(contract);
            }
        }

        Ok(contracts)
    }

    async fn store_execution(
        &self,
        execution: AutoContractExecution,
    ) -> Result<(), AutoContractError> {
        let key = execution.id.as_bytes();
        let value = serde_json::to_vec(&execution).map_err(|e| {
            AutoContractError::Storage(format!("Failed to serialize execution: {}", e))
        })?;

        let input = r3e_store::PutInput {
            key,
            value: &value,
            if_not_exists: false,
        };

        self.db
            .put(&self.executions_cf, input)
            .map_err(|e| AutoContractError::Storage(format!("Failed to store execution: {}", e)))?;

        Ok(())
    }

    async fn get_execution(&self, id: &str) -> Result<AutoContractExecution, AutoContractError> {
        let key = id.as_bytes();

        match self.db.get(&self.executions_cf, key) {
            Ok(value) => {
                let execution =
                    serde_json::from_slice::<AutoContractExecution>(&value).map_err(|e| {
                        AutoContractError::Storage(format!(
                            "Failed to deserialize execution: {}",
                            e
                        ))
                    })?;
                Ok(execution)
            }
            Err(r3e_store::GetError::NoSuchKey) => Err(AutoContractError::NotFound(format!(
                "Execution not found: {}",
                id
            ))),
            Err(e) => Err(AutoContractError::Storage(format!(
                "Failed to get execution: {}",
                e
            ))),
        }
    }

    async fn list_contract_executions(
        &self,
        contract_id: &str,
    ) -> Result<Vec<AutoContractExecution>, AutoContractError> {
        let input = r3e_store::ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 1000, // Reasonable limit
        };

        let output = self
            .db
            .scan(&self.executions_cf, input)
            .map_err(|e| AutoContractError::Storage(format!("Failed to scan executions: {}", e)))?;

        let mut executions = Vec::new();

        for (_, value) in output.kvs {
            let execution =
                serde_json::from_slice::<AutoContractExecution>(&value).map_err(|e| {
                    AutoContractError::Storage(format!("Failed to deserialize execution: {}", e))
                })?;

            if execution.contract_id == contract_id {
                executions.push(execution);
            }
        }

        Ok(executions)
    }
}
