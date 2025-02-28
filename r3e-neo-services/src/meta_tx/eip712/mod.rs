// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod types;
pub mod utils;

pub use types::{EIP712Domain, EIP712Type, EIP712TypedData, MetaTxMessage};
pub use utils::{hash_domain, hash_structured_data, verify_eip712_signature, create_meta_tx_typed_data};
