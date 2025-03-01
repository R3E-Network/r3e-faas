// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::function::{FunctionContext, FunctionResult};
use crate::function_executor::{ExecutorConfig, FunctionExecutor};
use crate::metrics::WorkerMetrics;

/// Worker pool for executing functions
pub struct WorkerPool {
    /// Number of workers in the pool
    workers: usize,

    /// Worker handles
    worker_handles: Vec<JoinHandle<()>>,

    /// Function request sender
    request_tx: mpsc::Sender<FunctionRequest>,

    /// Function result receivers
    result_rxs: Arc<Mutex<HashMap<String, mpsc::Receiver<FunctionResult>>>>,

    /// Worker metrics
    metrics: Arc<WorkerMetrics>,
}

/// Function request
#[derive(Debug)]
struct FunctionRequest {
    /// Function ID
    function_id: String,

    /// Function code
    code: String,

    /// Function context
    context: FunctionContext,

    /// Result sender
    result_tx: mpsc::Sender<FunctionResult>,
}

impl WorkerPool {
    /// Create a new worker pool
    pub fn new(workers: usize, executor_config: ExecutorConfig) -> Self {
        let (request_tx, request_rx) = mpsc::channel(100);
        let request_rx = Arc::new(Mutex::new(request_rx));
        let result_rxs = Arc::new(Mutex::new(HashMap::new()));
        let metrics = Arc::new(WorkerMetrics::new());

        let mut worker_handles = Vec::with_capacity(workers);

        for worker_id in 0..workers {
            let request_rx = Arc::clone(&request_rx);
            let metrics = Arc::clone(&metrics);
            let executor_config = executor_config.clone();

            let handle = tokio::spawn(async move {
                let executor = FunctionExecutor::new(executor_config);
                Self::worker_loop(worker_id, executor, request_rx, metrics).await;
            });

            worker_handles.push(handle);
        }

        Self {
            workers,
            worker_handles,
            request_tx,
            result_rxs,
            metrics,
        }
    }

    /// Execute a function
    pub async fn execute_function(
        &self,
        function_id: &str,
        code: &str,
        context: FunctionContext,
    ) -> mpsc::Receiver<FunctionResult> {
        let (result_tx, result_rx) = mpsc::channel(1);

        let request = FunctionRequest {
            function_id: function_id.to_string(),
            code: code.to_string(),
            context,
            result_tx,
        };

        // Store the receiver
        let request_id = Uuid::new_v4().to_string();
        {
            let mut result_rxs = self.result_rxs.lock().unwrap();
            result_rxs.insert(request_id.clone(), result_rx);
        }

        // Send the request
        if let Err(e) = self.request_tx.send(request).await {
            error!("Failed to send function request: {}", e);

            // Create a new channel to return the error
            let (error_tx, error_rx) = mpsc::channel(1);
            let _ = error_tx
                .send(FunctionResult::Error(format!(
                    "Failed to queue function: {}",
                    e
                )))
                .await;
            return error_rx;
        }

        // Return the receiver
        let mut result_rxs = self.result_rxs.lock().unwrap();
        result_rxs.remove(&request_id).unwrap()
    }

    /// Worker loop
    async fn worker_loop(
        worker_id: usize,
        executor: FunctionExecutor,
        request_rx: Arc<Mutex<mpsc::Receiver<FunctionRequest>>>,
        metrics: Arc<WorkerMetrics>,
    ) {
        info!("Worker {} started", worker_id);

        loop {
            // Get the next request
            let request = {
                let mut rx = request_rx.lock().unwrap();
                match rx.try_recv() {
                    Ok(request) => request,
                    Err(_) => {
                        // No request available, sleep and try again
                        drop(rx);
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        continue;
                    }
                }
            };

            debug!(
                "Worker {} executing function {}",
                worker_id, request.function_id
            );

            // Update metrics
            metrics.increment_active_functions();
            metrics.increment_total_functions();

            // Execute the function
            let start_time = std::time::Instant::now();
            let result = executor.execute(&request.function_id, &request.code, &request.context);
            let execution_time = start_time.elapsed();

            // Update metrics
            metrics.decrement_active_functions();
            metrics.record_execution_time(execution_time);

            // Send the result
            if let Err(e) = request.result_tx.send(result).await {
                error!("Failed to send function result: {}", e);
            }
        }
    }

    /// Get worker metrics
    pub fn metrics(&self) -> Arc<WorkerMetrics> {
        Arc::clone(&self.metrics)
    }
}
