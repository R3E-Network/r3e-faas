// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Service implementation for the Zero-Knowledge computing service.

use crate::{
    provider::{
        ArkworksProvider, BellmanProvider, BulletproofsProvider, CircomProvider, ZkProvider,
        ZokratesProvider,
    },
    storage::{MemoryZkStorage, RocksDbZkStorage, ZkStorage},
    ZkCircuit, ZkCircuitId, ZkConfig, ZkError, ZkPlatform, ZkProof, ZkProofId, ZkProvingKey,
    ZkProvingKeyId, ZkResult, ZkStorageType, ZkVerificationKey, ZkVerificationKeyId,
};
use log::{debug, info};
use serde_json::Value;
use std::{collections::HashMap, path::Path, sync::Arc};

/// Service for Zero-Knowledge operations.
#[derive(Debug)]
pub struct ZkService {
    /// Configuration for the service.
    config: ZkConfig,
    /// Providers for different ZK platforms.
    providers: HashMap<ZkPlatform, Arc<dyn ZkProvider>>,
    /// Storage for ZK data.
    storage: Arc<dyn ZkStorage>,
}

impl ZkService {
    /// Create a new ZK service with the given configuration.
    pub async fn new(config: ZkConfig) -> ZkResult<Self> {
        // Initialize storage
        let storage: Arc<dyn ZkStorage> = match config.storage.storage_type {
            ZkStorageType::Memory => Arc::new(MemoryZkStorage::new()),
            ZkStorageType::RocksDb => {
                let path = config.storage.rocksdb_path.as_ref().ok_or_else(|| {
                    ZkError::ConfigurationError("RocksDB path not specified".into())
                })?;
                Arc::new(RocksDbZkStorage::new(path)?)
            }
        };

        // Initialize providers
        let mut providers = HashMap::new();

        // Add Zokrates provider if enabled
        if let Some(zokrates_config) = &config.providers.zokrates {
            if zokrates_config.enabled {
                let provider = ZokratesProvider::new(zokrates_config.default_optimization_level);
                providers.insert(ZkPlatform::Zokrates, Arc::new(provider));
            }
        }

        // Add Bulletproofs provider if enabled
        if let Some(bulletproofs_config) = &config.providers.bulletproofs {
            if bulletproofs_config.enabled {
                let provider = BulletproofsProvider::new(bulletproofs_config.default_generators);
                providers.insert(ZkPlatform::Bulletproofs, Arc::new(provider));
            }
        }

        // Add Circom provider if enabled
        if let Some(circom_config) = &config.providers.circom {
            if circom_config.enabled {
                let provider = CircomProvider::new(circom_config.default_witness_strategy.clone());
                providers.insert(ZkPlatform::Circom, Arc::new(provider));
            }
        }

        // Add Bellman provider if enabled
        if let Some(bellman_config) = &config.providers.bellman {
            if bellman_config.enabled {
                let provider = BellmanProvider::new(bellman_config.default_curve.clone());
                providers.insert(ZkPlatform::Bellman, Arc::new(provider));
            }
        }

        // Add Arkworks provider if enabled
        if let Some(arkworks_config) = &config.providers.arkworks {
            if arkworks_config.enabled {
                let provider = ArkworksProvider::new(
                    arkworks_config.default_proving_system.clone(),
                    arkworks_config.default_curve.clone(),
                );
                providers.insert(ZkPlatform::Arkworks, Arc::new(provider));
            }
        }

        Ok(Self {
            config,
            providers,
            storage,
        })
    }

    /// Register a provider for a ZK platform.
    pub fn register_provider(&mut self, provider: Arc<dyn ZkProvider>) {
        let platform = provider.platform();
        info!("Registering provider for platform: {}", platform);
        self.providers.insert(platform, provider);
    }

    /// Get a provider for a ZK platform.
    fn get_provider(&self, platform: ZkPlatform) -> ZkResult<Arc<dyn ZkProvider>> {
        self.providers.get(&platform).cloned().ok_or_else(|| {
            ZkError::UnsupportedPlatformError(format!("Unsupported platform: {}", platform))
        })
    }

    /// Compile a circuit from source code.
    pub async fn compile_circuit(&self, code: &str, platform: ZkPlatform) -> ZkResult<ZkCircuitId> {
        info!("Compiling circuit for platform: {}", platform);
        debug!("Circuit code length: {}", code.len());

        // Get the provider for the platform
        let provider = self.get_provider(platform)?;

        // Compile the circuit
        let circuit = provider.compile_circuit(code).await?;

        // Store the circuit
        self.storage.store_circuit(&circuit).await?;

        Ok(circuit.id)
    }

    /// Generate proving and verification keys for a circuit.
    pub async fn generate_keys(
        &self,
        circuit_id: &ZkCircuitId,
    ) -> ZkResult<(ZkProvingKeyId, ZkVerificationKeyId)> {
        info!("Generating keys for circuit: {}", circuit_id);

        // Get the circuit
        let circuit = self.storage.get_circuit(circuit_id).await?;

        // Get the provider for the platform
        let provider = self.get_provider(circuit.platform)?;

        // Generate the keys
        let (proving_key, verification_key) = provider.generate_keys(&circuit).await?;

        // Store the keys
        self.storage.store_proving_key(&proving_key).await?;
        self.storage
            .store_verification_key(&verification_key)
            .await?;

        Ok((proving_key.id, verification_key.id))
    }

    /// Generate a proof for a circuit with the given inputs.
    pub async fn generate_proof(
        &self,
        circuit_id: &ZkCircuitId,
        inputs: &Value,
        proving_key_id: &ZkProvingKeyId,
    ) -> ZkResult<ZkProofId> {
        info!("Generating proof for circuit: {}", circuit_id);
        debug!("Inputs: {}", inputs);

        // Get the circuit
        let circuit = self.storage.get_circuit(circuit_id).await?;

        // Get the proving key
        let proving_key = self.storage.get_proving_key(proving_key_id).await?;

        // Ensure the proving key is for the correct circuit
        if proving_key.circuit_id != *circuit_id {
            return Err(ZkError::InvalidInputError(format!(
                "Proving key is for circuit {} but expected {}",
                proving_key.circuit_id, circuit_id
            )));
        }

        // Get the provider for the platform
        let provider = self.get_provider(circuit.platform)?;

        // Generate the proof
        let proof = provider
            .generate_proof(&circuit, inputs, &proving_key)
            .await?;

        // Store the proof
        self.storage.store_proof(&proof).await?;

        Ok(proof.id)
    }

    /// Verify a proof with the given public inputs.
    pub async fn verify_proof(
        &self,
        proof_id: &ZkProofId,
        public_inputs: &Value,
        verification_key_id: &ZkVerificationKeyId,
    ) -> ZkResult<bool> {
        info!("Verifying proof: {}", proof_id);
        debug!("Public inputs: {}", public_inputs);

        // Get the proof
        let proof = self.storage.get_proof(proof_id).await?;

        // Get the verification key
        let verification_key = self
            .storage
            .get_verification_key(verification_key_id)
            .await?;

        // Ensure the verification key is for the correct circuit
        if verification_key.circuit_id != proof.circuit_id {
            return Err(ZkError::InvalidInputError(format!(
                "Verification key is for circuit {} but proof is for {}",
                verification_key.circuit_id, proof.circuit_id
            )));
        }

        // Get the provider for the platform
        let provider = self.get_provider(proof.platform)?;

        // Verify the proof
        provider
            .verify_proof(&proof, public_inputs, &verification_key)
            .await
    }

    /// Get a circuit by ID.
    pub async fn get_circuit(&self, id: &ZkCircuitId) -> ZkResult<ZkCircuit> {
        self.storage.get_circuit(id).await
    }

    /// List all circuits.
    pub async fn list_circuits(&self) -> ZkResult<Vec<ZkCircuit>> {
        self.storage.list_circuits().await
    }

    /// Delete a circuit by ID.
    pub async fn delete_circuit(&self, id: &ZkCircuitId) -> ZkResult<()> {
        info!("Deleting circuit: {}", id);

        // Delete all associated keys and proofs
        let proving_keys = self.storage.list_proving_keys(id).await?;
        for key in proving_keys {
            self.storage.delete_proving_key(&key.id).await?;
        }

        let verification_keys = self.storage.list_verification_keys(id).await?;
        for key in verification_keys {
            self.storage.delete_verification_key(&key.id).await?;
        }

        let proofs = self.storage.list_proofs(id).await?;
        for proof in proofs {
            self.storage.delete_proof(&proof.id).await?;
        }

        // Delete the circuit
        self.storage.delete_circuit(id).await
    }

    /// Get a proving key by ID.
    pub async fn get_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<ZkProvingKey> {
        self.storage.get_proving_key(id).await
    }

    /// List all proving keys for a circuit.
    pub async fn list_proving_keys(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProvingKey>> {
        self.storage.list_proving_keys(circuit_id).await
    }

    /// Delete a proving key by ID.
    pub async fn delete_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<()> {
        info!("Deleting proving key: {}", id);
        self.storage.delete_proving_key(id).await
    }

    /// Get a verification key by ID.
    pub async fn get_verification_key(
        &self,
        id: &ZkVerificationKeyId,
    ) -> ZkResult<ZkVerificationKey> {
        self.storage.get_verification_key(id).await
    }

    /// List all verification keys for a circuit.
    pub async fn list_verification_keys(
        &self,
        circuit_id: &ZkCircuitId,
    ) -> ZkResult<Vec<ZkVerificationKey>> {
        self.storage.list_verification_keys(circuit_id).await
    }

    /// Delete a verification key by ID.
    pub async fn delete_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<()> {
        info!("Deleting verification key: {}", id);
        self.storage.delete_verification_key(id).await
    }

    /// Get a proof by ID.
    pub async fn get_proof(&self, id: &ZkProofId) -> ZkResult<ZkProof> {
        self.storage.get_proof(id).await
    }

    /// List all proofs for a circuit.
    pub async fn list_proofs(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProof>> {
        self.storage.list_proofs(circuit_id).await
    }

    /// Delete a proof by ID.
    pub async fn delete_proof(&self, id: &ZkProofId) -> ZkResult<()> {
        info!("Deleting proof: {}", id);
        self.storage.delete_proof(id).await
    }
}
