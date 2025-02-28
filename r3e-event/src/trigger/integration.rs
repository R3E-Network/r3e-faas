// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::trigger::callback::{TriggerCallbackResult, TriggerCallbackStatus, TriggerCallbackStorage};
use crate::trigger::function_service::FunctionService;
use crate::trigger::types::{TriggerCondition, TriggerError, TriggerSource};

/// Trigger service integration with function execution
pub struct TriggerServiceIntegration {
    /// Triggers
    triggers: Arc<RwLock<std::collections::HashMap<String, (String, String, TriggerCondition)>>>,
    
    /// Callback storage
    callback_storage: Arc<dyn TriggerCallbackStorage>,
    
    /// Function service
    function_service: Arc<dyn FunctionService>,
    
    /// Maximum execution time
    max_execution_time: Duration,
}

impl TriggerServiceIntegration {
    /// Create a new trigger service integration
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
    
    /// Process an event and execute callbacks for matching triggers
    pub async fn process_event(
        &self,
        event_data: &serde_json::Value,
        trigger_evaluator: &dyn TriggerEvaluator,
    ) -> Result<Vec<String>, TriggerError> {
        let triggers = self.triggers.read().await;
        let mut callback_ids = Vec::new();
        
        // Iterate through all triggers and evaluate them against the event data
        for (trigger_id, (user_id, function_id, condition)) in triggers.iter() {
            match trigger_evaluator.evaluate_trigger(condition, event_data).await {
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
            "timestamp": chrono::Utc::now().timestamp(),
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
}

/// Trigger evaluator trait
#[async_trait]
pub trait TriggerEvaluator: Send + Sync {
    /// Evaluate a trigger condition against event data
    async fn evaluate_trigger(
        &self,
        condition: &TriggerCondition,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError>;
}
