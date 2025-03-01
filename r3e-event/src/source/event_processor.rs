// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::source::{event, Task, TaskSource};
use crate::trigger::{TriggerEvaluator, TriggerServiceIntegration};

/// Event processor for handling events from task sources
pub struct EventProcessor {
    /// Task source
    task_source: Arc<RwLock<dyn TaskSource>>,

    /// Trigger service integration
    trigger_service: Arc<TriggerServiceIntegration>,

    /// Trigger evaluator
    trigger_evaluator: Arc<dyn TriggerEvaluator>,

    /// User ID
    user_id: u64,

    /// Function ID
    function_id: u64,

    /// Processing interval
    processing_interval: Duration,

    /// Is running
    is_running: Arc<RwLock<bool>>,
}

impl EventProcessor {
    /// Create a new event processor
    pub fn new(
        task_source: Arc<RwLock<dyn TaskSource>>,
        trigger_service: Arc<TriggerServiceIntegration>,
        trigger_evaluator: Arc<dyn TriggerEvaluator>,
        user_id: u64,
        function_id: u64,
    ) -> Self {
        Self {
            task_source,
            trigger_service,
            trigger_evaluator,
            user_id,
            function_id,
            processing_interval: Duration::from_secs(1),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Set the processing interval
    pub fn with_processing_interval(mut self, interval: Duration) -> Self {
        self.processing_interval = interval;
        self
    }

    /// Start processing events
    pub async fn start(&self) -> Result<(), String> {
        // Check if already running
        let mut is_running = self.is_running.write().await;

        if *is_running {
            return Err("Event processor is already running".to_string());
        }

        // Set running flag
        *is_running = true;

        // Clone the running flag for the processing task
        let is_running_clone = self.is_running.clone();

        // Clone the task source for the processing task
        let task_source_clone = self.task_source.clone();

        // Clone the trigger service for the processing task
        let trigger_service_clone = self.trigger_service.clone();

        // Clone the trigger evaluator for the processing task
        let trigger_evaluator_clone = self.trigger_evaluator.clone();

        // Clone the user ID and function ID for the processing task
        let user_id = self.user_id;
        let function_id = self.function_id;

        // Clone the processing interval for the processing task
        let processing_interval = self.processing_interval;

        // Spawn a task to process events
        tokio::spawn(async move {
            info!(
                "Event processor started for user {} and function {}",
                user_id, function_id
            );

            // Process events until stopped
            while *is_running_clone.read().await {
                // Acquire a task
                match task_source_clone
                    .write()
                    .await
                    .acquire_task(user_id, function_id)
                    .await
                {
                    Ok(task) => {
                        // Process the task
                        if let Err(e) = Self::process_task(
                            &task,
                            &trigger_service_clone,
                            &trigger_evaluator_clone,
                        )
                        .await
                        {
                            error!("Failed to process task: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to acquire task: {}", e);
                    }
                }

                // Sleep for the processing interval
                tokio::time::sleep(processing_interval).await;
            }

            info!(
                "Event processor stopped for user {} and function {}",
                user_id, function_id
            );
        });

        Ok(())
    }

    /// Stop processing events
    pub async fn stop(&self) -> Result<(), String> {
        // Check if running
        let mut is_running = self.is_running.write().await;

        if !*is_running {
            return Err("Event processor is not running".to_string());
        }

        // Clear running flag
        *is_running = false;

        Ok(())
    }

    /// Check if the processor is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get the user ID
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    /// Get the function ID
    pub fn function_id(&self) -> u64 {
        self.function_id
    }

    /// Process a task
    async fn process_task(
        task: &Task,
        trigger_service: &Arc<TriggerServiceIntegration>,
        trigger_evaluator: &Arc<dyn TriggerEvaluator>,
    ) -> Result<(), String> {
        // Convert the event to a JSON value
        let event_data = match task.event {
            event::Event::None => {
                // Skip empty events
                return Ok(());
            }
            event::Event::NeoBlock(ref block) => {
                json!({
                    "network": "neo",
                    "event_type": "block",
                    "block": block,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::NeoTransaction(ref tx) => {
                json!({
                    "network": "neo",
                    "event_type": "transaction",
                    "transaction": tx,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::NeoContractEvent {
                ref contract_address,
                ref events,
            } => {
                json!({
                    "network": "neo",
                    "event_type": "contract_event",
                    "contract_address": contract_address,
                    "events": events,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::EthereumBlock(ref block) => {
                json!({
                    "network": "ethereum",
                    "event_type": "block",
                    "block": block,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::EthereumTransaction(ref tx) => {
                json!({
                    "network": "ethereum",
                    "event_type": "transaction",
                    "transaction": tx,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::EthereumContractEvent {
                ref contract_address,
                ref events,
            } => {
                json!({
                    "network": "ethereum",
                    "event_type": "contract_event",
                    "contract_address": contract_address,
                    "events": events,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
            event::Event::Custom(ref data) => {
                json!({
                    "event_type": "custom",
                    "data": data,
                    "timestamp": chrono::Utc::now().timestamp(),
                })
            }
        };

        // Process the event
        match trigger_service
            .process_event(&event_data, trigger_evaluator.as_ref())
            .await
        {
            Ok(callback_ids) => {
                if !callback_ids.is_empty() {
                    info!("Processed event with {} callbacks", callback_ids.len());
                    debug!("Callback IDs: {:?}", callback_ids);
                } else {
                    debug!("Processed event with no callbacks");
                }

                Ok(())
            }
            Err(e) => Err(format!("Failed to process event: {}", e)),
        }
    }
}
