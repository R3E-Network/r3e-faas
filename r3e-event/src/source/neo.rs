// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::Duration;

// Import specific types from NeoRust to avoid name conflicts
use NeoRust::neo_clients::{
    HttpProvider, 
    RpcClient,
    APITrait
};
use url::Url;

use tokio::sync::Mutex;

// Import r3e-event types with explicit namespace
use crate::source::{*, events::{NeoBlock, NeoBlockHeader, NeoTx}};
use crate::Trigger;

pub struct NeoTaskSource {
    sleep: Duration,
    uid: u64,
    count: u64,
    client: Arc<Mutex<Option<RpcClient<HttpProvider>>>>,
    rpc_url: String,
    // Track the current trigger type to rotate between different event types
    current_trigger: crate::Trigger,
}

impl NeoTaskSource {
    pub fn new(sleep: Duration, uid: u64) -> Self {
        Self {
            sleep,
            uid,
            count: 0,
            client: Arc::new(Mutex::new(None)),
            // Default to Neo N3 TestNet
            rpc_url: "https://testnet1.neo.org:443".to_string(),
            // Start with NeoNewBlock trigger
            current_trigger: crate::Trigger::NeoNewBlock,
        }
    }
    
    pub fn with_rpc_url(mut self, rpc_url: impl Into<String>) -> Self {
        self.rpc_url = rpc_url.into();
        self
    }
    
    async fn ensure_client(&self) -> Result<Arc<RpcClient<HttpProvider>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut client_guard = self.client.lock().await;
        
        if client_guard.is_none() {
            // Create URL from string
            let url = Url::parse(&self.rpc_url).map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error + Send + Sync>
            })?;
            
            // Create HTTP provider and RPC client
            let provider = HttpProvider::new(url).expect("Failed to create HTTP provider");
            let client = RpcClient::new(provider);
            *client_guard = Some(client);
        }
        
        // Clone the client to return it
        let client = client_guard.as_ref().unwrap().clone();
        Ok(Arc::new(client))
    }
    
    // Fetch the latest Neo block
    async fn fetch_latest_block(&self) -> Result<NeoBlock, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.ensure_client().await?;
        
        // Get the current block count
        let block_count = client.get_block_count().await.map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Get the latest block (block_count - 1 is the latest block)
        let block_height = block_count - 1;
        
        // Convert block height to H256 for the get_block method
        let block_hash = client.get_block_hash(block_height).await.map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Get the block with full transaction details
        let block = client.get_block(block_hash, true).await.map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Parse nonce from hex string to u64
        let nonce_u64 = u64::from_str_radix(&block.nonce, 16).unwrap_or(0);
        
        // Create r3e-event NeoBlockHeader
        let header = Some(NeoBlockHeader {
            hash: block.hash.to_string(),
            version: block.version as u32,
            prev_block_hash: block.prev_block_hash.to_string(),
            merkle_root: block.merkle_root_hash.to_string(),
            time: block.time as u64,
            nonce: nonce_u64,
            height: block.index as u32,
            primary: block.primary.unwrap_or(0) as u32,
            next_consensus: block.next_consensus.to_string(),
            witnesses: vec![], // TODO: Convert witnesses if needed
        });
        
        // Convert transactions if available
        let txs = if let Some(transactions) = block.transactions {
            transactions.into_iter().map(|tx| {
                self.convert_transaction(tx)
            }).collect()
        } else {
            vec![]
        };
        
        Ok(NeoBlock {
            header,
            txs,
        })
    }
    
    // Fetch a specific transaction by hash
    async fn fetch_transaction(&self, tx_hash: &str) -> Result<NeoTx, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.ensure_client().await?;
        
        // Parse the transaction hash to H256
        let hash = tx_hash.parse().map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Get the transaction details
        // Note: We can't directly get the transaction object, so we'll create a mock one
        // In a real implementation, we would parse the raw transaction string
        let _tx_raw = client.get_raw_transaction(hash).await.map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Create a mock transaction for now
        // In a real implementation, we would parse the raw transaction
        Ok(NeoTx {
            hash: tx_hash.to_string(),
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
        })
    }
    
    // Fetch application logs for a transaction
    async fn fetch_application_log(&self, tx_hash: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.ensure_client().await?;
        
        // Parse the transaction hash to H256
        let hash = tx_hash.parse().map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Get the application log
        let app_log = client.get_application_log(hash).await.map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        // Convert to JSON string
        let app_log_json = serde_json::to_string(&app_log).map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
        
        Ok(app_log_json)
    }
    
    // Helper method to convert a NeoRust transaction to r3e-event NeoTx
    fn convert_transaction(&self, tx: NeoRust::neo_protocol::RTransaction) -> NeoTx {
        NeoTx {
            hash: tx.hash.to_string(),
            size: tx.size as u32,
            version: tx.version as u32,
            nonce: tx.nonce as u32,
            // Map fields according to r3e-event NeoTx structure
            sysfee: tx.sys_fee.parse::<u64>().unwrap_or(0),
            netfee: tx.net_fee.parse::<u64>().unwrap_or(0),
            // Required fields with default values
            script: tx.script.clone(),
            signers: vec![],
            attributes: vec![],
            witnesses: vec![],
            valid_until_block: tx.valid_until_block as u32,
        }
    }
}

#[async_trait::async_trait]
impl TaskSource for NeoTaskSource {
    async fn acquire_task(&mut self, _uid: u64, _fid_hint: u64) -> Result<Task, TaskError> {
        tokio::time::sleep(self.sleep).await;

        self.count += 1;
        let fid = 1 + (self.count & 1);
        
        // Rotate through different trigger types
        let event = match self.current_trigger {
            Trigger::NeoNewBlock => {
                // Get Neo block data from NeoRust SDK
                let neo_block = match self.fetch_latest_block().await {
                    Ok(block) => block,
                    Err(e) => {
                        // Log the error
                        eprintln!("Error fetching Neo block: {:?}", e);
                        
                        // Fallback to mock data if RPC fails
                        NeoBlock {
                            header: Some(NeoBlockHeader {
                                hash: format!("neo_block_hash_{}", self.count),
                                version: 0,
                                prev_block_hash: "previous_block_hash".to_string(),
                                merkle_root: "merkle_root".to_string(),
                                time: 0,
                                nonce: 0,
                                height: self.count as u32,
                                primary: 0,
                                next_consensus: "next_consensus".to_string(),
                                witnesses: vec![],
                            }),
                            txs: vec![],
                        }
                    }
                };
                
                // Update trigger for next time
                self.current_trigger = Trigger::NeoNewTx;
                
                // Create the event with the correct type
                event::Event::NeoBlock(neo_block)
            },
            Trigger::NeoNewTx => {
                // Get Neo block data to extract a transaction
                let neo_block = match self.fetch_latest_block().await {
                    Ok(block) => block,
                    Err(e) => {
                        eprintln!("Error fetching Neo block for transaction: {:?}", e);
                        // Fallback to empty block
                        NeoBlock {
                            header: None,
                            txs: vec![],
                        }
                    }
                };
                
                // Extract the first transaction if available
                let tx = if !neo_block.txs.is_empty() {
                    neo_block.txs[0].clone()
                } else {
                    // Fallback to mock transaction
                    NeoTx {
                        hash: format!("neo_tx_hash_{}", self.count),
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
                    }
                };
                
                // Update trigger for next time
                self.current_trigger = Trigger::NeoContractNotification;
                
                // Create the event with the transaction
                event::Event::NeoBlock(NeoBlock {
                    header: None,
                    txs: vec![tx],
                })
            },
            Trigger::NeoContractNotification | Trigger::NeoApplicationLog => {
                // For contract notifications and application logs, we need to get a transaction first
                let neo_block = match self.fetch_latest_block().await {
                    Ok(block) => block,
                    Err(e) => {
                        eprintln!("Error fetching Neo block for application log: {:?}", e);
                        // Fallback to empty block
                        NeoBlock {
                            header: None,
                            txs: vec![],
                        }
                    }
                };
                
                // Extract a transaction hash if available
                let tx_hash = if !neo_block.txs.is_empty() {
                    neo_block.txs[0].hash.clone()
                } else {
                    format!("mock_tx_hash_{}", self.count)
                };
                
                // Try to get application log for the transaction
                let app_log_json = match self.fetch_application_log(&tx_hash).await {
                    Ok(log) => log,
                    Err(e) => {
                        eprintln!("Error fetching application log: {:?}", e);
                        "{}".to_string() // Empty JSON object as fallback
                    }
                };
                
                // Parse the application log
                let app_log: serde_json::Value = match serde_json::from_str(&app_log_json) {
                    Ok(log) => log,
                    Err(e) => {
                        eprintln!("Error parsing application log: {:?}", e);
                        serde_json::json!({})
                    }
                };
                
                // Extract notifications from the application log
                let notifications = if let Some(executions) = app_log.get("executions").and_then(|e| e.as_array()) {
                    let mut all_notifications = Vec::new();
                    
                    for execution in executions {
                        if let Some(notifications_array) = execution.get("notifications").and_then(|n| n.as_array()) {
                            for notification in notifications_array {
                                all_notifications.push(notification.clone());
                            }
                        }
                    }
                    
                    all_notifications
                } else {
                    Vec::new()
                };
                
                // Update trigger for next time
                self.current_trigger = Trigger::NeoNewBlock;
                
                // Create the appropriate event based on the trigger type
                if self.current_trigger == Trigger::NeoContractNotification && !notifications.is_empty() {
                    // Return contract notification event
                    event::Event::NeoContractNotification {
                        tx_hash,
                        notifications,
                    }
                } else {
                    // Return application log event
                    event::Event::NeoApplicationLog {
                        tx_hash,
                        applicationLog: app_log,
                    }
                }
            },
            _ => {
                // For any other trigger type, default to NeoNewBlock
                self.current_trigger = Trigger::NeoNewBlock;
                
                // Get Neo block data
                let neo_block = match self.fetch_latest_block().await {
                    Ok(block) => block,
                    Err(e) => {
                        eprintln!("Error fetching Neo block: {:?}", e);
                        // Fallback to mock block
                        NeoBlock {
                            header: Some(NeoBlockHeader {
                                hash: format!("neo_block_hash_{}", self.count),
                                version: 0,
                                prev_block_hash: "previous_block_hash".to_string(),
                                merkle_root: "merkle_root".to_string(),
                                time: 0,
                                nonce: 0,
                                height: self.count as u32,
                                primary: 0,
                                next_consensus: "next_consensus".to_string(),
                                witnesses: vec![],
                            }),
                            txs: vec![],
                        }
                    }
                };
                
                event::Event::NeoBlock(neo_block)
            }
        };

        Ok(Task::new(self.uid, fid, event))
    }

    /// Filter events based on criteria
    fn filter_event(&self, event: &event::Event, filter: Option<&serde_json::Value>) -> bool {
        // If no filter is provided, return true (include all events)
        let filter = match filter {
            Some(f) => f,
            None => return true,
        };
        
        match event {
            event::Event::NeoBlock(block) => {
                // Filter by block height if specified
                if let Some(min_height) = filter.get("min_height").and_then(|h| h.as_u64()) {
                    if let Some(header) = &block.header {
                        if header.height as u64 < min_height {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by block time if specified
                if let Some(min_time) = filter.get("min_time").and_then(|t| t.as_u64()) {
                    if let Some(header) = &block.header {
                        if header.time < min_time {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by transaction count if specified
                if let Some(min_tx_count) = filter.get("min_tx_count").and_then(|c| c.as_u64()) {
                    if block.txs.len() < min_tx_count as usize {
                        return false;
                    }
                }
                
                // Filter by transaction hash if specified
                if let Some(tx_hash) = filter.get("tx_hash").and_then(|h| h.as_str()) {
                    if !block.txs.iter().any(|tx| tx.hash == tx_hash) {
                        return false;
                    }
                }
                
                true
            },
            event::Event::NeoContractNotification { tx_hash, notifications } => {
                // Filter by transaction hash if specified
                if let Some(filter_tx_hash) = filter.get("tx_hash").and_then(|h| h.as_str()) {
                    if tx_hash != filter_tx_hash {
                        return false;
                    }
                }
                
                // Filter by contract hash if specified
                if let Some(contract_hash) = filter.get("contract_hash").and_then(|h| h.as_str()) {
                    if !notifications.iter().any(|n| {
                        n.get("contract").and_then(|c| c.as_str()).unwrap_or("") == contract_hash
                    }) {
                        return false;
                    }
                }
                
                // Filter by event name if specified
                if let Some(event_name) = filter.get("event_name").and_then(|e| e.as_str()) {
                    if !notifications.iter().any(|n| {
                        n.get("eventName").and_then(|e| e.as_str()).unwrap_or("") == event_name
                    }) {
                        return false;
                    }
                }
                
                true
            },
            event::Event::NeoApplicationLog { tx_hash, applicationLog } => {
                // Filter by transaction hash if specified
                if let Some(filter_tx_hash) = filter.get("tx_hash").and_then(|h| h.as_str()) {
                    if tx_hash != filter_tx_hash {
                        return false;
                    }
                }
                
                // Filter by execution success if specified
                if let Some(success) = filter.get("success").and_then(|s| s.as_bool()) {
                    if let Some(executions) = applicationLog.get("executions").and_then(|e| e.as_array()) {
                        if !executions.iter().any(|e| e.get("vmstate").and_then(|s| s.as_str()).unwrap_or("") == "HALT") == success {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            },
            _ => true, // Include all other event types
        }
    }
    
    async fn acquire_fn(&mut self, _uid: u64, _fid: u64) -> Result<Func, FuncError> {
        let code = format!(
            r#"
            const delay = (n) => new Promise(r => setTimeout(r, n));

            export default async function(event) {{
                console.log("Neo event handler:", event);
                await delay(2100);
                
                // Determine the event type based on the data structure
                if (event.header) {{
                    // This is a Neo block event
                    console.log("Neo block event detected");
                    console.log("Block height:", event.header.height);
                    console.log("Block hash:", event.header.hash);
                    console.log("Block time:", event.header.time);
                    
                    // Process transactions if available
                    if (event.txs && event.txs.length > 0) {{
                        console.log("Transactions count:", event.txs.length);
                        for (let i = 0; i < Math.min(event.txs.length, 5); i++) {{
                            console.log(`Transaction ${{i}} hash:`, event.txs[i].hash);
                        }}
                    }}
                }} else if (event.txs && event.txs.length === 1) {{
                    // This is a Neo transaction event
                    console.log("Neo transaction event detected");
                    const tx = event.txs[0];
                    console.log("Transaction hash:", tx.hash);
                    console.log("Transaction size:", tx.size);
                    console.log("Transaction version:", tx.version);
                    console.log("Transaction script:", tx.script);
                    
                    // Process transaction details
                    console.log("System fee:", tx.sysfee);
                    console.log("Network fee:", tx.netfee);
                    console.log("Valid until block:", tx.valid_until_block);
                }} else if (event.applicationLog) {{
                    // This is a Neo application log event
                    console.log("Neo application log event detected");
                    console.log("Application log:", event.applicationLog);
                    
                    // Process notifications if available
                    if (event.applicationLog.notifications) {{
                        console.log("Notifications count:", event.applicationLog.notifications.length);
                        for (let i = 0; i < Math.min(event.applicationLog.notifications.length, 5); i++) {{
                            const notification = event.applicationLog.notifications[i];
                            console.log(`Notification ${{i}} contract:`, notification.contract);
                            console.log(`Notification ${{i}} event name:`, notification.eventName);
                            console.log(`Notification ${{i}} state:`, notification.state);
                        }}
                    }}
                }} else if (event.notifications) {{
                    // This is a Neo contract notification event
                    console.log("Neo contract notification event detected");
                    console.log("Transaction hash:", event.tx_hash);
                    console.log("Notifications count:", event.notifications.length);
                    
                    // Process notifications
                    for (let i = 0; i < Math.min(event.notifications.length, 5); i++) {{
                        const notification = event.notifications[i];
                        console.log(`Notification ${{i}} contract:`, notification.contract);
                        console.log(`Notification ${{i}} event name:`, notification.eventName);
                        console.log(`Notification ${{i}} state:`, notification.state);
                    }}
                }} else {{
                    // Unknown event type
                    console.log("Unknown Neo event type:", event);
                }}
            }}"#
        );
        Ok(Func {
            version: 1,
            code: code.into(),
        })
    }
}
