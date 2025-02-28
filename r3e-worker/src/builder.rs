// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use r3e_event::source::{
    neo::NeoTaskSource, 
    ethereum::EthereumTaskSource,
    mock::MockTaskSource,
    TaskSource,
};

use crate::TaskConfig;

pub struct TaskSourceBuilder {
    config: TaskConfig,
}

impl TaskSourceBuilder {
    pub fn new(config: TaskConfig) -> Self {
        Self { config }
    }

    pub fn build(&self) -> Box<dyn TaskSource> {
        let sleep = Duration::from_millis(self.config.sleep_ms);
        let uid = 0;
        
        // Create the appropriate task source based on the configuration
        match self.config.source_type.as_str() {
            "neo" => {
                let source = NeoTaskSource::new(sleep, uid);
                
                // Configure the source with RPC URL if provided
                let source = if let Some(rpc_url) = &self.config.rpc_url {
                    source.with_rpc_url(rpc_url)
                } else {
                    source
                };
                
                // Configure the source with filter if provided
                let source = if let Some(filter) = &self.config.filter {
                    source.with_filter(filter.clone())
                } else {
                    source
                };
                
                Box::new(source)
            },
            "ethereum" => {
                // Create an Ethereum task source
                log::info!("Creating Ethereum task source");
                let ethereum_source = EthereumTaskSource::new(
                    config.ethereum_rpc_url.clone(),
                    config.ethereum_chain_id,
                    config.ethereum_block_time,
                    config.ethereum_confirmations,
                )?;
                Box::new(ethereum_source)
                log::warn!("Using placeholder Ethereum task source");
                
                let source = EthereumTaskSource::new(sleep, uid);
                
                // Configure the source with RPC URL if provided
                let source = if let Some(rpc_url) = &self.config.rpc_url {
                    source.with_rpc_url(rpc_url)
                } else {
                    source
                };
                
                // Configure the source with filter if provided
                let source = if let Some(filter) = &self.config.filter {
                    source.with_filter(filter.clone())
                } else {
                    source
                };
                
                Box::new(source)
            },
            "mock" => {
                // Create a mock task source for testing
                Box::new(MockTaskSource::new(sleep, uid))
            },
            _ => {
                // Default to mock task source
                log::warn!("Unknown task source type: {}, using mock task source", self.config.source_type);
                Box::new(MockTaskSource::new(sleep, uid))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let config = TaskConfig::default();
        let config = serde_json::to_string_pretty(&config).unwrap();
        println!("config: \n{}", config);
    }
}
