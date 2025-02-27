// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde_json::json;
use sha2::{Sha256, Digest};

use crate::{OracleError, OracleProvider, OracleRequest, OracleRequestType, OracleResponse};
use crate::types::{RandomMethod, RandomRequest, RandomResponse};

/// Random number generation provider
pub struct RandomProvider {
    /// Neo RPC client for blockchain-based randomness
    neo_client: Option<NeoRust::neo_clients::RpcClient>,
}

impl RandomProvider {
    /// Create a new random provider
    pub fn new(neo_client: Option<NeoRust::neo_clients::RpcClient>) -> Self {
        Self {
            neo_client,
        }
    }
    
    /// Generate secure random numbers
    fn generate_secure_random(&self, min: u64, max: u64, count: u32) -> Vec<u64> {
        let mut rng = rand::thread_rng();
        let mut values = Vec::with_capacity(count as usize);
        
        for _ in 0..count {
            values.push(rng.gen_range(min..=max));
        }
        
        values
    }
    
    /// Generate deterministic random numbers using a seed
    fn generate_seeded_random(&self, min: u64, max: u64, count: u32, seed: &str) -> Vec<u64> {
        // Create a seed from the provided string
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();
        
        // Convert the first 32 bytes of the hash to a seed array
        let mut seed_array = [0u8; 32];
        for (i, byte) in hash.iter().enumerate().take(32) {
            seed_array[i] = *byte;
        }
        
        // Create a seeded RNG
        let mut rng = StdRng::from_seed(seed_array);
        let mut values = Vec::with_capacity(count as usize);
        
        for _ in 0..count {
            values.push(rng.gen_range(min..=max));
        }
        
        values
    }
    
    /// Generate blockchain-based random numbers
    async fn generate_blockchain_random(&self, min: u64, max: u64, count: u32) -> Result<(Vec<u64>, String), OracleError> {
        if let Some(client) = &self.neo_client {
            // Get the latest block hash as a source of randomness
            let block = client.get_best_block_hash()
                .await
                .map_err(|e| OracleError::Provider(format!("Failed to get block hash: {}", e)))?;
            
            // Use the block hash as a seed
            let seed = block.to_string();
            let values = self.generate_seeded_random(min, max, count, &seed);
            
            // Create a proof using the block hash
            let proof = format!("block_hash:{}", seed);
            
            Ok((values, proof))
        } else {
            Err(OracleError::Provider("Neo client not available for blockchain randomness".to_string()))
        }
    }
    
    /// Generate VRF-based random numbers
    async fn generate_vrf_random(&self, min: u64, max: u64, count: u32) -> Result<(Vec<u64>, String), OracleError> {
        // In a real implementation, this would use a verifiable random function
        // For this example, we'll use a simple hash-based approach
        
        // Generate a random seed
        let mut rng = rand::thread_rng();
        let random_seed: u64 = rng.gen();
        
        // Create a seed string
        let seed = format!("vrf:{}", random_seed);
        
        // Generate random values
        let values = self.generate_seeded_random(min, max, count, &seed);
        
        // Create a proof
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();
        let proof = format!("vrf_proof:{}", hex::encode(hash));
        
        Ok((values, proof))
    }
}

#[async_trait]
impl OracleProvider for RandomProvider {
    fn name(&self) -> &str {
        "random"
    }
    
    fn description(&self) -> &str {
        "Provides random number generation services"
    }
    
    fn supported_types(&self) -> Vec<OracleRequestType> {
        vec![OracleRequestType::Random]
    }
    
    async fn process_request(&self, request: &OracleRequest) -> Result<OracleResponse, OracleError> {
        if request.request_type != OracleRequestType::Random {
            return Err(OracleError::Validation(format!(
                "Unsupported request type: {:?}",
                request.request_type
            )));
        }
        
        // Parse request data
        let random_request: RandomRequest = serde_json::from_str(&request.data)
            .map_err(|e| OracleError::Validation(format!("Invalid random request data: {}", e)))?;
        
        // Validate request parameters
        if random_request.min > random_request.max {
            return Err(OracleError::Validation(
                "Minimum value cannot be greater than maximum value".to_string()
            ));
        }
        
        if random_request.count == 0 || random_request.count > 1000 {
            return Err(OracleError::Validation(
                "Count must be between 1 and 1000".to_string()
            ));
        }
        
        // Generate random values based on the method
        let (values, proof) = match random_request.method {
            RandomMethod::Secure => {
                let values = self.generate_secure_random(
                    random_request.min,
                    random_request.max,
                    random_request.count,
                );
                (values, None)
            }
            RandomMethod::Blockchain => {
                let (values, proof) = self.generate_blockchain_random(
                    random_request.min,
                    random_request.max,
                    random_request.count,
                ).await?;
                (values, Some(proof))
            }
            RandomMethod::Vrf => {
                let (values, proof) = self.generate_vrf_random(
                    random_request.min,
                    random_request.max,
                    random_request.count,
                ).await?;
                (values, Some(proof))
            }
        };
        
        // Create response
        let random_response = RandomResponse {
            values,
            method: random_request.method,
            proof,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        let response_data = serde_json::to_string(&random_response)
            .map_err(|e| OracleError::Internal(format!("Failed to serialize response: {}", e)))?;
        
        Ok(OracleResponse {
            request_id: request.id.clone(),
            data: response_data,
            status_code: 200,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            error: None,
        })
    }
}
