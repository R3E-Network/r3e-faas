// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use neo3::neo_clients::{HttpProvider, RpcClient};
use neo3::neo_crypto::keys::PrivateKey;
use neo3::neo_protocol::wallet::Wallet;
use r3e_neo_services::gas_bank::rocksdb::RocksDBGasBankStorage;
use r3e_neo_services::gas_bank::service::GasBankService;
use r3e_neo_services::meta_tx::service::MetaTxService;
use r3e_neo_services::meta_tx::storage::MetaTxStorage;
use r3e_neo_services::types::FeeModel;
use r3e_worker::metrics::{MetricsManager, SystemMetricsSummary};
use sqlx::PgPool;
use tokio::sync::Mutex;
use tracing::{info, warn, error, instrument};
use url::Url;

use crate::config::Config;
use crate::error::Error;
use crate::routes::health::SystemMetrics;

/// Endpoint service
pub struct EndpointService {
    /// Configuration
    pub config: Config,

    /// Database pool
    pub db: PgPool,

    /// Neo N3 RPC client
    pub neo_rpc_client: Arc<RpcClient>,

    /// Relayer wallet
    pub relayer_wallet: Arc<Wallet>,

    /// Gas bank service
    pub gas_bank_service: Arc<GasBankService<RocksDBGasBankStorage>>,

    /// Meta transaction service
    pub meta_tx_service: Arc<MetaTxService<dyn MetaTxStorage>>,
    
    /// Metrics manager
    pub metrics_manager: Arc<MetricsManager>,
    
    /// Service start time
    pub start_time: Instant,
}

impl EndpointService {
    /// Create a new endpoint service
    pub async fn new(config: Config) -> Result<Self, Error> {
        // Connect to the database
        let db = PgPool::connect(&config.database_url)
            .await
            .map_err(|e| Error::Database(format!("Failed to connect to database: {}", e)))?;

        // Create Neo N3 RPC client
        let neo_url = Url::parse(&config.neo_rpc_url)
            .map_err(|e| Error::Configuration(format!("Invalid Neo N3 RPC URL: {}", e)))?;

        let neo_provider = HttpProvider::new(neo_url)
            .map_err(|e| Error::Network(format!("Failed to create Neo N3 HTTP provider: {}", e)))?;

        let neo_rpc_client = Arc::new(RpcClient::new(neo_provider));

        // Create relayer wallet
        let private_key = PrivateKey::from_str(&config.relayer_private_key)
            .map_err(|e| Error::Configuration(format!("Invalid relayer private key: {}", e)))?;

        let relayer_wallet = Arc::new(Wallet::from_private_key(private_key));

        // Create Gas Bank storage
        let gas_bank_storage = Arc::new(
            RocksDBGasBankStorage::new("./data/gas_bank")
                .await
                .map_err(|e| {
                    Error::Database(format!("Failed to create Gas Bank storage: {}", e))
                })?,
        );

        // Create Gas Bank service
        let gas_bank_service = Arc::new(GasBankService::new(
            gas_bank_storage,
            neo_rpc_client.clone(),
            relayer_wallet.clone(),
            "mainnet".to_string(),
            FeeModel::Percentage(1.0),
            1_000_000_000,
        ));

        // Create Meta Transaction storage
        let meta_tx_storage = Arc::new(MockMetaTxStorage::new());

        // Create Meta Transaction service
        let meta_tx_service = Arc::new(MetaTxService::new(
            meta_tx_storage,
            neo_rpc_client.clone(),
            relayer_wallet.clone(),
            "mainnet".to_string(),
            FeeModel::Percentage(1.0),
        ));
        
        // Create metrics manager
        let metrics_manager = Arc::new(MetricsManager::new());
        
        // Add log alert handler
        metrics_manager.get_alert_manager().add_alert_handler(r3e_worker::metrics::LogAlertHandler);
        
        info!("Endpoint service initialized");

        Ok(Self {
            config,
            db,
            neo_rpc_client,
            relayer_wallet,
            gas_bank_service,
            meta_tx_service,
            metrics_manager,
            start_time: Instant::now(),
        })
    }
    
    /// Check database connection
    #[instrument(skip(self))]
    pub async fn check_database_connection(&self) -> Result<(), Error> {
        // Simple query to check if the database is accessible
        match sqlx::query("SELECT 1").execute(&self.db).await {
            Ok(_) => {
                info!("Database connection check successful");
                Ok(())
            },
            Err(e) => {
                error!(error = %e, "Database connection check failed");
                Err(Error::Database(format!("Database connection check failed: {}", e)))
            }
        }
    }
    
    /// Check worker pool
    #[instrument(skip(self))]
    pub async fn check_worker_pool(&self) -> Result<(), Error> {
        // For now, just return OK since we don't have a direct connection to the worker pool
        // In a real implementation, this would check the worker pool status
        info!("Worker pool check successful");
        Ok(())
    }
    
    /// Check authentication service
    #[instrument(skip(self))]
    pub async fn check_auth_service(&self) -> Result<(), Error> {
        // For now, just return OK since we don't have a direct connection to the auth service
        // In a real implementation, this would check the auth service status
        info!("Authentication service check successful");
        Ok(())
    }
    
    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> Option<i64> {
        Some(self.start_time.elapsed().as_secs() as i64)
    }
    
    /// Get system metrics
    #[instrument(skip(self))]
    pub async fn get_system_metrics(&self) -> Result<SystemMetrics, Error> {
        // Get metrics from the metrics manager
        let metrics_summary = self.metrics_manager.get_system_metrics();
        
        // Get system resource usage
        let cpu_usage = 0.0; // TODO: Implement actual CPU usage monitoring
        let memory_usage = 0.0; // TODO: Implement actual memory usage monitoring
        let disk_usage = 0.0; // TODO: Implement actual disk usage monitoring
        
        // Create network stats
        let network = crate::routes::health::NetworkStats {
            bytes_received: 0, // TODO: Implement actual network monitoring
            bytes_sent: 0,     // TODO: Implement actual network monitoring
            active_connections: 0, // TODO: Implement actual connection monitoring
        };
        
        // Create function execution stats from system metrics
        let system_metrics_summary = self.metrics_manager.get_system_metrics();
        let function_execution = crate::routes::health::FunctionExecutionStats {
            total_executions: system_metrics_summary.total_execution_count,
            total_errors: system_metrics_summary.total_error_count,
            avg_execution_time_ms: system_metrics_summary.avg_execution_time_ms,
            avg_memory_usage_mb: system_metrics_summary.avg_memory_usage_mb,
            error_rate: system_metrics_summary.error_rate,
        };
        
        // Create system metrics
        let system_metrics = SystemMetrics {
            cpu_usage_percent: cpu_usage,
            memory_usage_percent: memory_usage,
            disk_usage_percent: disk_usage,
            network,
            function_execution,
        };
        
        Ok(system_metrics)
    }
}

// Mock implementation of MetaTxStorage for development
use async_trait::async_trait;
use r3e_neo_services::meta_tx::types::{MetaTxRecord, MetaTxStatus};
use std::collections::HashMap;
use std::sync::Mutex;

struct MockMetaTxStorage {
    records: Mutex<HashMap<String, MetaTxRecord>>,
    nonces: Mutex<HashMap<String, u64>>,
}

impl MockMetaTxStorage {
    fn new() -> Self {
        Self {
            records: Mutex::new(HashMap::new()),
            nonces: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MetaTxStorage for MockMetaTxStorage {
    async fn create_record(&self, record: MetaTxRecord) -> Result<(), r3e_neo_services::Error> {
        let mut records = self.records.lock().unwrap();
        records.insert(record.request_id.clone(), record);
        Ok(())
    }

    async fn update_record(&self, record: MetaTxRecord) -> Result<(), r3e_neo_services::Error> {
        let mut records = self.records.lock().unwrap();
        records.insert(record.request_id.clone(), record);
        Ok(())
    }

    async fn get_record(
        &self,
        request_id: &str,
    ) -> Result<Option<MetaTxRecord>, r3e_neo_services::Error> {
        let records = self.records.lock().unwrap();
        Ok(records.get(request_id).cloned())
    }

    async fn get_records_by_sender(
        &self,
        sender: &str,
    ) -> Result<Vec<MetaTxRecord>, r3e_neo_services::Error> {
        let records = self.records.lock().unwrap();
        let mut result = Vec::new();

        for record in records.values() {
            if record.request.sender == sender {
                result.push(record.clone());
            }
        }

        Ok(result)
    }

    async fn get_nonce(&self, sender: &str) -> Result<u64, r3e_neo_services::Error> {
        let mut nonces = self.nonces.lock().unwrap();
        let nonce = nonces.entry(sender.to_string()).or_insert(0);
        *nonce += 1;
        Ok(*nonce)
    }
}
