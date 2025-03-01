// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::attestation::{AttestationService, AttestationServiceImpl};
use crate::enclave::{Enclave, EnclaveConfig, EnclaveManager};
use crate::key_management::{KeyManagementService, KeyManagementServiceImpl};
use crate::{AttestationReport, TeeError, TeePlatform, TeeProvider, TeeSecurityLevel};
use std::sync::Arc;

pub struct NitroProvider {
    /// Provider name
    name: String,

    /// Provider description
    description: String,

    /// Enclave manager
    enclave_manager: Arc<EnclaveManager>,

    /// Attestation service
    attestation_service: Arc<dyn AttestationService>,

    /// Key management service
    key_management_service: Arc<dyn KeyManagementService>,
}

impl NitroProvider {
    /// Create a new Nitro provider
    pub fn new(
        name: &str,
        description: &str,
        enclave_manager: Arc<EnclaveManager>,
        attestation_service: Arc<dyn AttestationService>,
        key_management_service: Arc<dyn KeyManagementService>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            enclave_manager,
            attestation_service,
            key_management_service,
        }
    }

    /// Create a default Nitro provider
    pub fn default() -> Self {
        let enclave_manager = Arc::new(EnclaveManager::new());
        let attestation_service =
            Arc::new(AttestationServiceImpl::new()) as Arc<dyn AttestationService>;
        let key_management_service =
            Arc::new(KeyManagementServiceImpl::new()) as Arc<dyn KeyManagementService>;

        Self::new(
            "AWS Nitro Provider",
            "TEE provider for AWS Nitro Enclaves",
            enclave_manager,
            attestation_service,
            key_management_service,
        )
    }
}

#[async_trait::async_trait]
impl TeeProvider for NitroProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn platform(&self) -> TeePlatform {
        TeePlatform::Nitro
    }

    async fn initialize(&self) -> Result<(), TeeError> {
        info!("Initializing AWS Nitro enclave");

        // Check if Nitro CLI is available
        let nitro_cli_check = std::process::Command::new("nitro-cli")
            .arg("--version")
            .output()
            .map_err(|e| TeeError::Initialization(format!("Failed to execute nitro-cli: {}", e)))?;

        if !nitro_cli_check.status.success() {
            return Err(TeeError::Initialization(
                "nitro-cli is not available".to_string(),
            ));
        }

        // Check if the enclave manager is initialized
        if !self.enclave_manager.is_initialized() {
            self.enclave_manager.initialize().await.map_err(|e| {
                TeeError::Initialization(format!("Failed to initialize enclave manager: {}", e))
            })?;
        }

        // Initialize the attestation service
        self.attestation_service.initialize().await.map_err(|e| {
            TeeError::Initialization(format!("Failed to initialize attestation service: {}", e))
        })?;

        // Initialize the key management service
        self.key_management_service
            .initialize()
            .await
            .map_err(|e| {
                TeeError::Initialization(format!(
                    "Failed to initialize key management service: {}",
                    e
                ))
            })?;

        info!("AWS Nitro enclave initialized successfully");
        Ok(())
    }

    async fn execute(
        &self,
        code: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, TeeError> {
        info!("Executing code in AWS Nitro enclave");
        debug!("Code length: {}, Input: {}", code.len(), input);

        // Create a temporary directory for the execution
        let temp_dir = tempfile::tempdir().map_err(|e| {
            TeeError::Execution(format!("Failed to create temporary directory: {}", e))
        })?;

        // Write the code to a file
        let code_path = temp_dir.path().join("code.js");
        std::fs::write(&code_path, code)
            .map_err(|e| TeeError::Execution(format!("Failed to write code to file: {}", e)))?;

        // Write the input to a file
        let input_path = temp_dir.path().join("input.json");
        std::fs::write(&input_path, input.to_string())
            .map_err(|e| TeeError::Execution(format!("Failed to write input to file: {}", e)))?;

        // Create an output file
        let output_path = temp_dir.path().join("output.json");

        // Create an enclave configuration
        let config = EnclaveConfig {
            memory_mib: 2048,
            cpu_count: 2,
            timeout_seconds: 60,
            debug_mode: false,
        };

        // Run the code in the enclave
        let enclave_id = self
            .enclave_manager
            .run_enclave(&config, &code_path, &input_path, &output_path)
            .await
            .map_err(|e| TeeError::Execution(format!("Failed to run enclave: {}", e)))?;

        // Wait for the enclave to complete
        self.enclave_manager
            .wait_for_enclave(&enclave_id)
            .await
            .map_err(|e| TeeError::Execution(format!("Failed to wait for enclave: {}", e)))?;

        // Read the output
        let output = std::fs::read_to_string(&output_path)
            .map_err(|e| TeeError::Execution(format!("Failed to read output: {}", e)))?;

        // Parse the output
        let result: serde_json::Value = serde_json::from_str(&output)
            .map_err(|e| TeeError::Execution(format!("Failed to parse output: {}", e)))?;

        // Clean up the enclave
        self.enclave_manager
            .terminate_enclave(&enclave_id)
            .await
            .map_err(|e| TeeError::Execution(format!("Failed to terminate enclave: {}", e)))?;

        info!("Code execution in AWS Nitro enclave completed successfully");
        Ok(result)
    }

    async fn generate_attestation(&self) -> Result<AttestationReport, TeeError> {
        info!("Generating attestation for AWS Nitro enclave");

        // Create a temporary directory for the attestation
        let temp_dir = tempfile::tempdir().map_err(|e| {
            TeeError::Attestation(format!("Failed to create temporary directory: {}", e))
        })?;

        // Create an attestation request file
        let request_path = temp_dir.path().join("attestation_request.json");
        let request_data = serde_json::json!({
            "nonce": uuid::Uuid::new_v4().to_string(),
            "timestamp": chrono::Utc::now().timestamp(),
            "user_data": "r3e-faas-attestation-request",
        });

        std::fs::write(&request_path, request_data.to_string()).map_err(|e| {
            TeeError::Attestation(format!("Failed to write attestation request: {}", e))
        })?;

        // Create an output file
        let output_path = temp_dir.path().join("attestation.json");

        // Generate the attestation
        self.attestation_service
            .generate_attestation(&request_path, &output_path)
            .await
            .map_err(|e| TeeError::Attestation(format!("Failed to generate attestation: {}", e)))?;

        // Read the attestation
        let attestation_data = std::fs::read_to_string(&output_path)
            .map_err(|e| TeeError::Attestation(format!("Failed to read attestation: {}", e)))?;

        // Parse the attestation
        let attestation_json: serde_json::Value = serde_json::from_str(&attestation_data)
            .map_err(|e| TeeError::Attestation(format!("Failed to parse attestation: {}", e)))?;

        // Extract the attestation fields
        let code_hash = attestation_json["document"]["pcrs"]["pcr0"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let signer_hash = attestation_json["document"]["pcrs"]["pcr8"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let product_id = attestation_json["document"]["module_id"]
            .as_u64()
            .unwrap_or(0) as u32;

        let security_version = attestation_json["document"]["version"]
            .as_u64()
            .unwrap_or(0) as u32;

        let signature = attestation_json["signature"]
            .as_str()
            .map(|s| hex::decode(s).unwrap_or_default())
            .unwrap_or_default();

        // Create the attestation report
        let attestation = AttestationReport {
            platform: TeePlatform::Nitro,
            security_level: TeeSecurityLevel::Production,
            code_hash,
            signer_hash,
            product_id,
            security_version,
            attributes: 0,
            extended_product_id: vec![],
            signature,
            platform_data: attestation_json,
        };

        info!("Attestation for AWS Nitro enclave generated successfully");
        Ok(attestation)
    }

    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError> {
        info!("Verifying attestation for AWS Nitro enclave");

        // Check if the attestation is for Nitro
        if attestation.platform != TeePlatform::Nitro {
            return Err(TeeError::Attestation(
                "Attestation is not for Nitro platform".to_string(),
            ));
        }

        // Create a temporary directory for the verification
        let temp_dir = tempfile::tempdir().map_err(|e| {
            TeeError::Attestation(format!("Failed to create temporary directory: {}", e))
        })?;

        // Write the attestation to a file
        let attestation_path = temp_dir.path().join("attestation.json");
        std::fs::write(
            &attestation_path,
            serde_json::to_string(&attestation.platform_data).unwrap_or_default(),
        )
        .map_err(|e| TeeError::Attestation(format!("Failed to write attestation: {}", e)))?;

        // Verify the attestation
        let result = self
            .attestation_service
            .verify_attestation(&attestation_path)
            .await
            .map_err(|e| TeeError::Attestation(format!("Failed to verify attestation: {}", e)))?;

        info!("Attestation verification result: {}", result);
        Ok(result)
    }
}
