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

pub struct NeoTaskSource {
    sleep: Duration,
    uid: u64,
    count: u64,
    client: Arc<Mutex<Option<RpcClient<HttpProvider>>>>,
    rpc_url: String,
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
            }).collect()
        } else {
            vec![]
        };
        
        Ok(NeoBlock {
            header,
            txs,
        })
    }
}

#[async_trait::async_trait]
impl TaskSource for NeoTaskSource {
    async fn acquire_task(&mut self, _uid: u64, _fid_hint: u64) -> Result<Task, TaskError> {
        tokio::time::sleep(self.sleep).await;

        self.count += 1;
        let fid = 1 + (self.count & 1);
        
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
        
        // Create the event with the correct type
        let event = event::Event::NeoBlock(neo_block);

        Ok(Task::new(self.uid, fid, event))
    }

    async fn acquire_fn(&mut self, _uid: u64, _fid: u64) -> Result<Func, FuncError> {
        let code = format!(
            r#"
            const delay = (n) => new Promise(r => setTimeout(r, n));

            export default async function(event) {{
                console.log("Neo event handler:", event);
                await delay(2100);
                
                // Access Neo block data
                if (event.header) {{
                    console.log("Neo block height:", event.header.height);
                    console.log("Neo block hash:", event.header.hash);
                    console.log("Neo block time:", event.header.time);
                }}
                
                // Process transactions if available
                if (event.txs && event.txs.length > 0) {{
                    console.log("Neo transactions count:", event.txs.length);
                    for (let i = 0; i < Math.min(event.txs.length, 5); i++) {{
                        console.log(`Transaction ${{i}} hash:`, event.txs[i].hash);
                    }}
                }}
            }}"#
        );
        Ok(Func {
            version: 1,
            code: code.into(),
        })
    }
}
