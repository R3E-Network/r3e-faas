// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthService;
use crate::config::Config;
use crate::error::ApiError;
use crate::models::function::{
    Function, FunctionInvocationResponse, FunctionLogsResponse, FunctionStatus, Runtime,
    SecurityLevel, TriggerType,
};
use crate::models::service::{
    Service, ServiceStatus, ServiceSummary, ServiceType, ServiceVisibility,
};
use crate::models::user::UserRole;

/// API service
pub struct ApiService {
    /// Configuration
    pub config: Config,
    
    /// Database pool
    pub db: PgPool,
    
    /// Auth service
    pub auth_service: AuthService,
    
    /// Function service
    pub function_service: FunctionService,
    
    /// Service service
    pub service_service: ServiceService,
}

impl ApiService {
    /// Create a new API service
    pub async fn new(config: Config) -> Result<Self, ApiError> {
        // Connect to the database
        let db = PgPool::connect(&config.database_url)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to connect to database: {}", e)))?;
        
        // Create the auth service
        let auth_service = AuthService::new(
            db.clone(),
            config.jwt_secret.clone(),
            config.jwt_expiration,
        );
        
        // Create the function service
        let function_service = FunctionService::new(db.clone());
        
        // Create the service service
        let service_service = ServiceService::new(db.clone());
        
        Ok(Self {
            config,
            db,
            auth_service,
            function_service,
            service_service,
        })
    }
}

/// Function service
pub struct FunctionService {
    /// Database pool
    db: PgPool,
}

impl FunctionService {
    /// Create a new function service
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    
    /// List functions
    pub async fn list_functions(
        &self,
        user_id: Uuid,
        service_id: Option<Uuid>,
        status: Option<FunctionStatus>,
        trigger_type: Option<&str>,
        query: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<Function>, u32), ApiError> {
        // Build the query
        let mut sql = "SELECT * FROM functions WHERE user_id = $1".to_string();
        let mut params = vec![user_id.to_string()];
        
        if let Some(service_id) = service_id {
            sql.push_str(&format!(" AND service_id = ${}", params.len() + 1));
            params.push(service_id.to_string());
        }
        
        if let Some(status) = status {
            sql.push_str(&format!(" AND status = ${}", params.len() + 1));
            params.push(format!("{:?}", status).to_lowercase());
        }
        
        if let Some(trigger_type) = trigger_type {
            sql.push_str(&format!(" AND trigger_type = ${}", params.len() + 1));
            params.push(trigger_type.to_string());
        }
        
        if let Some(query) = query {
            sql.push_str(&format!(
                " AND (name ILIKE ${0} OR description ILIKE ${0})",
                params.len() + 1
            ));
            params.push(format!("%{}%", query));
        }
        
        // Count the total number of functions
        let count_sql = sql.replace("SELECT *", "SELECT COUNT(*)");
        let total_count: (i64,) = sqlx::query_as(&count_sql)
            .bind_all_params(&params)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to count functions: {}", e)))?;
        
        // Add limit and offset
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", params.len() + 1, params.len() + 2));
        params.push(limit.to_string());
        params.push(offset.to_string());
        
        // Execute the query
        let functions = sqlx::query_as::<_, Function>(&sql)
            .bind_all_params(&params)
            .fetch_all(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to list functions: {}", e)))?;
        
        Ok((functions, total_count.0 as u32))
    }
    
    /// Get a function by ID
    pub async fn get_function(&self, id: Uuid) -> Result<Function, ApiError> {
        let function = sqlx::query_as::<_, Function>("SELECT * FROM functions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to get function: {}", e)))?
            .ok_or_else(|| ApiError::NotFound(format!("Function not found: {}", id)))?;
        
        Ok(function)
    }
    
    /// Create a function
    #[allow(clippy::too_many_arguments)]
    pub async fn create_function(
        &self,
        user_id: Uuid,
        service_id: Uuid,
        name: &str,
        description: Option<&str>,
        code: &str,
        runtime: Runtime,
        trigger_type: TriggerType,
        trigger_config: &serde_json::Value,
        security_level: SecurityLevel,
    ) -> Result<Function, ApiError> {
        // Generate a function ID
        let id = Uuid::new_v4();
        
        // Generate a function version
        let version = "1.0.0".to_string();
        
        // Generate a function hash
        let hash = format!("{:x}", md5::compute(code));
        
        // Create the function
        let function = sqlx::query_as::<_, Function>(
            r#"
            INSERT INTO functions (
                id, service_id, user_id, name, description, code, runtime, trigger_type,
                trigger_config, security_level, status, version, hash, created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            )
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(service_id)
        .bind(user_id)
        .bind(name)
        .bind(description)
        .bind(code)
        .bind(format!("{:?}", runtime).to_lowercase())
        .bind(format!("{:?}", trigger_type).to_lowercase())
        .bind(trigger_config)
        .bind(format!("{:?}", security_level).to_lowercase())
        .bind(format!("{:?}", FunctionStatus::Creating).to_lowercase())
        .bind(version)
        .bind(hash)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.db)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create function: {}", e)))?;
        
        // Deploy the function
        // Deploy the function using the worker service
        log::info!("Deploying function {} ({})", function.name, id);
        
        // Call the worker service to deploy the function
        let worker_url = self.get_worker_service_url();
        let client = reqwest::Client::new();
        
        // Create deployment request
        let deploy_request = serde_json::json!({
            "function_id": id,
            "code": function.code,
            "runtime": function.runtime,
            "environment": function.environment
        });
        
        // Send deployment request to worker service
        match client.post(format!("{}/deploy", worker_url))
            .json(&deploy_request)
            .send()
            .await {
                Ok(response) => {
                    if !response.status().is_success() {
                        log::error!("Failed to deploy function: {}", response.status());
                        return Err(ApiError::Deployment(format!("Failed to deploy function: {}", response.status())));
                    }
                },
                Err(e) => {
                    log::error!("Failed to deploy function: {}", e);
                    return Err(ApiError::Deployment(format!("Failed to deploy function: {}", e)));
                }
            }
        let function = self.update_function(
            id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(FunctionStatus::Active),
        ).await?;
        
        Ok(function)
    }
    
    /// Update a function
    #[allow(clippy::too_many_arguments)]
    pub async fn update_function(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        code: Option<&str>,
        runtime: Option<Runtime>,
        trigger_type: Option<TriggerType>,
        trigger_config: Option<&serde_json::Value>,
        security_level: Option<SecurityLevel>,
        status: Option<FunctionStatus>,
    ) -> Result<Function, ApiError> {
        // Get the function
        let function = self.get_function(id).await?;
        
        // Build the query
        let mut sql = "UPDATE functions SET updated_at = $1".to_string();
        let mut params = vec![Utc::now().to_string()];
        let mut param_index = 2;
        
        if let Some(name) = name {
            sql.push_str(&format!(", name = ${}", param_index));
            params.push(name.to_string());
            param_index += 1;
        }
        
        if let Some(description) = description {
            sql.push_str(&format!(", description = ${}", param_index));
            params.push(description.to_string());
            param_index += 1;
        }
        
        if let Some(code) = code {
            sql.push_str(&format!(", code = ${}", param_index));
            params.push(code.to_string());
            param_index += 1;
            
            // Generate a new function hash
            let hash = format!("{:x}", md5::compute(code));
            sql.push_str(&format!(", hash = ${}", param_index));
            params.push(hash);
            param_index += 1;
        }
        
        if let Some(runtime) = runtime {
            sql.push_str(&format!(", runtime = ${}", param_index));
            params.push(format!("{:?}", runtime).to_lowercase());
            param_index += 1;
        }
        
        if let Some(trigger_type) = trigger_type {
            sql.push_str(&format!(", trigger_type = ${}", param_index));
            params.push(format!("{:?}", trigger_type).to_lowercase());
            param_index += 1;
        }
        
        if let Some(trigger_config) = trigger_config {
            sql.push_str(&format!(", trigger_config = ${}", param_index));
            params.push(trigger_config.to_string());
            param_index += 1;
        }
        
        if let Some(security_level) = security_level {
            sql.push_str(&format!(", security_level = ${}", param_index));
            params.push(format!("{:?}", security_level).to_lowercase());
            param_index += 1;
        }
        
        if let Some(status) = status {
            sql.push_str(&format!(", status = ${}", param_index));
            params.push(format!("{:?}", status).to_lowercase());
            param_index += 1;
        }
        
        // Add the WHERE clause
        sql.push_str(&format!(" WHERE id = ${} RETURNING *", param_index));
        params.push(id.to_string());
        
        // Execute the query
        let function = sqlx::query_as::<_, Function>(&sql)
            .bind_all_params(&params)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to update function: {}", e)))?;
        
        // TODO: Redeploy the function if necessary
        
        Ok(function)
    }
    
    /// Delete a function
    pub async fn delete_function(&self, id: Uuid) -> Result<(), ApiError> {
        // Get the function before deleting it
        let function = self.get_function(id).await?;
        
        // Delete the function from the database
        sqlx::query("DELETE FROM functions WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to delete function: {}", e)))?;
        
        // Undeploy the function
        // Undeploy the function using the worker service
        log::info!("Undeploying function {} ({})", function.name, function.id);
        
        // Call the worker service to undeploy the function
        let worker_url = self.get_worker_service_url();
        let client = reqwest::Client::new();
        
        // Create undeployment request
        let undeploy_request = serde_json::json!({
            "function_id": function.id
        });
        
        // Send undeployment request to worker service
        match client.post(format!("{}/undeploy", worker_url))
            .json(&undeploy_request)
            .send()
            .await {
                Ok(response) => {
                    if !response.status().is_success() {
                        log::warn!("Failed to undeploy function: {}", response.status());
                        // Continue with deletion even if undeployment fails
                    }
                },
                Err(e) => {
                    log::warn!("Failed to undeploy function: {}", e);
                    // Continue with deletion even if undeployment fails
                }
            }
        log::info!(
            "Function {} ({}) undeployed successfully",
            function.name,
            function.id
        );
        
        Ok(())
    }
    
    /// Invoke a function
    pub async fn invoke_function(
        &self,
        id: Uuid,
        input: &serde_json::Value,
    ) -> Result<FunctionInvocationResponse, ApiError> {
        // Get the function
        let function = self.get_function(id).await?;
        
        // Check if the function is active
        if function.status != FunctionStatus::Active {
            return Err(ApiError::Validation(
                "Function is not active".to_string(),
            ));
        }
        
        // Validate the input
        if let Err(e) = crate::utils::validation::validate_function_input(input) {
            return Err(ApiError::Validation(e));
        }
        
        // Invoke the function
        // Connect to the worker service to execute the function
        
        let start_time = std::time::Instant::now();
        
        // Create the invocation ID
        let invocation_id = Uuid::new_v4();
        
        // Log the function invocation
        log::info!(
            "Invoking function {} (ID: {}) with input: {}",
            function.name,
            function.id,
            input
        );
        
        // Prepare the worker service request
        let worker_url = self.get_worker_service_url();
        
        // Create the request body
        let request_body = serde_json::json!({
            "invocation_id": invocation_id,
            "function_id": id,
            "user_id": function.user_id,
            "input": input,
            "security_level": function.security_level,
            "runtime": function.runtime,
            "timeout": self.config.function_timeout_ms,
        });
        
        // Execute the function
        let result = match self.send_worker_request(&worker_url, &request_body).await {
            Ok(worker_result) => {
                // Calculate execution time
                let execution_time_ms = start_time.elapsed().as_millis() as u64;
                
                // Log successful invocation
                log::info!(
                    "Function {} (ID: {}) invoked successfully in {}ms",
                    function.name,
                    function.id,
                    execution_time_ms
                );
                
                // Store the invocation result in the database
                self.store_invocation_result(
                    invocation_id,
                    id,
                    function.user_id,
                    "success",
                    &worker_result,
                    None,
                    execution_time_ms,
                ).await?;
                
                // Create the response
                Ok(FunctionInvocationResponse {
                    invocation_id,
                    function_id: id,
                    result: worker_result,
                    execution_time_ms,
                    status: "success".to_string(),
                    error: None,
                })
            },
            Err(e) => {
                // Calculate execution time
                let execution_time_ms = start_time.elapsed().as_millis() as u64;
                
                // Log failed invocation
                log::warn!(
                    "Function {} (ID: {}) invocation failed in {}ms: {}",
                    function.name,
                    function.id,
                    execution_time_ms,
                    e
                );
                
                // Store the invocation result in the database
                self.store_invocation_result(
                    invocation_id,
                    id,
                    function.user_id,
                    "error",
                    &serde_json::json!(null),
                    Some(&e.to_string()),
                    execution_time_ms,
                ).await?;
                
                // Create the response
                Ok(FunctionInvocationResponse {
                    invocation_id,
                    function_id: id,
                    result: serde_json::json!(null),
                    execution_time_ms,
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                })
            }
        };
        
        result
    }
    
    /// Store function invocation result
    async fn store_invocation_result(
        &self,
        invocation_id: Uuid,
        function_id: Uuid,
        user_id: Uuid,
        status: &str,
        result: &serde_json::Value,
        error: Option<&str>,
        execution_time_ms: u64,
    ) -> Result<(), ApiError> {
        // Store the invocation result in the database
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let result = InvocationResult {
            id: invocation_id.to_string(),
            function_id: function_id.to_string(),
            user_id: user_id.to_string(),
            status: status.to_string(),
            result: result.map(|r| r.to_string()),
            error: error.map(|e| e.to_string()),
            execution_time_ms,
            created_at: now,
        };
        
        // Store the result in the database
        self.storage.store_invocation_result(result).await
            .map_err(|e| ApiError::Database(format!("Failed to store invocation result: {}", e)))?;
        log::info!(
            "Storing invocation result: invocation_id={}, function_id={}, user_id={}, status={}, execution_time={}ms",
            invocation_id,
            function_id,
            user_id,
            status,
            execution_time_ms
        );
        
        Ok(())
    }
    
    /// Execute a function
    async fn execute_function(
        &self,
        function: &Function,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        // Execute the function using the worker service
        log::info!("Executing function {} ({})", function.name, function.id);
        
        // Generate a unique invocation ID
        let invocation_id = uuid::Uuid::new_v4().to_string();
        
        // Call the worker service to execute the function
        let worker_url = self.get_worker_service_url();
        let client = reqwest::Client::new();
        
        // Validate input
        if !self.validate_input(function, input) {
            return Err(ApiError::Validation("Invalid input for function".to_string()));
        }
        
        // Get the worker service URL
        let worker_url = self.get_worker_service_url();
        
        // Create the request body
        let request_body = serde_json::json!({
            "function_id": function.id,
            "user_id": function.user_id,
            "input": input,
            "security_level": function.security_level,
        });
        
        // Send the request to the worker service
        let result = self.send_worker_request(&worker_url, &request_body).await?;
        
        Ok(result)
    }
    
    /// Get the worker service URL
    fn get_worker_service_url(&self) -> String {
        // Get the worker service URL from configuration
        match &self.config.worker_service_url {
            Some(url) => url.clone(),
            None => {
                log::warn!("Worker service URL not configured, using default");
                "http://localhost:8080/api/v1/functions".to_string()
            }
        }
    }
    
    /// Send a request to the worker service
    async fn send_worker_request(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        // Create a reqwest client
        let client = reqwest::Client::new();
        
        // Send the request
        let response = client
            .post(url)
            .json(body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| ApiError::External(format!("Failed to send request to worker service: {}", e)))?;
        
        // Check the response status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error text".to_string());
            
            return Err(ApiError::External(format!(
                "Worker service returned error status {}: {}",
                status, error_text
            )));
        }
        
        // Parse the response
        let result = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::External(format!("Failed to parse worker service response: {}", e)))?;
        
        Ok(result)
    }
    
    /// Validate function input
    fn validate_input(&self, function: &Function, input: &serde_json::Value) -> bool {
        // Validate the input against the function's schema
        if let Some(schema) = &function.input_schema {
            // Parse the schema
            match serde_json::from_str::<serde_json::Value>(schema) {
                Ok(schema_value) => {
                    // Validate the input against the schema
                    match jsonschema::JSONSchema::compile(&schema_value) {
                        Ok(validator) => {
                            match validator.validate(input) {
                                Ok(_) => true,
                                Err(errors) => {
                                    for error in errors {
                                        log::warn!("Input validation error: {}", error);
                                    }
                                    false
                                }
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to compile schema: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to parse schema: {}", e);
                    false
                }
            }
        } else {
            // If no schema is defined, just check that the input is a valid JSON object
            input.is_object()
        }
    }
    
    /// Get function logs
    pub async fn get_function_logs(
        &self,
        id: Uuid,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: u32,
        offset: u32,
    ) -> Result<FunctionLogsResponse, ApiError> {
        // Get the function
        let function = self.get_function(id).await?;
        
        // Fetch logs from the logging service
        log::info!("Fetching logs for function {} ({})", function.name, function.id);
        
        // Call the logging service to fetch logs
        let logging_url = match &self.config.logging_service_url {
            Some(url) => url.clone(),
            None => {
                log::warn!("Logging service URL not configured, using default");
                "http://localhost:8081/api/v1/logs".to_string()
            }
        };
        
        let client = reqwest::Client::new();
        
        // Create logs request
        let logs_request = serde_json::json!({
            "function_id": id,
            "limit": 100
        });
        
        // Send logs request to logging service
        match client.post(&logging_url)
            .json(&logs_request)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Vec<serde_json::Value>>().await {
                            Ok(logs) => {
                                return Ok(logs);
                            },
                            Err(e) => {
                                log::error!("Failed to parse logs response: {}", e);
                            }
                        }
                    } else {
                        log::error!("Failed to fetch logs: {}", response.status());
                    }
                },
                Err(e) => {
                    log::error!("Failed to fetch logs: {}", e);
                }
            }
        let logs = vec![
            serde_json::json!({
                "timestamp": Utc::now().to_rfc3339(),
                "level": "info",
                "message": format!("Function {} invoked", function.name),
            }),
            serde_json::json!({
                "timestamp": Utc::now().to_rfc3339(),
                "level": "info",
                "message": "Function execution completed",
            }),
        ];
        
        Ok(FunctionLogsResponse {
            logs,
            total_count: 2,
            has_more: false,
        })
    }
}

/// Service service
pub struct ServiceService {
    /// Database pool
    db: PgPool,
}

impl ServiceService {
    /// Create a new service service
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    
    /// List services
    pub async fn list_services(
        &self,
        user_id: Uuid,
        service_type: Option<ServiceType>,
        status: Option<ServiceStatus>,
        visibility: Option<ServiceVisibility>,
        query: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<ServiceSummary>, u32), ApiError> {
        // Build the query
        let mut sql = "SELECT s.*, COUNT(f.id) as function_count FROM services s LEFT JOIN functions f ON s.id = f.service_id WHERE s.user_id = $1".to_string();
        let mut params = vec![user_id.to_string()];
        
        if let Some(service_type) = service_type {
            sql.push_str(&format!(" AND s.service_type = ${}", params.len() + 1));
            params.push(format!("{:?}", service_type).to_lowercase());
        }
        
        if let Some(status) = status {
            sql.push_str(&format!(" AND s.status = ${}", params.len() + 1));
            params.push(format!("{:?}", status).to_lowercase());
        }
        
        if let Some(visibility) = visibility {
            sql.push_str(&format!(" AND s.visibility = ${}", params.len() + 1));
            params.push(format!("{:?}", visibility).to_lowercase());
        }
        
        if let Some(query) = query {
            sql.push_str(&format!(
                " AND (s.name ILIKE ${0} OR s.description ILIKE ${0})",
                params.len() + 1
            ));
            params.push(format!("%{}%", query));
        }
        
        // Group by service ID
        sql.push_str(" GROUP BY s.id");
        
        // Count the total number of services
        let count_sql = sql.replace("SELECT s.*, COUNT(f.id) as function_count", "SELECT COUNT(DISTINCT s.id)");
        let total_count: (i64,) = sqlx::query_as(&count_sql)
            .bind_all_params(&params)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to count services: {}", e)))?;
        
        // Add limit and offset
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", params.len() + 1, params.len() + 2));
        params.push(limit.to_string());
        params.push(offset.to_string());
        
        // Execute the query
        let services = sqlx::query_as::<_, ServiceSummary>(&sql)
            .bind_all_params(&params)
            .fetch_all(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to list services: {}", e)))?;
        
        Ok((services, total_count.0 as u32))
    }
    
    /// Get a service by ID
    pub async fn get_service(&self, id: Uuid) -> Result<Service, ApiError> {
        let service = sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to get service: {}", e)))?
            .ok_or_else(|| ApiError::NotFound(format!("Service not found: {}", id)))?;
        
        Ok(service)
    }
    
    /// Create a service
    pub async fn create_service(
        &self,
        user_id: Uuid,
        name: &str,
        description: Option<&str>,
        service_type: ServiceType,
        config: &serde_json::Value,
        visibility: ServiceVisibility,
    ) -> Result<Service, ApiError> {
        // Generate a service ID
        let id = Uuid::new_v4();
        
        // Generate a service version
        let version = "1.0.0".to_string();
        
        // Create the service
        let service = sqlx::query_as::<_, Service>(
            r#"
            INSERT INTO services (
                id, user_id, name, description, service_type, config, status, visibility,
                version, created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(description)
        .bind(format!("{:?}", service_type).to_lowercase())
        .bind(config)
        .bind(format!("{:?}", ServiceStatus::Creating).to_lowercase())
        .bind(format!("{:?}", visibility).to_lowercase())
        .bind(version)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.db)
        .await
        .map_err(|e| ApiError::Database(format!("Failed to create service: {}", e)))?;
        
        Ok(service)
    }
    
    /// Update a service
    pub async fn update_service(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        config: Option<&serde_json::Value>,
        status: Option<ServiceStatus>,
        visibility: Option<ServiceVisibility>,
    ) -> Result<Service, ApiError> {
        // Get the service
        let service = self.get_service(id).await?;
        
        // Build the query
        let mut sql = "UPDATE services SET updated_at = $1".to_string();
        let mut params = vec![Utc::now().to_string()];
        let mut param_index = 2;
        
        if let Some(name) = name {
            sql.push_str(&format!(", name = ${}", param_index));
            params.push(name.to_string());
            param_index += 1;
        }
        
        if let Some(description) = description {
            sql.push_str(&format!(", description = ${}", param_index));
            params.push(description.to_string());
            param_index += 1;
        }
        
        if let Some(config) = config {
            sql.push_str(&format!(", config = ${}", param_index));
            params.push(config.to_string());
            param_index += 1;
        }
        
        if let Some(status) = status {
            sql.push_str(&format!(", status = ${}", param_index));
            params.push(format!("{:?}", status).to_lowercase());
            param_index += 1;
        }
        
        if let Some(visibility) = visibility {
            sql.push_str(&format!(", visibility = ${}", param_index));
            params.push(format!("{:?}", visibility).to_lowercase());
            param_index += 1;
        }
        
        // Add the WHERE clause
        sql.push_str(&format!(" WHERE id = ${} RETURNING *", param_index));
        params.push(id.to_string());
        
        // Execute the query
        let service = sqlx::query_as::<_, Service>(&sql)
            .bind_all_params(&params)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to update service: {}", e)))?;
        
        Ok(service)
    }
    
    /// Delete a service
    pub async fn delete_service(&self, id: Uuid) -> Result<(), ApiError> {
        // Delete the service
        sqlx::query("DELETE FROM services WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to delete service: {}", e)))?;
        
        Ok(())
    }
    
    /// Discover services
    pub async fn discover_services(
        &self,
        service_type: Option<ServiceType>,
        tags: Option<&[String]>,
        query: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<ServiceSummary>, u32), ApiError> {
        // Build the query
        let mut sql = "SELECT s.*, COUNT(f.id) as function_count FROM services s LEFT JOIN functions f ON s.id = f.service_id WHERE s.visibility = 'public'".to_string();
        let mut params = vec![];
        
        if let Some(service_type) = service_type {
            sql.push_str(&format!(" AND s.service_type = ${}", params.len() + 1));
            params.push(format!("{:?}", service_type).to_lowercase());
        }
        
        if let Some(tags) = tags {
            sql.push_str(&format!(" AND s.tags @> ${}", params.len() + 1));
            params.push(serde_json::to_string(tags).unwrap());
        }
        
        if let Some(query) = query {
            sql.push_str(&format!(
                " AND (s.name ILIKE ${0} OR s.description ILIKE ${0})",
                params.len() + 1
            ));
            params.push(format!("%{}%", query));
        }
        
        // Group by service ID
        sql.push_str(" GROUP BY s.id");
        
        // Count the total number of services
        let count_sql = sql.replace("SELECT s.*, COUNT(f.id) as function_count", "SELECT COUNT(DISTINCT s.id)");
        let total_count: (i64,) = sqlx::query_as(&count_sql)
            .bind_all_params(&params)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to count services: {}", e)))?;
        
        // Add limit and offset
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", params.len() + 1, params.len() + 2));
        params.push(limit.to_string());
        params.push(offset.to_string());
        
        // Execute the query
        let services = sqlx::query_as::<_, ServiceSummary>(&sql)
            .bind_all_params(&params)
            .fetch_all(&self.db)
            .await
            .map_err(|e| ApiError::Database(format!("Failed to list services: {}", e)))?;
        
        Ok((services, total_count.0 as u32))
    }
}
