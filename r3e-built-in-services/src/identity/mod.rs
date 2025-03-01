// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod service;
pub mod storage;
pub mod types;

pub use service::{IdentityService, IdentityServiceTrait};
pub use storage::{IdentityStorage, MemoryIdentityStorage};
pub use types::{
    IdentityCredential, IdentityError, IdentityProfile, IdentityVerification, RecoveryMethod,
};
