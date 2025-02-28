// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::trigger::types::{
    TriggerCondition, TriggerError, TriggerSource,
    BlockchainTriggerParams, TimeTriggerParams, MarketTriggerParams, CustomTriggerParams,
};

/// Trigger service trait
#[async_trait]
pub trait TriggerService: Send + Sync {
    /// Register a trigger
    async fn register_trigger(
        &self,
        user_id: &str,
        function_id: &str,
        condition: TriggerCondition,
    ) -> Result<String, TriggerError>;
    
    /// Unregister a trigger
    async fn unregister_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(), TriggerError>;
    
    /// Get trigger by ID
    async fn get_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(String, String, TriggerCondition), TriggerError>;
    
    /// List triggers for a function
    async fn list_function_triggers(
        &self,
        function_id: &str,
    ) -> Result<Vec<(String, TriggerCondition)>, TriggerError>;
    
    /// Evaluate a trigger condition
    async fn evaluate_trigger(
        &self,
        condition: &TriggerCondition,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError>;
}

/// Trigger service implementation
pub struct TriggerServiceImpl {
    /// Trigger storage
    triggers: Arc<tokio::sync::RwLock<HashMap<String, (String, String, TriggerCondition)>>>,
}

impl TriggerServiceImpl {
    /// Create a new trigger service
    pub fn new() -> Self {
        Self {
            triggers: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Parse blockchain trigger parameters
    fn parse_blockchain_params(
        &self,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<BlockchainTriggerParams, TriggerError> {
        let network = params
            .get("network")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing network parameter".to_string()))?
            .to_string();
        
        let contract_address = params
            .get("contract_address")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let event_name = params
            .get("event_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let method_name = params
            .get("method_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let block_number = params
            .get("block_number")
            .and_then(|v| v.as_u64());
        
        Ok(BlockchainTriggerParams {
            network,
            contract_address,
            event_name,
            method_name,
            block_number,
        })
    }
    
    /// Parse time trigger parameters
    fn parse_time_params(
        &self,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<TimeTriggerParams, TriggerError> {
        let cron = params
            .get("cron")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing cron parameter".to_string()))?
            .to_string();
        
        let timezone = params
            .get("timezone")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(TimeTriggerParams {
            cron,
            timezone,
        })
    }
    
    /// Parse market trigger parameters
    fn parse_market_params(
        &self,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<MarketTriggerParams, TriggerError> {
        let asset_pair = params
            .get("asset_pair")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing asset_pair parameter".to_string()))?
            .to_string();
        
        let condition = params
            .get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing condition parameter".to_string()))?
            .to_string();
        
        let price = params
            .get("price")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing price parameter".to_string()))?;
        
        Ok(MarketTriggerParams {
            asset_pair,
            condition,
            price,
        })
    }
    
    /// Parse custom trigger parameters
    fn parse_custom_params(
        &self,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<CustomTriggerParams, TriggerError> {
        let event_name = params
            .get("event_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::InvalidParameters("Missing event_name parameter".to_string()))?
            .to_string();
        
        let event_data = params
            .get("event_data")
            .cloned();
        
        Ok(CustomTriggerParams {
            event_name,
            event_data,
        })
    }
    
    /// Evaluate blockchain trigger
    async fn evaluate_blockchain_trigger(
        &self,
        params: &HashMap<String, serde_json::Value>,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        let blockchain_params = self.parse_blockchain_params(params)?;
        
        // Extract event data
        let event_network = event_data
            .get("network")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::EvaluationError("Missing network in event data".to_string()))?;
        
        // Check network
        if event_network != blockchain_params.network {
            return Ok(false);
        }
        
        // Check contract address if specified
        if let Some(contract_address) = &blockchain_params.contract_address {
            let event_contract = event_data
                .get("contract_address")
                .and_then(|v| v.as_str())
                .ok_or_else(|| TriggerError::EvaluationError("Missing contract_address in event data".to_string()))?;
            
            if event_contract != contract_address {
                return Ok(false);
            }
        }
        
        // Check event name if specified
        if let Some(event_name) = &blockchain_params.event_name {
            let event_type = event_data
                .get("event_name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| TriggerError::EvaluationError("Missing event_name in event data".to_string()))?;
            
            if event_type != event_name {
                return Ok(false);
            }
        }
        
        // Check method name if specified
        if let Some(method_name) = &blockchain_params.method_name {
            let event_method = event_data
                .get("method_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            if event_method != method_name {
                return Ok(false);
            }
        }
        
        // Check block number if specified
        if let Some(block_number) = blockchain_params.block_number {
            let event_block = event_data
                .get("block_number")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| TriggerError::EvaluationError("Missing block_number in event data".to_string()))?;
            
            if event_block < block_number {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Evaluate time trigger
    async fn evaluate_time_trigger(
        &self,
        params: &HashMap<String, serde_json::Value>,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        let time_params = self.parse_time_params(params)?;
        
        // Extract event data
        let event_timestamp = event_data
            .get("timestamp")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| TriggerError::EvaluationError("Missing timestamp in event data".to_string()))?;
        
        // TODO: Implement cron expression evaluation
        // For now, we'll just return true for time triggers
        Ok(true)
    }
    
    /// Evaluate market trigger
    async fn evaluate_market_trigger(
        &self,
        params: &HashMap<String, serde_json::Value>,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        let market_params = self.parse_market_params(params)?;
        
        // Extract event data
        let event_asset_pair = event_data
            .get("asset_pair")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::EvaluationError("Missing asset_pair in event data".to_string()))?;
        
        let event_price = event_data
            .get("price")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| TriggerError::EvaluationError("Missing price in event data".to_string()))?;
        
        // Check asset pair
        if event_asset_pair != market_params.asset_pair {
            return Ok(false);
        }
        
        // Check price condition
        match market_params.condition.as_str() {
            "above" => Ok(event_price > market_params.price),
            "below" => Ok(event_price < market_params.price),
            "equal" => Ok((event_price - market_params.price).abs() < 0.000001),
            _ => Err(TriggerError::InvalidParameters(format!("Invalid condition: {}", market_params.condition))),
        }
    }
    
    /// Evaluate custom trigger
    async fn evaluate_custom_trigger(
        &self,
        params: &HashMap<String, serde_json::Value>,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        let custom_params = self.parse_custom_params(params)?;
        
        // Extract event data
        let event_name = event_data
            .get("event_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TriggerError::EvaluationError("Missing event_name in event data".to_string()))?;
        
        // Check event name
        if event_name != custom_params.event_name {
            return Ok(false);
        }
        
        // If event data is specified, check it
        if let Some(expected_data) = &custom_params.event_data {
            let actual_data = event_data
                .get("event_data")
                .ok_or_else(|| TriggerError::EvaluationError("Missing event_data in event data".to_string()))?;
            
            // Simple equality check for now
            // TODO: Implement more sophisticated matching
            if expected_data != actual_data {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

#[async_trait]
impl TriggerService for TriggerServiceImpl {
    async fn register_trigger(
        &self,
        user_id: &str,
        function_id: &str,
        condition: TriggerCondition,
    ) -> Result<String, TriggerError> {
        // Generate a unique trigger ID
        let trigger_id = uuid::Uuid::new_v4().to_string();
        
        // Store the trigger
        let mut triggers = self.triggers.write().await;
        triggers.insert(
            trigger_id.clone(),
            (user_id.to_string(), function_id.to_string(), condition),
        );
        
        Ok(trigger_id)
    }
    
    async fn unregister_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(), TriggerError> {
        let mut triggers = self.triggers.write().await;
        
        if triggers.remove(trigger_id).is_none() {
            return Err(TriggerError::InvalidParameters(format!("Trigger not found: {}", trigger_id)));
        }
        
        Ok(())
    }
    
    async fn get_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(String, String, TriggerCondition), TriggerError> {
        let triggers = self.triggers.read().await;
        
        triggers
            .get(trigger_id)
            .cloned()
            .ok_or_else(|| TriggerError::InvalidParameters(format!("Trigger not found: {}", trigger_id)))
    }
    
    async fn list_function_triggers(
        &self,
        function_id: &str,
    ) -> Result<Vec<(String, TriggerCondition)>, TriggerError> {
        let triggers = self.triggers.read().await;
        
        let function_triggers = triggers
            .iter()
            .filter(|(_, (_, fn_id, _))| fn_id == function_id)
            .map(|(id, (_, _, condition))| (id.clone(), condition.clone()))
            .collect();
        
        Ok(function_triggers)
    }
    
    async fn evaluate_trigger(
        &self,
        condition: &TriggerCondition,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        match condition.source {
            TriggerSource::Blockchain => {
                self.evaluate_blockchain_trigger(&condition.params, event_data).await
            },
            TriggerSource::Time => {
                self.evaluate_time_trigger(&condition.params, event_data).await
            },
            TriggerSource::Market => {
                self.evaluate_market_trigger(&condition.params, event_data).await
            },
            TriggerSource::Custom => {
                self.evaluate_custom_trigger(&condition.params, event_data).await
            },
        }
    }
}
