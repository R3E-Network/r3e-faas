// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration loading functionality.

use std::path::Path;
use config::{Config, ConfigBuilder, Environment, File};

use crate::error::{Error, Result};
use crate::types::FaasConfig;

/// Configuration loader
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<FaasConfig> {
        let config_str = std::fs::read_to_string(path)?;
        
        if config_str.trim().starts_with('{') {
            // JSON format
            serde_json::from_str(&config_str).map_err(Error::JsonParsing)
        } else {
            // YAML format
            serde_yaml::from_str(&config_str).map_err(Error::YamlParsing)
        }
    }
    
    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<FaasConfig> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("R3E_FAAS").separator("__"))
            .build()?;
        
        config.try_deserialize().map_err(Error::Config)
    }
    
    /// Load configuration from multiple sources with precedence:
    /// 1. Environment variables
    /// 2. Config file
    /// 3. Default values
    pub fn load(config_path: Option<&str>) -> Result<FaasConfig> {
        let mut builder = Config::builder()
            .set_default("general.environment", "development")?
            .set_default("general.instance_id", uuid::Uuid::new_v4().to_string())?
            .set_default("general.data_dir", "./data")?;
        
        // Add config file if provided
        if let Some(path) = config_path {
            builder = builder.add_source(File::with_name(path));
        }
        
        // Add environment variables with highest precedence
        builder = builder.add_source(Environment::with_prefix("R3E_FAAS").separator("__"));
        
        // Build and deserialize
        let config = builder.build()?;
        let faas_config: FaasConfig = config.try_deserialize()?;
        
        Ok(faas_config)
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(config: &FaasConfig, path: P, format: ConfigFormat) -> Result<()> {
        let config_str = match format {
            ConfigFormat::Json => serde_json::to_string_pretty(config)?,
            ConfigFormat::Yaml => serde_yaml::to_string(config)?,
        };
        
        std::fs::write(path, config_str)?;
        
        Ok(())
    }
}

/// Configuration format
pub enum ConfigFormat {
    /// JSON format
    Json,
    
    /// YAML format
    Yaml,
}
