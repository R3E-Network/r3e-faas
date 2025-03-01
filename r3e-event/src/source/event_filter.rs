// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::source::event;

/// Event filter for filtering events from task sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// Network filter
    pub network: Option<String>,

    /// Event type filter
    pub event_type: Option<String>,

    /// Contract address filter
    pub contract_address: Option<String>,

    /// Event name filter
    pub event_name: Option<String>,

    /// Method name filter
    pub method_name: Option<String>,

    /// Block number filter
    pub min_block: Option<u64>,

    /// Transaction hash filter
    pub tx_hash: Option<String>,

    /// From address filter
    pub from: Option<String>,

    /// To address filter
    pub to: Option<String>,

    /// Value filter
    pub min_value: Option<u64>,

    /// Custom filter
    pub custom: Option<Value>,
}

impl EventFilter {
    /// Create a new event filter
    pub fn new() -> Self {
        Self {
            network: None,
            event_type: None,
            contract_address: None,
            event_name: None,
            method_name: None,
            min_block: None,
            tx_hash: None,
            from: None,
            to: None,
            min_value: None,
            custom: None,
        }
    }

    /// Set network filter
    pub fn with_network(mut self, network: &str) -> Self {
        self.network = Some(network.to_string());
        self
    }

    /// Set event type filter
    pub fn with_event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_string());
        self
    }

    /// Set contract address filter
    pub fn with_contract_address(mut self, contract_address: &str) -> Self {
        self.contract_address = Some(contract_address.to_string());
        self
    }

    /// Set event name filter
    pub fn with_event_name(mut self, event_name: &str) -> Self {
        self.event_name = Some(event_name.to_string());
        self
    }

    /// Set method name filter
    pub fn with_method_name(mut self, method_name: &str) -> Self {
        self.method_name = Some(method_name.to_string());
        self
    }

    /// Set minimum block number filter
    pub fn with_min_block(mut self, min_block: u64) -> Self {
        self.min_block = Some(min_block);
        self
    }

    /// Set transaction hash filter
    pub fn with_tx_hash(mut self, tx_hash: &str) -> Self {
        self.tx_hash = Some(tx_hash.to_string());
        self
    }

    /// Set from address filter
    pub fn with_from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    /// Set to address filter
    pub fn with_to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }

    /// Set minimum value filter
    pub fn with_min_value(mut self, min_value: u64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Set custom filter
    pub fn with_custom(mut self, custom: Value) -> Self {
        self.custom = Some(custom);
        self
    }

    /// Apply filter to an event
    pub fn apply(&self, event: &event::Event) -> bool {
        match event {
            event::Event::None => false,
            event::Event::NeoBlock(block) => self.filter_neo_block(block),
            event::Event::NeoTransaction(tx) => self.filter_neo_transaction(tx),
            event::Event::NeoContractEvent {
                contract_address,
                events,
            } => self.filter_neo_contract_event(contract_address, events),
            event::Event::EthereumBlock(block) => self.filter_ethereum_block(block),
            event::Event::EthereumTransaction(tx) => self.filter_ethereum_transaction(tx),
            event::Event::EthereumContractEvent {
                contract_address,
                events,
            } => self.filter_ethereum_contract_event(contract_address, events),
            event::Event::Custom(data) => self.filter_custom(data),
        }
    }

    /// Filter Neo block
    fn filter_neo_block(&self, block: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "neo" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "block" {
                return false;
            }
        }

        // Check block number
        if let Some(min_block) = self.min_block {
            if let Some(block_number) = block.get("index").and_then(|n| n.as_u64()) {
                if block_number < min_block {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter Neo transaction
    fn filter_neo_transaction(&self, tx: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "neo" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "transaction" {
                return false;
            }
        }

        // Check transaction hash
        if let Some(tx_hash) = &self.tx_hash {
            if let Some(hash) = tx.get("hash").and_then(|h| h.as_str()) {
                if hash != tx_hash {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check from address
        if let Some(from) = &self.from {
            if let Some(sender) = tx.get("sender").and_then(|s| s.as_str()) {
                if sender != from {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter Neo contract event
    fn filter_neo_contract_event(&self, contract_address: &str, events: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "neo" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "contract_event" {
                return false;
            }
        }

        // Check contract address
        if let Some(address) = &self.contract_address {
            if contract_address != address {
                return false;
            }
        }

        // Check event name
        if let Some(event_name) = &self.event_name {
            if let Some(events_array) = events.as_array() {
                if !events_array.iter().any(|event| {
                    event
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map_or(false, |name| name == event_name)
                }) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter Ethereum block
    fn filter_ethereum_block(&self, block: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "ethereum" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "block" {
                return false;
            }
        }

        // Check block number
        if let Some(min_block) = self.min_block {
            if let Some(block_number) = block.get("number").and_then(|n| n.as_str()) {
                // Convert hex string to number
                let block_num =
                    u64::from_str_radix(&block_number.trim_start_matches("0x"), 16).unwrap_or(0);
                if block_num < min_block {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter Ethereum transaction
    fn filter_ethereum_transaction(&self, tx: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "ethereum" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "transaction" {
                return false;
            }
        }

        // Check transaction hash
        if let Some(tx_hash) = &self.tx_hash {
            if let Some(hash) = tx.get("hash").and_then(|h| h.as_str()) {
                if hash != tx_hash {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check from address
        if let Some(from) = &self.from {
            if let Some(sender) = tx.get("from").and_then(|s| s.as_str()) {
                if !sender.eq_ignore_ascii_case(from) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check to address
        if let Some(to) = &self.to {
            if let Some(recipient) = tx.get("to").and_then(|t| t.as_str()) {
                if !recipient.eq_ignore_ascii_case(to) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check value
        if let Some(min_value) = self.min_value {
            if let Some(value) = tx.get("value").and_then(|v| v.as_str()) {
                // Convert hex string to number
                let value_num =
                    u64::from_str_radix(&value.trim_start_matches("0x"), 16).unwrap_or(0);
                if value_num < min_value {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter Ethereum contract event
    fn filter_ethereum_contract_event(&self, contract_address: &str, events: &Value) -> bool {
        // Check network
        if let Some(network) = &self.network {
            if network != "ethereum" {
                return false;
            }
        }

        // Check event type
        if let Some(event_type) = &self.event_type {
            if event_type != "contract_event" {
                return false;
            }
        }

        // Check contract address
        if let Some(address) = &self.contract_address {
            if !contract_address.eq_ignore_ascii_case(address) {
                return false;
            }
        }

        // Check event name (topic)
        if let Some(event_name) = &self.event_name {
            if let Some(events_array) = events.as_array() {
                if !events_array.iter().any(|event| {
                    event
                        .get("topics")
                        .and_then(|topics| topics.as_array())
                        .map_or(false, |topics| {
                            if topics.is_empty() {
                                return false;
                            }

                            // The first topic is the event signature
                            topics[0]
                                .as_str()
                                .map_or(false, |topic| topic == event_name)
                        })
                }) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Filter custom event
    fn filter_custom(&self, data: &Value) -> bool {
        // Check custom filter
        if let Some(custom) = &self.custom {
            // Check if custom is a subset of data
            if let (Some(custom_obj), Some(data_obj)) = (custom.as_object(), data.as_object()) {
                for (key, value) in custom_obj {
                    if !data_obj.contains_key(key) || data_obj[key] != *value {
                        return false;
                    }
                }
            } else if let (Some(custom_arr), Some(data_arr)) = (custom.as_array(), data.as_array())
            {
                for value in custom_arr {
                    if !data_arr.contains(value) {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_neo_block_filter() {
        let block = json!({
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

        // Test network filter
        let filter = EventFilter::new().with_network("neo");
        assert!(filter.filter_neo_block(&block));

        let filter = EventFilter::new().with_network("ethereum");
        assert!(!filter.filter_neo_block(&block));

        // Test event type filter
        let filter = EventFilter::new().with_event_type("block");
        assert!(filter.filter_neo_block(&block));

        let filter = EventFilter::new().with_event_type("transaction");
        assert!(!filter.filter_neo_block(&block));

        // Test block number filter
        let filter = EventFilter::new().with_min_block(12000);
        assert!(filter.filter_neo_block(&block));

        let filter = EventFilter::new().with_min_block(13000);
        assert!(!filter.filter_neo_block(&block));
    }

    #[test]
    fn test_ethereum_block_filter() {
        let block = json!({
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

        // Test network filter
        let filter = EventFilter::new().with_network("ethereum");
        assert!(filter.filter_ethereum_block(&block));

        let filter = EventFilter::new().with_network("neo");
        assert!(!filter.filter_ethereum_block(&block));

        // Test event type filter
        let filter = EventFilter::new().with_event_type("block");
        assert!(filter.filter_ethereum_block(&block));

        let filter = EventFilter::new().with_event_type("transaction");
        assert!(!filter.filter_ethereum_block(&block));

        // Test block number filter
        let filter = EventFilter::new().with_min_block(0x100);
        assert!(filter.filter_ethereum_block(&block));

        let filter = EventFilter::new().with_min_block(0x200);
        assert!(!filter.filter_ethereum_block(&block));
    }

    #[test]
    fn test_ethereum_transaction_filter() {
        let tx = json!({
            "hash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
            "nonce": "0x0",
            "blockHash": "0xbeab0aa2411b7ab17f30a99d3cb9c6ef2fc5426d6ad6fd9e2a26a6aed1d1055b",
            "blockNumber": "0x15df",
            "transactionIndex": "0x1",
            "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "to": "0x85h43d8a49eeb85d32cf465507dd71d507100c1",
            "value": "0x7f110",
            "gas": "0x7f110",
            "gasPrice": "0x09184e72a000",
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360"
        });

        // Test network filter
        let filter = EventFilter::new().with_network("ethereum");
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_network("neo");
        assert!(!filter.filter_ethereum_transaction(&tx));

        // Test event type filter
        let filter = EventFilter::new().with_event_type("transaction");
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_event_type("block");
        assert!(!filter.filter_ethereum_transaction(&tx));

        // Test transaction hash filter
        let filter = EventFilter::new()
            .with_tx_hash("0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b");
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_tx_hash("0x1234567890abcdef");
        assert!(!filter.filter_ethereum_transaction(&tx));

        // Test from address filter
        let filter = EventFilter::new().with_from("0x407d73d8a49eeb85d32cf465507dd71d507100c1");
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_from("0x1234567890abcdef");
        assert!(!filter.filter_ethereum_transaction(&tx));

        // Test to address filter
        let filter = EventFilter::new().with_to("0x85h43d8a49eeb85d32cf465507dd71d507100c1");
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_to("0x1234567890abcdef");
        assert!(!filter.filter_ethereum_transaction(&tx));

        // Test value filter
        let filter = EventFilter::new().with_min_value(0x7f100);
        assert!(filter.filter_ethereum_transaction(&tx));

        let filter = EventFilter::new().with_min_value(0x7f200);
        assert!(!filter.filter_ethereum_transaction(&tx));
    }
}
