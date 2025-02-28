// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;
use uuid::Uuid;

use crate::source::{event, Func, FuncError, Task, TaskError, TaskSource};

/// Mock task source for testing
pub struct MockTaskSource {
    /// Sleep duration between tasks
    sleep: Duration,
    /// User ID
    uid: u64,
    /// Current task index
    task_index: usize,
    /// Mock tasks
    tasks: Vec<Task>,
    /// Mock functions
    functions: Vec<(u64, u64, Func)>,
    /// Current event type index
    event_type_index: usize,
}

impl MockTaskSource {
    /// Create a new mock task source
    pub fn new() -> Self {
        Self {
            sleep: Duration::from_millis(100),
            uid: 1,
            task_index: 0,
            tasks: Vec::new(),
            functions: Vec::new(),
            event_type_index: 0,
        }
    }
    
    /// Set sleep duration
    pub fn with_sleep(mut self, sleep: Duration) -> Self {
        self.sleep = sleep;
        self
    }
    
    /// Set user ID
    pub fn with_uid(mut self, uid: u64) -> Self {
        self.uid = uid;
        self
    }
    
    /// Add a mock task
    pub fn add_task(mut self, uid: u64, fid: u64, event: event::Event) -> Self {
        self.tasks.push(Task::new(uid, fid, event));
        self
    }
    
    /// Add a mock function
    pub fn add_function(mut self, uid: u64, fid: u64, code: &str) -> Self {
        self.functions.push((uid, fid, Func {
            version: 1,
            code: code.to_string(),
        }));
        self
    }
    
    /// Create a mock task source with default tasks and functions
    pub fn with_defaults() -> Self {
        let neo_block = json!({
            "index": 12345,
            "hash": "0x1234567890abcdef",
            "size": 1024,
            "version": 0,
            "previousblockhash": "0x0987654321fedcba",
            "merkleroot": "0xabcdef1234567890",
            "time": 1612345678,
            "nonce": "0x1234567890abcdef",
            "nextconsensus": "0x1234567890abcdef",
            "witnesses": [],
            "tx": [],
            "confirmations": 100,
            "nextblockhash": "0xfedcba0987654321"
        });
        
        let ethereum_block = json!({
            "number": "0x1b4",
            "hash": "0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
            "nonce": "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "logsBloom": "0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
            "miner": "0x4e65fda2159562a496f9f3522f89122a3088497a",
            "difficulty": "0x027f07",
            "totalDifficulty": "0x027f07",
            "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "size": "0x027f07",
            "gasLimit": "0x9f759",
            "gasUsed": "0x9f759",
            "timestamp": "0x54e34e8e",
            "transactions": [],
            "uncles": []
        });
        
        let neo_function_code = r#"
        // Neo event handler
        export default function(event) {
            console.log("Neo event handler called");
            
            return {
                status: "success",
                message: "Neo event processed successfully",
                timestamp: new Date().toISOString(),
                event_type: "neo"
            };
        }
        "#;
        
        let ethereum_function_code = r#"
        // Ethereum event handler
        export default function(event) {
            console.log("Ethereum event handler called");
            
            return {
                status: "success",
                message: "Ethereum event processed successfully",
                timestamp: new Date().toISOString(),
                event_type: "ethereum"
            };
        }
        "#;
        
        Self::new()
            .add_task(1, 1, event::Event::NeoBlock(neo_block))
            .add_task(1, 2, event::Event::EthereumBlock(ethereum_block))
            .add_function(1, 1, neo_function_code)
            .add_function(1, 2, ethereum_function_code)
    }
    
    /// Generate a random event
    fn generate_random_event(&mut self) -> event::Event {
        // Rotate through different event types
        self.event_type_index = (self.event_type_index + 1) % 6;
        
        match self.event_type_index {
            0 => {
                // Neo block
                let block = json!({
                    "index": 12345 + self.task_index as u64,
                    "hash": format!("0x1234567890abcdef{:04x}", self.task_index),
                    "size": 1024,
                    "version": 0,
                    "previousblockhash": "0x0987654321fedcba",
                    "merkleroot": "0xabcdef1234567890",
                    "time": 1612345678 + self.task_index as u64,
                    "nonce": "0x1234567890abcdef",
                    "nextconsensus": "0x1234567890abcdef",
                    "witnesses": [],
                    "tx": [],
                    "confirmations": 100,
                    "nextblockhash": "0xfedcba0987654321"
                });
                
                event::Event::NeoBlock(block)
            },
            1 => {
                // Neo transaction
                let tx = json!({
                    "hash": format!("0xabcdef1234567890{:04x}", self.task_index),
                    "size": 256,
                    "version": 0,
                    "nonce": 12345,
                    "sender": "NZs2zXSPuuv9ZF6TDGSWT1RBmE8rfGj7UW",
                    "sysfee": "0.1",
                    "netfee": "0.05",
                    "validuntilblock": 12345,
                    "signers": [],
                    "attributes": [],
                    "script": "0x0123456789abcdef",
                    "witnesses": []
                });
                
                event::Event::NeoTransaction(tx)
            },
            2 => {
                // Neo contract event
                let events = json!([
                    {
                        "contract": "0x1234567890abcdef",
                        "eventname": "Transfer",
                        "state": {
                            "type": "Array",
                            "value": [
                                {
                                    "type": "ByteString",
                                    "value": "NZs2zXSPuuv9ZF6TDGSWT1RBmE8rfGj7UW"
                                },
                                {
                                    "type": "ByteString",
                                    "value": "NZnY1pL9XEXxTRGc9dh2S4JBzn9kRQeYdr"
                                },
                                {
                                    "type": "Integer",
                                    "value": "100000000"
                                }
                            ]
                        }
                    }
                ]);
                
                event::Event::NeoContractEvent {
                    contract_address: "0x1234567890abcdef".to_string(),
                    events,
                }
            },
            3 => {
                // Ethereum block
                let block = json!({
                    "number": format!("0x{:x}", 0x1b4 + self.task_index as u64),
                    "hash": format!("0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527{:03x}", self.task_index),
                    "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
                    "nonce": "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
                    "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                    "logsBloom": "0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
                    "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                    "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
                    "miner": "0x4e65fda2159562a496f9f3522f89122a3088497a",
                    "difficulty": "0x027f07",
                    "totalDifficulty": "0x027f07",
                    "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
                    "size": "0x027f07",
                    "gasLimit": "0x9f759",
                    "gasUsed": "0x9f759",
                    "timestamp": format!("0x{:x}", 0x54e34e8e + self.task_index as u64),
                    "transactions": [],
                    "uncles": []
                });
                
                event::Event::EthereumBlock(block)
            },
            4 => {
                // Ethereum transaction
                let tx = json!({
                    "hash": format!("0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d10{:03x}", self.task_index),
                    "nonce": "0x0",
                    "blockHash": "0xbeab0aa2411b7ab17f30a99d3cb9c6ef2fc5426d6ad6fd9e2a26a6aed1d1055b",
                    "blockNumber": format!("0x{:x}", 0x15df + self.task_index as u64),
                    "transactionIndex": "0x1",
                    "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
                    "to": "0x85h43d8a49eeb85d32cf465507dd71d507100c1",
                    "value": "0x7f110",
                    "gas": "0x7f110",
                    "gasPrice": "0x09184e72a000",
                    "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360"
                });
                
                event::Event::EthereumTransaction(tx)
            },
            5 => {
                // Ethereum contract event
                let events = json!([
                    {
                        "address": "0x4e65fda2159562a496f9f3522f89122a3088497a",
                        "topics": [
                            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                            "0x0000000000000000000000004e65fda2159562a496f9f3522f89122a3088497a",
                            "0x0000000000000000000000001234567890123456789012345678901234567890"
                        ],
                        "data": "0x0000000000000000000000000000000000000000000000000de0b6b3a7640000",
                        "blockNumber": format!("0x{:x}", 0x1b4 + self.task_index as u64),
                        "transactionHash": format!("0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d10{:03x}", self.task_index),
                        "transactionIndex": "0x1",
                        "blockHash": format!("0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527{:03x}", self.task_index),
                        "logIndex": "0x0",
                        "removed": false
                    }
                ]);
                
                event::Event::EthereumContractEvent {
                    contract_address: "0x4e65fda2159562a496f9f3522f89122a3088497a".to_string(),
                    events,
                }
            },
            _ => event::Event::None,
        }
    }
}

#[async_trait]
impl TaskSource for MockTaskSource {
    async fn acquire_task(&mut self, _uid: u64, _fid_hint: u64) -> Result<Task, TaskError> {
        // Sleep to avoid busy waiting
        tokio::time::sleep(self.sleep).await;
        
        // Check if there are any tasks
        if !self.tasks.is_empty() {
            // Get the next task
            let task_index = self.task_index;
            self.task_index = (self.task_index + 1) % self.tasks.len();
            
            // Return the task
            return Ok(self.tasks[task_index].clone());
        }
        
        // Generate a random event
        let event = self.generate_random_event();
        
        // Increment task index
        self.task_index += 1;
        
        // Create a task with the event
        Ok(Task::new(self.uid, 1 + (self.task_index as u64 % 2), event))
    }
    
    async fn acquire_fn(&mut self, uid: u64, fid: u64) -> Result<Func, FuncError> {
        // Find the function
        for (u, f, func) in &self.functions {
            if *u == uid && *f == fid {
                return Ok(func.clone());
            }
        }
        
        // If no function is found, return a default function
        let code = format!(
            r#"
            const delay = (n) => new Promise(r => setTimeout(r, n));

            export default async function(event) {{
                console.log("Processing event:", event);
                await delay(100);
                
                return {{
                    status: "success",
                    message: "Event processed successfully",
                    timestamp: new Date().toISOString(),
                    event_data: event
                }};
            }}"#
        );
        
        Ok(Func {
            version: 1,
            code: code,
        })
    }
}
