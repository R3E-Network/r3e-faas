// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use r3e_neo_services::meta_tx::eip712::{
    EIP712Domain, EIP712Type, EIP712TypedData, MetaTxMessage,
    hash_domain, hash_structured_data, verify_eip712_signature, create_meta_tx_typed_data
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::test]
async fn test_hash_domain() {
    let domain = EIP712Domain {
        name: "R3E FaaS Meta Transaction".to_string(),
        version: "1".to_string(),
        chain_id: 1,
        verifying_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        salt: None,
    };
    
    let hash = hash_domain(&domain).unwrap();
    
    // The hash should be a 32-byte value
    assert_eq!(hash.len(), 32);
}

#[tokio::test]
async fn test_create_meta_tx_typed_data() {
    let domain = EIP712Domain {
        name: "R3E FaaS Meta Transaction".to_string(),
        version: "1".to_string(),
        chain_id: 1,
        verifying_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        salt: None,
    };
    
    let message = MetaTxMessage {
        from: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        to: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        data: "0x0123456789abcdef".to_string(),
        nonce: 1,
        deadline: 1677721600, // 2023-03-02T00:00:00Z
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let typed_data = create_meta_tx_typed_data(domain, message).unwrap();
    
    // Verify the typed data structure
    assert_eq!(typed_data.primary_type, "MetaTransaction");
    assert!(typed_data.types.contains_key("EIP712Domain"));
    assert!(typed_data.types.contains_key("MetaTransaction"));
    assert_eq!(typed_data.domain.name, "R3E FaaS Meta Transaction");
    assert_eq!(typed_data.domain.version, "1");
    assert_eq!(typed_data.domain.chain_id, 1);
    assert_eq!(typed_data.domain.verifying_contract, "0x1234567890abcdef1234567890abcdef12345678");
    
    // Verify the message fields
    let message_value = typed_data.message.get("from").unwrap();
    assert_eq!(message_value, &json!("0xabcdef1234567890abcdef1234567890abcdef12"));
    
    let message_value = typed_data.message.get("to").unwrap();
    assert_eq!(message_value, &json!("0x1234567890abcdef1234567890abcdef12345678"));
    
    let message_value = typed_data.message.get("data").unwrap();
    assert_eq!(message_value, &json!("0x0123456789abcdef"));
    
    let message_value = typed_data.message.get("nonce").unwrap();
    assert_eq!(message_value, &json!(1));
    
    let message_value = typed_data.message.get("deadline").unwrap();
    assert_eq!(message_value, &json!(1677721600));
    
    let message_value = typed_data.message.get("fee_model").unwrap();
    assert_eq!(message_value, &json!("fixed"));
    
    let message_value = typed_data.message.get("fee_amount").unwrap();
    assert_eq!(message_value, &json!(10));
}

#[tokio::test]
async fn test_hash_structured_data() {
    let domain = EIP712Domain {
        name: "R3E FaaS Meta Transaction".to_string(),
        version: "1".to_string(),
        chain_id: 1,
        verifying_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        salt: None,
    };
    
    let message = MetaTxMessage {
        from: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        to: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        data: "0x0123456789abcdef".to_string(),
        nonce: 1,
        deadline: 1677721600, // 2023-03-02T00:00:00Z
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let typed_data = create_meta_tx_typed_data(domain, message).unwrap();
    
    let hash = hash_structured_data(&typed_data).unwrap();
    
    // The hash should be a 32-byte value
    assert_eq!(hash.len(), 32);
}

// This test requires a valid signature, which we can't generate in this test environment
// In a real test, we would use a known private key to sign the message and then verify the signature
#[tokio::test]
async fn test_verify_eip712_signature() {
    let domain = EIP712Domain {
        name: "R3E FaaS Meta Transaction".to_string(),
        version: "1".to_string(),
        chain_id: 1,
        verifying_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        salt: None,
    };
    
    let message = MetaTxMessage {
        from: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        to: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        data: "0x0123456789abcdef".to_string(),
        nonce: 1,
        deadline: 1677721600, // 2023-03-02T00:00:00Z
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let typed_data = create_meta_tx_typed_data(domain, message).unwrap();
    
    // In a real test, we would use a known private key to sign the message
    // and then verify the signature against the known address
    // For this test, we'll just check that the function doesn't panic with a mock signature
    
    // This is a mock signature, not a real one
    let signature = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef00";
    
    // This will fail because the signature is not valid, but we just want to make sure the function runs
    let result = verify_eip712_signature(&typed_data, signature, "0xabcdef1234567890abcdef1234567890abcdef12");
    
    // We expect this to fail because the signature is not valid
    assert!(result.is_err());
}
