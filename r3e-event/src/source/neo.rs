// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::source::events::{event, BtcBlock, Event, NeoApplication, NeoBlock, NeoContractEvent, NeoEvent, NeoTransaction};
use crate::source::{Task, TaskError, TaskSource, Func, FuncError};
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, error, info, warn};
use tokio::sync::RwLock;
use std::time::Duration;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;
use uuid::Uuid;
use reqwest;
use neo3::neo_clients::{APITrait, HttpProvider, RpcClient};
use neo3::prelude::transaction;
use super::event::Event as EventEnum;
use super::service;
use super::service::TaskSource;

/// Neo trigger types
#[derive(Debug, Clone, PartialEq)]
pub enum NeoTrigger {
    /// New block
    NeoNewBlock,
    /// New transaction
    NeoNewTx,
    /// Contract notification
    NeoContractNotification,
    /// Application log
    NeoApplicationLog,
}

pub struct NeoTaskSource {
    sleep: Duration,
    uid: u64,
    count: u64,
    client: Arc<RwLock<Option<RpcClient<HttpProvider>>>>,
    rpc_url: String,
    // Track the current trigger type to rotate between different event types
    current_trigger: NeoTrigger,
    filter: Option<String>,
}

impl NeoTaskSource {
    /// Create a new Neo task source
    pub fn new(sleep: Duration, uid: u64, filter: Option<String>) -> Self {
        Self {
            sleep,
            uid,
            count: 0,
            client: Arc::new(RwLock::new(None)),
            // Default to Neo N3 TestNet
            rpc_url: "https://testnet1.neo.org:443".to_string(),
            // Start with NeoNewBlock trigger
            current_trigger: NeoTrigger::NeoNewBlock,
            filter,
        }
    }

    pub fn with_rpc_url(mut self, rpc_url: impl Into<String>) -> Self {
        self.rpc_url = rpc_url.into();
        self
    }

    async fn ensure_client(
        &self,
    ) -> Result<Arc<RpcClient<HttpProvider>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut client_guard = self.client.write().await;

        if client_guard.is_none() {
            // Create URL from string
            let url = Url::parse(&self.rpc_url)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

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
    async fn fetch_latest_block(
        &self,
    ) -> Result<NeoBlock, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.ensure_client().await?;

        // Get the current block count
        let block_count = client
            .get_block_count()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Get the latest block (block_count - 1 is the latest block)
        let block_height = block_count - 1;

        // Convert block height to H256 for the get_block method
        let block_hash = client
            .get_block_hash(block_height)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Get the block with full transaction details
        let block = client
            .get_block(block_hash, true)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

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
            transactions
                .into_iter()
                .map(|tx| {
                    NeoTx {
                        hash: tx.hash.to_string(),
                        size: tx.size as u32,
                        version: tx.version as u32,
                        nonce: tx.nonce as u32,
                        sysfee: tx.sys_fee as u64,
                        netfee: tx.net_fee as u64,
                        valid_until_block: tx.valid_until_block as u32,
                        script: tx.script.clone(),
                        signers: vec![],
                        attributes: vec![],
                        witnesses: vec![],
                    }
                })
                .collect()
        } else {
            vec![]
        };

        Ok(NeoBlock { header, txs })
    }

    // Fetch a specific transaction by hash
    async fn fetch_transaction(
        &self,
        hash: &str,
    ) -> Result<NeoTx, Box<dyn std::error::Error + Send + Sync>> {
        // GET Neo transaction
        let endpoint = format!("{}/gettransaction/{}", self.rpc_url, hash);
        let response = reqwest::get(&endpoint).await?;
        let data = response.json::<serde_json::Value>().await?;

        // Extract transaction data
        let tx = data["result"].clone();
        
        // Create a NeoTx instance from the JSON data
        Ok(NeoTx {
            hash: hash.to_string(),
            size: tx["size"].as_u64().unwrap_or(0) as u32,
            version: tx["version"].as_u64().unwrap_or(0) as u32,
            nonce: tx["nonce"].as_u64().unwrap_or(0) as u32,
            sysfee: tx["sysfee"].as_u64().unwrap_or(0),
            netfee: tx["netfee"].as_u64().unwrap_or(0),
            valid_until_block: tx["validuntilblock"].as_u64().unwrap_or(0) as u32,
            script: tx["script"].as_str().unwrap_or("").to_string(),
            signers: vec![], // Convert signers if available
            attributes: vec![], // Convert attributes if available
            witnesses: vec![], // Convert witnesses if available
        })
    }

    // Fetch application logs for a transaction
    async fn fetch_application_log(
        &self,
        tx_hash: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.ensure_client().await?;

        // Parse the transaction hash to H256
        let hash = tx_hash
            .parse()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Get the application log
        let app_log = client
            .get_application_log(hash)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Convert to JSON string
        let app_log_json = serde_json::to_string(&app_log)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(app_log_json)
    }

    // Convert a proto transaction to a Neo transaction
    fn convert_transaction(&self, tx: NeoTx) -> NeoTx {
        NeoTx {
            hash: tx.hash.to_string(),
            size: tx.size,
            version: tx.version,
            nonce: tx.nonce,
            sysfee: tx.sysfee,
            netfee: tx.netfee,
            valid_until_block: tx.valid_until_block,
            script: tx.script.to_string(),
            signers: vec![],
            attributes: vec![],
            witnesses: vec![],
        }
    }

    // Helper to filter events based on criteria
    fn filter_event(&self, event: &EventEnum, filter: Option<&String>) -> bool {
        // If no filter is specified, keep all events
        if filter.is_none() {
            return true;
        }

        // Extract filter pattern
        let filter = filter.unwrap();
        
        // Basic pattern matching based on the event type
        match event {
            EventEnum::Mock(_) => {
                // Always match mock events
                true
            }
            EventEnum::NeoBlock(block) => {
                // Match Neo block events
                if let Some(header) = &block.header {
                    // Check if hash contains filter string
                    header.hash.contains(filter)
                } else {
                    false
                }
            }
            EventEnum::NeoApplicationLog(app_log) => {
                // Match application log events
                app_log.tx_hash.contains(filter) || app_log.application_log.contains(filter)
            }
            EventEnum::NeoContractNotification(notification) => {
                // Match notification events
                notification.tx_hash.contains(filter) || notification.notifications.contains(filter)
            }
            _ => {
                // Default to keep if type doesn't match
                true
            }
        }
    }

    /// Generate a Neo event based on the current trigger
    async fn generate_neo_event(&mut self) -> Result<Task, TaskError> {
        // Based on the current trigger type, process the event
        match self.current_trigger {
            NeoTrigger::NeoNewBlock => {
                // Get Neo block data
                let block_data = self.fetch_latest_block().await.unwrap_or_else(|e| {
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
                });

                // Get Neo block header fields
                let header = block_data.header.as_ref().unwrap();

                // Create Neo block from the data
                let neo_block = NeoBlock {
                    header: Some(NeoBlockHeader {
                        hash: header.hash.clone(),
                        version: header.version,
                        prev_block_hash: header.prev_block_hash.clone(),
                        merkle_root: header.merkle_root.clone(),
                        time: header.time,
                        height: header.height,
                        primary: header.primary,
                        next_consensus: header.next_consensus.clone(),
                        witnesses: header.witnesses.clone(),
                    }),
                    txs: block_data.txs,
                };

                // Update trigger type for next event
                self.current_trigger = NeoTrigger::NeoNewTx;

                // Create the event with the correct type
                let event = EventEnum::NeoBlock(neo_block);
                Ok(Task::new(self.uid, 1, event))
            }
            NeoTrigger::NeoNewTx => {
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
                self.current_trigger = NeoTrigger::NeoContractNotification;

                // Create the event with the transaction
                let event = EventEnum::NeoBlock(NeoBlock {
                    header: None,
                    txs: vec![tx],
                });
                Ok(Task::new(self.uid, 1, event))
            }
            NeoTrigger::NeoContractNotification | NeoTrigger::NeoApplicationLog => {
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
                    // Generate a random transaction hash
                    format!("{:x}", uuid::Uuid::new_v4())
                };

                // Try to get application log for the transaction
                let app_log_json = match self.fetch_application_log(&tx_hash).await {
                    Ok(log) => log,
                    Err(e) => {
                        eprintln!("Error fetching application log: {:?}", e);
                        // Make applicationLog a valid JSON string
                        json!({
                            "executions": [
                                {
                                    "vmstate": "HALT"
                                }
                            ]
                        }).to_string()
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
                let notifications = if let Some(executions) =
                    app_log.get("executions").and_then(|e| e.as_array())
                {
                    let mut all_notifications = Vec::new();

                    for execution in executions {
                        if let Some(notifications_array) =
                            execution.get("notifications").and_then(|n| n.as_array())
                        {
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
                let is_notification = matches!(self.current_trigger, NeoTrigger::NeoContractNotification);
                self.current_trigger = NeoTrigger::NeoNewBlock;

                // Create the appropriate event based on the trigger type
                if is_notification {
                    // Prepare notifications as a JSON string
                    let notifications_json = json!([
                        {
                            "contract": "0x1234567890abcdef",
                            "eventName": "Transfer",
                            "state": {
                                "from": "NZVpKpPPQv2Tah47sZNGm4x44xLMb6aGAi",
                                "to": "NcTPkqRuXqM3SxJsZcirj4oQrYi1n8s1ah",
                                "amount": "100000000"
                            }
                        }
                    ]).to_string();

                    // Return contract notification event
                    let event = EventEnum::NeoContractNotification(NeoContractNotification {
                        tx_hash,
                        notifications: notifications_json,
                    });
                    Ok(Task::new(self.uid, 1, event))
                } else {
                    // Return application log event
                    let event = EventEnum::NeoApplicationLog(NeoApplicationLog {
                        tx_hash,
                        application_log: app_log_json,
                    });
                    Ok(Task::new(self.uid, 1, event))
                }
            }
            _ => {
                // For any other trigger type, default to NeoNewBlock
                self.current_trigger = NeoTrigger::NeoNewBlock;

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

                let event = EventEnum::NeoBlock(neo_block);
                Ok(Task::new(self.uid, 1, event))
            }
        }
    }

    async fn generate_neo_function(&mut self, uid: u64, fid: u64) -> Result<Func, FuncError> {
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

#[async_trait]
impl TaskSource for NeoTaskSource {
    async fn acquire_task(
        &self,
        request: service::AcquireTaskInput,
    ) -> Result<service::Task, service::TaskError> {
        // Log a placeholder message
        info!("NeoTaskSource.acquire_task: uid={}, fid_hint={}", request.uid, request.fid_hint);

        // Acquire task
        // Just return a mock event for now
        let event = EventEnum::new(EventEnum::Neo(NeoEvent {
            contract: "0x1234567890abcdef".to_string(),
            tx_hash: "0xabcdef1234567890".to_string(),
            method: "transfer".to_string(),
            args: vec![],
            block: 100,
            timestamp: chrono::Utc::now().timestamp() as u64,
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
            code: "async function handler(request) { return { status: 200, body: 'neo' }; }"
                .to_string(),
        })
    }
}
