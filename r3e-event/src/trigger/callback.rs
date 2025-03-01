// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Trigger callback status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerCallbackStatus {
    /// Callback is pending execution
    Pending,

    /// Callback is being executed
    Executing,

    /// Callback was executed successfully
    Success,

    /// Callback execution failed
    Failed,

    /// Callback execution timed out
    Timeout,
}

/// Trigger callback result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCallbackResult {
    /// Callback ID
    pub id: String,

    /// Trigger ID
    pub trigger_id: String,

    /// User ID
    pub user_id: String,

    /// Function ID
    pub function_id: String,

    /// Callback status
    pub status: TriggerCallbackStatus,

    /// Callback result
    pub result: Option<serde_json::Value>,

    /// Callback error
    pub error: Option<String>,

    /// Callback execution time
    pub execution_time: Option<Duration>,

    /// Callback created at
    pub created_at: DateTime<Utc>,

    /// Callback updated at
    pub updated_at: DateTime<Utc>,
}

impl TriggerCallbackResult {
    /// Create a new trigger callback result
    pub fn new(id: String, trigger_id: String, user_id: String, function_id: String) -> Self {
        let now = Utc::now();

        Self {
            id,
            trigger_id,
            user_id,
            function_id,
            status: TriggerCallbackStatus::Pending,
            result: None,
            error: None,
            execution_time: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the callback status to executing
    pub fn set_executing(&mut self) {
        self.status = TriggerCallbackStatus::Executing;
        self.updated_at = Utc::now();
    }

    /// Set the callback result
    pub fn set_result(&mut self, result: serde_json::Value, execution_time: Duration) {
        self.status = TriggerCallbackStatus::Success;
        self.result = Some(result);
        self.execution_time = Some(execution_time);
        self.updated_at = Utc::now();
    }

    /// Set the callback error
    pub fn set_error(&mut self, error: String, execution_time: Option<Duration>) {
        self.status = TriggerCallbackStatus::Failed;
        self.error = Some(error);
        self.execution_time = execution_time;
        self.updated_at = Utc::now();
    }

    /// Set the callback timeout
    pub fn set_timeout(&mut self, execution_time: Duration) {
        self.status = TriggerCallbackStatus::Timeout;
        self.error = Some("Callback execution timed out".to_string());
        self.execution_time = Some(execution_time);
        self.updated_at = Utc::now();
    }
}

/// Trigger callback storage trait
#[async_trait::async_trait]
pub trait TriggerCallbackStorage: Send + Sync {
    /// Store a trigger callback result
    async fn store_callback(&self, callback: &TriggerCallbackResult) -> Result<(), String>;

    /// Get a trigger callback result by ID
    async fn get_callback(
        &self,
        callback_id: &str,
    ) -> Result<Option<TriggerCallbackResult>, String>;

    /// List trigger callback results for a trigger
    async fn list_trigger_callbacks(
        &self,
        trigger_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String>;

    /// List trigger callback results for a function
    async fn list_function_callbacks(
        &self,
        function_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String>;

    /// List trigger callback results for a user
    async fn list_user_callbacks(
        &self,
        user_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String>;
}

/// In-memory trigger callback storage
pub struct InMemoryTriggerCallbackStorage {
    /// Callbacks
    callbacks: tokio::sync::RwLock<Vec<TriggerCallbackResult>>,
}

impl InMemoryTriggerCallbackStorage {
    /// Create a new in-memory trigger callback storage
    pub fn new() -> Self {
        Self {
            callbacks: tokio::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl TriggerCallbackStorage for InMemoryTriggerCallbackStorage {
    async fn store_callback(&self, callback: &TriggerCallbackResult) -> Result<(), String> {
        let mut callbacks = self.callbacks.write().await;

        // Check if the callback already exists
        let index = callbacks.iter().position(|c| c.id == callback.id);

        if let Some(index) = index {
            // Update existing callback
            callbacks[index] = callback.clone();
        } else {
            // Add new callback
            callbacks.push(callback.clone());
        }

        Ok(())
    }

    async fn get_callback(
        &self,
        callback_id: &str,
    ) -> Result<Option<TriggerCallbackResult>, String> {
        let callbacks = self.callbacks.read().await;

        let callback = callbacks.iter().find(|c| c.id == callback_id).cloned();

        Ok(callback)
    }

    async fn list_trigger_callbacks(
        &self,
        trigger_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String> {
        let callbacks = self.callbacks.read().await;

        let trigger_callbacks = callbacks
            .iter()
            .filter(|c| c.trigger_id == trigger_id)
            .cloned()
            .collect();

        Ok(trigger_callbacks)
    }

    async fn list_function_callbacks(
        &self,
        function_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String> {
        let callbacks = self.callbacks.read().await;

        let function_callbacks = callbacks
            .iter()
            .filter(|c| c.function_id == function_id)
            .cloned()
            .collect();

        Ok(function_callbacks)
    }

    async fn list_user_callbacks(
        &self,
        user_id: &str,
    ) -> Result<Vec<TriggerCallbackResult>, String> {
        let callbacks = self.callbacks.read().await;

        let user_callbacks = callbacks
            .iter()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect();

        Ok(user_callbacks)
    }
}
