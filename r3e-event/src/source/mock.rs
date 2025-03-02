// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use rand::Rng;
use tokio::sync::Mutex;
use async_trait::async_trait;

use super::event::Event as event;
use super::service;
use super::service::TaskSource;

use log::{debug, error, info};
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use crate::source::{Func, FuncError, Task, TaskError, Trigger};
use crate::source::events::{Event, MockEvent};

/// Mock task source for testing
pub struct MockTaskSource {
    /// Sleep interval between tasks
    sleep: Duration,
    /// User ID
    uid: u64,
    /// Current task index
    task_index: usize,
    /// Predefined list of tasks
    tasks: Vec<Task>,
    /// Predefined list of functions
    functions: Vec<(u64, u64, String)>,
    /// Current event type index for rotation
    event_type_index: usize,
    /// Count for generating events
    count: u64,
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
            count: 0,
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
    pub fn add_task(mut self, uid: u64, fid: u64, event: Event) -> Self {
        self.tasks.push(Task::new(uid, fid, event));
        self
    }

    /// Add a mock function
    pub fn add_function(mut self, uid: u64, fid: u64, code: &str) -> Self {
        self.functions.push((
            uid,
            fid,
            code.to_string(),
        ));
        self
    }

    /// Create a mock task source with default tasks and functions
    pub fn sample() -> Self {
        // Sample Neo block
        let neo_block = Event::NeoBlock(NeoBlock {
            header: Some(NeoBlockHeader {
                hash: "0x123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0".to_string(),
                version: 0,
                prev_block_hash: "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789".to_string(),
                merkle_root: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
                time: 1625097600,
                height: 12345,
                primary: 0,
                next_consensus: "NXV7ZhHiyM1aHXwpVsRJx8h1PwgXqveRj".to_string(),
                witnesses: vec![],
            }),
            txs: vec![],
        });

        // Sample Ethereum block 
        // For now, using a placeholder since we need to add EthereumBlock to the Event enum
        
        let neo_function_code = r#"
        export default async function(event) {
            console.log("Processing Neo block:", event);
            return {
                result: "success",
                block_height: event.header?.height || 0,
                timestamp: new Date().toISOString(),
                event_type: "neo"
            };
        }
        "#;

        let ethereum_function_code = r#"
        export default async function(event) {
            console.log("Processing Ethereum event:", event);
            return {
                result: "success",
                block_number: event.number || 0,
                timestamp: new Date().toISOString(),
                event_type: "ethereum"
            };
        }
        "#;

        Self::new()
            .add_task(1, 1, Event::NeoBlock(neo_block))
            // Commenting out the Ethereum block until we add that event type
            // .add_task(1, 2, Event::EthereumBlock(ethereum_block))
            .add_function(1, 1, neo_function_code)
            .add_function(1, 2, ethereum_function_code)
    }

    /// Get next random task
    fn random_task(&mut self) -> Result<Task, TaskError> {
        // Generate a random event
        let event = self.generate_random_event();

        // Increment task index
        self.task_index += 1;
        self.count += 1;

        // Create a task with the event
        let wrapper_event = Event::new(event);
        
        Ok(Task::new(self.uid, 1 + (self.task_index as u64 % 2), wrapper_event.event))
    }

    /// Generate a random event
    fn generate_random_event(&mut self) -> Event {
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

                let mock_event = MockEvent {
                    message: format!("NeoBlock: {}", block.to_string())
                };

                Event::Mock(mock_event)
            }
            1 => {
                // Neo transaction
                let tx = json!({
                    "hash": format!("0x{:064x}", self.event_type_index),
                    "size": 124,
                    "version": 0,
                    "nonce": 123456,
                    "sysfee": 1000000,
                    "netfee": 500000,
                    "valid_until_block": 1000000,
                    "signers": [],
                    "attributes": [],
                    "script": "AQLTDQYe+8CkNY3IkXHy+YK4HAo=",
                    "witnesses": []
                });

                let mock_event = MockEvent {
                    message: format!("NeoTransaction: {}", tx.to_string())
                };

                Event::Mock(mock_event)
            }
            2 => {
                // Neo contract event
                let events = json!([
                    {
                        "contract": "0xb9d7ea3062e6aeeb3e8ad9548220c4ba1361d263",
                        "eventname": "Transfer",
                        "state": {
                            "type": "Array",
                            "value": [
                                {
                                    "type": "ByteString",
                                    "value": "AVfYx6Nba7dN7RwTLzaOLeJ3idQ="
                                },
                                {
                                    "type": "ByteString",
                                    "value": "AUgYeZryV4b5HeWn3hQp+ZxJrjQ="
                                },
                                {
                                    "type": "Integer",
                                    "value": "1000000"
                                }
                            ]
                        }
                    }
                ]);

                let mock_event = MockEvent {
                    message: format!("NeoContractEvent: {}", events.to_string())
                };

                Event::Mock(mock_event)
            }
            3 => {
                // Block
                let block = json!({
                    "hash": format!("0x{:064x}", self.event_type_index),
                    "size": 686,
                    "version": 0,
                    "previousblockhash": format!("0x{:064x}", self.event_type_index - 1),
                    "merkleroot": "0x5df40f3d8f18c2a51a710c1d2b2a1eb7181e4cbd87f9ad427336a9434ac57b27",
                    "time": chrono::Utc::now().timestamp(),
                    "index": self.event_type_index,
                    "nonce": "0000000000000000",
                    "nextconsensus": "NVg7LjGvUQV1qA7kwyqVxyqJ9wNZx3ZG3m",
                    "script": {
                        "invocation": "DEAvCgKQp/cG1JjkJw+ZZRUrFx8LswNVHAVCbZ5XCGqoVh1T050hOXxlcXIsAgP+CtAjIQFCfQJGNCPV9F8nCMbDojFJZq8nO0CtH1EBLzfs/0APQAqyKwyA8KOYbMjKI6O948QK5JbvvxwznTy+np0dqJnSIWRBIXRB+ACw8KBbNBQqv59iogFvbFnD5RUm+GFO3vTs",
                        "verification": "EwwhA866C7Uh/+VhvJQPIMXVRNuwMn+u49ppGEqXknpYIEvvDCECzakQKl0/JQQELb2E5YHH4HBgAoLpibBf03VSp2dExgwhA8zcLjJgHg0WSiRaLMVJSPNFE6UVSQKjwNLfZgqMvLKhGBEO"
                    },
                    "confirmations": 12,
                    "nextblockhash": "0xfedcba0987654321"
                });

                let mock_event = MockEvent {
                    message: format!("EthereumBlock: {}", block.to_string())
                };

                Event::Mock(mock_event)
            }
            4 => {
                // Transaction
                let tx = json!({
                    "hash": format!("0x{:064x}", self.event_type_index),
                    "nonce": "0x1",
                    "blockHash": format!("0x{:064x}", self.event_type_index),
                    "blockNumber": format!("0x{:x}", self.event_type_index),
                    "transactionIndex": "0x1",
                    "from": "0x88172dae580eee5a4bcfe522d3963604e51c986d",
                    "to": "0x2c2b9c9a4a25e24b174f26114e8926a9f2128fe4",
                    "value": "0x0",
                    "gas": "0x30d40",
                    "gasPrice": "0x59682f00",
                    "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360"
                });

                let mock_event = MockEvent {
                    message: format!("EthereumTransaction: {}", tx.to_string())
                };

                Event::Mock(mock_event)
            }
            5 => {
                // Events
                let events = json!([
                    {
                        "address": "0x4e65fda2159562a496f9f3522f89122a3088497a",
                        "topics": [
                            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                            "0x0000000000000000000000004e65fda2159562a496f9f3522f89122a3088497a",
                            "0x000000000000000000000000a59b89afeac9c93058e891c8e8147e2b3ff995a9"
                        ],
                        "data": "0x0000000000000000000000000000000000000000000000001bc16d674ec80000",
                        "blockNumber": format!("0x{:x}", self.event_type_index),
                        "transactionHash": format!("0x{:064x}", self.event_type_index),
                        "transactionIndex": "0x1",
                        "blockHash": format!("0x{:064x}", self.event_type_index),
                        "logIndex": "0x2",
                        "removed": false
                    }
                ]);

                let mock_event = MockEvent {
                    message: format!("EthereumContractEvent: {}", events.to_string())
                };

                Event::Mock(mock_event)
            }
            _ => {
                match self.count % 2 {
                    0 => {
                        // Ethereum new block
                        let block_number = self.count.to_string();
                        let timestamp = chrono::Utc::now().timestamp().to_string();
                        let hash = format!("0x{:064x}", self.count);
                        let parent_hash = format!("0x{:064x}", self.count - 1);

                        // Create mock event data string
                        let data_str = format!(
                            "EthereumNewBlock: {{\"block_number\": {}, \"timestamp\": {}, \"hash\": \"{}\", \"parent_hash\": \"{}\"}}",
                            block_number, timestamp, hash, parent_hash
                        );

                        // Create mock event for Ethereum block
                        let mock_event = MockEvent {
                            message: data_str,
                        };

                        Event::Mock(mock_event)
                    }
                    1 => {
                        // Ethereum contract event
                        let events = serde_json::json!([
                            {
                                "address": "0x4e65fda2159562a496f9f3522f89122a3088497a",
                                "topics": [
                                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                                    "0x0000000000000000000000004e65fda2159562a496f9f3522f89122a3088497a",
                                    "0x000000000000000000000000a59b89afeac9c93058e891c8e8147e2b3ff995a9"
                                ],
                                "data": "0x0000000000000000000000000000000000000000000000001bc16d674ec80000",
                                "blockNumber": format!("0x{:x}", self.count),
                                "transactionHash": format!("0x{:064x}", self.count),
                                "transactionIndex": "0x1",
                                "blockHash": format!("0x{:064x}", self.count),
                                "logIndex": "0x2",
                                "removed": false
                            }
                        ]);

                        // Create mock event for Ethereum contract event with message field
                        let mock_event = MockEvent {
                            message: format!("EthereumContractEvent: {}", events.to_string()),
                        };

                        Event::Mock(mock_event)
                    }
                    _ => {
                        // Default mock event for other trigger types
                        let mock_event = MockEvent {
                            message: "DefaultMock: {}".to_string(),
                        };
                        
                        Event::Mock(mock_event)
                    }
                }
            }
        }
    }

    /// Process next event in sequence
    fn sequential_task(&mut self) -> Result<Task, TaskError> {
        // If we're just starting, seed the task list with defaults
        if self.tasks.is_empty() {
            let sample = Self::sample();
            self.tasks = sample.tasks;
        }

        // Find the next task
        if self.task_index < self.tasks.len() {
            let task = &self.tasks[self.task_index];
            
            // Increment task index
            self.task_index += 1;
            self.count += 1;
            
            // Return a clone of the task
            Ok(task.clone())
        } else {
            // Generate a random task if we're out of predefined tasks
            self.random_task()
        }
    }
}

#[async_trait]
impl TaskSource for MockTaskSource {
    async fn acquire_task(
        &self,
        request: service::AcquireTaskInput,
    ) -> Result<service::Task, service::TaskError> {
        // Acquire task
        let event = Event::new(Event::Mock(MockEvent {
            timestamp: chrono::Utc::now().timestamp() as u64,
            data: "mock".to_string(),
        }));

        Ok(service::Task {
            uid: request.uid,
            fid: request.fid_hint,
            event: event.event,
        })
    }

    async fn acquire_fn(
        &self,
        request: service::AcquireFuncInput,
    ) -> Result<service::Func, service::TaskError> {
        // Acquire function
        Ok(service::Func {
            version: 1,
            code: "async function handler(request) { return { status: 200, body: 'mock' }; }"
                .to_string(),
        })
    }
}
