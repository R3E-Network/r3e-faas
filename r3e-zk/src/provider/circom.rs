// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Circom provider for the Zero-Knowledge computing service.

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
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
use tokio::fs;

// Circom-specific imports
use circom_snark_verifier::{
    CircomCircuit, CircomConfig, CircomProof, CircomVerifier, CircomWitness, ProvingSystem,
};

use super::ZkProvider;

/// Circom provider for Zero-Knowledge operations.
#[derive(Debug)]
pub struct CircomProvider {
    /// Default witness generation strategy.
    pub default_witness_strategy: String,
    /// Working directory for temporary files.
    temp_dir: TempDir,
    /// Proving system to use.
    proving_system: ProvingSystem,
}

impl CircomProvider {
    /// Create a new Circom provider.
    pub fn new(default_witness_strategy: String) -> ZkResult<Self> {
        let temp_dir = TempDir::new().map_err(|e| {
            ZkError::Provider(format!("Failed to create temporary directory: {}", e))
        })?;
        
        Ok(Self {
            default_witness_strategy,
            temp_dir,
            proving_system: ProvingSystem::Groth16,
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
        std::fs::write(&path, data).map_err(|e| {
            ZkError::Provider(format!("Failed to write temporary file: {}", e))
        })?;
        Ok(path)
    }
    
    /// Run the Circom compiler.
    async fn run_circom_compiler(&self, source_path: &Path) -> ZkResult<PathBuf> {
        let output_dir = self.get_temp_file_path("output");
        fs::create_dir_all(&output_dir).await.map_err(|e| {
            ZkError::Provider(format!("Failed to create output directory: {}", e))
        })?;
        
        let output_path = output_dir.join("circuit.r1cs");
        
        // Run the Circom compiler
        let status = Command::new("circom")
            .arg(source_path)
            .arg("--r1cs")
            .arg("--output")
            .arg(&output_dir)
            .status()
            .map_err(|e| {
                ZkError::Compilation(format!("Failed to run Circom compiler: {}", e))
            })?;
        
        if !status.success() {
            return Err(ZkError::Compilation(format!(
                "Circom compiler exited with status: {}",
                status
            )));
        }
        
        Ok(output_path)
    }
    
    /// Generate a witness for a circuit.
    async fn generate_witness(&self, r1cs_path: &Path, inputs: &Value) -> ZkResult<CircomWitness> {
        // Write inputs to a JSON file
        let inputs_path = self.write_temp_file("inputs.json", inputs.to_string().as_bytes())?;
        
        let witness_path = self.get_temp_file_path("witness.wtns");
        
        // Run the witness generator
        let status = Command::new("snarkjs")
            .arg("wtns")
            .arg("calculate")
            .arg(r1cs_path)
            .arg(&inputs_path)
            .arg(&witness_path)
            .status()
            .map_err(|e| {
                ZkError::Provider(format!("Failed to run witness generator: {}", e))
            })?;
        
        if !status.success() {
            return Err(ZkError::Provider(format!(
                "Witness generator exited with status: {}",
                status
            )));
        }
        
        // Read the witness file
        let witness_data = std::fs::read(&witness_path).map_err(|e| {
            ZkError::Provider(format!("Failed to read witness file: {}", e))
        })?;
        
        // Parse the witness
        let witness = CircomWitness::from_binary(&witness_data).map_err(|e| {
            ZkError::Provider(format!("Failed to parse witness: {}", e))
        })?;
        
        Ok(witness)
    }
    
    /// Parse a circuit from R1CS file.
    fn parse_circuit(&self, r1cs_path: &Path) -> ZkResult<CircomCircuit> {
        // Read the R1CS file
        let r1cs_data = std::fs::read(r1cs_path).map_err(|e| {
            ZkError::Provider(format!("Failed to read R1CS file: {}", e))
        })?;
        
        // Parse the circuit
        let circuit = CircomCircuit::from_r1cs(&r1cs_data).map_err(|e| {
            ZkError::Provider(format!("Failed to parse R1CS: {}", e))
        })?;
        
        Ok(circuit)
    }
    
    /// Serialize a proof to bytes.
    fn serialize_proof(&self, proof: &CircomProof) -> ZkResult<Vec<u8>> {
        let serialized = serde_json::to_vec(proof).map_err(|e| {
            ZkError::Provider(format!("Failed to serialize proof: {}", e))
        })?;
        
        Ok(serialized)
    }
    
    /// Deserialize a proof from bytes.
    fn deserialize_proof(&self, data: &[u8]) -> ZkResult<CircomProof> {
        let proof: CircomProof = serde_json::from_slice(data).map_err(|e| {
            ZkError::Provider(format!("Failed to deserialize proof: {}", e))
        })?;
        
        Ok(proof)
    }
}

#[async_trait]
impl ZkProvider for CircomProvider {
    fn name(&self) -> &str {
        "Circom"
    }

    fn platform(&self) -> ZkPlatform {
        ZkPlatform::Circom
    }

    async fn compile_circuit(&self, code: &str) -> ZkResult<ZkCircuit> {
        info!("Compiling circuit with Circom provider");
        debug!("Circuit code length: {}", code.len());

        // Write the code to a temporary file
        let source_path = self.write_temp_file("circuit.circom", code.as_bytes())?;
        
        // Run the Circom compiler
        let r1cs_path = self.run_circom_compiler(&source_path).await?;
        
        // Read the compiled circuit
        let r1cs_data = std::fs::read(&r1cs_path).map_err(|e| {
            ZkError::Provider(format!("Failed to read R1CS file: {}", e))
        })?;
        
        // Parse the circuit to get metadata
        let circuit = self.parse_circuit(&r1cs_path)?;
        
        let circuit_id = ZkCircuitId::new();
        let timestamp = Self::current_timestamp();
        
        // Create metadata
        let metadata = ZkCircuitMetadata {
            name: Some("Circom Circuit".to_string()),
            description: Some("Compiled with Circom provider".to_string()),
            input_count: circuit.num_inputs(),
            output_count: circuit.num_outputs(),
            constraint_count: circuit.num_constraints(),
            created_at: timestamp,
            properties: serde_json::json!({
                "witness_strategy": self.default_witness_strategy,
                "version": env!("CARGO_PKG_VERSION"),
                "proving_system": format!("{:?}", self.proving_system),
            }),
        };

        Ok(ZkCircuit {
            id: circuit_id,
            platform: ZkPlatform::Circom,
            source_code: code.to_string(),
            compiled_data: r1cs_data,
            metadata,
        })
    }

    async fn generate_keys(
        &self,
        circuit: &ZkCircuit,
    ) -> ZkResult<(ZkProvingKey, ZkVerificationKey)> {
        info!("Generating keys with Circom provider");
        debug!("Circuit ID: {}", circuit.id);

        // Write the R1CS data to a temporary file
        let r1cs_path = self.write_temp_file("circuit.r1cs", &circuit.compiled_data)?;
        
        // Parse the circuit
        let circom_circuit = self.parse_circuit(&r1cs_path)?;
        
        // Create a config for the setup
        let config = CircomConfig {
            proving_system: self.proving_system,
            curve: "bn128".to_string(),
        };
        
        // Generate the keys
        let (pk, vk) = circom_circuit.setup(&config).map_err(|e| {
            ZkError::Provider(format!("Failed to generate keys: {}", e))
        })?;
        
        // Serialize the keys
        let proving_key_data = serde_json::to_vec(&pk).map_err(|e| {
            ZkError::Provider(format!("Failed to serialize proving key: {}", e))
        })?;
        
        let verification_key_data = serde_json::to_vec(&vk).map_err(|e| {
            ZkError::Provider(format!("Failed to serialize verification key: {}", e))
        })?;
        
        let timestamp = Self::current_timestamp();
        
        let proving_key = ZkProvingKey {
            id: ZkProvingKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Circom,
            key_data: proving_key_data,
            created_at: timestamp,
        };

        let verification_key = ZkVerificationKey {
            id: ZkVerificationKeyId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Circom,
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
        info!("Generating proof with Circom provider");
        debug!("Circuit ID: {}, Inputs: {}", circuit.id, inputs);

        // Write the R1CS data to a temporary file
        let r1cs_path = self.write_temp_file("circuit.r1cs", &circuit.compiled_data)?;
        
        // Parse the circuit
        let circom_circuit = self.parse_circuit(&r1cs_path)?;
        
        // Generate a witness
        let witness = self.generate_witness(&r1cs_path, inputs).await?;
        
        // Deserialize the proving key
        let pk = serde_json::from_slice(&proving_key.key_data).map_err(|e| {
            ZkError::Provider(format!("Failed to deserialize proving key: {}", e))
        })?;
        
        // Create a config for the proof generation
        let config = CircomConfig {
            proving_system: self.proving_system,
            curve: "bn128".to_string(),
        };
        
        // Generate the proof
        let circom_proof = circom_circuit.prove(&witness, &pk, &config).map_err(|e| {
            ZkError::Provider(format!("Failed to generate proof: {}", e))
        })?;
        
        // Serialize the proof
        let proof_data = self.serialize_proof(&circom_proof)?;
        
        let timestamp = Self::current_timestamp();
        
        let proof = ZkProof {
            id: ZkProofId::new(),
            circuit_id: circuit.id.clone(),
            platform: ZkPlatform::Circom,
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
        info!("Verifying proof with Circom provider");
        debug!(
            "Proof ID: {}, Public inputs: {}",
            proof.id, public_inputs
        );

        // Deserialize the verification key
        let vk = serde_json::from_slice(&verification_key.key_data).map_err(|e| {
            ZkError::Provider(format!("Failed to deserialize verification key: {}", e))
        })?;
        
        // Deserialize the proof
        let circom_proof = self.deserialize_proof(&proof.proof_data)?;
        
        // Create a verifier
        let verifier = CircomVerifier::new();
        
        // Verify the proof
        let result = verifier.verify(&circom_proof, &vk, public_inputs).map_err(|e| {
            ZkError::Provider(format!("Failed to verify proof: {}", e))
        })?;
        
        Ok(result)
    }
}
