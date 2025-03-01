// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use log::{debug, error, info, warn};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::source::event_processor::EventProcessor;
use crate::source::{event, Task, TaskSource};
use crate::trigger::{TriggerEvaluator, TriggerServiceIntegration};

/// Event processor service for managing multiple event processors
pub struct EventProcessorService {
    /// Task sources
    task_sources: Arc<RwLock<HashMap<String, Arc<RwLock<dyn TaskSource>>>>>,

    /// Trigger service integration
    trigger_service: Arc<TriggerServiceIntegration>,

    /// Trigger evaluator
    trigger_evaluator: Arc<dyn TriggerEvaluator>,

    /// Event processors
    processors: Arc<RwLock<HashMap<String, Arc<EventProcessor>>>>,
}

impl EventProcessorService {
    /// Create a new event processor service
    pub fn new(
        trigger_service: Arc<TriggerServiceIntegration>,
        trigger_evaluator: Arc<dyn TriggerEvaluator>,
    ) -> Self {
        Self {
            task_sources: Arc::new(RwLock::new(HashMap::new())),
            trigger_service,
            trigger_evaluator,
            processors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a task source
    pub async fn register_task_source(
        &self,
        name: &str,
        task_source: Arc<RwLock<dyn TaskSource>>,
    ) -> Result<(), String> {
        let mut task_sources = self.task_sources.write().await;

        if task_sources.contains_key(name) {
            return Err(format!("Task source already registered: {}", name));
        }

        task_sources.insert(name.to_string(), task_source);

        Ok(())
    }

    /// Unregister a task source
    pub async fn unregister_task_source(&self, name: &str) -> Result<(), String> {
        let mut task_sources = self.task_sources.write().await;

        if !task_sources.contains_key(name) {
            return Err(format!("Task source not registered: {}", name));
        }

        task_sources.remove(name);

        Ok(())
    }

    /// Create an event processor
    pub async fn create_processor(
        &self,
        task_source_name: &str,
        user_id: u64,
        function_id: u64,
    ) -> Result<String, String> {
        // Get the task source
        let task_sources = self.task_sources.read().await;
        let task_source = task_sources
            .get(task_source_name)
            .cloned()
            .ok_or_else(|| format!("Task source not registered: {}", task_source_name))?;

        // Create a processor ID
        let processor_id = Uuid::new_v4().to_string();

        // Create the processor
        let processor = Arc::new(EventProcessor::new(
            task_source,
            self.trigger_service.clone(),
            self.trigger_evaluator.clone(),
            user_id,
            function_id,
        ));

        // Add the processor to the list
        let mut processors = self.processors.write().await;
        processors.insert(processor_id.clone(), processor);

        Ok(processor_id)
    }

    /// Start an event processor
    pub async fn start_processor(&self, processor_id: &str) -> Result<(), String> {
        // Get the processor
        let processors = self.processors.read().await;
        let processor = processors
            .get(processor_id)
            .cloned()
            .ok_or_else(|| format!("Processor not found: {}", processor_id))?;

        // Start the processor
        processor.start().await
    }

    /// Stop an event processor
    pub async fn stop_processor(&self, processor_id: &str) -> Result<(), String> {
        // Get the processor
        let processors = self.processors.read().await;
        let processor = processors
            .get(processor_id)
            .cloned()
            .ok_or_else(|| format!("Processor not found: {}", processor_id))?;

        // Stop the processor
        processor.stop().await
    }

    /// Delete an event processor
    pub async fn delete_processor(&self, processor_id: &str) -> Result<(), String> {
        // Get the processor
        let mut processors = self.processors.write().await;
        let processor = processors
            .get(processor_id)
            .cloned()
            .ok_or_else(|| format!("Processor not found: {}", processor_id))?;

        // Stop the processor
        processor.stop().await?;

        // Remove the processor from the list
        processors.remove(processor_id);

        Ok(())
    }

    /// List all processors
    pub async fn list_processors(&self) -> Vec<String> {
        let processors = self.processors.read().await;
        processors.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::mock::MockTaskSource;
    use crate::trigger::mock::MockTriggerEvaluator;
    use crate::trigger::mock::MockTriggerService;

    #[tokio::test]
    async fn test_event_processor_service() {
        // Create a mock trigger service
        let trigger_service = Arc::new(MockTriggerService::new());

        // Create a mock trigger evaluator
        let trigger_evaluator = Arc::new(MockTriggerEvaluator::new());

        // Create an event processor service
        let service =
            EventProcessorService::new(trigger_service.clone(), trigger_evaluator.clone());

        // Create a mock task source
        let task_source = Arc::new(RwLock::new(MockTaskSource::new()));

        // Register the task source
        service
            .register_task_source("mock", task_source.clone())
            .await
            .unwrap();

        // Create a processor
        let processor_id = service.create_processor("mock", 1, 1).await.unwrap();

        // Start the processor
        service.start_processor(&processor_id).await.unwrap();

        // Wait for a bit
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Stop the processor
        service.stop_processor(&processor_id).await.unwrap();

        // Delete the processor
        service.delete_processor(&processor_id).await.unwrap();

        // Unregister the task source
        service.unregister_task_source("mock").await.unwrap();
    }
}
