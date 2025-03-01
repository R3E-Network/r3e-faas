// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::auto_contract::types::{AutoContract, AutoContractError, AutoContractExecution};

/// Auto contract storage trait
#[async_trait]
pub trait AutoContractStorage: Send + Sync {
    /// Store auto contract
    async fn store_contract(&self, contract: AutoContract) -> Result<(), AutoContractError>;

    /// Get auto contract
    async fn get_contract(&self, id: &str) -> Result<AutoContract, AutoContractError>;

    /// Delete auto contract
    async fn delete_contract(&self, id: &str) -> Result<(), AutoContractError>;

    /// List auto contracts for user
    async fn list_user_contracts(
        &self,
        user_id: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError>;

    /// List auto contracts by trigger type
    async fn list_contracts_by_trigger(
        &self,
        trigger_type: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError>;

    /// Store execution result
    async fn store_execution(
        &self,
        execution: AutoContractExecution,
    ) -> Result<(), AutoContractError>;

    /// Get execution result
    async fn get_execution(&self, id: &str) -> Result<AutoContractExecution, AutoContractError>;

    /// List execution results for contract
    async fn list_contract_executions(
        &self,
        contract_id: &str,
    ) -> Result<Vec<AutoContractExecution>, AutoContractError>;
}

/// Memory-based implementation of AutoContractStorage
pub struct MemoryAutoContractStorage {
    contracts: Arc<RwLock<HashMap<String, AutoContract>>>,
    executions: Arc<RwLock<HashMap<String, AutoContractExecution>>>,
}

impl MemoryAutoContractStorage {
    /// Create a new memory-based auto contract storage
    pub fn new() -> Self {
        Self {
            contracts: Arc::new(RwLock::new(HashMap::new())),
            executions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl AutoContractStorage for MemoryAutoContractStorage {
    async fn store_contract(&self, contract: AutoContract) -> Result<(), AutoContractError> {
        let mut contracts = self.contracts.write().await;
        contracts.insert(contract.id.clone(), contract);
        Ok(())
    }

    async fn get_contract(&self, id: &str) -> Result<AutoContract, AutoContractError> {
        let contracts = self.contracts.read().await;

        match contracts.get(id) {
            Some(contract) => Ok(contract.clone()),
            None => Err(AutoContractError::NotFound(format!(
                "Contract not found: {}",
                id
            ))),
        }
    }

    async fn delete_contract(&self, id: &str) -> Result<(), AutoContractError> {
        let mut contracts = self.contracts.write().await;

        if contracts.remove(id).is_none() {
            return Err(AutoContractError::NotFound(format!(
                "Contract not found: {}",
                id
            )));
        }

        Ok(())
    }

    async fn list_user_contracts(
        &self,
        user_id: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError> {
        let contracts = self.contracts.read().await;

        let user_contracts = contracts
            .values()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect();

        Ok(user_contracts)
    }

    async fn list_contracts_by_trigger(
        &self,
        trigger_type: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError> {
        let contracts = self.contracts.read().await;

        let filtered_contracts = contracts
            .values()
            .filter(|c| c.trigger.trigger_type.to_string() == trigger_type)
            .cloned()
            .collect();

        Ok(filtered_contracts)
    }

    async fn store_execution(
        &self,
        execution: AutoContractExecution,
    ) -> Result<(), AutoContractError> {
        let mut executions = self.executions.write().await;
        executions.insert(execution.id.clone(), execution);
        Ok(())
    }

    async fn get_execution(&self, id: &str) -> Result<AutoContractExecution, AutoContractError> {
        let executions = self.executions.read().await;

        match executions.get(id) {
            Some(execution) => Ok(execution.clone()),
            None => Err(AutoContractError::NotFound(format!(
                "Execution not found: {}",
                id
            ))),
        }
    }

    async fn list_contract_executions(
        &self,
        contract_id: &str,
    ) -> Result<Vec<AutoContractExecution>, AutoContractError> {
        let executions = self.executions.read().await;

        let contract_executions = executions
            .values()
            .filter(|e| e.contract_id == contract_id)
            .cloned()
            .collect();

        Ok(contract_executions)
    }
}
