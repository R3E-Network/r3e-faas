// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Storage interface for the Fully Homomorphic Encryption service.

use crate::{
    FheCiphertext, FheCiphertextId, FheError, FheKeyPair, FheKeyPairId, FhePrivateKey,
    FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult,
};
use async_trait::async_trait;
use std::fmt::Debug;

mod memory;
mod rocksdb;

pub use memory::MemoryFheStorage;
pub use rocksdb::RocksDbFheStorage;

/// Storage interface for Fully Homomorphic Encryption data.
#[async_trait]
pub trait FheStorage: Send + Sync + Debug {
    /// Store a key pair.
    async fn store_key_pair(&self, key_pair: &FheKeyPair) -> FheResult<()>;

    /// Retrieve a key pair by ID.
    async fn get_key_pair(&self, id: &FheKeyPairId) -> FheResult<FheKeyPair>;

    /// List all key pairs.
    async fn list_key_pairs(&self) -> FheResult<Vec<FheKeyPair>>;

    /// Delete a key pair by ID.
    async fn delete_key_pair(&self, id: &FheKeyPairId) -> FheResult<()>;

    /// Store a public key.
    async fn store_public_key(&self, key: &FhePublicKey) -> FheResult<()>;

    /// Retrieve a public key by ID.
    async fn get_public_key(&self, id: &FhePublicKeyId) -> FheResult<FhePublicKey>;

    /// List all public keys.
    async fn list_public_keys(&self) -> FheResult<Vec<FhePublicKey>>;

    /// Delete a public key by ID.
    async fn delete_public_key(&self, id: &FhePublicKeyId) -> FheResult<()>;

    /// Store a private key.
    async fn store_private_key(&self, key: &FhePrivateKey) -> FheResult<()>;

    /// Retrieve a private key by ID.
    async fn get_private_key(&self, id: &FhePrivateKeyId) -> FheResult<FhePrivateKey>;

    /// List all private keys.
    async fn list_private_keys(&self) -> FheResult<Vec<FhePrivateKey>>;

    /// Delete a private key by ID.
    async fn delete_private_key(&self, id: &FhePrivateKeyId) -> FheResult<()>;

    /// Store a ciphertext.
    async fn store_ciphertext(&self, ciphertext: &FheCiphertext) -> FheResult<()>;

    /// Retrieve a ciphertext by ID.
    async fn get_ciphertext(&self, id: &FheCiphertextId) -> FheResult<FheCiphertext>;

    /// List all ciphertexts.
    async fn list_ciphertexts(&self) -> FheResult<Vec<FheCiphertext>>;

    /// List ciphertexts by public key ID.
    async fn list_ciphertexts_by_public_key(
        &self,
        public_key_id: &FhePublicKeyId,
    ) -> FheResult<Vec<FheCiphertext>>;

    /// Delete a ciphertext by ID.
    async fn delete_ciphertext(&self, id: &FheCiphertextId) -> FheResult<()>;
}
