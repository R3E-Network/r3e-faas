// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use chrono::Utc;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::trigger::callback::{TriggerCallbackResult, TriggerCallbackStatus, TriggerCallbackStorage};
use crate::trigger::types::{TriggerCondition, TriggerError, TriggerSource};

/// Function service trait for executing functions
#[async_trait]
pub trait FunctionService: Send + Sync {
    /// Execute a function
    async fn execute_function(
        &self,
        user_id: &str,
        function_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, String>;
}

/// Trigger service implementation with function execution
pub struct TriggerServiceWithFunction {
    /// Triggers
    triggers: Arc<RwLock<std::collections::HashMap<String, (String, String, TriggerCondition)>>>,
    
    /// Callback storage
    callback_storage: Arc<dyn TriggerCallbackStorage>,
    
    /// Function service
    function_service: Arc<dyn FunctionService>,
    
    /// Maximum execution time
    max_execution_time: Duration,
}

impl TriggerServiceWithFunction {
    /// Create a new trigger service with function execution
    pub fn new(
        callback_storage: Arc<dyn TriggerCallbackStorage>,
        function_service: Arc<dyn FunctionService>,
    ) -> Self {
        Self {
            triggers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            callback_storage,
            function_service,
            max_execution_time: Duration::from_secs(30),
        }
    }
    
    /// Set the maximum execution time
    pub fn with_max_execution_time(mut self, max_execution_time: Duration) -> Self {
        self.max_execution_time = max_execution_time;
        self
    }
    
    /// Register a trigger
    pub async fn register_trigger(
        &self,
        user_id: &str,
        function_id: &str,
        condition: TriggerCondition,
    ) -> Result<String, TriggerError> {
        // Generate a unique trigger ID
        let trigger_id = Uuid::new_v4().to_string();
        
        // Add the trigger to the map
        let mut triggers = self.triggers.write().await;
        triggers.insert(
            trigger_id.clone(),
            (user_id.to_string(), function_id.to_string(), condition),
        );
        
        Ok(trigger_id)
    }
    
    /// Unregister a trigger
    pub async fn unregister_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(), TriggerError> {
        // Remove the trigger from the map
        let mut triggers = self.triggers.write().await;
        
        if triggers.remove(trigger_id).is_none() {
            return Err(TriggerError::InvalidParameters(format!("Trigger not found: {}", trigger_id)));
        }
        
        Ok(())
    }
    
    /// Get a trigger
    pub async fn get_trigger(
        &self,
        trigger_id: &str,
    ) -> Result<(String, String, TriggerCondition), TriggerError> {
        let triggers = self.triggers.read().await;
        
        triggers
            .get(trigger_id)
            .cloned()
            .ok_or_else(|| TriggerError::InvalidParameters(format!("Trigger not found: {}", trigger_id)))
    }
    
    /// List function triggers
    pub async fn list_function_triggers(
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
    
    /// Evaluate a trigger condition against event data
    pub async fn evaluate_trigger(
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
    
    /// Process an event and execute callbacks for matching triggers
    pub async fn process_event(
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
                            error!("Failed to execute callback for trigger {}: {}", trigger_id, e);
                        }
                    }
                },
                Ok(false) => {
                    // Trigger condition did not match, do nothing
                },
                Err(e) => {
                    error!("Failed to evaluate trigger {}: {}", trigger_id, e);
                }
            }
        }
        
        Ok(callback_ids)
    }
    
    /// Execute a callback for a trigger
    pub async fn execute_callback(
        &self,
        user_id: &str,
        function_id: &str,
        trigger_id: &str,
        event_data: &serde_json::Value,
    ) -> Result<String, TriggerError> {
        // Generate a unique callback ID
        let callback_id = Uuid::new_v4().to_string();
        
        // Create a new callback result
        let mut callback_result = TriggerCallbackResult::new(
            callback_id.clone(),
            trigger_id.to_string(),
            user_id.to_string(),
            function_id.to_string(),
        );
        
        // Log the callback execution
        info!(
            "Executing callback for trigger {} (user: {}, function: {})",
            trigger_id, user_id, function_id
        );
        
        // Update callback status to executing
        callback_result.set_executing();
        
        // Store the initial callback result
        if let Err(e) = self.callback_storage.store_callback(&callback_result).await {
            error!("Failed to store callback result: {}", e);
        }
        
        // Start execution timer
        let start_time = Instant::now();
        
        // Create callback data for function invocation
        let callback_data = json!({
            "callback_id": callback_id,
            "trigger_id": trigger_id,
            "user_id": user_id,
            "function_id": function_id,
            "event_data": event_data,
            "timestamp": Utc::now().timestamp(),
        });
        
        // Log callback data
        debug!("Callback data: {}", callback_data);
        
        // Execute the function
        let execution_result = tokio::time::timeout(
            self.max_execution_time,
            self.function_service.execute_function(user_id, function_id, callback_data.clone()),
        ).await;
        
        // Calculate execution time
        let execution_time = start_time.elapsed();
        
        // Process execution result
        match execution_result {
            Ok(Ok(result)) => {
                // Function executed successfully
                callback_result.set_result(result, execution_time);
                info!(
                    "Callback executed successfully for trigger {} (execution time: {:?})",
                    trigger_id, execution_time
                );
            },
            Ok(Err(error)) => {
                // Function execution failed
                callback_result.set_error(error, Some(execution_time));
                warn!(
                    "Callback execution failed for trigger {}: {} (execution time: {:?})",
                    trigger_id, error, execution_time
                );
            },
            Err(_) => {
                // Function execution timed out
                callback_result.set_timeout(execution_time);
                warn!(
                    "Callback execution timed out for trigger {} (execution time: {:?})",
                    trigger_id, execution_time
                );
            }
        }
        
        // Store the final callback result
        if let Err(e) = self.callback_storage.store_callback(&callback_result).await {
            error!("Failed to store callback result: {}", e);
        }
        
        Ok(callback_id)
    }
    
    /// Evaluate a blockchain trigger
    async fn evaluate_blockchain_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse blockchain parameters
        let network = params.get("network").and_then(|v| v.as_str()).unwrap_or("*");
        let contract_address = params.get("contract_address").and_then(|v| v.as_str()).unwrap_or("*");
        let event_name = params.get("event_name").and_then(|v| v.as_str()).unwrap_or("*");
        let method_name = params.get("method_name").and_then(|v| v.as_str()).unwrap_or("*");
        let block_number = params.get("block_number").and_then(|v| v.as_u64());
        
        // Extract event data
        let event_network = event_data.get("network").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check network
        if network != "*" && network != event_network {
            return Ok(false);
        }
        
        // Extract contract address
        let event_contract = event_data.get("contract_address").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check contract address
        if contract_address != "*" && contract_address != event_contract {
            return Ok(false);
        }
        
        // Extract event type
        let event_type = event_data.get("event_name").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check event name
        if event_name != "*" && event_name != event_type {
            return Ok(false);
        }
        
        // Extract method name
        let event_method = event_data.get("method_name").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check method name
        if method_name != "*" && method_name != event_method {
            return Ok(false);
        }
        
        // Extract block number
        let event_block = event_data.get("block_number").and_then(|v| v.as_u64()).unwrap_or(0);
        
        // Check block number
        if let Some(min_block) = block_number {
            if event_block < min_block {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Evaluate a time trigger
    async fn evaluate_time_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse time parameters
        let cron = params.get("cron").and_then(|v| v.as_str()).unwrap_or("* * * * *");
        let timezone = params.get("timezone").and_then(|v| v.as_str()).unwrap_or("UTC");
        
        // Extract event data
        let event_timestamp = event_data.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0);
        
        // Convert timestamp to datetime
        let event_time = chrono::DateTime::from_timestamp(event_timestamp, 0)
            .ok_or_else(|| TriggerError::InvalidParameters("Invalid timestamp".to_string()))?;
        
        // Parse cron expression
        let schedule = cron_parser::parse(cron)
            .map_err(|e| TriggerError::InvalidParameters(format!("Invalid cron expression: {}", e)))?;
        
        // Parse timezone
        let timezone = chrono_tz::Tz::from_str_insensitive(timezone)
            .map_err(|_| TriggerError::InvalidParameters(format!("Invalid timezone: {}", timezone)))?;
        
        // Check if the event time matches the cron schedule
        let prev_time = schedule.prev_from(event_time.with_timezone(&timezone))
            .ok_or_else(|| TriggerError::InvalidParameters("Failed to calculate previous cron time".to_string()))?;
        
        // Check if the event time is within 1 minute of the scheduled time
        let diff = (event_time.timestamp() - prev_time.timestamp()).abs();
        
        Ok(diff <= 60)
    }
    
    /// Evaluate a market trigger
    async fn evaluate_market_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse market parameters
        let asset_pair = params.get("asset_pair").and_then(|v| v.as_str()).unwrap_or("*");
        let condition = params.get("condition").and_then(|v| v.as_str()).unwrap_or("eq");
        let price = params.get("price").and_then(|v| v.as_f64());
        
        // Extract event data
        let event_asset_pair = event_data.get("asset_pair").and_then(|v| v.as_str()).unwrap_or("");
        let event_price = event_data.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let event_volume = event_data.get("volume").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let event_timestamp = event_data.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0);
        let event_exchange = event_data.get("exchange").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check asset pair
        if asset_pair != "*" && asset_pair != event_asset_pair {
            return Ok(false);
        }
        
        // Check price condition
        if let Some(target_price) = price {
            match condition {
                "eq" => {
                    // Equal to
                    if (event_price - target_price).abs() > 0.000001 {
                        return Ok(false);
                    }
                },
                "gt" => {
                    // Greater than
                    if event_price <= target_price {
                        return Ok(false);
                    }
                },
                "lt" => {
                    // Less than
                    if event_price >= target_price {
                        return Ok(false);
                    }
                },
                "gte" => {
                    // Greater than or equal to
                    if event_price < target_price {
                        return Ok(false);
                    }
                },
                "lte" => {
                    // Less than or equal to
                    if event_price > target_price {
                        return Ok(false);
                    }
                },
                "pct_change" => {
                    // Percentage change
                    // Get previous price from event data
                    let previous_price = event_data.get("previous_price").and_then(|v| v.as_f64());
                    
                    if let Some(prev_price) = previous_price {
                        // Calculate percentage change
                        let pct_change = (event_price - prev_price) / prev_price * 100.0;
                        
                        // Check if percentage change is greater than or equal to target
                        if pct_change.abs() < target_price.abs() {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                },
                "range" => {
                    // Price is within range
                    // Get upper bound from params
                    let upper_bound = params.get("upper_bound").and_then(|v| v.as_f64());
                    
                    if let Some(upper) = upper_bound {
                        // Check if price is within range [target_price, upper_bound]
                        if event_price < target_price || event_price > upper {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                },
                _ => {
                    return Err(TriggerError::InvalidParameters(format!("Invalid price condition: {}", condition)));
                }
            }
        }
        
        Ok(true)
    }
    
    /// Evaluate a custom trigger
    async fn evaluate_custom_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse custom parameters
        let event_name = params.get("event_name").and_then(|v| v.as_str()).unwrap_or("*");
        let event_data_filter = params.get("event_data");
        
        // Extract event data
        let event_name_actual = event_data.get("event_name").and_then(|v| v.as_str()).unwrap_or("");
        
        // Check event name
        if event_name != "*" && event_name != event_name_actual {
            return Ok(false);
        }
        
        // Extract actual data
        let actual_data = event_data.get("data");
        
        // Check event data
        if let (Some(filter), Some(actual)) = (event_data_filter, actual_data) {
            // Get matching mode
            let matching_mode = params.get("matching_mode").and_then(|v| v.as_str()).unwrap_or("exact");
            
            match matching_mode {
                "exact" => {
                    // Exact match
                    if filter != actual {
                        return Ok(false);
                    }
                },
                "partial" => {
                    // Partial match (check if filter is a subset of actual)
                    if let (Some(filter_obj), Some(actual_obj)) = (filter.as_object(), actual.as_object()) {
                        for (key, value) in filter_obj {
                            if !actual_obj.contains_key(key) || actual_obj[key] != *value {
                                return Ok(false);
                            }
                        }
                    } else if let (Some(filter_arr), Some(actual_arr)) = (filter.as_array(), actual.as_array()) {
                        for value in filter_arr {
                            if !actual_arr.contains(value) {
                                return Ok(false);
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                },
                "regex" => {
                    // Regex match
                    if let (Some(filter_str), Some(actual_str)) = (filter.as_str(), actual.as_str()) {
                        let regex = regex::Regex::new(filter_str)
                            .map_err(|e| TriggerError::InvalidParameters(format!("Invalid regex: {}", e)))?;
                        
                        if !regex.is_match(actual_str) {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                },
                "jsonpath" => {
                    // JSONPath match
                    if let Some(filter_str) = filter.as_str() {
                        // In a real implementation, we would use a JSONPath library
                        // For now, we'll just check if the filter string is "*" (match all)
                        if filter_str != "*" {
                            // Simple check for key existence
                            let key = filter_str.trim_start_matches("$.");
                            let expected_value = params.get("expected_value");
                            
                            if let Some(expected) = expected_value {
                                // Check if the key exists and has the expected value
                                let actual_value = jsonpath_lib::select(actual, filter_str)
                                    .map_err(|e| TriggerError::InvalidParameters(format!("Invalid JSONPath: {}", e)))?;
                                
                                if actual_value.is_empty() || actual_value[0] != *expected {
                                    return Ok(false);
                                }
                            } else {
                                // Just check if the key exists
                                let actual_value = jsonpath_lib::select(actual, filter_str)
                                    .map_err(|e| TriggerError::InvalidParameters(format!("Invalid JSONPath: {}", e)))?;
                                
                                if actual_value.is_empty() {
                                    return Ok(false);
                                }
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                },
                _ => {
                    return Err(TriggerError::InvalidParameters(format!("Invalid matching mode: {}", matching_mode)));
                }
            }
        }
        
        Ok(true)
    }
}
