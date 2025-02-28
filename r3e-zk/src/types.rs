// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the Zero-Knowledge computing service.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Identifier for a ZK circuit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZkCircuitId(pub Uuid);

impl ZkCircuitId {
    /// Create a new random circuit ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ZkCircuitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for a ZK proving key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZkProvingKeyId(pub Uuid);

impl ZkProvingKeyId {
    /// Create a new random proving key ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ZkProvingKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for a ZK verification key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZkVerificationKeyId(pub Uuid);

impl ZkVerificationKeyId {
    /// Create a new random verification key ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ZkVerificationKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for a ZK proof.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZkProofId(pub Uuid);

impl ZkProofId {
    /// Create a new random proof ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ZkProofId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Supported ZK platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZkPlatform {
    /// Zokrates platform.
    Zokrates,
    /// Bulletproofs platform.
    Bulletproofs,
    /// Circom platform.
    Circom,
    /// libsnark platform.
    Libsnark,
    /// StarkWare platform.
    StarkWare,
    /// Bellman platform.
    Bellman,
    /// Arkworks platform.
    Arkworks,
}

impl fmt::Display for ZkPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkPlatform::Zokrates => write!(f, "Zokrates"),
            ZkPlatform::Bulletproofs => write!(f, "Bulletproofs"),
            ZkPlatform::Circom => write!(f, "Circom"),
            ZkPlatform::Libsnark => write!(f, "libsnark"),
            ZkPlatform::StarkWare => write!(f, "StarkWare"),
            ZkPlatform::Bellman => write!(f, "Bellman"),
            ZkPlatform::Arkworks => write!(f, "Arkworks"),
        }
    }
}

/// Representation of a ZK circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkCircuit {
    /// Unique identifier for the circuit.
    pub id: ZkCircuitId,
    /// Platform used for the circuit.
    pub platform: ZkPlatform,
    /// Source code of the circuit.
    pub source_code: String,
    /// Compiled representation of the circuit (platform-specific).
    pub compiled_data: Vec<u8>,
    /// Metadata about the circuit.
    pub metadata: ZkCircuitMetadata,
}

/// Metadata about a ZK circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkCircuitMetadata {
    /// Name of the circuit.
    pub name: Option<String>,
    /// Description of the circuit.
    pub description: Option<String>,
    /// Number of inputs to the circuit.
    pub input_count: usize,
    /// Number of outputs from the circuit.
    pub output_count: usize,
    /// Number of constraints in the circuit.
    pub constraint_count: usize,
    /// Creation timestamp.
    pub created_at: u64,
    /// Additional properties.
    pub properties: serde_json::Value,
}

/// A proving key for a ZK circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProvingKey {
    /// Unique identifier for the proving key.
    pub id: ZkProvingKeyId,
    /// ID of the circuit this key is for.
    pub circuit_id: ZkCircuitId,
    /// Platform used for the proving key.
    pub platform: ZkPlatform,
    /// Binary data of the proving key (platform-specific).
    pub key_data: Vec<u8>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// A verification key for a ZK circuit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVerificationKey {
    /// Unique identifier for the verification key.
    pub id: ZkVerificationKeyId,
    /// ID of the circuit this key is for.
    pub circuit_id: ZkCircuitId,
    /// Platform used for the verification key.
    pub platform: ZkPlatform,
    /// Binary data of the verification key (platform-specific).
    pub key_data: Vec<u8>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// A zero-knowledge proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Unique identifier for the proof.
    pub id: ZkProofId,
    /// ID of the circuit this proof is for.
    pub circuit_id: ZkCircuitId,
    /// Platform used for the proof.
    pub platform: ZkPlatform,
    /// Binary data of the proof (platform-specific).
    pub proof_data: Vec<u8>,
    /// Public inputs used for the proof.
    pub public_inputs: serde_json::Value,
    /// Creation timestamp.
    pub created_at: u64,
}

/// Configuration for ZK circuit compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkCompilationConfig {
    /// Platform to use for compilation.
    pub platform: ZkPlatform,
    /// Optimization level (0-3).
    pub optimization_level: u8,
    /// Whether to enable verbose output.
    pub verbose: bool,
    /// Additional platform-specific options.
    pub options: serde_json::Value,
}

/// Configuration for ZK proof generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofGenerationConfig {
    /// Platform to use for proof generation.
    pub platform: ZkPlatform,
    /// Whether to enable verbose output.
    pub verbose: bool,
    /// Additional platform-specific options.
    pub options: serde_json::Value,
}

/// Configuration for ZK proof verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVerificationConfig {
    /// Platform to use for verification.
    pub platform: ZkPlatform,
    /// Whether to enable verbose output.
    pub verbose: bool,
    /// Additional platform-specific options.
    pub options: serde_json::Value,
}

/// Result of a ZK proof verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVerificationResult {
    /// Whether the proof is valid.
    pub is_valid: bool,
    /// Time taken for verification (in milliseconds).
    pub verification_time_ms: u64,
    /// Additional platform-specific information.
    pub additional_info: serde_json::Value,
}
