use crate::neo_task_source::NeoTaskSource;
// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use serde::{Deserialize, Serialize};

use r3e_event::source::{MockTaskSource, TaskSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventSource {
    #[serde(rename = "mock")]
    Mock,
    #[serde(rename = "neo")]
    Neo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub source: EventSource,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self { source: EventSource::Mock }
    }
}

pub struct TaskSourceBuilder {
    config: TaskConfig,
}

impl TaskSourceBuilder {
    pub fn new(config: TaskConfig) -> Self {
        Self { config }
    }

    // TODO: add more task acquirers
    pub fn build(&self) -> Box<dyn TaskSource> {
        match self.config.source {
            EventSource::Mock => Box::new(MockTaskSource::new(Duration::from_secs(2), 1)),
            EventSource::Neo => Box::new(NeoTaskSource::new(Duration::from_secs(2), 1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let config = TaskConfig::default();
        let config = serde_yaml::to_string(&config).unwrap();
        std::println!("config: \n{}", config);
    }
}
