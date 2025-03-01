// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod price;
pub mod random;

use std::collections::HashMap;
use std::sync::Arc;

use crate::{OracleError, OracleProvider, OracleRequest, OracleRequestType, OracleResponse};

/// Provider registry for managing oracle providers
pub struct ProviderRegistry {
    providers: HashMap<OracleRequestType, Vec<Arc<dyn OracleProvider>>>,
}

impl ProviderRegistry {
    /// Create a new provider registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider for a specific request type
    pub fn register_provider(&mut self, provider: Arc<dyn OracleProvider>) {
        for request_type in provider.supported_types() {
            self.providers
                .entry(request_type)
                .or_insert_with(Vec::new)
                .push(Arc::clone(&provider));
        }
    }

    /// Get providers for a specific request type
    pub fn get_providers(&self, request_type: OracleRequestType) -> Vec<Arc<dyn OracleProvider>> {
        self.providers
            .get(&request_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Process a request using the appropriate provider
    pub async fn process_request(
        &self,
        request: &OracleRequest,
    ) -> Result<OracleResponse, OracleError> {
        let providers = self.get_providers(request.request_type);

        if providers.is_empty() {
            return Err(OracleError::Provider(format!(
                "No provider available for request type: {:?}",
                request.request_type
            )));
        }

        // Use the first provider for now
        // In a more advanced implementation, we could use multiple providers and aggregate results
        let provider = &providers[0];

        provider.process_request(request).await
    }
}
