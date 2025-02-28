// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Bellman provider for the Zero-Knowledge computing service.

use crate::{
    ZkCircuit, ZkCircuitId, ZkCircuitMetadata, ZkError, ZkPlatform, ZkProof, ZkProofId,
    ZkProvingKey, ZkProvingKeyId, ZkResult, ZkVerificationKey, ZkVerificationKeyId,
};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
use tokio::fs;

// Bellman-specific imports
use bellman::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Parameters, Proof, VerifyingKey,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use bls12_381::Bls12;
use ff::Field;
use rand::rngs::OsRng;

use super::ZkProvider;

/// Bellman provider for Zero-Knowledge operations.
#[derive(Debug)]
pub struct BellmanProvider {
    /// Default curve type.
    pub default_curve: String,
    /// Working directory for temporary files.
    temp_dir: TempDir,
}

/// A simple example circuit for testing purposes.
#[derive(Clone)]
pub struct ExampleCircuit<E: bellman::pairing::Engine> {
    /// The inputs to the circuit.
    pub inputs: Vec<Option<E::Fr>>,
    /// The number of constraints in the circuit.
    pub num_constraints: usize,
}

impl<E: bellman::pairing::Engine> Circuit<E> for ExampleCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Create variables for inputs
        let mut variables = Vec::new();
        for (i, input) in self.inputs.iter().enumerate() {
            let var = cs.alloc(
                || format!("input {}", i),
                || input.ok_or(SynthesisError::AssignmentMissing),
            )?;
            variables.push(var);
        }

        // Create constraints
        for i in 0..self.num_constraints.saturating_sub(1) {
            cs.enforce(
                || format!("constraint {}", i),
                |lc| lc + variables[i],
                |lc| lc + variables[i + 1],
                |lc| lc + variables[(i + 2) % variables.len()],
            );
        }

        Ok(())
    }
}

impl BellmanProvider {
    /// Create a new Bellman provider.
    pub fn new(default_curve: String) -> ZkResult<Self> {
        let temp_dir = TempDir::new().map_err(|e| {
            ZkError::Provider(format!("Failed to create temporary directory: {}", e))
        })?;

        Ok(Self {
            default_curve,
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

    /// Parse inputs from a JSON value.
    fn parse_inputs(&self, inputs: &Value) -> ZkResult<Vec<Option<bls12_381::Scalar>>> {
        let mut result = Vec::new();

        if let Value::Array(arr) = inputs {
            for val in arr {
                if val.is_null() {
                    result.push(None);
                } else if let Some(num_str) = val.as_str() {
                    // Parse the string as a field element
                    let fe = self.parse_field_element(num_str)?;
                    result.push(Some(fe));
                } else if let Some(num) = val.as_u64() {
                    // Convert the number to a field element
                    let mut fe = bls12_381::Scalar::zero();
                    fe += num;
                    result.push(Some(fe));
                } else {
                    return Err(ZkError::Validation(format!("Invalid input value: {}", val)));
                }
            }
        } else {
            return Err(ZkError::Validation("Inputs must be an array".to_string()));
        }

        Ok(result)
    }

    /// Parse a field element from a string.
    fn parse_field_element(&self, s: &str) -> ZkResult<bls12_381::Scalar> {
        // For simplicity, we'll just parse decimal numbers
        let num = s
            .parse::<u64>()
            .map_err(|e| ZkError::Validation(format!("Failed to parse field element: {}", e)))?;

        let mut fe = bls12_381::Scalar::zero();
        fe += num;

        Ok(fe)
    }

    /// Create a circuit from the compiled data.
    fn create_circuit(
        &self,
        compiled_data: &[u8],
        inputs: &Value,
    ) -> ZkResult<ExampleCircuit<Bls12>> {
        // Parse the compiled data
        let num_constraints = u32::from_le_bytes(
            compiled_data
                .get(0..4)
                .ok_or_else(|| ZkError::Provider("Invalid compiled data".to_string()))?
                .try_into()
                .map_err(|_| ZkError::Provider("Failed to parse constraint count".to_string()))?,
        ) as usize;

        // Parse the inputs
        let inputs = self.parse_inputs(inputs)?;

        Ok(ExampleCircuit {
            inputs,
            num_constraints,
        })
    }

    /// Serialize parameters to bytes.
    fn serialize_parameters(&self, params: &Parameters<Bls12>) -> ZkResult<Vec<u8>> {
        let mut buffer = Vec::new();
        params
            .write(&mut buffer)
            .map_err(|e| ZkError::Provider(format!("Failed to serialize parameters: {}", e)))?;

        Ok(buffer)
    }

    /// Deserialize parameters from bytes.
    fn deserialize_parameters(&self, data: &[u8]) -> ZkResult<Parameters<Bls12>> {
        let mut reader = std::io::Cursor::new(data);
        let params = Parameters::read(&mut reader, false)
            .map_err(|e| ZkError::Provider(format!("Failed to deserialize parameters: {}", e)))?;

        Ok(params)
    }

    /// Serialize a proof to bytes.
    fn serialize_proof(&self, proof: &Proof<Bls12>) -> ZkResult<Vec<u8>> {
        let mut buffer = Vec::new();
        proof
            .write(&mut buffer)
            .map_err(|e| ZkError::Provider(format!("Failed to serialize proof: {}", e)))?;

        Ok(buffer)
    }

    /// Deserialize a proof from bytes.
    fn deserialize_proof(&self, data: &[u8]) -> ZkResult<Proof<Bls12>> {
        let mut reader = std::io::Cursor::new(data);
        let proof = Proof::read(&mut reader)
            .map_err(|e| ZkError::Provider(format!("Failed to deserialize proof: {}", e)))?;

        Ok(proof)
    }

    /// Serialize a verifying key to bytes.
    fn serialize_verifying_key(&self, vk: &VerifyingKey<Bls12>) -> ZkResult<Vec<u8>> {
        let mut buffer = Vec::new();
        vk.write(&mut buffer)
            .map_err(|e| ZkError::Provider(format!("Failed to serialize verifying key: {}", e)))?;

        Ok(buffer)
    }

    /// Deserialize a verifying key from bytes.
    fn deserialize_verifying_key(&self, data: &[u8]) -> ZkResult<VerifyingKey<Bls12>> {
        let mut reader = std::io::Cursor::new(data);
        let vk = VerifyingKey::read(&mut reader).map_err(|e| {
            ZkError::Provider(format!("Failed to deserialize verifying key: {}", e))
        })?;

        Ok(vk)
    }
}

#[async_trait]
impl ZkProvider for BellmanProvider {
    fn name(&self) -> &str {
        "Bellman"
    }

    fn platform(&self) -> ZkPlatform {
        ZkPlatform::Bellman
    }

    async fn compile_circuit(&self, code: &str) -> ZkResult<ZkCircuit> {
        info!("Compiling circuit with Bellman provider");
        debug!("Circuit code length: {}", code.len());

        // Parse the circuit description from the code
        // For simplicity, we'll assume the code is a JSON string with the following format:
        // {
        //   "num_constraints": 10,
        //   "input_count": 3
        // }

        let circuit_description: Value = serde_json::from_str(code).map_err(|e| {
            ZkError::Compilation(format!("Failed to parse circuit description: {}", e))
        })?;

        // Extract the number of constraints
        let num_constraints = circuit_description
            .get("num_constraints")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

        // Extract the input count
        let input_count = circuit_description
            .get("input_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as u32;

        // Create compiled data (just store the number of constraints for now)
        let mut compiled_data = Vec::new();
        compiled_data.extend_from_slice(&num_constraints.to_le_bytes());

        let circuit_id = ZkCircuitId::new();
        let timestamp = Self::current_timestamp();

        // Create metadata
        let metadata = ZkCircuitMetadata {
            name: Some("Bellman Circuit".to_string()),
            description: Some("Compiled with Bellman provider".to_string()),
            input_count: input_count as usize,
            output_count: 1, // Always 1 output for our example circuit
            constraint_count: num_constraints as usize,
            created_at: timestamp,
            properties: serde_json::json!({
                "curve": self.default_curve,
                "proving_system": "groth16",
                "version": env!("CARGO_PKG_VERSION"),
            }),
        };

        Ok(ZkCircuit {
            id: circuit_id,
            platform: ZkPlatform::Bellman,
            source_code: code.to_string(),
            compiled_data,
            metadata,
        })
    }

    async fn generate_keys(
        &self,
        circuit: &ZkCircuit,
    ) -> ZkResult<(ZkProvingKey, ZkVerificationKey)> {
        info!("Generating keys with Bellman provider");
        debug!("Circuit ID: {}", circuit.id);

        // Create a circuit instance from the compiled data
        let example_circuit = ExampleCircuit::<Bls12> {
            inputs: Vec::new(), // No inputs needed for key generation
            num_constraints: circuit.metadata.constraint_count,
        };

        // Generate parameters (proving key and verification key)
        let params = generate_random_parameters::<Bls12, _, _>(example_circuit.clone(), &mut OsRng)
            .map_err(|e| ZkError::Provider(format!("Failed to generate parameters: {}", e)))?;

        // Extract the verification key
        let vk = params.vk.clone();

        // Serialize the parameters and verification key
        let proving_key_data = self.serialize_parameters(&params)?;
        let verification_key_data = self.serialize_verifying_key(&vk)?;

        let timestamp = Self::current_timestamp();

        let proving_key = ZkProvingKey {
            id: ZkProvingKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Bellman,
            key_data: proving_key_data,
            created_at: timestamp,
        };

        let verification_key = ZkVerificationKey {
            id: ZkVerificationKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Bellman,
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
        info!("Generating proof with Bellman provider");
        debug!("Circuit ID: {}, Inputs: {}", circuit.id, inputs);

        // Create a circuit instance from the compiled data and inputs
        let example_circuit = self.create_circuit(&circuit.compiled_data, inputs)?;

        // Deserialize the proving key
        let params = self.deserialize_parameters(&proving_key.key_data)?;

        // Generate the proof
        let proof = create_random_proof(example_circuit, &params, &mut OsRng)
            .map_err(|e| ZkError::Provider(format!("Failed to generate proof: {}", e)))?;

        // Serialize the proof
        let proof_data = self.serialize_proof(&proof)?;

        let timestamp = Self::current_timestamp();

        let proof = ZkProof {
            id: ZkProofId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Bellman,
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
        info!("Verifying proof with Bellman provider");
        debug!("Proof ID: {}, Public inputs: {}", proof.id, public_inputs);

        // Deserialize the verification key
        let vk = self.deserialize_verifying_key(&verification_key.key_data)?;

        // Deserialize the proof
        let bellman_proof = self.deserialize_proof(&proof.proof_data)?;

        // Prepare the verification key
        let pvk = prepare_verifying_key(&vk);

        // Parse the public inputs
        let public_inputs = self.parse_inputs(public_inputs)?;

        // Convert public inputs to the format expected by Bellman
        let public_inputs: Vec<bls12_381::Scalar> = public_inputs
            .into_iter()
            .filter_map(|input| input)
            .collect();

        // Verify the proof
        let result = verify_proof(&pvk, &bellman_proof, &public_inputs)
            .map_err(|e| ZkError::Provider(format!("Failed to verify proof: {}", e)))?;

        Ok(result)
    }
}
