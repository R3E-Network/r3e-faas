// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Provider interface for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkError, ZkPlatform, ZkProof, ZkProvingKey, ZkResult, ZkVerificationKey,
};
use async_trait::async_trait;
use serde_json::Value;
use std::fmt::Debug;

mod bulletproofs;
mod zokrates;

pub use bulletproofs::BulletproofsProvider;
pub use zokrates::ZokratesProvider;

/// Provider interface for Zero-Knowledge operations.
#[async_trait]
pub trait ZkProvider: Send + Sync + Debug {
    /// Get the name of the provider.
    fn name(&self) -> &str;

    /// Get the platform of the provider.
    fn platform(&self) -> ZkPlatform;

    /// Compile a circuit from source code.
    async fn compile_circuit(&self, code: &str) -> ZkResult<ZkCircuit>;

    /// Generate proving and verification keys for a circuit.
    async fn generate_keys(
        &self,
        circuit: &ZkCircuit,
    ) -> ZkResult<(ZkProvingKey, ZkVerificationKey)>;

    /// Generate a proof for a circuit with the given inputs.
    async fn generate_proof(
        &self,
        circuit: &ZkCircuit,
        inputs: &Value,
        proving_key: &ZkProvingKey,
    ) -> ZkResult<ZkProof>;

    /// Verify a proof with the given public inputs.
    async fn verify_proof(
        &self,
        proof: &ZkProof,
        public_inputs: &Value,
        verification_key: &ZkVerificationKey,
    ) -> ZkResult<bool>;
}
