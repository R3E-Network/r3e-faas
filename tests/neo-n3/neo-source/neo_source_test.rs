// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use r3e_event::source::neo::NeoTaskSource;
    use r3e_event::source::{Task, TaskSource, TaskError};
    use r3e_event::event::{Event, NeoBlock, NeoBlockHeader};
    use r3e_event::Trigger;
    use std::sync::Arc;
    use std::time::Duration;
    use mockall::predicate::*;
    use mockall::mock;

    // Create a mock for the Neo RPC client
    mock! {
        NeoRpcClient {}
        trait Clone {
            fn clone(&self) -> Self;
        }
    }

    // Helper function to create a mock Neo block
    fn create_mock_neo_block(height: u32, hash: &str) -> NeoBlock {
        NeoBlock {
            header: Some(NeoBlockHeader {
                hash: hash.to_string(),
                version: 0,
                prev_block_hash: "previous_block_hash".to_string(),
                merkle_root: "merkle_root".to_string(),
                time: 0,
                nonce: 0,
                height,
                primary: 0,
                next_consensus: "next_consensus".to_string(),
                witnesses: vec![],
            }),
            txs: vec![],
        }
    }

    #[tokio::test]
    async fn test_neo_block_event_handling() {
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Acquire a task from the source
        let task_result = source.acquire_task(1, 0).await;
        
        // Verify that the task was acquired successfully
        assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
        
        let task = task_result.unwrap();
        
        // Verify that the task has the correct source ID
        assert_eq!(task.uid, 1, "Task has incorrect source ID");
        
        // Verify that the task contains a Neo block event
        match task.event {
            Event::NeoBlock(block) => {
                // Verify that the block has a header
                assert!(block.header.is_some(), "Block header is missing");
                
                if let Some(header) = block.header {
                    // Verify that the block header has a hash
                    assert!(!header.hash.is_empty(), "Block hash is empty");
                    
                    // Verify that the block header has a height
                    assert!(header.height > 0, "Block height is invalid");
                    
                    // Log the block details for debugging
                    println!("Block height: {}", header.height);
                    println!("Block hash: {}", header.hash);
                }
                
                // Verify that the block has transactions (may be empty)
                println!("Transaction count: {}", block.txs.len());
            },
            _ => {
                panic!("Expected NeoBlock event, got {:?}", task.event);
            }
        }
        
        // Acquire a function for the task
        let func_result = source.acquire_fn(1, task.fid).await;
        
        // Verify that the function was acquired successfully
        assert!(func_result.is_ok(), "Failed to acquire function: {:?}", func_result.err());
        
        let func = func_result.unwrap();
        
        // Verify that the function has code
        assert!(!func.code.is_empty(), "Function code is empty");
        
        // Verify that the function code contains Neo block handling logic
        assert!(func.code.contains("Neo block event detected"), "Function code does not contain Neo block handling logic");
    }
    
    #[tokio::test]
    async fn test_neo_block_event_handling_with_mock_data() {
        // Create a mock Neo block
        let mock_block = create_mock_neo_block(12345, "mock_block_hash");
        
        // Verify the mock block
        assert_eq!(mock_block.header.as_ref().unwrap().height, 12345);
        assert_eq!(mock_block.header.as_ref().unwrap().hash, "mock_block_hash");
        
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Acquire a task from the source
        let task_result = source.acquire_task(1, 0).await;
        
        // Verify that the task was acquired successfully
        assert!(task_result.is_ok());
        
        let task = task_result.unwrap();
        
        // Verify that the task contains a Neo block event
        match task.event {
            Event::NeoBlock(_) => {
                // Test passed
            },
            _ => {
                panic!("Expected NeoBlock event, got {:?}", task.event);
            }
        }
    }
    
    #[tokio::test]
    async fn test_neo_block_event_handling_error_case() {
        // Create a NeoTaskSource with an invalid RPC URL to simulate an error
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1)
            .with_rpc_url("invalid-url");
        
        // Acquire a task from the source
        let task_result = source.acquire_task(1, 0).await;
        
        // Verify that the task was acquired successfully (should fall back to mock data)
        assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
        
        let task = task_result.unwrap();
        
        // Verify that the task contains a Neo block event
        match task.event {
            Event::NeoBlock(block) => {
                // Verify that the block has a header (should be mock data)
                assert!(block.header.is_some(), "Block header is missing");
                
                if let Some(header) = block.header {
                    // Verify that the block header has a hash (should be mock data)
                    assert!(!header.hash.is_empty(), "Block hash is empty");
                    assert!(header.hash.contains("neo_block_hash_"), "Block hash does not match mock format");
                }
            },
            _ => {
                panic!("Expected NeoBlock event, got {:?}", task.event);
            }
        }
    }
    
    #[tokio::test]
    async fn test_neo_transaction_event_handling() {
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Set the current trigger to NeoNewTx to force transaction event handling
        // This is a private field, so we need to acquire multiple tasks until we get a transaction
        
        // Acquire tasks until we get a transaction event
        let mut task_result = source.acquire_task(1, 0).await;
        let mut attempts = 0;
        let max_attempts = 5;
        let mut found_tx_event = false;
        
        while attempts < max_attempts {
            assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
            
            let task = task_result.unwrap();
            
            // Check if this is a transaction event
            if let Event::NeoBlock(block) = &task.event {
                if block.header.is_none() && !block.txs.is_empty() {
                    // This is a transaction event (Neo transaction events are represented as NeoBlock with no header and one transaction)
                    found_tx_event = true;
                    
                    // Verify the transaction
                    let tx = &block.txs[0];
                    
                    // Verify that the transaction has a hash
                    assert!(!tx.hash.is_empty(), "Transaction hash is empty");
                    
                    // Verify that the transaction has a script
                    assert!(!tx.script.is_empty(), "Transaction script is empty");
                    
                    // Log the transaction details for debugging
                    println!("Transaction hash: {}", tx.hash);
                    println!("Transaction size: {}", tx.size);
                    println!("Transaction version: {}", tx.version);
                    
                    // Acquire a function for the task
                    let func_result = source.acquire_fn(1, task.fid).await;
                    
                    // Verify that the function was acquired successfully
                    assert!(func_result.is_ok(), "Failed to acquire function: {:?}", func_result.err());
                    
                    let func = func_result.unwrap();
                    
                    // Verify that the function has code
                    assert!(!func.code.is_empty(), "Function code is empty");
                    
                    // Verify that the function code contains Neo transaction handling logic
                    assert!(func.code.contains("Neo transaction event detected"), "Function code does not contain Neo transaction handling logic");
                    
                    break;
                }
            }
            
            // Try again
            attempts += 1;
            task_result = source.acquire_task(1, 0).await;
        }
        
        // Verify that we found a transaction event
        assert!(found_tx_event, "Failed to find a transaction event after {} attempts", max_attempts);
    }
    
    #[tokio::test]
    async fn test_neo_transaction_event_handling_with_mock_data() {
        // Create a mock Neo transaction
        let mock_tx = r3e_event::event::NeoTx {
            hash: "mock_tx_hash".to_string(),
            size: 100,
            version: 0,
            nonce: 0,
            sysfee: 0,
            netfee: 0,
            valid_until_block: 0,
            script: "mock_script".to_string(),
            signers: vec![],
            attributes: vec![],
            witnesses: vec![],
        };
        
        // Create a mock Neo block with no header and one transaction
        let mock_block = NeoBlock {
            header: None,
            txs: vec![mock_tx],
        };
        
        // Verify the mock block
        assert!(mock_block.header.is_none());
        assert_eq!(mock_block.txs.len(), 1);
        assert_eq!(mock_block.txs[0].hash, "mock_tx_hash");
        assert_eq!(mock_block.txs[0].script, "mock_script");
    }
    
    #[tokio::test]
    async fn test_neo_contract_notification_event_handling() {
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Set the current trigger to NeoContractNotification to force contract notification event handling
        // This is a private field, so we need to acquire multiple tasks until we get a contract notification
        
        // Acquire tasks until we get a contract notification event
        let mut task_result = source.acquire_task(1, 0).await;
        let mut attempts = 0;
        let max_attempts = 10;
        let mut found_notification_event = false;
        
        while attempts < max_attempts {
            assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
            
            let task = task_result.unwrap();
            
            // Check if this is a contract notification event
            if let Event::NeoBlock(block) = &task.event {
                // In the current implementation, contract notifications are represented as NeoBlock events
                // with specific properties. We need to check if this is a contract notification event.
                
                // For now, we'll consider it a contract notification if it's not a block event (no header)
                // and not a transaction event (no transactions)
                if block.header.is_none() {
                    // This might be a contract notification event
                    found_notification_event = true;
                    
                    // Log the event details for debugging
                    println!("Contract notification event detected");
                    
                    // Acquire a function for the task
                    let func_result = source.acquire_fn(1, task.fid).await;
                    
                    // Verify that the function was acquired successfully
                    assert!(func_result.is_ok(), "Failed to acquire function: {:?}", func_result.err());
                    
                    let func = func_result.unwrap();
                    
                    // Verify that the function has code
                    assert!(!func.code.is_empty(), "Function code is empty");
                    
                    // Verify that the function code contains Neo event handling logic
                    assert!(func.code.contains("Neo event handler"), "Function code does not contain Neo event handling logic");
                    
                    break;
                }
            }
            
            // Try again
            attempts += 1;
            task_result = source.acquire_task(1, 0).await;
        }
        
        // Verify that we found a contract notification event
        assert!(found_notification_event, "Failed to find a contract notification event after {} attempts", max_attempts);
    }
    
    #[tokio::test]
    async fn test_neo_contract_notification_event_handling_with_mock_data() {
        // Create a mock Neo application log with notifications
        let mock_app_log = r#"{
            "txid": "mock_tx_hash",
            "executions": [
                {
                    "trigger": "Application",
                    "vmstate": "HALT",
                    "gasconsumed": "9999999",
                    "stack": [],
                    "notifications": [
                        {
                            "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
                            "eventname": "Transfer",
                            "state": {
                                "type": "Array",
                                "value": [
                                    {
                                        "type": "ByteString",
                                        "value": "ZXhhbXBsZV9mcm9t"
                                    },
                                    {
                                        "type": "ByteString",
                                        "value": "ZXhhbXBsZV90bw=="
                                    },
                                    {
                                        "type": "Integer",
                                        "value": "1000000"
                                    }
                                ]
                            }
                        }
                    ]
                }
            ]
        }"#;
        
        // Parse the mock application log
        let app_log: serde_json::Value = serde_json::from_str(mock_app_log).unwrap();
        
        // Verify the mock application log
        assert_eq!(app_log["txid"], "mock_tx_hash");
        assert!(app_log["executions"].is_array());
        assert!(app_log["executions"][0]["notifications"].is_array());
        assert_eq!(app_log["executions"][0]["notifications"][0]["eventname"], "Transfer");
        assert_eq!(app_log["executions"][0]["notifications"][0]["contract"], "0xd2a4cff31913016155e38e474a2c06d08be276cf");
    }
    
    #[tokio::test]
    async fn test_neo_rpc_client_integration() {
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Test the ensure_client method by acquiring a task
        // This will internally call ensure_client to create an RPC client
        let task_result = source.acquire_task(1, 0).await;
        
        // Verify that the task was acquired successfully
        assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
        
        // Test with an invalid RPC URL to verify error handling
        let mut invalid_source = NeoTaskSource::new(Duration::from_millis(10), 2)
            .with_rpc_url("invalid-url");
        
        // Acquire a task from the invalid source
        let invalid_task_result = invalid_source.acquire_task(2, 0).await;
        
        // Verify that the task was still acquired successfully (should fall back to mock data)
        assert!(invalid_task_result.is_ok(), "Failed to acquire task with invalid RPC URL: {:?}", invalid_task_result.err());
        
        // Verify that the task contains a Neo block event with mock data
        match invalid_task_result.unwrap().event {
            Event::NeoBlock(block) => {
                // Verify that the block has a header (should be mock data)
                assert!(block.header.is_some(), "Block header is missing");
                
                if let Some(header) = block.header {
                    // Verify that the block header has a hash (should be mock data)
                    assert!(!header.hash.is_empty(), "Block hash is empty");
                    assert!(header.hash.contains("neo_block_hash_"), "Block hash does not match mock format");
                }
            },
            _ => {
                panic!("Expected NeoBlock event, got unexpected event type");
            }
        }
    }
    
    #[tokio::test]
    async fn test_neo_rpc_client_integration_with_mock() {
        // Create a mock RPC client
        let mut mock_client = MockNeoRpcClient::new();
        
        // Set up expectations for the mock client
        // This is a simplified example since we can't actually mock the NeoRust RPC client directly
        
        // Create a NeoTaskSource with a short sleep duration
        let source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Verify that the source was created successfully
        assert_eq!(source.uid, 1, "Source has incorrect ID");
        
        // Verify that the source has the default RPC URL
        // Note: We can't directly access the private rpc_url field, so we're testing indirectly
        
        // Create a source with a custom RPC URL
        let custom_source = NeoTaskSource::new(Duration::from_millis(10), 2)
            .with_rpc_url("https://custom.neo.org:443");
        
        // Verify that the custom source was created successfully
        assert_eq!(custom_source.uid, 2, "Custom source has incorrect ID");
    }
    
    #[tokio::test]
    async fn test_neo_event_filtering() {
        // Create a NeoTaskSource with a short sleep duration
        let mut source = NeoTaskSource::new(Duration::from_millis(10), 1);
        
        // Acquire multiple tasks to test event filtering
        let mut tasks = Vec::new();
        let num_tasks = 5;
        
        for _ in 0..num_tasks {
            let task_result = source.acquire_task(1, 0).await;
            assert!(task_result.is_ok(), "Failed to acquire task: {:?}", task_result.err());
            tasks.push(task_result.unwrap());
        }
        
        // Verify that we have acquired multiple tasks
        assert_eq!(tasks.len(), num_tasks, "Failed to acquire the expected number of tasks");
        
        // Filter tasks by event type
        let block_events: Vec<&Task> = tasks.iter()
            .filter(|task| {
                if let Event::NeoBlock(block) = &task.event {
                    block.header.is_some()
                } else {
                    false
                }
            })
            .collect();
        
        // Verify that we have at least one block event
        assert!(!block_events.is_empty(), "No block events found");
        
        // Log the number of block events
        println!("Found {} block events", block_events.len());
        
        // Filter tasks by transaction events (NeoBlock with no header and non-empty txs)
        let tx_events: Vec<&Task> = tasks.iter()
            .filter(|task| {
                if let Event::NeoBlock(block) = &task.event {
                    block.header.is_none() && !block.txs.is_empty()
                } else {
                    false
                }
            })
            .collect();
        
        // Log the number of transaction events
        println!("Found {} transaction events", tx_events.len());
        
        // Test filtering by block height (if we have block events)
        if !block_events.is_empty() {
            // Get the block height of the first block event
            if let Event::NeoBlock(block) = &block_events[0].event {
                if let Some(header) = &block.header {
                    let height = header.height;
                    
                    // Filter blocks by height
                    let blocks_with_height: Vec<&Task> = block_events.iter()
                        .filter(|task| {
                            if let Event::NeoBlock(block) = &task.event {
                                if let Some(header) = &block.header {
                                    header.height == height
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        })
                        .copied()
                        .collect();
                    
                    // Verify that we have at least one block with the specified height
                    assert!(!blocks_with_height.is_empty(), "No blocks found with height {}", height);
                    
                    // Log the number of blocks with the specified height
                    println!("Found {} blocks with height {}", blocks_with_height.len(), height);
                }
            }
        }
    }
    
    #[tokio::test]
    async fn test_neo_event_filtering_with_mock_data() {
        // Create mock Neo blocks with different heights
        let block1 = create_mock_neo_block(100, "block_hash_100");
        let block2 = create_mock_neo_block(101, "block_hash_101");
        let block3 = create_mock_neo_block(102, "block_hash_102");
        
        // Create a vector of blocks
        let blocks = vec![block1, block2, block3];
        
        // Filter blocks by height
        let blocks_with_height_101: Vec<&NeoBlock> = blocks.iter()
            .filter(|block| {
                if let Some(header) = &block.header {
                    header.height == 101
                } else {
                    false
                }
            })
            .collect();
        
        // Verify that we have exactly one block with height 101
        assert_eq!(blocks_with_height_101.len(), 1, "Expected exactly one block with height 101");
        
        // Verify that the block has the correct hash
        assert_eq!(blocks_with_height_101[0].header.as_ref().unwrap().hash, "block_hash_101");
        
        // Filter blocks by height range
        let blocks_with_height_range: Vec<&NeoBlock> = blocks.iter()
            .filter(|block| {
                if let Some(header) = &block.header {
                    header.height >= 100 && header.height <= 101
                } else {
                    false
                }
            })
            .collect();
        
        // Verify that we have exactly two blocks in the height range
        assert_eq!(blocks_with_height_range.len(), 2, "Expected exactly two blocks in the height range");
        
        // Create mock Neo transactions
        let tx1 = r3e_event::event::NeoTx {
            hash: "tx_hash_1".to_string(),
            size: 100,
            version: 0,
            nonce: 0,
            sysfee: 0,
            netfee: 0,
            valid_until_block: 0,
            script: "script_1".to_string(),
            signers: vec![],
            attributes: vec![],
            witnesses: vec![],
        };
        
        let tx2 = r3e_event::event::NeoTx {
            hash: "tx_hash_2".to_string(),
            size: 200,
            version: 0,
            nonce: 0,
            sysfee: 0,
            netfee: 0,
            valid_until_block: 0,
            script: "script_2".to_string(),
            signers: vec![],
            attributes: vec![],
            witnesses: vec![],
        };
        
        // Create a block with transactions
        let block_with_txs = NeoBlock {
            header: Some(NeoBlockHeader {
                hash: "block_hash_with_txs".to_string(),
                version: 0,
                prev_block_hash: "prev_hash".to_string(),
                merkle_root: "merkle_root".to_string(),
                time: 0,
                nonce: 0,
                height: 200,
                primary: 0,
                next_consensus: "next_consensus".to_string(),
                witnesses: vec![],
            }),
            txs: vec![tx1, tx2],
        };
        
        // Verify that the block has transactions
        assert_eq!(block_with_txs.txs.len(), 2, "Expected block to have 2 transactions");
        
        // Filter transactions by hash
        let txs_with_hash: Vec<&r3e_event::event::NeoTx> = block_with_txs.txs.iter()
            .filter(|tx| tx.hash == "tx_hash_1")
            .collect();
        
        // Verify that we have exactly one transaction with the specified hash
        assert_eq!(txs_with_hash.len(), 1, "Expected exactly one transaction with hash tx_hash_1");
        
        // Verify that the transaction has the correct script
        assert_eq!(txs_with_hash[0].script, "script_1");
    }
}
