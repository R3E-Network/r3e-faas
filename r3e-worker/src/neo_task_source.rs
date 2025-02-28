// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;
use async_trait::async_trait;
use r3e_event::source::{FnCode, Task, TaskSource};

/// Neo task source implementation
pub struct NeoTaskSource {
    poll_interval: Duration,
    batch_size: usize,
}

impl NeoTaskSource {
    pub fn new(poll_interval: Duration, batch_size: usize) -> Self {
        Self {
            poll_interval,
            batch_size,
        }
    }
}

#[async_trait]
impl TaskSource for NeoTaskSource {
    async fn acquire_task(&self, uid: u64, fid: u64) -> Result<Task, String> {
        // Query the Neo blockchain for tasks
        tokio::time::sleep(self.poll_interval).await;
        
        // Get the latest block number
        let latest_block = self.neo_client.get_latest_block()
            .await
            .map_err(|e| format!("Failed to get latest block: {}", e))?;
            
        // Get events from the latest block
        let events = self.neo_client.get_block_events(latest_block.number)
            .await
            .map_err(|e| format!("Failed to get block events: {}", e))?;
            
        // Find events matching the function ID
        for event in events {
            if event.function_id == fid {
                return Ok(Task {
                    fid,
                    event: serde_json::json!({
                        "type": "neo_event",
                        "data": {
                            "tx_hash": event.tx_hash,
                            "block_number": latest_block.number,
                            "timestamp": latest_block.timestamp,
                            "event_data": event.data
                        }
                    }),
                });
            }
        }
        
        // No matching events found
        Err("No tasks available".to_string())
    }

    async fn acquire_fn(&self, uid: u64, fid: u64) -> Result<FnCode, String> {
        // Query the function storage for the function code
        let function = self.storage.get_function(uid, fid)
            .await
            .map_err(|e| format!("Failed to get function: {}", e))?;
            
        // Verify function exists
        let function = function.ok_or_else(|| format!("Function not found: {}", fid))?;
        
        // Return the function code
        Ok(FnCode {
            version: function.version,
            code: function.code,
        })
    }
}
