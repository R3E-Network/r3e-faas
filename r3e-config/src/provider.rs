// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration provider.

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::types::FaasConfig;

/// Configuration provider
pub struct ConfigProvider {
    /// Configuration
    config: Arc<RwLock<FaasConfig>>,
}

impl ConfigProvider {
    /// Create a new configuration provider
    pub fn new(config: FaasConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// Get a reference to the configuration
    pub async fn get_config(&self) -> FaasConfig {
        self.config.read().await.clone()
    }

    /// Update the configuration
    pub async fn update_config(&self, config: FaasConfig) {
        let mut config_lock = self.config.write().await;
        *config_lock = config;
    }

    /// Get a specific configuration value
    pub async fn get<T, F>(&self, getter: F) -> T
    where
        F: FnOnce(&FaasConfig) -> T,
        T: Clone,
    {
        let config = self.config.read().await;
        getter(&config)
    }

    /// Update a specific configuration value
    pub async fn update<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut FaasConfig) -> Result<()>,
    {
        let mut config = self.config.write().await;
        updater(&mut config)
    }
}

/// Create a global configuration provider
pub fn create_global_provider(config: FaasConfig) -> Arc<ConfigProvider> {
    Arc::new(ConfigProvider::new(config))
}

/// Global configuration provider
static mut GLOBAL_PROVIDER: Option<Arc<ConfigProvider>> = None;

/// Initialize the global configuration provider
pub fn init_global_provider(config: FaasConfig) {
    unsafe {
        GLOBAL_PROVIDER = Some(create_global_provider(config));
    }
}

/// Get the global configuration provider
pub fn get_global_provider() -> Result<Arc<ConfigProvider>> {
    unsafe {
        GLOBAL_PROVIDER.clone().ok_or_else(|| {
            Error::MissingConfig("Global configuration provider not initialized".to_string())
        })
    }
}
