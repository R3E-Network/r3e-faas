// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::trigger::integration::TriggerEvaluator;
use crate::trigger::types::{TriggerCondition, TriggerError, TriggerSource};

/// Mock trigger evaluator
pub struct MockTriggerEvaluator {
    /// Should match
    pub should_match: bool,
}

impl MockTriggerEvaluator {
    /// Create a new mock trigger evaluator
    pub fn new() -> Self {
        Self { should_match: true }
    }

    /// Set should match
    pub fn with_should_match(mut self, should_match: bool) -> Self {
        self.should_match = should_match;
        self
    }
}

#[async_trait]
impl TriggerEvaluator for MockTriggerEvaluator {
    async fn evaluate_trigger(
        &self,
        _condition: &TriggerCondition,
        _event_data: &Value,
    ) -> Result<bool, TriggerError> {
        Ok(self.should_match)
    }
}

/// Mock trigger service
pub struct MockTriggerService {
    /// Should succeed
    pub should_succeed: bool,
}

impl MockTriggerService {
    /// Create a new mock trigger service
    pub fn new() -> Self {
        Self {
            should_succeed: true,
        }
    }

    /// Set should succeed
    pub fn with_should_succeed(mut self, should_succeed: bool) -> Self {
        self.should_succeed = should_succeed;
        self
    }

    /// Process an event
    pub async fn process_event(
        &self,
        _event_data: &Value,
        _evaluator: &dyn TriggerEvaluator,
    ) -> Result<Vec<Uuid>, String> {
        if self.should_succeed {
            Ok(vec![Uuid::new_v4()])
        } else {
            Err("Mock trigger service error".to_string())
        }
    }
}
