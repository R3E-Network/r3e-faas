// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! In-memory storage implementation for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkCircuitId, ZkError, ZkProof, ZkProofId, ZkProvingKey, ZkProvingKeyId, ZkResult,
    ZkStorage, ZkVerificationKey, ZkVerificationKeyId,
};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, RwLock},
};

/// In-memory storage for Zero-Knowledge data.
#[derive(Debug)]
pub struct MemoryZkStorage {
    /// Circuits stored in memory.
    circuits: Arc<RwLock<HashMap<ZkCircuitId, ZkCircuit>>>,
    /// Proving keys stored in memory.
    proving_keys: Arc<RwLock<HashMap<ZkProvingKeyId, ZkProvingKey>>>,
    /// Verification keys stored in memory.
    verification_keys: Arc<RwLock<HashMap<ZkVerificationKeyId, ZkVerificationKey>>>,
    /// Proofs stored in memory.
    proofs: Arc<RwLock<HashMap<ZkProofId, ZkProof>>>,
}

impl MemoryZkStorage {
    /// Create a new in-memory storage.
    pub fn new() -> Self {
        Self {
            circuits: Arc::new(RwLock::new(HashMap::new())),
            proving_keys: Arc::new(RwLock::new(HashMap::new())),
            verification_keys: Arc::new(RwLock::new(HashMap::new())),
            proofs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryZkStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ZkStorage for MemoryZkStorage {
    async fn store_circuit(&self, circuit: &ZkCircuit) -> ZkResult<()> {
        let mut circuits = self
            .circuits
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        circuits.insert(circuit.id.clone(), circuit.clone());
        Ok(())
    }

    async fn get_circuit(&self, id: &ZkCircuitId) -> ZkResult<ZkCircuit> {
        let circuits = self
            .circuits
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        circuits
            .get(id)
            .cloned()
            .ok_or_else(|| ZkError::MissingDataError(format!("Circuit not found: {}", id)))
    }

    async fn list_circuits(&self) -> ZkResult<Vec<ZkCircuit>> {
        let circuits = self
            .circuits
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(circuits.values().cloned().collect())
    }

    async fn delete_circuit(&self, id: &ZkCircuitId) -> ZkResult<()> {
        let mut circuits = self
            .circuits
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        circuits.remove(id);
        Ok(())
    }

    async fn store_proving_key(&self, key: &ZkProvingKey) -> ZkResult<()> {
        let mut proving_keys = self
            .proving_keys
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        proving_keys.insert(key.id.clone(), key.clone());
        Ok(())
    }

    async fn get_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<ZkProvingKey> {
        let proving_keys = self
            .proving_keys
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        proving_keys
            .get(id)
            .cloned()
            .ok_or_else(|| ZkError::MissingDataError(format!("Proving key not found: {}", id)))
    }

    async fn list_proving_keys(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProvingKey>> {
        let proving_keys = self
            .proving_keys
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(proving_keys
            .values()
            .filter(|key| key.circuit_id == *circuit_id)
            .cloned()
            .collect())
    }

    async fn delete_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<()> {
        let mut proving_keys = self
            .proving_keys
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        proving_keys.remove(id);
        Ok(())
    }

    async fn store_verification_key(&self, key: &ZkVerificationKey) -> ZkResult<()> {
        let mut verification_keys = self.verification_keys.write().map_err(|e| {
            ZkError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        verification_keys.insert(key.id.clone(), key.clone());
        Ok(())
    }

    async fn get_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<ZkVerificationKey> {
        let verification_keys = self.verification_keys.read().map_err(|e| {
            ZkError::StorageError(format!("Failed to acquire read lock: {}", e))
        })?;
        verification_keys
            .get(id)
            .cloned()
            .ok_or_else(|| ZkError::MissingDataError(format!("Verification key not found: {}", id)))
    }

    async fn list_verification_keys(
        &self,
        circuit_id: &ZkCircuitId,
    ) -> ZkResult<Vec<ZkVerificationKey>> {
        let verification_keys = self.verification_keys.read().map_err(|e| {
            ZkError::StorageError(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(verification_keys
            .values()
            .filter(|key| key.circuit_id == *circuit_id)
            .cloned()
            .collect())
    }

    async fn delete_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<()> {
        let mut verification_keys = self.verification_keys.write().map_err(|e| {
            ZkError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        verification_keys.remove(id);
        Ok(())
    }

    async fn store_proof(&self, proof: &ZkProof) -> ZkResult<()> {
        let mut proofs = self
            .proofs
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        proofs.insert(proof.id.clone(), proof.clone());
        Ok(())
    }

    async fn get_proof(&self, id: &ZkProofId) -> ZkResult<ZkProof> {
        let proofs = self
            .proofs
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        proofs
            .get(id)
            .cloned()
            .ok_or_else(|| ZkError::MissingDataError(format!("Proof not found: {}", id)))
    }

    async fn list_proofs(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProof>> {
        let proofs = self
            .proofs
            .read()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(proofs
            .values()
            .filter(|proof| proof.circuit_id == *circuit_id)
            .cloned()
            .collect())
    }

    async fn delete_proof(&self, id: &ZkProofId) -> ZkResult<()> {
        let mut proofs = self
            .proofs
            .write()
            .map_err(|e| ZkError::StorageError(format!("Failed to acquire write lock: {}", e)))?;
        proofs.remove(id);
        Ok(())
    }
}
