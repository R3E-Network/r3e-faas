// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

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

/// Worker function service implementation
pub struct WorkerFunctionService {
    /// Worker service URL
    worker_url: String,

    /// HTTP client
    client: reqwest::Client,

    /// Request timeout
    timeout: Duration,
}

impl WorkerFunctionService {
    /// Create a new worker function service
    pub fn new(worker_url: &str) -> Self {
        Self {
            worker_url: worker_url.to_string(),
            client: reqwest::Client::new(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait]
impl FunctionService for WorkerFunctionService {
    async fn execute_function(
        &self,
        user_id: &str,
        function_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // Create the request URL
        let url = format!("{}/functions/{}/invoke", self.worker_url, function_id);

        // Create the request body
        let body = json!({
            "user_id": user_id,
            "input": input,
        });

        // Log the function execution request
        debug!(
            "Executing function {} for user {} with input: {}",
            function_id, user_id, input
        );

        // Execute the function
        let response = self
            .client
            .post(&url)
            .json(&body)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| format!("Failed to send function execution request: {}", e))?;

        // Check the response status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error text".to_string());

            return Err(format!(
                "Function execution failed with status {}: {}",
                status, error_text
            ));
        }

        // Parse the response
        let result = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse function execution response: {}", e))?;

        // Log the function execution result
        debug!(
            "Function {} executed successfully for user {} with result: {}",
            function_id, user_id, result
        );

        Ok(result)
    }
}

/// Mock function service implementation for testing
pub struct MockFunctionService {
    /// Execution delay
    delay: Duration,

    /// Execution results
    results: Arc<RwLock<std::collections::HashMap<String, Result<serde_json::Value, String>>>>,
}

impl MockFunctionService {
    /// Create a new mock function service
    pub fn new() -> Self {
        Self {
            delay: Duration::from_millis(100),
            results: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Set the execution delay
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set a function execution result
    pub async fn set_result(&self, function_id: &str, result: Result<serde_json::Value, String>) {
        let mut results = self.results.write().await;
        results.insert(function_id.to_string(), result);
    }
}

#[async_trait]
impl FunctionService for MockFunctionService {
    async fn execute_function(
        &self,
        user_id: &str,
        function_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // Log the function execution request
        debug!(
            "Mock executing function {} for user {} with input: {}",
            function_id, user_id, input
        );

        // Simulate execution delay
        tokio::time::sleep(self.delay).await;

        // Get the function execution result
        let results = self.results.read().await;

        if let Some(result) = results.get(function_id) {
            // Return the predefined result
            result.clone()
        } else {
            // Return a default success result
            Ok(json!({
                "status": "success",
                "message": "Function executed successfully (mock)",
                "function_id": function_id,
                "user_id": user_id,
                "input": input,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}

/// Direct function service implementation using the worker service directly
pub struct DirectFunctionService {
    /// Worker service
    worker_service: Arc<dyn WorkerServiceTrait>,
}

/// Worker service trait
#[async_trait]
pub trait WorkerServiceTrait: Send + Sync {
    /// Invoke a function
    async fn invoke_function(
        &self,
        user_id: &str,
        function_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, String>;
}

impl DirectFunctionService {
    /// Create a new direct function service
    pub fn new(worker_service: Arc<dyn WorkerServiceTrait>) -> Self {
        Self { worker_service }
    }
}

#[async_trait]
impl FunctionService for DirectFunctionService {
    async fn execute_function(
        &self,
        user_id: &str,
        function_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // Log the function execution request
        debug!(
            "Directly executing function {} for user {} with input: {}",
            function_id, user_id, input
        );

        // Execute the function
        let result = self
            .worker_service
            .invoke_function(user_id, function_id, input)
            .await?;

        // Log the function execution result
        debug!(
            "Function {} executed successfully for user {} with result: {}",
            function_id, user_id, result
        );

        Ok(result)
    }
}
