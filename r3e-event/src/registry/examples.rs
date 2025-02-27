// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Examples of function registration for different use cases

use crate::registry::{
    BlockchainPermissions, BlockchainTrigger, FunctionMetadata, NetworkPermissions,
    PermissionConfig, RegisterFunctionRequest, ResourceLimits, StoragePermissions,
    TriggerConfig, TriggerType,
};

/// Create a Neo block event handler function registration request
pub fn create_neo_block_handler() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo Block Handler".to_string(),
        description: "Handles new Neo N3 blockchain blocks".to_string(),
        trigger: Some(TriggerConfig {
            r#type: TriggerType::TriggerTypeBlockchain as i32,
            config: Some(
                crate::registry::trigger_config::Config::Blockchain(BlockchainTrigger {
                    source: "Neo".to_string(),
                    event_type: "NeoNewBlock".to_string(),
                    filter: "".to_string(),
                }),
            ),
        }),
        permissions: Some(PermissionConfig {
            network: Some(NetworkPermissions {
                allow_outbound: true,
                allowed_domains: vec!["api.neoscan.io".to_string()],
            }),
            storage: Some(StoragePermissions {
                allow_read: true,
                allow_write: true,
                namespace: "neo_blocks".to_string(),
            }),
            blockchain: Some(BlockchainPermissions {
                allow_read: true,
                allow_write: false,
                allowed_contracts: vec![],
            }),
        }),
        resources: Some(ResourceLimits {
            memory_mb: 128,
            cpu_ms: 1000,
            execution_time_ms: 5000,
            storage_kb: 1024,
        }),
        code: r#"
// Neo Block Handler Function
import { Neo } from "r3e";

export default async function(event) {
  console.log("Received Neo block:", event);
  
  // Connect to Neo blockchain
  const client = Neo.createClient({ url: "https://testnet1.neo.org:443" });
  
  // Process block data
  const block = event.neo_block;
  const blockHeight = block.header.height;
  const blockHash = block.header.hash;
  const timestamp = block.header.time;
  
  console.log(`Processing Neo block ${blockHeight} (${blockHash}) at ${new Date(timestamp * 1000)}`);
  
  // Count transactions in block
  const txCount = block.txs.length;
  console.log(`Block contains ${txCount} transactions`);
  
  // Return block summary
  return {
    height: blockHeight,
    hash: blockHash,
    timestamp: timestamp,
    tx_count: txCount
  };
}
"#.to_string(),
    }
}

/// Create a Neo transaction event handler function registration request
pub fn create_neo_tx_handler() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo Transaction Handler".to_string(),
        description: "Handles new Neo N3 blockchain transactions".to_string(),
        trigger: Some(TriggerConfig {
            r#type: TriggerType::TriggerTypeBlockchain as i32,
            config: Some(
                crate::registry::trigger_config::Config::Blockchain(BlockchainTrigger {
                    source: "Neo".to_string(),
                    event_type: "NeoNewTx".to_string(),
                    filter: "".to_string(),
                }),
            ),
        }),
        permissions: Some(PermissionConfig {
            network: Some(NetworkPermissions {
                allow_outbound: true,
                allowed_domains: vec!["api.neoscan.io".to_string()],
            }),
            storage: Some(StoragePermissions {
                allow_read: true,
                allow_write: true,
                namespace: "neo_txs".to_string(),
            }),
            blockchain: Some(BlockchainPermissions {
                allow_read: true,
                allow_write: false,
                allowed_contracts: vec![],
            }),
        }),
        resources: Some(ResourceLimits {
            memory_mb: 128,
            cpu_ms: 1000,
            execution_time_ms: 5000,
            storage_kb: 1024,
        }),
        code: r#"
// Neo Transaction Handler Function
import { Neo } from "r3e";

export default async function(event) {
  console.log("Received Neo transaction:", event);
  
  // Connect to Neo blockchain
  const client = Neo.createClient({ url: "https://testnet1.neo.org:443" });
  
  // Process transaction data
  const tx = event.neo_tx;
  const txHash = tx.hash;
  const sender = tx.signers[0]?.account || "unknown";
  const sysFee = tx.sysfee;
  const netFee = tx.netfee;
  
  console.log(`Processing Neo transaction ${txHash} from ${sender}`);
  console.log(`Fees: ${sysFee} (system) + ${netFee} (network)`);
  
  // Analyze transaction script
  const script = tx.script;
  console.log(`Script length: ${script.length} bytes`);
  
  // Return transaction summary
  return {
    hash: txHash,
    sender: sender,
    sys_fee: sysFee,
    net_fee: netFee,
    script_size: script.length
  };
}
"#.to_string(),
    }
}

/// Create a Neo contract notification handler function registration request
pub fn create_neo_contract_notification_handler() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo Contract Notification Handler".to_string(),
        description: "Handles Neo N3 smart contract notifications".to_string(),
        trigger: Some(TriggerConfig {
            r#type: TriggerType::TriggerTypeBlockchain as i32,
            config: Some(
                crate::registry::trigger_config::Config::Blockchain(BlockchainTrigger {
                    source: "Neo".to_string(),
                    event_type: "NeoContractNotification".to_string(),
                    filter: "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5".to_string(), // Example NEP-17 token contract
                }),
            ),
        }),
        permissions: Some(PermissionConfig {
            network: Some(NetworkPermissions {
                allow_outbound: true,
                allowed_domains: vec!["api.neoscan.io".to_string()],
            }),
            storage: Some(StoragePermissions {
                allow_read: true,
                allow_write: true,
                namespace: "neo_notifications".to_string(),
            }),
            blockchain: Some(BlockchainPermissions {
                allow_read: true,
                allow_write: false,
                allowed_contracts: vec!["0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5".to_string()],
            }),
        }),
        resources: Some(ResourceLimits {
            memory_mb: 128,
            cpu_ms: 1000,
            execution_time_ms: 5000,
            storage_kb: 1024,
        }),
        code: r#"
// Neo Contract Notification Handler Function
import { Neo } from "r3e";

export default async function(event) {
  console.log("Received Neo contract notification:", event);
  
  // Connect to Neo blockchain
  const client = Neo.createClient({ url: "https://testnet1.neo.org:443" });
  
  // Process notification data
  const notification = event.neo_notification;
  const contractHash = notification.contract_hash;
  const eventName = notification.event_name;
  const state = notification.state;
  
  console.log(`Processing Neo contract notification from ${contractHash}`);
  console.log(`Event: ${eventName}`);
  
  // Handle NEP-17 Transfer events
  if (eventName === "Transfer") {
    const from = state[0]?.value;
    const to = state[1]?.value;
    const amount = state[2]?.value;
    
    console.log(`Transfer: ${from} -> ${to}: ${amount}`);
    
    // Return transfer details
    return {
      contract: contractHash,
      event: eventName,
      from: from,
      to: to,
      amount: amount
    };
  }
  
  // Return general notification details
  return {
    contract: contractHash,
    event: eventName,
    state: state
  };
}
"#.to_string(),
    }
}

/// Create a Neo oracle service function registration request
pub fn create_neo_oracle_service() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo Price Oracle".to_string(),
        description: "Provides price data for Neo N3 blockchain".to_string(),
        trigger: Some(TriggerConfig {
            r#type: TriggerType::TriggerTypeOracle as i32,
            config: Some(
                crate::registry::trigger_config::Config::Oracle(
                    crate::registry::OracleTrigger {
                        r#type: "price".to_string(),
                        config: r#"{"assets": ["NEO", "GAS"], "providers": ["coinmarketcap", "coingecko"]}"#.to_string(),
                    },
                ),
            ),
        }),
        permissions: Some(PermissionConfig {
            network: Some(NetworkPermissions {
                allow_outbound: true,
                allowed_domains: vec![
                    "api.coinmarketcap.com".to_string(),
                    "api.coingecko.com".to_string(),
                ],
            }),
            storage: Some(StoragePermissions {
                allow_read: true,
                allow_write: true,
                namespace: "neo_oracle".to_string(),
            }),
            blockchain: Some(BlockchainPermissions {
                allow_read: true,
                allow_write: true,
                allowed_contracts: vec![],
            }),
        }),
        resources: Some(ResourceLimits {
            memory_mb: 128,
            cpu_ms: 1000,
            execution_time_ms: 10000,
            storage_kb: 1024,
        }),
        code: r#"
// Neo Price Oracle Function
import { Neo } from "r3e";

// Fetch price data from CoinGecko
async function fetchPriceFromCoinGecko(asset) {
  const response = await fetch(`https://api.coingecko.com/api/v3/simple/price?ids=${asset.toLowerCase()}&vs_currencies=usd`);
  const data = await response.json();
  return data[asset.toLowerCase()]?.usd || null;
}

// Fetch price data from CoinMarketCap (mock implementation)
async function fetchPriceFromCoinMarketCap(asset) {
  // In a real implementation, this would use the CoinMarketCap API
  // For this example, we'll return mock data
  const mockPrices = {
    'neo': 12.34,
    'gas': 5.67
  };
  
  return mockPrices[asset.toLowerCase()] || null;
}

export default async function(request) {
  console.log("Received oracle request:", request);
  
  // Parse the request
  const { asset, requestId } = request;
  
  if (!asset) {
    return {
      status: "error",
      message: "Missing asset parameter",
      requestId
    };
  }
  
  console.log(`Processing price oracle request for ${asset}`);
  
  // Fetch prices from multiple sources
  const prices = [];
  
  try {
    const geckoPrice = await fetchPriceFromCoinGecko(asset);
    if (geckoPrice !== null) {
      prices.push({ source: "coingecko", price: geckoPrice });
    }
  } catch (error) {
    console.error("Error fetching from CoinGecko:", error);
  }
  
  try {
    const cmcPrice = await fetchPriceFromCoinMarketCap(asset);
    if (cmcPrice !== null) {
      prices.push({ source: "coinmarketcap", price: cmcPrice });
    }
  } catch (error) {
    console.error("Error fetching from CoinMarketCap:", error);
  }
  
  if (prices.length === 0) {
    return {
      status: "error",
      message: "Failed to fetch price data from any source",
      requestId
    };
  }
  
  // Calculate the average price
  const totalPrice = prices.reduce((sum, item) => sum + item.price, 0);
  const averagePrice = totalPrice / prices.length;
  
  // Return the price data
  return {
    status: "success",
    asset: asset,
    price: averagePrice,
    sources: prices,
    timestamp: Date.now(),
    requestId
  };
}
"#.to_string(),
    }
}

/// Create a Neo TEE computing service function registration request
pub fn create_neo_tee_service() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo TEE Computation Service".to_string(),
        description: "Provides secure computation services using Trusted Execution Environment".to_string(),
        trigger: Some(TriggerConfig {
            r#type: TriggerType::TriggerTypeHttp as i32,
            config: Some(
                crate::registry::trigger_config::Config::Http(
                    crate::registry::HttpTrigger {
                        path: "/api/tee/compute".to_string(),
                        methods: vec!["POST".to_string()],
                        auth_required: true,
                    },
                ),
            ),
        }),
        permissions: Some(PermissionConfig {
            network: Some(NetworkPermissions {
                allow_outbound: false,
                allowed_domains: vec![],
            }),
            storage: Some(StoragePermissions {
                allow_read: true,
                allow_write: true,
                namespace: "neo_tee".to_string(),
            }),
            blockchain: Some(BlockchainPermissions {
                allow_read: true,
                allow_write: true,
                allowed_contracts: vec![],
            }),
        }),
        resources: Some(ResourceLimits {
            memory_mb: 256,
            cpu_ms: 2000,
            execution_time_ms: 15000,
            storage_kb: 2048,
        }),
        code: r#"
// Neo TEE Computation Service
import { Neo } from "r3e";

// Simulate TEE environment
const isTEE = true;
const attestationReport = {
  platform: "SGX",
  version: "2.0",
  mrenclave: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  mrsigner: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
  isDebug: false
};

// Verify the TEE environment
function verifyTEE() {
  if (!isTEE) {
    throw new Error("This function must run in a TEE environment");
  }
  return attestationReport;
}

// Perform secure computation
function secureCompute(data, operation) {
  // In a real TEE, this would be executed in the secure enclave
  console.log("Performing secure computation in TEE");
  
  switch (operation) {
    case "sign":
      // Simulate secure signing operation
      return {
        operation: "sign",
        result: `0x${Array.from(crypto.getRandomValues(new Uint8Array(64)))
          .map(b => b.toString(16).padStart(2, '0'))
          .join('')}`
      };
      
    case "encrypt":
      // Simulate secure encryption operation
      return {
        operation: "encrypt",
        result: `0x${Array.from(crypto.getRandomValues(new Uint8Array(32)))
          .map(b => b.toString(16).padStart(2, '0'))
          .join('')}`
      };
      
    case "decrypt":
      // Simulate secure decryption operation
      return {
        operation: "decrypt",
        result: "decrypted_data_would_be_here"
      };
      
    default:
      throw new Error(`Unsupported operation: ${operation}`);
  }
}

export default async function(request) {
  console.log("Received TEE computation request:", request);
  
  try {
    // Verify TEE environment
    const attestation = verifyTEE();
    
    // Parse the request
    const { data, operation, requestId } = request.body || {};
    
    if (!data || !operation) {
      return {
        status: "error",
        message: "Missing required parameters: data and operation",
        requestId
      };
    }
    
    console.log(`Processing TEE computation request: ${operation}`);
    
    // Perform the secure computation
    const result = secureCompute(data, operation);
    
    // Return the result with attestation
    return {
      status: "success",
      result: result,
      attestation: attestation,
      timestamp: Date.now(),
      requestId
    };
  } catch (error) {
    console.error("TEE computation error:", error);
    
    return {
      status: "error",
      message: error.message,
      requestId: request.body?.requestId
    };
  }
}
"#.to_string(),
    }
}
