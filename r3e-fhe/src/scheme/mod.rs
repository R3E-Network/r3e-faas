// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Scheme interface for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheError, FheKeyPair, FheParameters, FhePrivateKey,
    FhePublicKey, FheResult, FheSchemeType,
};
use async_trait::async_trait;
use serde_json::Value;
use std::fmt::Debug;

mod openfhe;
mod tfhe;

pub use openfhe::OpenFheScheme;
pub use tfhe::TfheScheme;

/// Scheme interface for Fully Homomorphic Encryption operations.
#[async_trait]
pub trait FheScheme: Send + Sync + Debug {
    /// Get the name of the scheme.
    fn name(&self) -> &str;

    /// Get the scheme type.
    fn scheme_type(&self) -> FheSchemeType;

    /// Generate a key pair for FHE operations.
    async fn generate_key_pair(&self, params: &FheParameters) -> FheResult<FheKeyPair>;

    /// Encrypt data using a public key.
    async fn encrypt(
        &self,
        public_key: &FhePublicKey,
        plaintext: &[u8],
    ) -> FheResult<FheCiphertext>;

    /// Decrypt data using a private key.
    async fn decrypt(
        &self,
        private_key: &FhePrivateKey,
        ciphertext: &FheCiphertext,
    ) -> FheResult<Vec<u8>>;

    /// Add two ciphertexts homomorphically.
    async fn add(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext>;

    /// Subtract one ciphertext from another homomorphically.
    async fn subtract(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext>;

    /// Multiply two ciphertexts homomorphically.
    async fn multiply(
        &self,
        ciphertext1: &FheCiphertext,
        ciphertext2: &FheCiphertext,
    ) -> FheResult<FheCiphertext>;

    /// Negate a ciphertext homomorphically.
    async fn negate(&self, ciphertext: &FheCiphertext) -> FheResult<FheCiphertext>;

    /// Estimate the noise budget of a ciphertext.
    async fn estimate_noise_budget(&self, ciphertext: &FheCiphertext) -> FheResult<Option<u32>>;

    /// Get the supported operations for this scheme.
    fn supported_operations(&self) -> Vec<crate::HomomorphicOperation>;

    /// Get scheme-specific information.
    fn get_info(&self) -> Value;
}
