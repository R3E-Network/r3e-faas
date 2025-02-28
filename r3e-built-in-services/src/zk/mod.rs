// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Zero-Knowledge computing service integration for R3E FaaS.

use r3e_zk::{ZkError, ZkResult, ZkService};

pub use r3e_zk::{
    ZkCircuit, ZkCircuitId, ZkParameters, ZkProof, ZkProofId, ZkProviderType, ZkProvingKey,
    ZkProvingKeyId, ZkStorageType, ZkVerificationKey, ZkVerificationKeyId,
};

// Re-export the error type
pub use r3e_zk::ZkError;

/// Get the Zero-Knowledge service instance.
pub fn get_zk_service() -> ZkResult<ZkService> {
    // This would typically load configuration from a central source
    // and initialize the service with the appropriate parameters.
    // For now, we'll use default configuration.
    ZkService::new_with_default_config()
}
