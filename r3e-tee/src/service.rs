// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::{AttestationReport, TeeError, TeeExecutionRequest, TeeExecutionResponse, TeePlatform, TeeSecurityLevel, TeeService};
use crate::attestation::{AttestationOptions, AttestationService, AttestationServiceImpl};
use crate::enclave::{EnclaveConfig, EnclaveManager};
use crate::key_management::{KeyManagementService, KeyManagementServiceImpl};
#[cfg(feature = "nitro")]
use crate::provider::NitroProvider;
use crate::provider::{NeoTeeProvider, TeeProviderImpl, create_default_neo_tee_provider};
use crate::types::{ExecutionOptions, ExecutionStats, NeoTeeRequest, NeoTeeResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// TEE service implementation
pub struct TeeServiceImpl {
    /// Providers for different platforms
    providers: HashMap<TeePlatform, Arc<dyn TeeProvider>>,
    
    /// Enclave manager
    enclave_manager: Arc<EnclaveManager>,
    
    /// Attestation service
    attestation_service: Arc<dyn AttestationService>,
    
    /// Key management service
    key_management_service: Arc<dyn KeyManagementService>,
}

impl TeeServiceImpl {
    /// Create a new TEE service
    pub fn new() -> Self {
        let enclave_manager = Arc::new(EnclaveManager::new());
        let attestation_service = Arc::new(AttestationServiceImpl::new()) as Arc<dyn AttestationService>;
        let key_management_service = Arc::new(KeyManagementServiceImpl::new()) as Arc<dyn KeyManagementService>;
        
        let mut providers = HashMap::new();
        
        // Register providers for different platforms
        #[cfg(feature = "sgx")]
        {
            let provider = TeeProviderImpl::default_for_platform(TeePlatform::Sgx);
            providers.insert(
                TeePlatform::Sgx,
                Arc::new(provider) as Arc<dyn TeeProvider>,
            );
        }
        
        #[cfg(feature = "sev")]
        {
            let provider = TeeProviderImpl::default_for_platform(TeePlatform::Sev);
            providers.insert(
                TeePlatform::Sev,
                Arc::new(provider) as Arc<dyn TeeProvider>,
            );
        }
        
        #[cfg(feature = "trustzone")]
        {
            let provider = TeeProviderImpl::default_for_platform(TeePlatform::TrustZone);
            providers.insert(
                TeePlatform::TrustZone,
                Arc::new(provider) as Arc<dyn TeeProvider>,
            );
        }
        
        #[cfg(feature = "nitro")]
        {
            // Register Nitro provider
            let provider = NitroProvider::default();
            providers.insert(
                TeePlatform::Nitro,
                Arc::new(provider) as Arc<dyn TeeProvider>,
            );
        }
        
        // Always register simulated provider
        let provider = TeeProviderImpl::default_for_platform(TeePlatform::Simulated);
        providers.insert(
            TeePlatform::Simulated,
            Arc::new(provider) as Arc<dyn TeeProvider>,
        );
        
        Self {
            providers,
            enclave_manager,
            attestation_service,
            key_management_service,
        }
    }
    
    /// Register a provider for a platform
    pub fn register_provider(&mut self, platform: TeePlatform, provider: Arc<dyn TeeProvider>) {
        self.providers.insert(platform, provider);
    }
    
    /// Get a provider for a platform
    pub fn get_provider(&self, platform: TeePlatform) -> Result<Arc<dyn TeeProvider>, TeeError> {
        self.providers.get(&platform).cloned().ok_or_else(|| {
            TeeError::Provider(format!("No provider available for platform: {:?}", platform))
        })
    }
}

#[async_trait::async_trait]
impl TeeService for TeeServiceImpl {
    fn supported_platforms(&self) -> Vec<TeePlatform> {
        self.providers.keys().cloned().collect()
    }
    
    async fn execute(&self, request: TeeExecutionRequest) -> Result<TeeExecutionResponse, TeeError> {
        // Get the platform to use
        let platform = request.platform.unwrap_or(TeePlatform::Simulated);
        
        // Get the provider for the platform
        let provider = self.get_provider(platform)?;
        
        // Record start time
        let start_time = std::time::Instant::now();
        
        // Execute the code
        let result = provider.execute(&request.code, &request.input).await?;
        
        // Record end time
        let execution_time = start_time.elapsed();
        
        // Generate attestation if requested
        let attestation = if request.require_attestation {
            Some(provider.generate_attestation().await?)
        } else {
            None
        };
        
        // Create execution stats
        let stats = ExecutionStats {
            execution_time_ms: execution_time.as_millis() as u64,
            memory_usage_mb: 10, // Simulated memory usage
            cpu_usage_percent: 5.0, // Simulated CPU usage
            io_operations: 0,
            network_operations: 0,
        };
        
        // Create response
        let response = TeeExecutionResponse {
            request_id: request.id,
            result,
            attestation,
            execution_time_ms: stats.execution_time_ms,
            memory_usage_mb: stats.memory_usage_mb,
            error: None,
        };
        
        Ok(response)
    }
    
    async fn generate_attestation(&self, platform: TeePlatform) -> Result<AttestationReport, TeeError> {
        // Get the provider for the platform
        let provider = self.get_provider(platform)?;
        
        // Generate attestation
        provider.generate_attestation().await
    }
    
    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError> {
        // Get the provider for the platform
        let provider = self.get_provider(attestation.platform)?;
        
        // Verify attestation
        provider.verify_attestation(attestation).await
    }
}

/// Neo N3 specific TEE service
pub struct NeoTeeService {
    /// Base TEE service
    base_service: TeeServiceImpl,
    
    /// Neo TEE provider
    neo_provider: Arc<NeoTeeProvider>,
}

impl NeoTeeService {
    /// Create a new Neo TEE service
    pub fn new(rpc_url: &str) -> Result<Self, TeeError> {
        let base_service = TeeServiceImpl::new();
        
        // Create a Neo TEE provider
        let neo_provider = create_default_neo_tee_provider(rpc_url)?;
        
        Ok(Self {
            base_service,
            neo_provider: Arc::new(neo_provider),
        })
    }
    
    /// Execute a Neo-specific TEE request
    pub async fn execute_neo_request(&self, request: &NeoTeeRequest) -> Result<NeoTeeResponse, TeeError> {
        self.neo_provider.execute_neo_request(request).await
    }
}

#[async_trait::async_trait]
impl TeeService for NeoTeeService {
    fn supported_platforms(&self) -> Vec<TeePlatform> {
        self.base_service.supported_platforms()
    }
    
    async fn execute(&self, request: TeeExecutionRequest) -> Result<TeeExecutionResponse, TeeError> {
        self.base_service.execute(request).await
    }
    
    async fn generate_attestation(&self, platform: TeePlatform) -> Result<AttestationReport, TeeError> {
        self.base_service.generate_attestation(platform).await
    }
    
    async fn verify_attestation(&self, attestation: &AttestationReport) -> Result<bool, TeeError> {
        self.base_service.verify_attestation(attestation).await
    }
}

/// Create a default TEE service
pub fn create_default_tee_service() -> TeeServiceImpl {
    TeeServiceImpl::new()
}

/// Create a default Neo TEE service
pub fn create_default_neo_tee_service(rpc_url: &str) -> Result<NeoTeeService, TeeError> {
    NeoTeeService::new(rpc_url)
}
