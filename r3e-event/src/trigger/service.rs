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
    
    /// Process an event and execute callbacks for matching triggers
    async fn process_event(
        &self,
        event_data: &serde_json::Value,
    ) -> Result<Vec<String>, TriggerError>;
    
    /// Execute a callback for a matched trigger
    async fn execute_callback(
        &self,
        user_id: &str,
        function_id: &str,
        trigger_id: &str,
        event_data: &serde_json::Value,
    ) -> Result<String, TriggerError>;
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
        
        // Convert timestamp to DateTime<Utc>
        let event_time = chrono::DateTime::<chrono::Utc>::from_timestamp(
            event_timestamp as i64,
            0,
        ).ok_or_else(|| TriggerError::EvaluationError("Invalid timestamp".to_string()))?;
        
        // Parse the cron expression
        let schedule = cron::Schedule::from_str(&time_params.cron)
            .map_err(|e| TriggerError::InvalidParameters(format!("Invalid cron expression: {}", e)))?;
        
        // Get the timezone
        let timezone = match time_params.timezone.as_deref() {
            Some(tz) => chrono_tz::Tz::from_str(tz)
                .map_err(|_| TriggerError::InvalidParameters(format!("Invalid timezone: {}", tz)))?,
            None => chrono_tz::UTC,
        };
        
        // Convert UTC time to the specified timezone
        let event_time_tz = event_time.with_timezone(&timezone);
        
        // Check if the event time matches the cron schedule
        // We'll check if the event time is within 1 minute of a scheduled time
        let prev_time = schedule.prev_from(event_time_tz)
            .ok_or_else(|| TriggerError::EvaluationError("Failed to get previous scheduled time".to_string()))?;
        
        let time_diff = event_time_tz.signed_duration_since(prev_time);
        
        // If the event time is within 1 minute of a scheduled time, consider it a match
        Ok(time_diff.num_seconds().abs() <= 60)
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
        
        // Extract optional event data
        let event_volume = event_data
            .get("volume")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
            
        let event_timestamp = event_data
            .get("timestamp")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
            
        let event_exchange = event_data
            .get("exchange")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        // Check asset pair
        if event_asset_pair != market_params.asset_pair {
            return Ok(false);
        }
        
        // Check if exchange is specified in parameters
        if let Some(exchange) = params.get("exchange").and_then(|v| v.as_str()) {
            if exchange != event_exchange {
                return Ok(false);
            }
        }
        
        // Check if minimum volume is specified in parameters
        if let Some(min_volume) = params.get("min_volume").and_then(|v| v.as_f64()) {
            if event_volume < min_volume {
                return Ok(false);
            }
        }
        
        // Check if time range is specified in parameters
        if let Some(start_time) = params.get("start_time").and_then(|v| v.as_u64()) {
            if event_timestamp < start_time {
                return Ok(false);
            }
        }
        
        if let Some(end_time) = params.get("end_time").and_then(|v| v.as_u64()) {
            if event_timestamp > end_time {
                return Ok(false);
            }
        }
        
        // Check price condition
        match market_params.condition.as_str() {
            "above" => Ok(event_price > market_params.price),
            "below" => Ok(event_price < market_params.price),
            "equal" => Ok((event_price - market_params.price).abs() < 0.000001),
            "percent_increase" => {
                // Check if previous price is specified in parameters
                let previous_price = params
                    .get("previous_price")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| TriggerError::InvalidParameters("Missing previous_price parameter for percent_increase condition".to_string()))?;
                
                let percent_change = (event_price - previous_price) / previous_price * 100.0;
                Ok(percent_change >= market_params.price)
            },
            "percent_decrease" => {
                // Check if previous price is specified in parameters
                let previous_price = params
                    .get("previous_price")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| TriggerError::InvalidParameters("Missing previous_price parameter for percent_decrease condition".to_string()))?;
                
                let percent_change = (previous_price - event_price) / previous_price * 100.0;
                Ok(percent_change >= market_params.price)
            },
            "range" => {
                // Check if upper bound is specified in parameters
                let upper_bound = params
                    .get("upper_bound")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| TriggerError::InvalidParameters("Missing upper_bound parameter for range condition".to_string()))?;
                
                Ok(event_price >= market_params.price && event_price <= upper_bound)
            },
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
            
            // Get the matching mode from parameters
            let matching_mode = params
                .get("matching_mode")
                .and_then(|v| v.as_str())
                .unwrap_or("exact");
            
            match matching_mode {
                "exact" => {
                    // Exact equality check
                    if expected_data != actual_data {
                        return Ok(false);
                    }
                },
                "partial" => {
                    // Partial matching - check if all fields in expected_data exist in actual_data with the same values
                    if let (Some(expected_obj), Some(actual_obj)) = (expected_data.as_object(), actual_data.as_object()) {
                        for (key, value) in expected_obj {
                            if !actual_obj.contains_key(key) || actual_obj[key] != *value {
                                return Ok(false);
                            }
                        }
                    } else {
                        // If not objects, fall back to exact matching
                        if expected_data != actual_data {
                            return Ok(false);
                        }
                    }
                },
                "regex" => {
                    // Regex matching - check if the string value in expected_data matches the regex pattern
                    if let (Some(pattern), Some(text)) = (expected_data.as_str(), actual_data.as_str()) {
                        let regex = regex::Regex::new(pattern)
                            .map_err(|e| TriggerError::InvalidParameters(format!("Invalid regex pattern: {}", e)))?;
                        
                        if !regex.is_match(text) {
                            return Ok(false);
                        }
                    } else {
                        // If not strings, fall back to exact matching
                        if expected_data != actual_data {
                            return Ok(false);
                        }
                    }
                },
                "jsonpath" => {
                    // JSONPath matching - check if the JSONPath expression in expected_data matches the actual_data
                    if let Some(path) = expected_data.as_str() {
                        // Simple JSONPath implementation for basic cases
                        // In a real implementation, we would use a proper JSONPath library
                        
                        // For now, we'll just support simple path expressions like "$.field1.field2"
                        let path_parts: Vec<&str> = path.trim_start_matches("$.").split('.').collect();
                        
                        let mut current_value = actual_data;
                        for part in path_parts {
                            current_value = match current_value.get(part) {
                                Some(value) => value,
                                None => return Ok(false),
                            };
                        }
                        
                        // Check if the value at the path matches the expected value
                        let expected_value = params
                            .get("expected_value")
                            .ok_or_else(|| TriggerError::InvalidParameters("Missing expected_value parameter for jsonpath matching".to_string()))?;
                        
                        if current_value != expected_value {
                            return Ok(false);
                        }
                    } else {
                        return Err(TriggerError::InvalidParameters("Invalid JSONPath expression".to_string()));
                    }
                },
                _ => return Err(TriggerError::InvalidParameters(format!("Invalid matching mode: {}", matching_mode))),
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
    
    async fn process_event(
        &self,
        event_data: &serde_json::Value,
    ) -> Result<Vec<String>, TriggerError> {
        let triggers = self.triggers.read().await;
        let mut callback_ids = Vec::new();
        
        // Iterate through all triggers and evaluate them against the event data
        for (trigger_id, (user_id, function_id, condition)) in triggers.iter() {
            match self.evaluate_trigger(condition, event_data).await {
                Ok(true) => {
                    // Trigger condition matched, execute callback
                    match self.execute_callback(user_id, function_id, trigger_id, event_data).await {
                        Ok(callback_id) => {
                            callback_ids.push(callback_id);
                        },
                        Err(e) => {
                            log::error!("Failed to execute callback for trigger {}: {}", trigger_id, e);
                        }
                    }
                },
                Ok(false) => {
                    // Trigger condition did not match, do nothing
                },
                Err(e) => {
                    log::error!("Failed to evaluate trigger {}: {}", trigger_id, e);
                }
            }
        }
        
        Ok(callback_ids)
    }
    
    async fn execute_callback(
        &self,
        user_id: &str,
        function_id: &str,
        trigger_id: &str,
        event_data: &serde_json::Value,
    ) -> Result<String, TriggerError> {
        use std::time::{Duration, Instant};
        use crate::trigger::callback::{TriggerCallbackResult, TriggerCallbackStatus};
        
        // Generate a unique callback ID
        let callback_id = uuid::Uuid::new_v4().to_string();
        
        // Create a new callback result
        let mut callback_result = TriggerCallbackResult::new(
            callback_id.clone(),
            trigger_id.to_string(),
            user_id.to_string(),
            function_id.to_string(),
        );
        
        // Log the callback execution
        log::info!(
            "Executing callback for trigger {} (user: {}, function: {})",
            trigger_id, user_id, function_id
        );
        
        // Update callback status to executing
        callback_result.set_executing();
        
        // Start execution timer
        let start_time = Instant::now();
        
        // Create callback data for function invocation
        let callback_data = serde_json::json!({
            "callback_id": callback_id,
            "trigger_id": trigger_id,
            "user_id": user_id,
            "function_id": function_id,
            "event_data": event_data,
            "timestamp": chrono::Utc::now().timestamp(),
        });
        
        // Log callback data
        log::debug!("Callback data: {}", callback_data);
        
        // In a production implementation, we would use a function service to invoke the function
        // For now, we'll simulate a successful execution
        
        // Simulate function execution
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Calculate execution time
        let execution_time = start_time.elapsed();
        
        // Update callback result with success
        callback_result.set_result(
            serde_json::json!({
                "status": "success",
                "message": "Function executed successfully",
                "data": callback_data,
            }),
            execution_time,
        );
        
        // In a production implementation, we would store the callback result in a database
        
        Ok(callback_id)
    }
}
