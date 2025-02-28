// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod assign;
pub mod builder;
pub mod runner;
pub mod worker;
pub mod neo_task_source;
pub mod sandbox;
pub mod function;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[allow(unused_imports)]
use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};

pub use {assign::*, builder::*, runner::*, worker::*, sandbox::*};

pub const MAX_RUNNERS: u32 = 1024;

lazy_static::lazy_static! {
    pub static ref NUM_CPUS: u32 = num_cpus::get() as u32;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub sleep_ms: u64,
    pub source_type: String,
    pub rpc_url: Option<String>,
    pub filter: Option<serde_json::Value>,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            sleep_ms: 1000,
            source_type: "neo".to_string(),
            rpc_url: None,
            filter: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    #[serde(deserialize_with = "deserialize_duration")]
    pub graceful: Duration,

    pub max_pending: u32,
    pub max_runners: u32,
    pub max_runtimes_per_runner: u32,
    pub tasks: TaskConfig,
    pub sandbox: SandboxConfig,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            graceful: Duration::from_secs(2),
            max_pending: 128,
            max_runners: *NUM_CPUS * 2,
            max_runtimes_per_runner: 16,
            tasks: TaskConfig::default(),
            sandbox: SandboxConfig::default(),
        }
    }
}

impl WorkerConfig {
    #[inline]
    pub fn max_runners(&self) -> u32 {
        if self.max_runners == 0 {
            return (*NUM_CPUS * 2).min(MAX_RUNNERS);
        }
        self.max_runners.min(MAX_RUNNERS)
    }
}

pub trait Stopper {
    fn stopped(&self) -> bool;

    fn stop(&self);
}

impl Stopper for Arc<AtomicBool> {
    #[inline]
    fn stopped(&self) -> bool {
        self.load(Ordering::Relaxed)
    }

    #[inline]
    fn stop(&self) {
        self.store(true, Ordering::Relaxed);
    }
}
