// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use r3e_deno::{ExecError, JsRuntime, RuntimeConfig, SandboxConfig};

use crate::sandbox::SandboxManager;

/// Function deployment status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    /// Function is being deployed
    Deploying,

    /// Function is deployed and ready to be invoked
    Deployed,

    /// Function deployment failed
    Failed,

    /// Function is being undeployed
    Undeploying,

    /// Function is undeployed
    Undeployed,
}

/// Function deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeployment {
    /// Function ID
    pub id: String,

    /// User ID
    pub user_id: String,

    /// Function name
    pub name: String,

    /// Function code
    pub code: String,

    /// Function runtime
    pub runtime: String,

    /// Function security level
    pub security_level: String,

    /// Function deployment status
    pub status: DeploymentStatus,

    /// Function deployment error
    pub error: Option<String>,

    /// Function deployment created at
    pub created_at: DateTime<Utc>,

    /// Function deployment updated at
    pub updated_at: DateTime<Utc>,
}

impl FunctionDeployment {
    /// Create a new function deployment
    pub fn new(
        id: String,
        user_id: String,
        name: String,
        code: String,
        runtime: String,
        security_level: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            user_id,
            name,
            code,
            runtime,
            security_level,
            status: DeploymentStatus::Deploying,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the deployment status
    pub fn set_status(&mut self, status: DeploymentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Set the deployment error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.status = DeploymentStatus::Failed;
        self.updated_at = Utc::now();
    }
}

/// Function invocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInvocationResult {
    /// Invocation ID
    pub id: String,

    /// Function ID
    pub function_id: String,

    /// User ID
    pub user_id: String,

    /// Invocation input
    pub input: serde_json::Value,

    /// Invocation output
    pub output: Option<serde_json::Value>,

    /// Invocation error
    pub error: Option<String>,

    /// Invocation execution time in milliseconds
    pub execution_time_ms: u64,

    /// Invocation created at
    pub created_at: DateTime<Utc>,
}

impl FunctionInvocationResult {
    /// Create a new function invocation result
    pub fn new(function_id: String, user_id: String, input: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            function_id,
            user_id,
            input,
            output: None,
            error: None,
            execution_time_ms: 0,
            created_at: Utc::now(),
        }
    }

    /// Set the invocation output
    pub fn set_output(&mut self, output: serde_json::Value, execution_time_ms: u64) {
        self.output = Some(output);
        self.execution_time_ms = execution_time_ms;
    }

    /// Set the invocation error
    pub fn set_error(&mut self, error: String, execution_time_ms: u64) {
        self.error = Some(error);
        self.execution_time_ms = execution_time_ms;
    }
}

/// Function deployment service
pub struct FunctionDeploymentService {
    /// Sandbox manager
    sandbox_manager: SandboxManager,

    /// Function deployments
    deployments: Arc<RwLock<Vec<FunctionDeployment>>>,
}

impl FunctionDeploymentService {
    /// Create a new function deployment service
    pub fn new() -> Self {
        Self {
            sandbox_manager: SandboxManager::default(),
            deployments: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Deploy a function
    pub async fn deploy_function(
        &self,
        id: String,
        user_id: String,
        name: String,
        code: String,
        runtime: String,
        security_level: String,
    ) -> Result<FunctionDeployment, String> {
        // Create a new function deployment
        let mut deployment = FunctionDeployment::new(
            id.clone(),
            user_id.clone(),
            name.clone(),
            code.clone(),
            runtime.clone(),
            security_level.clone(),
        );

        // Get the sandbox configuration for the security level
        let sandbox_config = self
            .sandbox_manager
            .create_config_for_security_level(&security_level);

        // Create a runtime configuration
        let runtime_config = RuntimeConfig {
            max_heap_size: sandbox_config.max_heap_size,
            sandbox_config: Some(sandbox_config),
        };

        // Create a new runtime
        let mut runtime = JsRuntime::new(runtime_config);

        // Try to load the function code
        match runtime.load_main_module(code.clone()).await {
            Ok(_) => {
                // Update the deployment status
                deployment.set_status(DeploymentStatus::Deployed);

                // Add the deployment to the list
                let mut deployments = self.deployments.write().await;
                deployments.push(deployment.clone());

                Ok(deployment)
            }
            Err(err) => {
                // Update the deployment status
                deployment.set_error(format!("Failed to deploy function: {}", err));

                Err(format!("Failed to deploy function: {}", err))
            }
        }
    }

    /// Undeploy a function
    pub async fn undeploy_function(&self, id: &str) -> Result<(), String> {
        // Find the deployment
        let mut deployments = self.deployments.write().await;
        let deployment_index = deployments.iter().position(|d| d.id == id);

        match deployment_index {
            Some(index) => {
                // Update the deployment status
                deployments[index].set_status(DeploymentStatus::Undeploying);

                // Remove the deployment from the list
                deployments.remove(index);

                Ok(())
            }
            None => Err(format!("Function deployment not found: {}", id)),
        }
    }

    /// Get a function deployment
    pub async fn get_function_deployment(&self, id: &str) -> Option<FunctionDeployment> {
        // Find the deployment
        let deployments = self.deployments.read().await;
        deployments.iter().find(|d| d.id == id).cloned()
    }

    /// Invoke a function
    pub async fn invoke_function(
        &self,
        id: &str,
        user_id: &str,
        input: serde_json::Value,
    ) -> Result<FunctionInvocationResult, String> {
        // Find the deployment
        let deployment = match self.get_function_deployment(id).await {
            Some(deployment) => deployment,
            None => {
                return Err(format!("Function deployment not found: {}", id));
            }
        };

        // Check if the function is deployed
        if deployment.status != DeploymentStatus::Deployed {
            return Err(format!("Function is not deployed: {}", id));
        }

        // Create a new invocation result
        let mut result =
            FunctionInvocationResult::new(id.to_string(), user_id.to_string(), input.clone());

        // Get the sandbox configuration for the security level
        let sandbox_config = self
            .sandbox_manager
            .create_config_for_security_level(&deployment.security_level);

        // Create a runtime configuration
        let runtime_config = RuntimeConfig {
            max_heap_size: sandbox_config.max_heap_size,
            sandbox_config: Some(sandbox_config),
        };

        // Create a new runtime
        let mut runtime = JsRuntime::new(runtime_config);

        // Start the execution timer
        let start_time = Instant::now();

        // Try to load the function code
        match runtime.load_main_module(deployment.code.clone()).await {
            Ok(module) => {
                // Try to evaluate the module
                match runtime.eval_module(module).await {
                    Ok(_) => {
                        // Try to convert the input to a global value
                        match runtime.to_global(&input) {
                            Ok(input_value) => {
                                // Try to run the module default function with the input
                                match runtime.run_module_default(module, &[input_value]).await {
                                    Ok(_) => {
                                        // Calculate the execution time
                                        let execution_time = start_time.elapsed();

                                        // Set the output
                                        result.set_output(
                                            serde_json::json!({
                                                "message": "Function executed successfully",
                                                "execution_time_ms": execution_time.as_millis(),
                                            }),
                                            execution_time.as_millis() as u64,
                                        );

                                        Ok(result)
                                    }
                                    Err(err) => {
                                        // Calculate the execution time
                                        let execution_time = start_time.elapsed();

                                        // Set the error
                                        result.set_error(
                                            format!("Failed to run function: {}", err),
                                            execution_time.as_millis() as u64,
                                        );

                                        Err(format!("Failed to run function: {}", err))
                                    }
                                }
                            }
                            Err(err) => {
                                // Calculate the execution time
                                let execution_time = start_time.elapsed();

                                // Set the error
                                result.set_error(
                                    format!("Failed to convert input to global value: {}", err),
                                    execution_time.as_millis() as u64,
                                );

                                Err(format!("Failed to convert input to global value: {}", err))
                            }
                        }
                    }
                    Err(err) => {
                        // Calculate the execution time
                        let execution_time = start_time.elapsed();

                        // Set the error
                        result.set_error(
                            format!("Failed to evaluate module: {}", err),
                            execution_time.as_millis() as u64,
                        );

                        Err(format!("Failed to evaluate module: {}", err))
                    }
                }
            }
            Err(err) => {
                // Calculate the execution time
                let execution_time = start_time.elapsed();

                // Set the error
                result.set_error(
                    format!("Failed to load module: {}", err),
                    execution_time.as_millis() as u64,
                );

                Err(format!("Failed to load module: {}", err))
            }
        }
    }

    /// List function deployments
    pub async fn list_function_deployments(&self) -> Vec<FunctionDeployment> {
        let deployments = self.deployments.read().await;
        deployments.clone()
    }
}
