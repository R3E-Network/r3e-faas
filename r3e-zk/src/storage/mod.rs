// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage interface for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkCircuitId, ZkError, ZkProof, ZkProofId, ZkProvingKey, ZkProvingKeyId, ZkResult,
    ZkVerificationKey, ZkVerificationKeyId,
};
use async_trait::async_trait;
use std::fmt::Debug;

mod memory;
mod rocksdb;

pub use memory::MemoryZkStorage;
pub use rocksdb::RocksDbZkStorage;

/// Storage interface for Zero-Knowledge data.
#[async_trait]
pub trait ZkStorage: Send + Sync + Debug {
    /// Store a circuit.
    async fn store_circuit(&self, circuit: &ZkCircuit) -> ZkResult<()>;

    /// Retrieve a circuit by ID.
    async fn get_circuit(&self, id: &ZkCircuitId) -> ZkResult<ZkCircuit>;

    /// List all circuits.
    async fn list_circuits(&self) -> ZkResult<Vec<ZkCircuit>>;

    /// Delete a circuit by ID.
    async fn delete_circuit(&self, id: &ZkCircuitId) -> ZkResult<()>;

    /// Store a proving key.
    async fn store_proving_key(&self, key: &ZkProvingKey) -> ZkResult<()>;

    /// Retrieve a proving key by ID.
    async fn get_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<ZkProvingKey>;

    /// List all proving keys for a circuit.
    async fn list_proving_keys(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProvingKey>>;

    /// Delete a proving key by ID.
    async fn delete_proving_key(&self, id: &ZkProvingKeyId) -> ZkResult<()>;

    /// Store a verification key.
    async fn store_verification_key(&self, key: &ZkVerificationKey) -> ZkResult<()>;

    /// Retrieve a verification key by ID.
    async fn get_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<ZkVerificationKey>;

    /// List all verification keys for a circuit.
    async fn list_verification_keys(
        &self,
        circuit_id: &ZkCircuitId,
    ) -> ZkResult<Vec<ZkVerificationKey>>;

    /// Delete a verification key by ID.
    async fn delete_verification_key(&self, id: &ZkVerificationKeyId) -> ZkResult<()>;

    /// Store a proof.
    async fn store_proof(&self, proof: &ZkProof) -> ZkResult<()>;

    /// Retrieve a proof by ID.
    async fn get_proof(&self, id: &ZkProofId) -> ZkResult<ZkProof>;

    /// List all proofs for a circuit.
    async fn list_proofs(&self, circuit_id: &ZkCircuitId) -> ZkResult<Vec<ZkProof>>;

    /// Delete a proof by ID.
    async fn delete_proof(&self, id: &ZkProofId) -> ZkResult<()>;
}
