// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::attestation::{AttestationService, AttestationServiceImpl};
use crate::enclave::{Enclave, EnclaveConfig, EnclaveManager};
use crate::key_management::{KeyManagementService, KeyManagementServiceImpl};
use crate::types::{ExecutionOptions, ExecutionStats, NeoTeeRequest, NeoTeeResponse};
use crate::{AttestationReport, TeeError, TeePlatform, TeeProvider, TeeSecurityLevel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// TEE provider implementation
pub struct TeeProviderImpl {
    /// Provider name
    name: String,

    /// Provider description
    description: String,

    /// TEE platform
    platform: TeePlatform,

    /// Enclave manager
    enclave_manager: Arc<EnclaveManager>,

    /// Attestation service
    attestation_service: Arc<dyn AttestationService>,

    /// Key management service
    key_management_service: Arc<dyn KeyManagementService>,
}

impl TeeProviderImpl {
    /// Create a new TEE provider
    pub fn new(
        name: &str,
        description: &str,
        platform: TeePlatform,
        enclave_manager: Arc<EnclaveManager>,
        attestation_service: Arc<dyn AttestationService>,
        key_management_service: Arc<dyn KeyManagementService>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            platform,
            enclave_manager,
            attestation_service,
            key_management_service,
        }
    }

    /// Create a default TEE provider for a platform
    pub fn default_for_platform(platform: TeePlatform) -> Self {
        let enclave_manager = Arc::new(EnclaveManager::new());
        let attestation_service =
            Arc::new(AttestationServiceImpl::new()) as Arc<dyn AttestationService>;
        let key_management_service =
            Arc::new(KeyManagementServiceImpl::new()) as Arc<dyn KeyManagementService>;

        let (name, description) = match platform {
            TeePlatform::Sgx => ("Intel SGX Provider", "TEE provider for Intel SGX"),
            TeePlatform::Sev => ("AMD SEV Provider", "TEE provider for AMD SEV"),
            TeePlatform::TrustZone => ("ARM TrustZone Provider", "TEE provider for ARM TrustZone"),
            TeePlatform::Simulated => (
                "Simulated TEE Provider",
                "Simulated TEE provider for development and testing",
            ),
        };

        Self::new(
            name,
            description,
            platform,
            enclave_manager,
            attestation_service,
            key_management_service,
        )
    }
}

#[async_trait::async_trait]
impl TeeProvider for TeeProviderImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn platform(&self) -> TeePlatform {
        self.platform
    }

    async fn initialize(&self) -> Result<(), TeeError> {
        // Nothing to do for initialization
        Ok(())
    }

    async fn execute(
        &self,
        code: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, TeeError> {
        // Create enclave configuration
        let config = EnclaveConfig {
            name: format!("{}-enclave", self.name),
            description: format!("Enclave for {}", self.name),
            platform: self.platform,
            security_level: TeeSecurityLevel::Debug, // Use debug for development
            memory_size_mb: 128,
            thread_count: 1,
            debug: true,
        };

        // Create an enclave
        let enclave = self.enclave_manager.create_enclave(config).await?;

        // Create execution options
        let options = ExecutionOptions::default();

        // Execute the code
        let (result, _stats) = enclave.execute(code, input, &options).await?;

        // Terminate the enclave
        self.enclave_manager.terminate_enclave(enclave.id()).await?;

        Ok(result)
    }

    async fn generate_attestation(&self) -> Result<AttestationReport, TeeError> {
        // Generate attestation using the attestation service
        self.attestation_service
            .generate_attestation(self.platform, &Default::default())
            .await
    }

    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError> {
        // Verify attestation using the attestation service
        let result = self
            .attestation_service
            .verify_attestation(attestation)
            .await?;

        Ok(result.is_valid)
    }
}

/// Neo N3 specific TEE provider
pub struct NeoTeeProvider {
    /// Base TEE provider
    base_provider: TeeProviderImpl,

    /// Neo RPC client
    neo_rpc_client: Option<Arc<neo3::prelude::RpcClient>>,
}

impl NeoTeeProvider {
    /// Create a new Neo TEE provider
    pub fn new(base_provider: TeeProviderImpl) -> Self {
        Self {
            base_provider,
            neo_rpc_client: None,
        }
    }

    /// Set the Neo RPC client
    pub fn with_rpc_client(mut self, rpc_client: Arc<neo3::prelude::RpcClient>) -> Self {
        self.neo_rpc_client = Some(rpc_client);
        self
    }

    /// Execute a Neo-specific TEE request
    pub async fn execute_neo_request(
        &self,
        request: &NeoTeeRequest,
    ) -> Result<NeoTeeResponse, TeeError> {
        // Check if we have an RPC client
        let rpc_client = self
            .neo_rpc_client
            .as_ref()
            .ok_or_else(|| TeeError::Provider("Neo RPC client not configured".to_string()))?;

        // Convert the request to a JavaScript function
        let code = format!(
            r#"
            function(input) {{
                // Import Neo module
                const {{ Neo }} = globalThis.r3e;
                
                // Create RPC client
                const rpcClient = Neo.createRpcClient("{}");
                
                // Create a script builder
                const sb = Neo.createScriptBuilder();
                
                // Add the operation to the script
                sb.contractCall(
                    "{}",
                    "{}",
                    {}
                );
                
                // Build the script
                const script = sb.build();
                
                // Execute the script
                const result = rpcClient.invokeScript(script);
                
                return result;
            }}
            "#,
            rpc_client.get_url(),
            request.script_hash,
            request.operation,
            serde_json::to_string(&request.args).unwrap()
        );

        // Execute the code in the TEE
        let result = self
            .base_provider
            .execute(&code, &serde_json::json!({}))
            .await?;

        // Convert the result to a Neo TEE response
        let response = NeoTeeResponse {
            tx_hash: result
                .get("txid")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            result: result
                .get("result")
                .cloned()
                .unwrap_or(serde_json::json!(null)),
            vm_state: result
                .get("state")
                .and_then(|v| v.as_str())
                .unwrap_or("NONE")
                .to_string(),
            gas_consumed: result
                .get("gas_consumed")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            exception: result
                .get("exception")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            stack: result
                .get("stack")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
        };

        Ok(response)
    }
}

#[async_trait::async_trait]
impl TeeProvider for NeoTeeProvider {
    fn name(&self) -> &str {
        self.base_provider.name()
    }

    fn description(&self) -> &str {
        self.base_provider.description()
    }

    fn platform(&self) -> TeePlatform {
        self.base_provider.platform()
    }

    async fn initialize(&self) -> Result<(), TeeError> {
        self.base_provider.initialize().await
    }

    async fn execute(
        &self,
        code: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, TeeError> {
        self.base_provider.execute(code, input).await
    }

    async fn generate_attestation(&self) -> Result<AttestationReport, TeeError> {
        self.base_provider.generate_attestation().await
    }

    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError> {
        self.base_provider.verify_attestation(attestation).await
    }
}

/// Create a default Neo TEE provider
pub fn create_default_neo_tee_provider(rpc_url: &str) -> Result<NeoTeeProvider, TeeError> {
    // Create a Neo RPC client
    let rpc_client = neo3::prelude::RpcClient::new(rpc_url);

    // Create a base TEE provider
    let base_provider = TeeProviderImpl::default_for_platform(TeePlatform::Simulated);

    // Create a Neo TEE provider
    let provider = NeoTeeProvider::new(base_provider).with_rpc_client(Arc::new(rpc_client));

    Ok(provider)
}
