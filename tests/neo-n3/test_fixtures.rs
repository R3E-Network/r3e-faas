// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Test fixtures for Neo N3 FaaS platform tests

/// Neo block fixture
pub fn neo_block_fixture() -> serde_json::Value {
    serde_json::json!({
        "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "size": 1024,
        "version": 0,
        "previousblockhash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        "merkleroot": "0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba",
        "time": 1612345678,
        "index": 12345,
        "nonce": "0000000000000000",
        "nextconsensus": "NXV7ZhHiyM1aHXwvUNBJdAjvNkQTGW3V2Q",
        "script": {
            "invocation": "0123456789abcdef",
            "verification": "abcdef0123456789"
        },
        "tx": []
    })
}

/// Neo transaction fixture
pub fn neo_transaction_fixture() -> serde_json::Value {
    serde_json::json!({
        "hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        "size": 512,
        "version": 0,
        "nonce": 12345,
        "sender": "NXV7ZhHiyM1aHXwvUNBJdAjvNkQTGW3V2Q",
        "sysfee": "0.1",
        "netfee": "0.05",
        "validuntilblock": 12400,
        "signers": [],
        "attributes": [],
        "script": "0123456789abcdef",
        "witnesses": []
    })
}

/// Neo contract notification fixture
pub fn neo_contract_notification_fixture() -> serde_json::Value {
    serde_json::json!({
        "contract": "0x1234567890abcdef1234567890abcdef1234567890abcdef",
        "eventname": "Transfer",
        "state": {
            "type": "Array",
            "value": [
                {
                    "type": "ByteString",
                    "value": "NXV7ZhHiyM1aHXwvUNBJdAjvNkQTGW3V2Q"
                },
                {
                    "type": "ByteString",
                    "value": "NZHf1NJvz1YMJEEHtL9QT9xw9Kdmb3mfos"
                },
                {
                    "type": "Integer",
                    "value": "100000000"
                }
            ]
        }
    })
}

/// JavaScript function fixture
pub fn javascript_function_fixture() -> &'static str {
    r#"
    export default async function(event, context) {
        const blockHeight = await context.neo.getCurrentBlockHeight();
        return {
            message: "Hello, Neo N3 FaaS!",
            blockHeight: blockHeight,
            timestamp: new Date().toISOString()
        };
    }
    "#
}

/// Oracle request fixture
pub fn oracle_request_fixture() -> serde_json::Value {
    serde_json::json!({
        "asset": "NEO",
        "currency": "USD",
        "timestamp": 1612345678
    })
}

/// TEE request fixture
pub fn tee_request_fixture() -> serde_json::Value {
    serde_json::json!({
        "operation": "sign",
        "data": "Hello, Neo N3 FaaS!",
        "keyId": "key-12345"
    })
}

/// API request fixture
pub fn api_request_fixture() -> serde_json::Value {
    serde_json::json!({
        "name": "test-function",
        "description": "A test function",
        "runtime": "JAVASCRIPT",
        "trigger": {
            "type": "HTTP",
            "http": {
                "path": "/test-function",
                "method": "GET"
            }
        },
        "code": "export default async function(event, context) { return { message: \"Hello, World!\" }; }"
    })
}
