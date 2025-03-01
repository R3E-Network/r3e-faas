// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use r3e_core::error::{Error, Result};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

use crate::function_executor::FunctionExecutor;
use crate::sandbox::SandboxConfig;
use crate::function::FunctionDeployment;

/// Maximum age of a warm function executor in seconds
const MAX_WARM_EXECUTOR_AGE_SECS: u64 = 300; // 5 minutes

/// Maximum size of the warm pool
const MAX_WARM_POOL_SIZE: usize = 100;

/// Warm function executor
struct WarmFunctionExecutor {
    /// Function ID
    function_id: String,
    
    /// Function version
    version: String,
    
    /// Function executor
    executor: FunctionExecutor,
    
    /// Last used time
    last_used: Instant,
    
    /// Creation time
    created_at: Instant,
}

/// Function executor pool
pub struct FunctionExecutorPool {
    /// Sandbox configuration
    sandbox_config: SandboxConfig,
    
    /// Warm function executors
    warm_executors: Arc<RwLock<HashMap<String, WarmFunctionExecutor>>>,
    
    /// Cleanup task handle
    _cleanup_task: tokio::task::JoinHandle<()>,
}

impl FunctionExecutorPool {
    /// Create a new function executor pool
    pub fn new(sandbox_config: SandboxConfig) -> Self {
        let warm_executors = Arc::new(RwLock::new(HashMap::new()));
        let warm_executors_clone = warm_executors.clone();
        
        // Start the cleanup task
        let cleanup_task = tokio::spawn(async move {
            loop {
                // Sleep for 60 seconds
                tokio::time::sleep(Duration::from_secs(60)).await;
                
                // Cleanup old executors
                Self::cleanup_old_executors(&warm_executors_clone).await;
            }
        });
        
        Self {
            sandbox_config,
            warm_executors,
            _cleanup_task: cleanup_task,
        }
    }
    
    /// Get a function executor
    #[instrument(skip(self))]
    pub async fn get_executor(&self, function_id: &str, version: &str) -> Result<FunctionExecutor> {
        let key = Self::get_key(function_id, version);
        
        // Try to get a warm executor
        let mut warm_executors = self.warm_executors.write().await;
        
        if let Some(warm_executor) = warm_executors.remove(&key) {
            // Check if the executor is too old
            if warm_executor.created_at.elapsed().as_secs() > MAX_WARM_EXECUTOR_AGE_SECS {
                info!(
                    function_id = %function_id,
                    version = %version,
                    age_secs = warm_executor.created_at.elapsed().as_secs(),
                    "Discarding old warm executor"
                );
                
                // Create a new executor
                return Ok(FunctionExecutor::new(self.sandbox_config.clone()));
            }
            
            info!(
                function_id = %function_id,
                version = %version,
                age_secs = warm_executor.created_at.elapsed().as_secs(),
                idle_secs = warm_executor.last_used.elapsed().as_secs(),
                "Using warm executor"
            );
            
            return Ok(warm_executor.executor);
        }
        
        // Create a new executor
        info!(
            function_id = %function_id,
            version = %version,
            "Creating new executor"
        );
        
        Ok(FunctionExecutor::new(self.sandbox_config.clone()))
    }
    
    /// Return a function executor to the pool
    #[instrument(skip(self, executor))]
    pub async fn return_executor(
        &self,
        function_id: &str,
        version: &str,
        executor: FunctionExecutor,
    ) -> Result<()> {
        let key = Self::get_key(function_id, version);
        
        // Add the executor to the warm pool
        let mut warm_executors = self.warm_executors.write().await;
        
        // Check if the warm pool is full
        if warm_executors.len() >= MAX_WARM_POOL_SIZE {
            // Find the oldest executor
            if let Some((oldest_key, _)) = warm_executors
                .iter()
                .min_by_key(|(_, e)| e.last_used)
            {
                let oldest_key = oldest_key.clone();
                warm_executors.remove(&oldest_key);
                
                info!(
                    function_id = %function_id,
                    version = %version,
                    "Removed oldest executor from warm pool"
                );
            }
        }
        
        // Add the executor to the warm pool
        warm_executors.insert(
            key,
            WarmFunctionExecutor {
                function_id: function_id.to_string(),
                version: version.to_string(),
                executor,
                last_used: Instant::now(),
                created_at: Instant::now(),
            },
        );
        
        info!(
            function_id = %function_id,
            version = %version,
            warm_pool_size = warm_executors.len(),
            "Added executor to warm pool"
        );
        
        Ok(())
    }
    
    /// Warm up a function
    #[instrument(skip(self, function))]
    pub async fn warm_up_function(&self, function: &FunctionDeployment) -> Result<()> {
        info!(
            function_id = %function.id,
            version = %function.version,
            "Warming up function"
        );
        
        // Create a new executor
        let executor = FunctionExecutor::new(self.sandbox_config.clone());
        
        // Preload the function
        let preload_result = executor.preload_function(function).await;
        
        if let Err(e) = preload_result {
            error!(
                function_id = %function.id,
                version = %function.version,
                error = %e,
                "Failed to preload function"
            );
            return Err(e);
        }
        
        // Add the executor to the warm pool
        self.return_executor(&function.id, &function.version, executor).await?;
        
        Ok(())
    }
    
    /// Cleanup old executors
    async fn cleanup_old_executors(warm_executors: &Arc<RwLock<HashMap<String, WarmFunctionExecutor>>>) {
        let mut warm_executors = warm_executors.write().await;
        
        // Find executors that are too old
        let old_keys: Vec<String> = warm_executors
            .iter()
            .filter(|(_, e)| e.created_at.elapsed().as_secs() > MAX_WARM_EXECUTOR_AGE_SECS)
            .map(|(k, _)| k.clone())
            .collect();
        
        // Remove old executors
        for key in old_keys {
            warm_executors.remove(&key);
        }
    }
    
    /// Get the key for a function
    fn get_key(function_id: &str, version: &str) -> String {
        format!("{}:{}", function_id, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_get_executor() {
        let config = SandboxConfig::default();
        let pool = FunctionExecutorPool::new(config);
        
        let executor = pool.get_executor("test-function", "1.0.0").await.unwrap();
        
        // Return the executor to the pool
        pool.return_executor("test-function", "1.0.0", executor).await.unwrap();
        
        // Get the executor again
        let executor = pool.get_executor("test-function", "1.0.0").await.unwrap();
        
        // The executor should be from the warm pool
    }
    
    #[tokio::test]
    async fn test_warm_up_function() {
        let config = SandboxConfig::default();
        let pool = FunctionExecutorPool::new(config);
        
        let function = FunctionDeployment {
            id: "test-function".to_string(),
            user_id: "test-user".to_string(),
            name: "Test Function".to_string(),
            code: r#"
            export default function(input) {
                return { message: "Hello, " + input.name };
            }
            "#.to_string(),
            runtime: "javascript".to_string(),
            security_level: "standard".to_string(),
            status: crate::function::DeploymentStatus::Deployed,
            error: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            version: "1.0.0".to_string(),
        };
        
        // Warm up the function
        pool.warm_up_function(&function).await.unwrap();
        
        // Get the executor
        let executor = pool.get_executor("test-function", "1.0.0").await.unwrap();
        
        // The executor should be from the warm pool
    }
}
