// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Fully Homomorphic Encryption service integration for R3E FaaS.

mod mock;

pub use mock::{
    FheCiphertext, FheCiphertextId, FheError, FheKeyPair, FheKeyPairId, FheParameters,
    FhePrivateKey, FhePrivateKeyId, FhePublicKey, FhePublicKeyId, FheResult, FheSchemeType,
    FheService, FheStorageType, HomomorphicOperation,
};

/// Get the Fully Homomorphic Encryption service instance.
pub fn get_fhe_service() -> FheResult<FheService> {
    // This would typically load configuration from a central source
    // and initialize the service with the appropriate parameters.
    // For now, we'll use default configuration.
    FheService::new_with_default_config()
}
