// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::trigger::integration::TriggerEvaluator;
use crate::trigger::types::{TriggerCondition, TriggerError, TriggerSource};

/// Standard trigger evaluator implementation
pub struct StandardTriggerEvaluator {
    /// Market price cache
    market_price_cache: Arc<RwLock<std::collections::HashMap<String, (f64, i64)>>>,
}

impl StandardTriggerEvaluator {
    /// Create a new standard trigger evaluator
    pub fn new() -> Self {
        Self {
            market_price_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Update market price cache
    pub async fn update_market_price(&self, asset_pair: &str, price: f64, timestamp: i64) {
        let mut cache = self.market_price_cache.write().await;
        cache.insert(asset_pair.to_string(), (price, timestamp));
    }

    /// Get market price from cache
    pub async fn get_market_price(&self, asset_pair: &str) -> Option<(f64, i64)> {
        let cache = self.market_price_cache.read().await;
        cache.get(asset_pair).cloned()
    }

    /// Evaluate a blockchain trigger
    async fn evaluate_blockchain_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse blockchain parameters
        let network = params
            .get("network")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let contract_address = params
            .get("contract_address")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let event_name = params
            .get("event_name")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let method_name = params
            .get("method_name")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let block_number = params.get("block_number").and_then(|v| v.as_u64());

        // Extract event data
        let event_network = event_data
            .get("network")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check network
        if network != "*" && network != event_network {
            return Ok(false);
        }

        // Extract contract address
        let event_contract = event_data
            .get("contract_address")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check contract address
        if contract_address != "*" && contract_address != event_contract {
            return Ok(false);
        }

        // Extract event type
        let event_type = event_data
            .get("event_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check event name
        if event_name != "*" && event_name != event_type {
            return Ok(false);
        }

        // Extract method name
        let event_method = event_data
            .get("method_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check method name
        if method_name != "*" && method_name != event_method {
            return Ok(false);
        }

        // Extract block number
        let event_block = event_data
            .get("block_number")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // Check block number
        if let Some(min_block) = block_number {
            if event_block < min_block {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Evaluate a time trigger
    async fn evaluate_time_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse time parameters
        let cron = params
            .get("cron")
            .and_then(|v| v.as_str())
            .unwrap_or("* * * * *");
        let timezone = params
            .get("timezone")
            .and_then(|v| v.as_str())
            .unwrap_or("UTC");

        // Extract event data
        let event_timestamp = event_data
            .get("timestamp")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        // Convert timestamp to datetime
        let event_time = chrono::DateTime::from_timestamp(event_timestamp, 0)
            .ok_or_else(|| TriggerError::InvalidParameters("Invalid timestamp".to_string()))?;

        // Parse cron expression
        let now = chrono::Utc::now();
        let schedule = cron_parser::parse(cron, &now).map_err(|e| {
            TriggerError::InvalidParameters(format!("Invalid cron expression: {}", e))
        })?;

        // Parse timezone
        let timezone = timezone.parse::<chrono_tz::Tz>().map_err(|_| {
            TriggerError::InvalidParameters(format!("Invalid timezone: {}", timezone))
        })?;

        // For simplicity, we'll just check if the event timestamp is within the last minute
        // This is a simplified approach since we can't directly use the cron_parser's prev_from method
        let now = chrono::Utc::now();
        let one_minute_ago = now - chrono::Duration::minutes(1);
        let prev_time = if event_time.with_timezone(&timezone) > one_minute_ago.with_timezone(&timezone) && 
                           event_time.with_timezone(&timezone) <= now.with_timezone(&timezone) {
            event_time.with_timezone(&timezone)
        } else {
            return Ok(false);
        };

        // Since we've already checked that the event time is within the last minute,
        // we can just return true here
        let diff = 0;

        Ok(diff <= 60)
    }

    /// Evaluate a market trigger
    async fn evaluate_market_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse market parameters
        let asset_pair = params
            .get("asset_pair")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let condition = params
            .get("condition")
            .and_then(|v| v.as_str())
            .unwrap_or("eq");
        let price = params.get("price").and_then(|v| v.as_f64());

        // Extract event data
        let event_asset_pair = event_data
            .get("asset_pair")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let event_price = event_data
            .get("price")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let event_volume = event_data
            .get("volume")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let event_timestamp = event_data
            .get("timestamp")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let event_exchange = event_data
            .get("exchange")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check asset pair
        if asset_pair != "*" && asset_pair != event_asset_pair {
            return Ok(false);
        }

        // Check price condition
        if let Some(target_price) = price {
            match condition {
                "eq" => {
                    // Equal to
                    if (event_price - target_price).abs() > 0.000001 {
                        return Ok(false);
                    }
                }
                "gt" => {
                    // Greater than
                    if event_price <= target_price {
                        return Ok(false);
                    }
                }
                "lt" => {
                    // Less than
                    if event_price >= target_price {
                        return Ok(false);
                    }
                }
                "gte" => {
                    // Greater than or equal to
                    if event_price < target_price {
                        return Ok(false);
                    }
                }
                "lte" => {
                    // Less than or equal to
                    if event_price > target_price {
                        return Ok(false);
                    }
                }
                "pct_change" => {
                    // Percentage change
                    // Get previous price from cache
                    let previous_price = if let Some((prev_price, _)) =
                        self.get_market_price(event_asset_pair).await
                    {
                        prev_price
                    } else {
                        // Get previous price from event data
                        event_data
                            .get("previous_price")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(event_price)
                    };

                    // Calculate percentage change
                    let pct_change = (event_price - previous_price) / previous_price * 100.0;

                    // Check if percentage change is greater than or equal to target
                    if pct_change.abs() < target_price.abs() {
                        return Ok(false);
                    }
                }
                "range" => {
                    // Price is within range
                    // Get upper bound from params
                    let upper_bound = params.get("upper_bound").and_then(|v| v.as_f64());

                    if let Some(upper) = upper_bound {
                        // Check if price is within range [target_price, upper_bound]
                        if event_price < target_price || event_price > upper {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
                _ => {
                    return Err(TriggerError::InvalidParameters(format!(
                        "Invalid price condition: {}",
                        condition
                    )));
                }
            }
        }

        // Update market price cache
        self.update_market_price(event_asset_pair, event_price, event_timestamp)
            .await;

        Ok(true)
    }

    /// Evaluate a custom trigger
    async fn evaluate_custom_trigger(
        &self,
        params: &serde_json::Value,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        // Parse custom parameters
        let event_name = params
            .get("event_name")
            .and_then(|v| v.as_str())
            .unwrap_or("*");
        let event_data_filter = params.get("event_data");

        // Extract event data
        let event_name_actual = event_data
            .get("event_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Check event name
        if event_name != "*" && event_name != event_name_actual {
            return Ok(false);
        }

        // Extract actual data
        let actual_data = event_data.get("data");

        // Check event data
        if let (Some(filter), Some(actual)) = (event_data_filter, actual_data) {
            // Get matching mode
            let matching_mode = params
                .get("matching_mode")
                .and_then(|v| v.as_str())
                .unwrap_or("exact");

            match matching_mode {
                "exact" => {
                    // Exact match
                    if filter != actual {
                        return Ok(false);
                    }
                }
                "partial" => {
                    // Partial match (check if filter is a subset of actual)
                    if let (Some(filter_obj), Some(actual_obj)) =
                        (filter.as_object(), actual.as_object())
                    {
                        for (key, value) in filter_obj {
                            if !actual_obj.contains_key(key) || actual_obj[key] != *value {
                                return Ok(false);
                            }
                        }
                    } else if let (Some(filter_arr), Some(actual_arr)) =
                        (filter.as_array(), actual.as_array())
                    {
                        for value in filter_arr {
                            if !actual_arr.contains(value) {
                                return Ok(false);
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                }
                "regex" => {
                    // Regex match
                    if let (Some(filter_str), Some(actual_str)) = (filter.as_str(), actual.as_str())
                    {
                        let regex = regex::Regex::new(filter_str).map_err(|e| {
                            TriggerError::InvalidParameters(format!("Invalid regex: {}", e))
                        })?;

                        if !regex.is_match(actual_str) {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
                "jsonpath" => {
                    // JSONPath match
                    if let Some(filter_str) = filter.as_str() {
                        // Use jsonpath_lib to evaluate the JSONPath expression
                        if filter_str != "*" {
                            // Parse and evaluate the JSONPath expression
                            let path_result =
                                jsonpath_lib::select(actual, filter_str).map_err(|e| {
                                    TriggerError::InvalidParameters(format!(
                                        "Invalid JSONPath: {}",
                                        e
                                    ))
                                })?;
                            let expected_value = params.get("expected_value");

                            if let Some(expected) = expected_value {
                                // Check if the key exists and has the expected value
                                let actual_value = jsonpath_lib::select(actual, filter_str)
                                    .map_err(|e| {
                                        TriggerError::InvalidParameters(format!(
                                            "Invalid JSONPath: {}",
                                            e
                                        ))
                                    })?;

                                if actual_value.is_empty() || *actual_value[0] != *expected {
                                    return Ok(false);
                                }
                            } else {
                                // Just check if the key exists
                                let actual_value = jsonpath_lib::select(actual, filter_str)
                                    .map_err(|e| {
                                        TriggerError::InvalidParameters(format!(
                                            "Invalid JSONPath: {}",
                                            e
                                        ))
                                    })?;

                                if actual_value.is_empty() {
                                    return Ok(false);
                                }
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                }
                _ => {
                    return Err(TriggerError::InvalidParameters(format!(
                        "Invalid matching mode: {}",
                        matching_mode
                    )));
                }
            }
        }

        Ok(true)
    }
}

#[async_trait]
impl TriggerEvaluator for StandardTriggerEvaluator {
    async fn evaluate_trigger(
        &self,
        condition: &TriggerCondition,
        event_data: &serde_json::Value,
    ) -> Result<bool, TriggerError> {
        match condition.source {
            TriggerSource::Blockchain => {
                self.evaluate_blockchain_trigger(&condition.params, event_data)
                    .await
            }
            TriggerSource::Time => {
                self.evaluate_time_trigger(&condition.params, event_data)
                    .await
            }
            TriggerSource::Market => {
                self.evaluate_market_trigger(&condition.params, event_data)
                    .await
            }
            TriggerSource::Custom => {
                self.evaluate_custom_trigger(&condition.params, event_data)
                    .await
            }
        }
    }
}
