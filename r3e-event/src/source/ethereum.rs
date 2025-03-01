// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use async_trait::async_trait;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use log::{debug, info, warn};
use serde_json::json;
use uuid::Uuid;

use crate::source::{event, Func, FuncError, Task, TaskError, TaskSource};

/// Ethereum task source
pub struct EthereumTaskSource {
    /// Sleep duration between tasks
    sleep: Duration,
    /// User ID
    uid: u64,
    /// Current trigger
    current_trigger: Trigger,
    /// RPC URL
    rpc_url: String,
    /// Filter
    filter: Option<serde_json::Value>,
}

/// Ethereum trigger types
#[derive(Debug, Clone, PartialEq)]
pub enum Trigger {
    /// New block
    EthereumNewBlock,
    /// Contract event
    EthereumContractEvent,
    /// Transaction
    EthereumTransaction,
}

impl EthereumTaskSource {
    /// Create a new Ethereum task source
    pub fn new(sleep: Duration, uid: u64) -> Self {
        Self {
            sleep,
            uid,
            current_trigger: Trigger::EthereumNewBlock,
            rpc_url: "https://mainnet.infura.io/v3/your-project-id".to_string(),
            filter: None,
        }
    }

    /// Set RPC URL
    pub fn with_rpc_url(mut self, rpc_url: &str) -> Self {
        self.rpc_url = rpc_url.to_string();
        self
    }

    /// Set filter
    pub fn with_filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Fetch latest block
    async fn fetch_latest_block(&self) -> Result<serde_json::Value, String> {
        // Use ethers-rs to fetch the latest block
        log::info!("Fetching latest Ethereum block from {}", self.rpc_url);

        // Create a provider
        let provider = match Provider::<Http>::try_from(self.rpc_url.clone()) {
            Ok(provider) => provider,
            Err(e) => {
                return Err(format!("Failed to create Ethereum provider: {}", e));
            }
        };

        // Fetch the latest block
        let block = match provider.get_block(BlockNumber::Latest).await {
            Ok(Some(block)) => {
                // Convert the block to a JSON value
                let block_json = serde_json::to_value(block)
                    .map_err(|e| format!("Failed to serialize block: {}", e))?;

                block_json
            }
            Ok(None) => {
                return Err("No latest block found".to_string());
            }
            Err(e) => {
                return Err(format!("Failed to fetch latest block: {}", e));
            }
        };

        log::debug!("Fetched Ethereum block: {}", block["number"]);

        Ok(block)
    }

    /// Fetch contract events
    async fn fetch_contract_events(
        &self,
        contract_address: &str,
    ) -> Result<serde_json::Value, String> {
        // Use ethers-rs to fetch contract events
        log::info!("Fetching Ethereum contract events for {}", contract_address);

        // Create a provider
        let provider = match Provider::<Http>::try_from(self.rpc_url.clone()) {
            Ok(provider) => provider,
            Err(e) => {
                return Err(format!("Failed to create Ethereum provider: {}", e));
            }
        };

        // Create a filter for the contract events
        let filter = Filter::new()
            .address(vec![contract_address
                .parse::<Address>()
                .map_err(|e| format!("Invalid contract address: {}", e))?])
            .from_block(BlockNumber::Latest);

        // Fetch the events
        let logs = match provider.get_logs(&filter).await {
            Ok(logs) => {
                // Convert the logs to a JSON value
                let logs_json = serde_json::to_value(logs)
                    .map_err(|e| format!("Failed to serialize logs: {}", e))?;

                logs_json
            }
            Err(e) => {
                return Err(format!("Failed to fetch contract events: {}", e));
            }
        };

        log::debug!(
            "Fetched {} Ethereum contract events",
            logs.as_array().map_or(0, |a| a.len())
        );

        Ok(logs)
    }

    /// Fetch transaction
    async fn fetch_transaction(&self, tx_hash: &str) -> Result<serde_json::Value, String> {
        // Use ethers-rs to fetch the transaction
        log::info!("Fetching Ethereum transaction {}", tx_hash);

        // Create a provider
        let provider = match Provider::<Http>::try_from(self.rpc_url.clone()) {
            Ok(provider) => provider,
            Err(e) => {
                return Err(format!("Failed to create Ethereum provider: {}", e));
            }
        };

        // Parse the transaction hash
        let hash = tx_hash
            .parse::<H256>()
            .map_err(|e| format!("Invalid transaction hash: {}", e))?;

        // Fetch the transaction
        let transaction = match provider.get_transaction(hash).await {
            Ok(Some(tx)) => {
                // Convert the transaction to a JSON value
                let tx_json = serde_json::to_value(tx)
                    .map_err(|e| format!("Failed to serialize transaction: {}", e))?;

                tx_json
            }
            Ok(None) => {
                return Err(format!("Transaction not found: {}", tx_hash));
            }
            Err(e) => {
                return Err(format!("Failed to fetch transaction: {}", e));
            }
        };

        log::debug!("Fetched Ethereum transaction: {}", tx_hash);

        Ok(transaction)
    }

    /// Filter events based on criteria
    fn filter_event(&self, event: &event::Event, filter: Option<&serde_json::Value>) -> bool {
        // If no filter is provided, return true (include all events)
        let filter = match filter {
            Some(f) => f,
            None => return true,
        };

        match event {
            event::Event::EthereumBlock(block) => {
                // Filter by block number if specified
                if let Some(min_block) = filter.get("min_block").and_then(|b| b.as_str()) {
                    if let Some(block_number) = block.get("number").and_then(|n| n.as_str()) {
                        // Convert hex strings to numbers for comparison
                        let min_block_num =
                            u64::from_str_radix(&min_block.trim_start_matches("0x"), 16)
                                .unwrap_or(0);
                        let block_num =
                            u64::from_str_radix(&block_number.trim_start_matches("0x"), 16)
                                .unwrap_or(0);

                        if block_num < min_block_num {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by miner if specified
                if let Some(miner) = filter.get("miner").and_then(|m| m.as_str()) {
                    if let Some(block_miner) = block.get("miner").and_then(|m| m.as_str()) {
                        if !block_miner.eq_ignore_ascii_case(miner) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by transaction count if specified
                if let Some(min_tx_count) = filter.get("min_tx_count").and_then(|c| c.as_u64()) {
                    if let Some(txs) = block.get("transactions").and_then(|t| t.as_array()) {
                        if txs.len() < min_tx_count as usize {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            }
            event::Event::EthereumContractEvent {
                contract_address,
                events,
            } => {
                // Filter by contract address if specified
                if let Some(filter_address) =
                    filter.get("contract_address").and_then(|a| a.as_str())
                {
                    if !contract_address.eq_ignore_ascii_case(filter_address) {
                        return false;
                    }
                }

                // Filter by event topic if specified
                if let Some(topic) = filter.get("topic").and_then(|t| t.as_str()) {
                    if !events.iter().any(|event| {
                        event
                            .get("topics")
                            .and_then(|topics| topics.as_array())
                            .map_or(false, |topics| {
                                topics
                                    .iter()
                                    .any(|t| t.as_str().map_or(false, |s| s == topic))
                            })
                    }) {
                        return false;
                    }
                }

                true
            }
            event::Event::EthereumTransaction(tx) => {
                // Filter by from address if specified
                if let Some(from) = filter.get("from").and_then(|f| f.as_str()) {
                    if let Some(tx_from) = tx.get("from").and_then(|f| f.as_str()) {
                        if !tx_from.eq_ignore_ascii_case(from) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by to address if specified
                if let Some(to) = filter.get("to").and_then(|t| t.as_str()) {
                    if let Some(tx_to) = tx.get("to").and_then(|t| t.as_str()) {
                        if !tx_to.eq_ignore_ascii_case(to) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by value if specified
                if let Some(min_value) = filter.get("min_value").and_then(|v| v.as_str()) {
                    if let Some(tx_value) = tx.get("value").and_then(|v| v.as_str()) {
                        // Convert hex strings to numbers for comparison
                        let min_value_num =
                            u64::from_str_radix(&min_value.trim_start_matches("0x"), 16)
                                .unwrap_or(0);
                        let tx_value_num =
                            u64::from_str_radix(&tx_value.trim_start_matches("0x"), 16)
                                .unwrap_or(0);

                        if tx_value_num < min_value_num {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            }
            _ => true, // Include all other event types
        }
    }
}

#[async_trait]
impl TaskSource for EthereumTaskSource {
    async fn acquire_task(&mut self, uid: u64, fid: u64) -> Result<Task, TaskError> {
        // Sleep to avoid busy waiting
        tokio::time::sleep(self.sleep).await;

        // Create an event based on the current trigger
        let event = match self.current_trigger {
            Trigger::EthereumNewBlock => {
                // Fetch the latest block
                let block = match self.fetch_latest_block().await {
                    Ok(block) => block,
                    Err(e) => {
                        return Err(TaskError::Error(format!(
                            "Failed to fetch latest block: {}",
                            e
                        )));
                    }
                };

                // Update trigger for next time
                self.current_trigger = Trigger::EthereumContractEvent;

                // Return block event
                event::Event::EthereumBlock(block)
            }
            Trigger::EthereumContractEvent => {
                // Use a mock contract address
                let contract_address = "0x4e65fda2159562a496f9f3522f89122a3088497a";

                // Fetch contract events
                let events = match self.fetch_contract_events(contract_address).await {
                    Ok(events) => events,
                    Err(e) => {
                        return Err(TaskError::Error(format!(
                            "Failed to fetch contract events: {}",
                            e
                        )));
                    }
                };

                // Update trigger for next time
                self.current_trigger = Trigger::EthereumTransaction;

                // Return contract event
                event::Event::EthereumContractEvent {
                    contract_address: contract_address.to_string(),
                    events,
                }
            }
            Trigger::EthereumTransaction => {
                // Use a mock transaction hash
                let tx_hash = "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b";

                // Fetch transaction
                let transaction = match self.fetch_transaction(tx_hash).await {
                    Ok(tx) => tx,
                    Err(e) => {
                        return Err(TaskError::Error(format!(
                            "Failed to fetch transaction: {}",
                            e
                        )));
                    }
                };

                // Update trigger for next time
                self.current_trigger = Trigger::EthereumNewBlock;

                // Return transaction event
                event::Event::EthereumTransaction(transaction)
            }
        };

        // Apply filter if provided
        if !self.filter_event(&event, self.filter.as_ref()) {
            // If the event doesn't match the filter, return a default event
            return Ok(Task::new(uid, fid, event::Event::None));
        }

        // Create a task with the event
        Ok(Task::new(uid, fid, event))
    }

    async fn acquire_fn(&mut self, _uid: u64, _fid: u64) -> Result<Func, FuncError> {
        let code = format!(
            r#"
            // Ethereum event handler
            export default function(event) {{
                console.log("Ethereum event handler called");
                
                if (event.block) {{
                    // This is an Ethereum block event
                    console.log("Ethereum block event detected");
                    console.log("Block number:", event.block.number);
                    console.log("Block hash:", event.block.hash);
                    console.log("Block timestamp:", event.block.timestamp);
                    console.log("Transaction count:", event.block.transactions.length);
                    
                    // Process transactions
                    for (let i = 0; i < Math.min(event.block.transactions.length, 5); i++) {{
                        const tx = event.block.transactions[i];
                        console.log(`Transaction ${{i}} hash:`, tx.hash);
                        console.log(`Transaction ${{i}} from:`, tx.from);
                        console.log(`Transaction ${{i}} to:`, tx.to);
                        console.log(`Transaction ${{i}} value:`, tx.value);
                    }}
                }} else if (event.contract_address) {{
                    // This is an Ethereum contract event
                    console.log("Ethereum contract event detected");
                    console.log("Contract address:", event.contract_address);
                    console.log("Events count:", event.events.length);
                    
                    // Process events
                    for (let i = 0; i < Math.min(event.events.length, 5); i++) {{
                        const evt = event.events[i];
                        console.log(`Event ${{i}} topics:`, evt.topics);
                        console.log(`Event ${{i}} data:`, evt.data);
                        console.log(`Event ${{i}} block number:`, evt.blockNumber);
                    }}
                }} else if (event.hash) {{
                    // This is an Ethereum transaction event
                    console.log("Ethereum transaction event detected");
                    console.log("Transaction hash:", event.hash);
                    console.log("Transaction from:", event.from);
                    console.log("Transaction to:", event.to);
                    console.log("Transaction value:", event.value);
                    console.log("Transaction gas:", event.gas);
                    console.log("Transaction gas price:", event.gasPrice);
                }} else {{
                    // Unknown event type
                    console.log("Unknown Ethereum event type:", event);
                }}
                
                return {{
                    status: "success",
                    message: "Ethereum event processed successfully",
                    timestamp: new Date().toISOString(),
                    event_type: event.block ? "block" : event.contract_address ? "contract_event" : event.hash ? "transaction" : "unknown"
                }};
            }}
            "#
        );

        Ok(Func { code, version: 1 })
    }
}
