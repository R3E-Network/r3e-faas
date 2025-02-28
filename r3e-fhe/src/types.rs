// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the Fully Homomorphic Encryption service.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Identifier for an FHE key pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FheKeyPairId(pub Uuid);

impl FheKeyPairId {
    /// Create a new random key pair ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for FheKeyPairId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for an FHE public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FhePublicKeyId(pub Uuid);

impl FhePublicKeyId {
    /// Create a new random public key ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for FhePublicKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for an FHE private key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FhePrivateKeyId(pub Uuid);

impl FhePrivateKeyId {
    /// Create a new random private key ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for FhePrivateKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for an FHE ciphertext.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FheCiphertextId(pub Uuid);

impl FheCiphertextId {
    /// Create a new random ciphertext ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for FheCiphertextId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Supported FHE schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FheSchemeType {
    /// TFHE scheme.
    Tfhe,
    /// OpenFHE scheme.
    OpenFhe,
    /// Microsoft SEAL scheme.
    Seal,
    /// IBM HElib scheme.
    Helib,
    /// Lattigo scheme.
    Lattigo,
}

impl fmt::Display for FheSchemeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FheSchemeType::Tfhe => write!(f, "TFHE"),
            FheSchemeType::OpenFhe => write!(f, "OpenFHE"),
            FheSchemeType::Seal => write!(f, "SEAL"),
            FheSchemeType::Helib => write!(f, "HElib"),
            FheSchemeType::Lattigo => write!(f, "Lattigo"),
        }
    }
}

/// Parameters for FHE schemes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheParameters {
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Security level in bits.
    pub security_level: u32,
    /// Polynomial modulus degree.
    pub polynomial_modulus_degree: u32,
    /// Plaintext modulus.
    pub plaintext_modulus: u32,
    /// Additional scheme-specific parameters.
    pub additional_params: serde_json::Value,
}

/// A key pair for FHE operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheKeyPair {
    /// Unique identifier for the key pair.
    pub id: FheKeyPairId,
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Public key.
    pub public_key: FhePublicKey,
    /// Private key.
    pub private_key: FhePrivateKey,
    /// Parameters used to generate the key pair.
    pub parameters: FheParameters,
    /// Creation timestamp.
    pub created_at: u64,
}

/// A public key for FHE operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhePublicKey {
    /// Unique identifier for the public key.
    pub id: FhePublicKeyId,
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Binary data of the public key (scheme-specific).
    pub key_data: Vec<u8>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// A private key for FHE operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhePrivateKey {
    /// Unique identifier for the private key.
    pub id: FhePrivateKeyId,
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Binary data of the private key (scheme-specific).
    pub key_data: Vec<u8>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// A ciphertext for FHE operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheCiphertext {
    /// Unique identifier for the ciphertext.
    pub id: FheCiphertextId,
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Public key ID used for encryption.
    pub public_key_id: FhePublicKeyId,
    /// Binary data of the ciphertext (scheme-specific).
    pub ciphertext_data: Vec<u8>,
    /// Creation timestamp.
    pub created_at: u64,
    /// Metadata about the ciphertext.
    pub metadata: FheCiphertextMetadata,
}

/// Metadata about a ciphertext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheCiphertextMetadata {
    /// Original plaintext size in bytes.
    pub plaintext_size: usize,
    /// Ciphertext size in bytes.
    pub ciphertext_size: usize,
    /// Number of homomorphic operations performed on this ciphertext.
    pub operation_count: usize,
    /// Estimated noise budget remaining.
    pub noise_budget: Option<u32>,
    /// Additional properties.
    pub properties: serde_json::Value,
}

/// Supported homomorphic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HomomorphicOperation {
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Negation.
    Negate,
    /// Rotation.
    Rotate,
}

impl fmt::Display for HomomorphicOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HomomorphicOperation::Add => write!(f, "Add"),
            HomomorphicOperation::Subtract => write!(f, "Subtract"),
            HomomorphicOperation::Multiply => write!(f, "Multiply"),
            HomomorphicOperation::Negate => write!(f, "Negate"),
            HomomorphicOperation::Rotate => write!(f, "Rotate"),
        }
    }
}

/// Configuration for FHE operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheOperationConfig {
    /// Scheme type.
    pub scheme_type: FheSchemeType,
    /// Whether to enable verbose output.
    pub verbose: bool,
    /// Additional scheme-specific options.
    pub options: serde_json::Value,
}
