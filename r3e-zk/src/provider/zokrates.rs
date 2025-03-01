// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Zokrates provider for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkCircuitId, ZkCircuitMetadata, ZkError, ZkPlatform, ZkProof, ZkProofId,
    ZkProvingKey, ZkProvingKeyId, ZkResult, ZkVerificationKey, ZkVerificationKeyId,
};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
use tokio::fs;
use zokrates_core::compile::{compile as zokrates_compile, CompileConfig};
use zokrates_core::ir;
use zokrates_core::proof_system::{
    bellman::Bellman, groth16::G16, Proof as ZokratesProof, ProofSystem, SetupKeypair,
};
use zokrates_field::Bn128Field;

use super::ZkProvider;

/// Zokrates provider for Zero-Knowledge operations.
#[derive(Debug)]
pub struct ZokratesProvider {
    /// Default optimization level.
    pub default_optimization_level: u8,
    /// Working directory for temporary files.
    temp_dir: TempDir,
}

impl ZokratesProvider {
    /// Create a new Zokrates provider.
    pub fn new(default_optimization_level: u8) -> ZkResult<Self> {
        let temp_dir = TempDir::new().map_err(|e| {
            ZkError::Provider(format!("Failed to create temporary directory: {}", e))
        })?;

        Ok(Self {
            default_optimization_level,
            temp_dir,
        })
    }

    /// Get the current timestamp.
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get a temporary file path.
    fn get_temp_file_path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    /// Write data to a temporary file.
    fn write_temp_file(&self, name: &str, data: &[u8]) -> ZkResult<PathBuf> {
        let path = self.get_temp_file_path(name);
        std::fs::write(&path, data)
            .map_err(|e| ZkError::Provider(format!("Failed to write temporary file: {}", e)))?;
        Ok(path)
    }

    /// Parse inputs from JSON value.
    fn parse_inputs(&self, inputs: &Value) -> ZkResult<Vec<Bn128Field>> {
        let mut result = Vec::new();

        if let Some(array) = inputs.as_array() {
            for value in array {
                if let Some(num_str) = value.as_str() {
                    let field = Bn128Field::try_from_str(num_str).map_err(|_| {
                        ZkError::InvalidInput(format!("Invalid field element: {}", num_str))
                    })?;
                    result.push(field);
                } else if let Some(num) = value.as_u64() {
                    let field = Bn128Field::try_from_dec_str(&num.to_string()).map_err(|_| {
                        ZkError::InvalidInput(format!("Invalid field element: {}", num))
                    })?;
                    result.push(field);
                } else {
                    return Err(ZkError::InvalidInput(format!(
                        "Invalid input value: {}",
                        value
                    )));
                }
            }
        } else {
            return Err(ZkError::InvalidInput("Inputs must be an array".to_string()));
        }

        Ok(result)
    }

    /// Serialize proof to bytes.
    fn serialize_proof(&self, proof: ZokratesProof<Bn128Field, G16>) -> ZkResult<Vec<u8>> {
        let serialized = serde_json::to_vec(&proof)
            .map_err(|e| ZkError::Provider(format!("Failed to serialize proof: {}", e)))?;

        Ok(serialized)
    }

    /// Deserialize proof from bytes.
    fn deserialize_proof(&self, data: &[u8]) -> ZkResult<ZokratesProof<Bn128Field, G16>> {
        let proof: ZokratesProof<Bn128Field, G16> = serde_json::from_slice(data)
            .map_err(|e| ZkError::Provider(format!("Failed to deserialize proof: {}", e)))?;

        Ok(proof)
    }
}

#[async_trait]
impl ZkProvider for ZokratesProvider {
    fn name(&self) -> &str {
        "Zokrates"
    }

    fn platform(&self) -> ZkPlatform {
        ZkPlatform::Zokrates
    }

    async fn compile_circuit(&self, code: &str) -> ZkResult<ZkCircuit> {
        info!("Compiling circuit with Zokrates provider");
        debug!("Circuit code length: {}", code.len());

        // Write the code to a temporary file
        let source_path = self.write_temp_file("source.zok", code.as_bytes())?;

        // Compile the circuit
        let compile_config = CompileConfig::default();
        let (compiled_program, compile_info) =
            zokrates_compile(source_path.to_str().unwrap(), None, Some(&compile_config)).map_err(
                |e| ZkError::Compilation(format!("Failed to compile Zokrates circuit: {}", e)),
            )?;

        // Serialize the compiled program
        let compiled_data = bincode::serialize(&compiled_program).map_err(|e| {
            ZkError::Provider(format!("Failed to serialize compiled program: {}", e))
        })?;

        let circuit_id = ZkCircuitId::new();
        let timestamp = Self::current_timestamp();

        // Create metadata
        let metadata = ZkCircuitMetadata {
            name: Some("Zokrates Circuit".to_string()),
            description: Some("Compiled with Zokrates provider".to_string()),
            input_count: compile_info.arguments.len(),
            output_count: compile_info.return_count,
            constraint_count: compile_info.constraint_count,
            created_at: timestamp,
            properties: serde_json::json!({
                "optimization_level": self.default_optimization_level,
                "compiler_version": env!("CARGO_PKG_VERSION"),
                "field_type": "Bn128Field",
                "proof_system": "Groth16",
            }),
        };

        Ok(ZkCircuit {
            id: circuit_id,
            platform: ZkPlatform::Zokrates,
            source_code: code.to_string(),
            compiled_data,
            metadata,
        })
    }

    async fn generate_keys(
        &self,
        circuit: &ZkCircuit,
    ) -> ZkResult<(ZkProvingKey, ZkVerificationKey)> {
        info!("Generating keys with Zokrates provider");
        debug!("Circuit ID: {}", circuit.id);

        // Deserialize the compiled program
        let program: ir::ProgIterator<Bn128Field> = bincode::deserialize(&circuit.compiled_data)
            .map_err(|e| {
                ZkError::Provider(format!("Failed to deserialize compiled program: {}", e))
            })?;

        // Setup the proving system
        let proving_system = Bellman::<Bn128Field, G16>::new();
        let keypair = proving_system
            .setup(program)
            .map_err(|e| ZkError::Provider(format!("Failed to setup proving system: {}", e)))?;

        // Serialize the keys
        let proving_key_data = bincode::serialize(&keypair.pk)
            .map_err(|e| ZkError::Provider(format!("Failed to serialize proving key: {}", e)))?;

        let verification_key_data = bincode::serialize(&keypair.vk).map_err(|e| {
            ZkError::Provider(format!("Failed to serialize verification key: {}", e))
        })?;

        let timestamp = Self::current_timestamp();

        let proving_key = ZkProvingKey {
            id: ZkProvingKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Zokrates,
            key_data: proving_key_data,
            created_at: timestamp,
        };

        let verification_key = ZkVerificationKey {
            id: ZkVerificationKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Zokrates,
            key_data: verification_key_data,
            created_at: timestamp,
        };

        Ok((proving_key, verification_key))
    }

    async fn generate_proof(
        &self,
        circuit: &ZkCircuit,
        inputs: &Value,
        proving_key: &ZkProvingKey,
    ) -> ZkResult<ZkProof> {
        info!("Generating proof with Zokrates provider");
        debug!("Circuit ID: {}, Inputs: {}", circuit.id, inputs);

        // Deserialize the compiled program
        let program: ir::ProgIterator<Bn128Field> = bincode::deserialize(&circuit.compiled_data)
            .map_err(|e| {
                ZkError::Provider(format!("Failed to deserialize compiled program: {}", e))
            })?;

        // Deserialize the proving key
        let pk = bincode::deserialize(&proving_key.key_data)
            .map_err(|e| ZkError::Provider(format!("Failed to deserialize proving key: {}", e)))?;

        // Parse inputs
        let witness_inputs = self.parse_inputs(inputs)?;

        // Create the witness
        let witness = program
            .execute(&witness_inputs)
            .map_err(|e| ZkError::Provider(format!("Failed to execute program: {}", e)))?;

        // Generate the proof
        let proving_system = Bellman::<Bn128Field, G16>::new();
        let proof = proving_system
            .generate_proof(program, witness, pk)
            .map_err(|e| ZkError::Provider(format!("Failed to generate proof: {}", e)))?;

        // Serialize the proof
        let proof_data = self.serialize_proof(proof)?;

        let timestamp = Self::current_timestamp();

        let proof = ZkProof {
            id: ZkProofId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Zokrates,
            proof_data,
            public_inputs: inputs.clone(),
            created_at: timestamp,
        };

        Ok(proof)
    }

    async fn verify_proof(
        &self,
        proof: &ZkProof,
        public_inputs: &Value,
        verification_key: &ZkVerificationKey,
    ) -> ZkResult<bool> {
        info!("Verifying proof with Zokrates provider");
        debug!("Proof ID: {}, Public inputs: {}", proof.id, public_inputs);

        // Deserialize the verification key
        let vk = bincode::deserialize(&verification_key.key_data).map_err(|e| {
            ZkError::Provider(format!("Failed to deserialize verification key: {}", e))
        })?;

        // Deserialize the proof
        let zokrates_proof = self.deserialize_proof(&proof.proof_data)?;

        // Parse public inputs
        let inputs = self.parse_inputs(public_inputs)?;

        // Verify the proof
        let proving_system = Bellman::<Bn128Field, G16>::new();
        let result = proving_system
            .verify(zokrates_proof, vk, &inputs)
            .map_err(|e| ZkError::Provider(format!("Failed to verify proof: {}", e)))?;

        Ok(result)
    }
}
