// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::pricing::types::{
    BillingRecord, EcosystemIncentive, NeoEcosystemIntegration, PricingError, PricingTier,
    ResourcePricing, ResourceType, SubscriptionModel, SubscriptionType, UserBillingProfile,
    ValueAddedService, VolumeDiscount,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Trait defining the pricing storage functionality
#[async_trait]
pub trait PricingStorage: Send + Sync {
    // Resource pricing methods
    async fn get_resource_pricing(
        &self,
        resource_type: ResourceType,
        tier: PricingTier,
    ) -> Result<ResourcePricing, PricingError>;
    async fn get_all_resource_pricing(&self) -> Result<Vec<ResourcePricing>, PricingError>;
    async fn add_resource_pricing(&self, pricing: ResourcePricing) -> Result<(), PricingError>;
    async fn update_resource_pricing(&self, pricing: ResourcePricing) -> Result<(), PricingError>;

    // Subscription model methods
    async fn get_subscription_model(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionModel, PricingError>;
    async fn get_all_subscription_models(&self) -> Result<Vec<SubscriptionModel>, PricingError>;
    async fn get_subscription_models_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<SubscriptionModel>, PricingError>;
    async fn get_subscription_models_by_type(
        &self,
        subscription_type: SubscriptionType,
    ) -> Result<Vec<SubscriptionModel>, PricingError>;
    async fn add_subscription_model(&self, model: SubscriptionModel) -> Result<(), PricingError>;
    async fn update_subscription_model(&self, model: SubscriptionModel)
        -> Result<(), PricingError>;

    // Value-added service methods
    async fn get_value_added_service(
        &self,
        service_id: &str,
    ) -> Result<ValueAddedService, PricingError>;
    async fn get_all_value_added_services(&self) -> Result<Vec<ValueAddedService>, PricingError>;
    async fn get_value_added_services_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<ValueAddedService>, PricingError>;

    // Ecosystem incentive methods
    async fn get_ecosystem_incentive(
        &self,
        incentive_id: &str,
    ) -> Result<EcosystemIncentive, PricingError>;
    async fn get_all_ecosystem_incentives(&self) -> Result<Vec<EcosystemIncentive>, PricingError>;

    // Neo ecosystem integration methods
    async fn get_neo_ecosystem_integration(
        &self,
        integration_id: &str,
    ) -> Result<NeoEcosystemIntegration, PricingError>;
    async fn get_all_neo_ecosystem_integrations(
        &self,
    ) -> Result<Vec<NeoEcosystemIntegration>, PricingError>;

    // User billing profile methods
    async fn get_user_billing_profile(
        &self,
        user_id: &str,
    ) -> Result<UserBillingProfile, PricingError>;
    async fn create_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError>;
    async fn update_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError>;

    // Billing record methods
    async fn get_billing_record(&self, record_id: &str) -> Result<BillingRecord, PricingError>;
    async fn create_billing_record(
        &self,
        user_id: &str,
        record: BillingRecord,
    ) -> Result<(), PricingError>;
    async fn update_billing_record(&self, record: BillingRecord) -> Result<(), PricingError>;
    
    // Resource usage record methods
    async fn store_resource_usage_record(
        &self,
        record: ResourceUsageRecord,
    ) -> Result<(), PricingError>;
    
    async fn get_resource_usage_records(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError>;
    
    async fn get_resource_usage_records_by_type(
        &self,
        user_id: &str,
        resource_type: ResourceType,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError>;
    
    async fn get_resource_usage_records_by_function(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError>;
    
    async fn get_resource_usage_records_by_service(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError>;
}

/// In-memory implementation of the pricing storage
pub struct MemoryPricingStorage {
    resource_pricing: RwLock<HashMap<(ResourceType, PricingTier), ResourcePricing>>,
    subscription_models: RwLock<HashMap<String, SubscriptionModel>>,
    value_added_services: RwLock<HashMap<String, ValueAddedService>>,
    ecosystem_incentives: RwLock<HashMap<String, EcosystemIncentive>>,
    neo_integrations: RwLock<HashMap<String, NeoEcosystemIntegration>>,
    user_profiles: RwLock<HashMap<String, UserBillingProfile>>,
    billing_records: RwLock<HashMap<String, BillingRecord>>,
    user_billing_records: RwLock<HashMap<String, Vec<String>>>,
    resource_usage_records: RwLock<HashMap<String, Vec<ResourceUsageRecord>>>,
}

impl MemoryPricingStorage {
    /// Create a new memory-based pricing storage
    pub fn new() -> Self {
        Self {
            resource_pricing: RwLock::new(HashMap::new()),
            subscription_models: RwLock::new(HashMap::new()),
            value_added_services: RwLock::new(HashMap::new()),
            ecosystem_incentives: RwLock::new(HashMap::new()),
            neo_integrations: RwLock::new(HashMap::new()),
            user_profiles: RwLock::new(HashMap::new()),
            billing_records: RwLock::new(HashMap::new()),
            user_billing_records: RwLock::new(HashMap::new()),
            resource_usage_records: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new memory-based pricing storage with default pricing models
    pub fn with_defaults() -> Self {
        let storage = Self::new();

        // Initialize with default resource pricing
        let resource_pricing = vec![
            // Basic tier pricing
            ResourcePricing {
                resource_type: ResourceType::ExecutionTime,
                tier: PricingTier::Basic,
                base_price: 0.0,
                price_per_unit: 0.0001,         // GAS per millisecond
                free_tier_limit: Some(1000000), // 1000 seconds free
                min_billable_units: 100,        // 100ms minimum
                max_billable_units: None,
                volume_discounts: vec![
                    VolumeDiscount {
                        threshold: 10000000,
                        discount_percentage: 10.0,
                    }, // 10% off after 10000 seconds
                    VolumeDiscount {
                        threshold: 100000000,
                        discount_percentage: 20.0,
                    }, // 20% off after 100000 seconds
                ],
            },
            ResourcePricing {
                resource_type: ResourceType::MemoryUsage,
                tier: PricingTier::Basic,
                base_price: 0.0,
                price_per_unit: 0.0005,      // GAS per MB
                free_tier_limit: Some(1024), // 1GB free
                min_billable_units: 64,      // 64MB minimum
                max_billable_units: None,
                volume_discounts: vec![
                    VolumeDiscount {
                        threshold: 10240,
                        discount_percentage: 10.0,
                    }, // 10% off after 10GB
                    VolumeDiscount {
                        threshold: 102400,
                        discount_percentage: 20.0,
                    }, // 20% off after 100GB
                ],
            },
            // Standard tier pricing
            ResourcePricing {
                resource_type: ResourceType::ExecutionTime,
                tier: PricingTier::Standard,
                base_price: 0.0,
                price_per_unit: 0.00008, // GAS per millisecond (20% cheaper than Basic)
                free_tier_limit: Some(2000000), // 2000 seconds free
                min_billable_units: 100, // 100ms minimum
                max_billable_units: None,
                volume_discounts: vec![
                    VolumeDiscount {
                        threshold: 10000000,
                        discount_percentage: 15.0,
                    }, // 15% off after 10000 seconds
                    VolumeDiscount {
                        threshold: 100000000,
                        discount_percentage: 25.0,
                    }, // 25% off after 100000 seconds
                ],
            },
            // Premium tier pricing
            ResourcePricing {
                resource_type: ResourceType::ExecutionTime,
                tier: PricingTier::Premium,
                base_price: 0.0,
                price_per_unit: 0.00006, // GAS per millisecond (40% cheaper than Basic)
                free_tier_limit: Some(5000000), // 5000 seconds free
                min_billable_units: 100, // 100ms minimum
                max_billable_units: None,
                volume_discounts: vec![
                    VolumeDiscount {
                        threshold: 10000000,
                        discount_percentage: 20.0,
                    }, // 20% off after 10000 seconds
                    VolumeDiscount {
                        threshold: 100000000,
                        discount_percentage: 30.0,
                    }, // 30% off after 100000 seconds
                ],
            },
        ];

        for pricing in resource_pricing {
            let key = (pricing.resource_type, pricing.tier);
            let _ = storage
                .resource_pricing
                .write()
                .unwrap()
                .insert(key, pricing);
        }

        // Initialize with default subscription models
        let subscription_models = vec![
            SubscriptionModel {
                id: "basic-monthly".to_string(),
                name: "Basic Monthly".to_string(),
                subscription_type: SubscriptionType::Monthly,
                tier: PricingTier::Basic,
                base_price: 10.0, // 10 GAS per month
                included_resources: HashMap::from([
                    (ResourceType::ExecutionTime, 5000000), // 5000 seconds
                    (ResourceType::MemoryUsage, 2048),      // 2GB
                ]),
                overage_pricing: HashMap::from([
                    (ResourceType::ExecutionTime, 0.00009), // GAS per millisecond
                    (ResourceType::MemoryUsage, 0.00045),   // GAS per MB
                ]),
                discount_percentage: 0.0,
                min_commitment_period: 1, // 1 month
                early_termination_fee: None,
            },
            SubscriptionModel {
                id: "standard-monthly".to_string(),
                name: "Standard Monthly".to_string(),
                subscription_type: SubscriptionType::Monthly,
                tier: PricingTier::Standard,
                base_price: 50.0, // 50 GAS per month
                included_resources: HashMap::from([
                    (ResourceType::ExecutionTime, 30000000), // 30000 seconds
                    (ResourceType::MemoryUsage, 10240),      // 10GB
                ]),
                overage_pricing: HashMap::from([
                    (ResourceType::ExecutionTime, 0.00007), // GAS per millisecond
                    (ResourceType::MemoryUsage, 0.0004),    // GAS per MB
                ]),
                discount_percentage: 5.0, // 5% discount
                min_commitment_period: 1, // 1 month
                early_termination_fee: None,
            },
            SubscriptionModel {
                id: "premium-annual".to_string(),
                name: "Premium Annual".to_string(),
                subscription_type: SubscriptionType::Annual,
                tier: PricingTier::Premium,
                base_price: 1000.0, // 1000 GAS per year
                included_resources: HashMap::from([
                    (ResourceType::ExecutionTime, 500000000), // 500000 seconds
                    (ResourceType::MemoryUsage, 102400),      // 100GB
                ]),
                overage_pricing: HashMap::from([
                    (ResourceType::ExecutionTime, 0.00005), // GAS per millisecond
                    (ResourceType::MemoryUsage, 0.0003),    // GAS per MB
                ]),
                discount_percentage: 20.0,          // 20% discount
                min_commitment_period: 12,          // 12 months
                early_termination_fee: Some(200.0), // 200 GAS early termination fee
            },
        ];

        for model in subscription_models {
            let _ = storage
                .subscription_models
                .write()
                .unwrap()
                .insert(model.id.clone(), model);
        }

        storage
    }
}

// Basic implementation of the PricingStorage trait for MemoryPricingStorage
#[async_trait]
impl PricingStorage for MemoryPricingStorage {
    async fn get_resource_pricing(
        &self,
        resource_type: ResourceType,
        tier: PricingTier,
    ) -> Result<ResourcePricing, PricingError> {
        let pricing = self
            .resource_pricing
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        pricing.get(&(resource_type, tier)).cloned().ok_or_else(|| {
            PricingError::NotFound(format!(
                "Resource pricing not found for {:?} in tier {:?}",
                resource_type, tier
            ))
        })
    }

    async fn get_all_resource_pricing(&self) -> Result<Vec<ResourcePricing>, PricingError> {
        let pricing = self
            .resource_pricing
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(pricing.values().cloned().collect())
    }

    async fn add_resource_pricing(&self, pricing: ResourcePricing) -> Result<(), PricingError> {
        let mut pricing_map = self
            .resource_pricing
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let key = (pricing.resource_type, pricing.tier);
        pricing_map.insert(key, pricing);

        Ok(())
    }

    async fn update_resource_pricing(&self, pricing: ResourcePricing) -> Result<(), PricingError> {
        let mut pricing_map = self
            .resource_pricing
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let key = (pricing.resource_type, pricing.tier);
        if !pricing_map.contains_key(&key) {
            return Err(PricingError::NotFound(format!(
                "Resource pricing not found for {:?} in tier {:?}",
                pricing.resource_type, pricing.tier
            )));
        }

        pricing_map.insert(key, pricing);

        Ok(())
    }

    async fn get_subscription_model(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionModel, PricingError> {
        let models = self
            .subscription_models
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        models.get(subscription_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!("Subscription model not found: {}", subscription_id))
        })
    }

    async fn get_all_subscription_models(&self) -> Result<Vec<SubscriptionModel>, PricingError> {
        let models = self
            .subscription_models
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(models.values().cloned().collect())
    }

    async fn get_subscription_models_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<SubscriptionModel>, PricingError> {
        let models = self
            .subscription_models
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let filtered_models = models
            .values()
            .filter(|m| m.tier == tier)
            .cloned()
            .collect();

        Ok(filtered_models)
    }

    async fn get_subscription_models_by_type(
        &self,
        subscription_type: SubscriptionType,
    ) -> Result<Vec<SubscriptionModel>, PricingError> {
        let models = self
            .subscription_models
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let filtered_models = models
            .values()
            .filter(|m| m.subscription_type == subscription_type)
            .cloned()
            .collect();

        Ok(filtered_models)
    }

    async fn add_subscription_model(&self, model: SubscriptionModel) -> Result<(), PricingError> {
        let mut models = self
            .subscription_models
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        models.insert(model.id.clone(), model);

        Ok(())
    }

    async fn update_subscription_model(
        &self,
        model: SubscriptionModel,
    ) -> Result<(), PricingError> {
        let mut models = self
            .subscription_models
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if !models.contains_key(&model.id) {
            return Err(PricingError::NotFound(format!(
                "Subscription model not found: {}",
                model.id
            )));
        }

        models.insert(model.id.clone(), model);

        Ok(())
    }

    // Minimal implementation for value-added services
    async fn get_value_added_service(
        &self,
        service_id: &str,
    ) -> Result<ValueAddedService, PricingError> {
        let services = self
            .value_added_services
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        services.get(service_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!("Value-added service not found: {}", service_id))
        })
    }

    async fn get_all_value_added_services(&self) -> Result<Vec<ValueAddedService>, PricingError> {
        let services = self
            .value_added_services
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(services.values().cloned().collect())
    }

    async fn get_value_added_services_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<ValueAddedService>, PricingError> {
        let services = self
            .value_added_services
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        let filtered_services = services
            .values()
            .filter(|s| s.available_in_tiers.contains(&tier))
            .cloned()
            .collect();

        Ok(filtered_services)
    }

    // Minimal implementation for ecosystem incentives
    async fn get_ecosystem_incentive(
        &self,
        incentive_id: &str,
    ) -> Result<EcosystemIncentive, PricingError> {
        let incentives = self
            .ecosystem_incentives
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        incentives.get(incentive_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!("Ecosystem incentive not found: {}", incentive_id))
        })
    }

    async fn get_all_ecosystem_incentives(&self) -> Result<Vec<EcosystemIncentive>, PricingError> {
        let incentives = self
            .ecosystem_incentives
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(incentives.values().cloned().collect())
    }

    // Minimal implementation for Neo ecosystem integrations
    async fn get_neo_ecosystem_integration(
        &self,
        integration_id: &str,
    ) -> Result<NeoEcosystemIntegration, PricingError> {
        let integrations = self
            .neo_integrations
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        integrations.get(integration_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!(
                "Neo ecosystem integration not found: {}",
                integration_id
            ))
        })
    }

    async fn get_all_neo_ecosystem_integrations(
        &self,
    ) -> Result<Vec<NeoEcosystemIntegration>, PricingError> {
        let integrations = self
            .neo_integrations
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        Ok(integrations.values().cloned().collect())
    }

    // Minimal implementation for user billing profiles
    async fn get_user_billing_profile(
        &self,
        user_id: &str,
    ) -> Result<UserBillingProfile, PricingError> {
        let profiles = self
            .user_profiles
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        profiles.get(user_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!("User billing profile not found: {}", user_id))
        })
    }

    async fn create_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError> {
        let mut profiles = self
            .user_profiles
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if profiles.contains_key(&profile.user_id) {
            return Err(PricingError::InvalidInput(format!(
                "User billing profile already exists: {}",
                profile.user_id
            )));
        }

        profiles.insert(profile.user_id.clone(), profile);

        Ok(())
    }

    async fn update_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError> {
        let mut profiles = self
            .user_profiles
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if !profiles.contains_key(&profile.user_id) {
            return Err(PricingError::NotFound(format!(
                "User billing profile not found: {}",
                profile.user_id
            )));
        }

        profiles.insert(profile.user_id.clone(), profile);

        Ok(())
    }

    // Minimal implementation for billing records
    async fn get_billing_record(&self, record_id: &str) -> Result<BillingRecord, PricingError> {
        let records = self
            .billing_records
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        records.get(record_id).cloned().ok_or_else(|| {
            PricingError::NotFound(format!("Billing record not found: {}", record_id))
        })
    }

    async fn create_billing_record(
        &self,
        user_id: &str,
        record: BillingRecord,
    ) -> Result<(), PricingError> {
        // Store the billing record
        let mut records = self
            .billing_records
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        records.insert(record.id.clone(), record.clone());

        // Update the user's billing records
        let mut user_records = self
            .user_billing_records
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        let user_record_ids = user_records
            .entry(user_id.to_string())
            .or_insert_with(Vec::new);
        user_record_ids.push(record.id.clone());

        Ok(())
    }

    async fn update_billing_record(&self, record: BillingRecord) -> Result<(), PricingError> {
        let mut records = self
            .billing_records
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        if !records.contains_key(&record.id) {
            return Err(PricingError::NotFound(format!(
                "Billing record not found: {}",
                record.id
            )));
        }

        records.insert(record.id.clone(), record);

        Ok(())
    }
    
    // Resource usage record methods
    async fn store_resource_usage_record(
        &self,
        record: ResourceUsageRecord,
    ) -> Result<(), PricingError> {
        let mut records = self
            .resource_usage_records
            .write()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire write lock: {}", e)))?;
            
        let user_records = records
            .entry(record.user_id.clone())
            .or_insert_with(Vec::new);
            
        user_records.push(record);
        
        Ok(())
    }
    
    async fn get_resource_usage_records(
        &self,
        user_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError> {
        let records = self
            .resource_usage_records
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;
            
        let user_records = records.get(user_id).cloned().unwrap_or_default();
        
        Ok(user_records)
    }
    
    async fn get_resource_usage_records_by_type(
        &self,
        user_id: &str,
        resource_type: ResourceType,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError> {
        let records = self
            .resource_usage_records
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;
            
        let user_records = records.get(user_id).cloned().unwrap_or_default();
        
        let filtered_records = user_records
            .into_iter()
            .filter(|r| r.resource_type == resource_type)
            .collect();
            
        Ok(filtered_records)
    }
    
    async fn get_resource_usage_records_by_function(
        &self,
        user_id: &str,
        function_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError> {
        let records = self
            .resource_usage_records
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;
            
        let user_records = records.get(user_id).cloned().unwrap_or_default();
        
        let filtered_records = user_records
            .into_iter()
            .filter(|r| r.function_id.as_ref().map_or(false, |id| id == function_id))
            .collect();
            
        Ok(filtered_records)
    }
    
    async fn get_resource_usage_records_by_service(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<Vec<ResourceUsageRecord>, PricingError> {
        let records = self
            .resource_usage_records
            .read()
            .map_err(|e| PricingError::Storage(format!("Failed to acquire read lock: {}", e)))?;
            
        let user_records = records.get(user_id).cloned().unwrap_or_default();
        
        let filtered_records = user_records
            .into_iter()
            .filter(|r| r.service_id.as_ref().map_or(false, |id| id == service_id))
            .collect();
            
        Ok(filtered_records)
    }
}
