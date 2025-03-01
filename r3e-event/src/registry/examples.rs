// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Examples of function registration for different use cases

use crate::registry::proto::{
    FunctionMetadata, Permissions, RegisterFunctionRequest, Resources, TriggerConfig,
};

/// Create a Neo block event handler function registration request
pub fn create_neo_block_handler() -> RegisterFunctionRequest {
    RegisterFunctionRequest {
        name: "Neo Block Handler".to_string(),
        description: "Handles new Neo N3 blockchain blocks".to_string(),
        trigger: Some(TriggerConfig {
            trigger_type: "blockchain".to_string(),
            config: serde_json::json!({
                "source": "Neo",
                "event_type": "NeoNewBlock",
                "filter": ""
            }),
        }),
        permissions: Some(Permissions {
            network: true,
            filesystem: true,
            environment: false,
        }),
        resources: Some(Resources {
            memory_mb: 128,
            cpu_units: 1000,
            timeout_ms: 5000,
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
            trigger_type: "blockchain".to_string(),
            config: serde_json::json!({
                "source": "Neo",
                "event_type": "NeoNewTx",
                "filter": ""
            }),
        }),
        permissions: Some(Permissions {
            network: true,
            filesystem: true,
            environment: false,
        }),
        resources: Some(Resources {
            memory_mb: 128,
            cpu_units: 1000,
            timeout_ms: 5000,
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
            trigger_type: "blockchain".to_string(),
            config: serde_json::json!({
                "source": "Neo",
                "event_type": "NeoContractNotification",
                "filter": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5" // Example NEP-17 token contract
            }),
        }),
        permissions: Some(Permissions {
            network: true,
            filesystem: true,
            environment: false,
        }),
        resources: Some(Resources {
            memory_mb: 128,
            cpu_units: 1000,
            timeout_ms: 5000,
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
            trigger_type: "oracle".to_string(),
            config: serde_json::json!({
                "type": "price",
                "assets": ["NEO", "GAS"],
                "providers": ["coinmarketcap", "coingecko"]
            }),
        }),
        permissions: Some(Permissions {
            network: true,
            filesystem: true,
            environment: true,
        }),
        resources: Some(Resources {
            memory_mb: 128,
            cpu_units: 1000,
            timeout_ms: 10000,
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
            trigger_type: "http".to_string(),
            config: serde_json::json!({
                "path": "/api/tee/compute",
                "methods": ["POST"],
                "auth_required": true
            }),
        }),
        permissions: Some(Permissions {
            network: false,
            filesystem: true,
            environment: true,
        }),
        resources: Some(Resources {
            memory_mb: 256,
            cpu_units: 2000,
            timeout_ms: 15000,
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

/// Get the price of a cryptocurrency from CoinMarketCap
pub fn get_price(parameters: &serde_json::Value) -> Result<serde_json::Value, String> {
    use reqwest::blocking::Client;
    use serde_json::json;
    use std::env;
    
    // Get the symbol from parameters
    let symbol = match parameters.get("symbol") {
        Some(serde_json::Value::String(s)) => s.to_uppercase(),
        _ => return Err("Missing or invalid 'symbol' parameter".to_string()),
    };
    
    // Get the convert currency from parameters (default to USD)
    let convert = match parameters.get("convert") {
        Some(serde_json::Value::String(s)) => s.to_uppercase(),
        _ => "USD".to_string(),
    };
    
    // Get API key from environment or use a fallback mechanism
    let api_key = env::var("COINMARKETCAP_API_KEY").unwrap_or_else(|_| {
        // Fallback to a config file if environment variable is not set
        match std::fs::read_to_string("config/api_keys.json") {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(config) => {
                        config.get("coinmarketcap")
                              .and_then(|v| v.as_str())
                              .unwrap_or("").to_string()
                    },
                    Err(_) => "".to_string(),
                }
            },
            Err(_) => "".to_string(),
        }
    });
    
    if api_key.is_empty() {
        // Fallback to mock data if no API key is available
        log::warn!("No CoinMarketCap API key found, using mock data");
        return mock_price_data(&symbol, &convert);
    }
    
    // Create the client
    let client = Client::new();
    
    // Build the URL
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}&convert={}",
        symbol, convert
    );
    
    // Make the request
    let response = match client.get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .header("Accept", "application/json")
        .send() {
            Ok(res) => res,
            Err(e) => return Err(format!("API request failed: {}", e)),
        };
    
    // Check for successful status code
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()));
    }
    
    // Parse the response
    let response_data: serde_json::Value = match response.json() {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to parse API response: {}", e)),
    };
    
    // Extract the price data
    let price = response_data.get("data")
        .and_then(|data| data.get(&symbol))
        .and_then(|symbol_data| symbol_data.get("quote"))
        .and_then(|quote| quote.get(&convert))
        .and_then(|convert_data| convert_data.get("price"))
        .and_then(|price| price.as_f64());
    
    let price = match price {
        Some(p) => p,
        None => return Err(format!("Failed to extract price from API response")),
    };
    
    // Extract additional data
    let percent_change_24h = response_data.get("data")
        .and_then(|data| data.get(&symbol))
        .and_then(|symbol_data| symbol_data.get("quote"))
        .and_then(|quote| quote.get(&convert))
        .and_then(|convert_data| convert_data.get("percent_change_24h"))
        .and_then(|change| change.as_f64())
        .unwrap_or(0.0);
    
    let market_cap = response_data.get("data")
        .and_then(|data| data.get(&symbol))
        .and_then(|symbol_data| symbol_data.get("quote"))
        .and_then(|quote| quote.get(&convert))
        .and_then(|convert_data| convert_data.get("market_cap"))
        .and_then(|cap| cap.as_f64())
        .unwrap_or(0.0);
    
    let volume_24h = response_data.get("data")
        .and_then(|data| data.get(&symbol))
        .and_then(|symbol_data| symbol_data.get("quote"))
        .and_then(|quote| quote.get(&convert))
        .and_then(|convert_data| convert_data.get("volume_24h"))
        .and_then(|volume| volume.as_f64())
        .unwrap_or(0.0);
    
    let timestamp = response_data.get("status")
        .and_then(|status| status.get("timestamp"))
        .and_then(|ts| ts.as_str())
        .unwrap_or("");
    
    // Construct the response
    Ok(json!({
        "symbol": symbol,
        "convert": convert,
        "price": price,
        "percent_change_24h": percent_change_24h,
        "market_cap": market_cap,
        "volume_24h": volume_24h,
        "timestamp": timestamp,
        "source": "CoinMarketCap API"
    }))
}

/// Generate mock price data if API key is not available
fn mock_price_data(symbol: &str, convert: &str) -> Result<serde_json::Value, String> {
    use serde_json::json;
    use chrono::Utc;
    
    // Mock prices for common cryptocurrencies in USD
    let price = match symbol {
        "BTC" => 45000.0,
        "ETH" => 2500.0,
        "BNB" => 350.0,
        "SOL" => 100.0,
        "ADA" => 1.2,
        "XRP" => 0.5,
        "DOT" => 10.0,
        "DOGE" => 0.15,
        "AVAX" => 25.0,
        "SHIB" => 0.00003,
        _ => 1.0, // Default value for unknown symbols
    };
    
    // Apply a simple conversion for non-USD prices
    let price = match convert {
        "USD" => price,
        "EUR" => price * 0.85, // Rough EUR conversion
        "JPY" => price * 110.0, // Rough JPY conversion
        "GBP" => price * 0.75, // Rough GBP conversion
        _ => price, // Default to USD for unknown conversions
    };
    
    // Generate mock additional data
    let percent_change_24h = -0.5 + (rand::random::<f64>() * 5.0); // Random between -0.5% and +4.5%
    let market_cap = price * 1_000_000.0 * (1.0 + rand::random::<f64>() * 10.0);
    let volume_24h = price * 100_000.0 * (1.0 + rand::random::<f64>() * 5.0);
    
    // Current timestamp
    let timestamp = Utc::now().to_rfc3339();
    
    // Construct the mock response
    Ok(json!({
        "symbol": symbol,
        "convert": convert,
        "price": price,
        "percent_change_24h": percent_change_24h,
        "market_cap": market_cap,
        "volume_24h": volume_24h,
        "timestamp": timestamp,
        "source": "Mock Data (No API Key)"
    }))
}
