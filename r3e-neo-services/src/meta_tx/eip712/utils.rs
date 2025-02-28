// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use super::types::{EIP712Domain, EIP712Type, EIP712TypedData, MetaTxMessage};
use crate::Error;
use ethers_core::types::{Address, Signature, H256};
use ethers_core::utils::keccak256;
use std::collections::HashMap;
use std::str::FromStr;

const EIP712_DOMAIN_TYPEHASH: &str = "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract,bytes32 salt)";
const META_TX_TYPEHASH: &str = "MetaTransaction(address from,address to,bytes data,uint256 nonce,uint256 deadline,string fee_model,uint256 fee_amount)";

/// Encode and hash the domain separator
pub fn hash_domain(domain: &EIP712Domain) -> Result<[u8; 32], Error> {
    // Create a buffer to hold the encoded domain data
    let mut buffer = Vec::new();
    
    // Add the domain type hash
    buffer.extend_from_slice(&keccak256(EIP712_DOMAIN_TYPEHASH.as_bytes()));
    
    // Add the name hash
    buffer.extend_from_slice(&keccak256(domain.name.as_bytes()));
    
    // Add the version hash
    buffer.extend_from_slice(&keccak256(domain.version.as_bytes()));
    
    // Add the chain ID
    let chain_id_bytes = domain.chain_id.to_be_bytes();
    let mut padded_chain_id = [0u8; 32];
    padded_chain_id[32 - chain_id_bytes.len()..].copy_from_slice(&chain_id_bytes);
    buffer.extend_from_slice(&padded_chain_id);
    
    // Add the verifying contract address
    let contract_address = match Address::from_str(&domain.verifying_contract) {
        Ok(addr) => addr,
        Err(_) => return Err(Error::InvalidParameter(format!("Invalid contract address: {}", domain.verifying_contract))),
    };
    let mut padded_address = [0u8; 32];
    padded_address[12..].copy_from_slice(contract_address.as_bytes());
    buffer.extend_from_slice(&padded_address);
    
    // Add the salt if present
    if let Some(salt) = &domain.salt {
        buffer.extend_from_slice(&keccak256(salt.as_bytes()));
    } else {
        buffer.extend_from_slice(&[0u8; 32]);
    }
    
    // Hash the encoded domain data
    Ok(keccak256(&buffer))
}

/// Encode and hash a meta transaction message
pub fn hash_meta_tx_message(message: &MetaTxMessage) -> Result<[u8; 32], Error> {
    // Create a buffer to hold the encoded message data
    let mut buffer = Vec::new();
    
    // Add the message type hash
    buffer.extend_from_slice(&keccak256(META_TX_TYPEHASH.as_bytes()));
    
    // Add the from address
    let from_address = match Address::from_str(&message.from) {
        Ok(addr) => addr,
        Err(_) => return Err(Error::InvalidParameter(format!("Invalid from address: {}", message.from))),
    };
    let mut padded_from = [0u8; 32];
    padded_from[12..].copy_from_slice(from_address.as_bytes());
    buffer.extend_from_slice(&padded_from);
    
    // Add the to address
    let to_address = match Address::from_str(&message.to) {
        Ok(addr) => addr,
        Err(_) => return Err(Error::InvalidParameter(format!("Invalid to address: {}", message.to))),
    };
    let mut padded_to = [0u8; 32];
    padded_to[12..].copy_from_slice(to_address.as_bytes());
    buffer.extend_from_slice(&padded_to);
    
    // Add the data hash
    buffer.extend_from_slice(&keccak256(message.data.as_bytes()));
    
    // Add the nonce
    let nonce_bytes = message.nonce.to_be_bytes();
    let mut padded_nonce = [0u8; 32];
    padded_nonce[32 - nonce_bytes.len()..].copy_from_slice(&nonce_bytes);
    buffer.extend_from_slice(&padded_nonce);
    
    // Add the deadline
    let deadline_bytes = message.deadline.to_be_bytes();
    let mut padded_deadline = [0u8; 32];
    padded_deadline[32 - deadline_bytes.len()..].copy_from_slice(&deadline_bytes);
    buffer.extend_from_slice(&padded_deadline);
    
    // Add the fee model hash
    buffer.extend_from_slice(&keccak256(message.fee_model.as_bytes()));
    
    // Add the fee amount
    let fee_amount_bytes = message.fee_amount.to_be_bytes();
    let mut padded_fee_amount = [0u8; 32];
    padded_fee_amount[32 - fee_amount_bytes.len()..].copy_from_slice(&fee_amount_bytes);
    buffer.extend_from_slice(&padded_fee_amount);
    
    // Hash the encoded message data
    Ok(keccak256(&buffer))
}

/// Encode and hash the structured data
pub fn hash_structured_data(typed_data: &EIP712TypedData) -> Result<[u8; 32], Error> {
    // Hash the domain separator
    let domain_hash = hash_domain(&typed_data.domain)?;
    
    // Extract the message as a MetaTxMessage
    let message = extract_meta_tx_message(typed_data)?;
    
    // Hash the message
    let message_hash = hash_meta_tx_message(&message)?;
    
    // Create the final hash
    let mut buffer = Vec::with_capacity(66);
    buffer.push(0x19); // EIP-191 version byte
    buffer.push(0x01); // EIP-712 version byte
    buffer.extend_from_slice(&domain_hash);
    buffer.extend_from_slice(&message_hash);
    
    Ok(keccak256(&buffer))
}

/// Extract a MetaTxMessage from EIP712TypedData
fn extract_meta_tx_message(typed_data: &EIP712TypedData) -> Result<MetaTxMessage, Error> {
    // Extract the message fields
    let from = typed_data.message.get("from")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::InvalidParameter("Missing 'from' field in message".to_string()))?
        .to_string();
    
    let to = typed_data.message.get("to")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::InvalidParameter("Missing 'to' field in message".to_string()))?
        .to_string();
    
    let data = typed_data.message.get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::InvalidParameter("Missing 'data' field in message".to_string()))?
        .to_string();
    
    let nonce = typed_data.message.get("nonce")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| Error::InvalidParameter("Missing 'nonce' field in message".to_string()))?;
    
    let deadline = typed_data.message.get("deadline")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| Error::InvalidParameter("Missing 'deadline' field in message".to_string()))?;
    
    let fee_model = typed_data.message.get("fee_model")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::InvalidParameter("Missing 'fee_model' field in message".to_string()))?
        .to_string();
    
    let fee_amount = typed_data.message.get("fee_amount")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| Error::InvalidParameter("Missing 'fee_amount' field in message".to_string()))?;
    
    Ok(MetaTxMessage {
        from,
        to,
        data,
        nonce,
        deadline,
        fee_model,
        fee_amount,
    })
}

/// Verify an EIP-712 signature
pub fn verify_eip712_signature(
    typed_data: &EIP712TypedData,
    signature: &str,
    expected_signer: &str,
) -> Result<bool, Error> {
    // Remove '0x' prefix if present
    let signature = signature.trim_start_matches("0x");
    
    // Parse the signature
    let signature_bytes = hex::decode(signature)
        .map_err(|e| Error::InvalidParameter(format!("Invalid signature format: {}", e)))?;
    
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| Error::InvalidParameter(format!("Invalid signature: {}", e)))?;
    
    // Hash the typed data
    let message_hash = hash_structured_data(typed_data)?;
    
    // Recover the signer's address
    let recovered_address = signature.recover(H256::from(message_hash))
        .map_err(|e| Error::InvalidParameter(format!("Failed to recover address: {}", e)))?;
    
    // Convert the expected signer to an address
    let expected_address = Address::from_str(expected_signer.trim_start_matches("0x"))
        .map_err(|e| Error::InvalidParameter(format!("Invalid expected signer address: {}", e)))?;
    
    // Compare the recovered address with the expected signer
    Ok(recovered_address == expected_address)
}

/// Create EIP-712 typed data for a meta transaction
pub fn create_meta_tx_typed_data(
    domain: EIP712Domain,
    message: MetaTxMessage,
) -> Result<EIP712TypedData, Error> {
    // Create the types map
    let mut types = HashMap::new();
    
    // Add the EIP712Domain type
    let domain_type = vec![
        EIP712Type { name: "name".to_string(), r#type: "string".to_string() },
        EIP712Type { name: "version".to_string(), r#type: "string".to_string() },
        EIP712Type { name: "chainId".to_string(), r#type: "uint256".to_string() },
        EIP712Type { name: "verifyingContract".to_string(), r#type: "address".to_string() },
        EIP712Type { name: "salt".to_string(), r#type: "bytes32".to_string() },
    ];
    types.insert("EIP712Domain".to_string(), domain_type);
    
    // Add the MetaTransaction type
    let meta_tx_type = vec![
        EIP712Type { name: "from".to_string(), r#type: "address".to_string() },
        EIP712Type { name: "to".to_string(), r#type: "address".to_string() },
        EIP712Type { name: "data".to_string(), r#type: "bytes".to_string() },
        EIP712Type { name: "nonce".to_string(), r#type: "uint256".to_string() },
        EIP712Type { name: "deadline".to_string(), r#type: "uint256".to_string() },
        EIP712Type { name: "fee_model".to_string(), r#type: "string".to_string() },
        EIP712Type { name: "fee_amount".to_string(), r#type: "uint256".to_string() },
    ];
    types.insert("MetaTransaction".to_string(), meta_tx_type);
    
    // Create the message map
    let mut message_map = HashMap::new();
    message_map.insert("from".to_string(), serde_json::Value::String(message.from));
    message_map.insert("to".to_string(), serde_json::Value::String(message.to));
    message_map.insert("data".to_string(), serde_json::Value::String(message.data));
    message_map.insert("nonce".to_string(), serde_json::Value::Number(serde_json::Number::from(message.nonce)));
    message_map.insert("deadline".to_string(), serde_json::Value::Number(serde_json::Number::from(message.deadline)));
    message_map.insert("fee_model".to_string(), serde_json::Value::String(message.fee_model));
    message_map.insert("fee_amount".to_string(), serde_json::Value::Number(serde_json::Number::from(message.fee_amount)));
    
    // Create the typed data
    Ok(EIP712TypedData {
        domain,
        primary_type: "MetaTransaction".to_string(),
        types,
        message: message_map,
    })
}
