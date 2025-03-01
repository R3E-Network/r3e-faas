// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::pricing::storage::PricingStorage;
use crate::pricing::types::{
    BillingRecord, EcosystemIncentive, NeoEcosystemIntegration, PaymentStatus, PricingError,
    PricingTier, ResourcePricing, ResourceType, SubscriptionModel, SubscriptionType,
    UserBillingProfile, ValueAddedService,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Trait defining the pricing service functionality
#[async_trait]
pub trait PricingServiceTrait: Send + Sync {
    /// Get resource pricing
    async fn get_resource_pricing(
        &self,
        resource_type: ResourceType,
        tier: PricingTier,
    ) -> Result<ResourcePricing, PricingError>;

    /// Get all resource pricing
    async fn get_all_resource_pricing(&self) -> Result<Vec<ResourcePricing>, PricingError>;

    /// Get subscription model
    async fn get_subscription_model(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionModel, PricingError>;

    /// Get all subscription models
    async fn get_all_subscription_models(&self) -> Result<Vec<SubscriptionModel>, PricingError>;

    /// Get subscription models by tier
    async fn get_subscription_models_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<SubscriptionModel>, PricingError>;

    /// Get subscription models by type
    async fn get_subscription_models_by_type(
        &self,
        subscription_type: SubscriptionType,
    ) -> Result<Vec<SubscriptionModel>, PricingError>;

    /// Get value-added service
    async fn get_value_added_service(
        &self,
        service_id: &str,
    ) -> Result<ValueAddedService, PricingError>;

    /// Get all value-added services
    async fn get_all_value_added_services(&self) -> Result<Vec<ValueAddedService>, PricingError>;

    /// Get value-added services by tier
    async fn get_value_added_services_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<ValueAddedService>, PricingError>;

    /// Get ecosystem incentive
    async fn get_ecosystem_incentive(
        &self,
        incentive_id: &str,
    ) -> Result<EcosystemIncentive, PricingError>;

    /// Get all ecosystem incentives
    async fn get_all_ecosystem_incentives(&self) -> Result<Vec<EcosystemIncentive>, PricingError>;

    /// Get Neo ecosystem integration
    async fn get_neo_ecosystem_integration(
        &self,
        integration_id: &str,
    ) -> Result<NeoEcosystemIntegration, PricingError>;

    /// Get all Neo ecosystem integrations
    async fn get_all_neo_ecosystem_integrations(
        &self,
    ) -> Result<Vec<NeoEcosystemIntegration>, PricingError>;

    /// Get user billing profile
    async fn get_user_billing_profile(
        &self,
        user_id: &str,
    ) -> Result<UserBillingProfile, PricingError>;

    /// Create user billing profile
    async fn create_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError>;

    /// Update user billing profile
    async fn update_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError>;

    /// Subscribe user to a subscription model
    async fn subscribe_user(
        &self,
        user_id: &str,
        subscription_id: &str,
    ) -> Result<(), PricingError>;

    /// Unsubscribe user from a subscription model
    async fn unsubscribe_user(&self, user_id: &str) -> Result<(), PricingError>;

    /// Add value-added service to user
    async fn add_value_added_service_to_user(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<(), PricingError>;

    /// Remove value-added service from user
    async fn remove_value_added_service_from_user(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<(), PricingError>;

    /// Calculate resource usage cost
    async fn calculate_resource_usage_cost(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
    ) -> Result<f64, PricingError>;

    /// Record resource usage
    async fn record_resource_usage(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
    ) -> Result<(), PricingError>;

    /// Generate billing record
    async fn generate_billing_record(&self, user_id: &str) -> Result<BillingRecord, PricingError>;

    /// Process payment
    async fn process_payment(
        &self,
        billing_record_id: &str,
        payment_method_id: &str,
    ) -> Result<PaymentStatus, PricingError>;

    /// Apply ecosystem incentive
    async fn apply_ecosystem_incentive(
        &self,
        user_id: &str,
        incentive_id: &str,
        data: serde_json::Value,
    ) -> Result<(), PricingError>;
    
    /// Set resource quota
    async fn set_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        quota: u64,
    ) -> Result<(), PricingError>;
    
    /// Get resource quota
    async fn get_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
    ) -> Result<u64, PricingError>;
    
    /// Check if resource usage would exceed quota
    async fn check_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        requested_usage: u64,
    ) -> Result<bool, PricingError>;
    
    /// Track detailed resource usage
    async fn track_resource_usage(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
        function_id: Option<String>,
        service_id: Option<String>,
    ) -> Result<(), PricingError>;
    
    /// Generate usage analytics
    async fn generate_usage_analytics(&self, user_id: &str) -> Result<UsageAnalytics, PricingError>;
    
    /// Get optimization recommendations
    async fn get_optimization_recommendations(
        &self,
        user_id: &str,
    ) -> Result<Vec<OptimizationRecommendation>, PricingError>;
}

/// Implementation of the pricing service
pub struct PricingService<S: PricingStorage> {
    /// Storage backend
    storage: Arc<S>,
}

impl<S: PricingStorage> PricingService<S> {
    /// Create a new pricing service
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Generate a new ID
    fn generate_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Get current timestamp
    fn get_current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Calculate resource cost based on pricing tier and usage
    async fn calculate_resource_cost(&self, pricing: &ResourcePricing, usage: u64) -> f64 {
        // Check if usage is within free tier limit
        if let Some(free_limit) = pricing.free_tier_limit {
            if usage <= free_limit {
                return 0.0;
            }
        }

        // Calculate billable units
        let billable_units = if usage < pricing.min_billable_units {
            pricing.min_billable_units
        } else if let Some(max_units) = pricing.max_billable_units {
            std::cmp::min(usage, max_units)
        } else {
            usage
        };

        // Calculate base cost
        let mut cost = pricing.base_price;

        // Add per-unit cost
        cost += pricing.price_per_unit * billable_units as f64;

        // Apply volume discounts
        let mut discount_percentage = 0.0;
        for discount in &pricing.volume_discounts {
            if billable_units >= discount.threshold {
                discount_percentage = discount.discount_percentage;
            } else {
                break;
            }
        }

        // Apply discount
        if discount_percentage > 0.0 {
            cost *= 1.0 - (discount_percentage / 100.0);
        }

        cost
    }
}

#[async_trait]
impl<S: PricingStorage> PricingServiceTrait for PricingService<S> {
    async fn get_resource_pricing(
        &self,
        resource_type: ResourceType,
        tier: PricingTier,
    ) -> Result<ResourcePricing, PricingError> {
        self.storage.get_resource_pricing(resource_type, tier).await
    }

    async fn get_all_resource_pricing(&self) -> Result<Vec<ResourcePricing>, PricingError> {
        self.storage.get_all_resource_pricing().await
    }

    async fn get_subscription_model(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionModel, PricingError> {
        self.storage.get_subscription_model(subscription_id).await
    }

    async fn get_all_subscription_models(&self) -> Result<Vec<SubscriptionModel>, PricingError> {
        self.storage.get_all_subscription_models().await
    }

    async fn get_subscription_models_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<SubscriptionModel>, PricingError> {
        self.storage.get_subscription_models_by_tier(tier).await
    }

    async fn get_subscription_models_by_type(
        &self,
        subscription_type: SubscriptionType,
    ) -> Result<Vec<SubscriptionModel>, PricingError> {
        self.storage
            .get_subscription_models_by_type(subscription_type)
            .await
    }

    async fn get_value_added_service(
        &self,
        service_id: &str,
    ) -> Result<ValueAddedService, PricingError> {
        self.storage.get_value_added_service(service_id).await
    }

    async fn get_all_value_added_services(&self) -> Result<Vec<ValueAddedService>, PricingError> {
        self.storage.get_all_value_added_services().await
    }

    async fn get_value_added_services_by_tier(
        &self,
        tier: PricingTier,
    ) -> Result<Vec<ValueAddedService>, PricingError> {
        self.storage.get_value_added_services_by_tier(tier).await
    }

    async fn get_ecosystem_incentive(
        &self,
        incentive_id: &str,
    ) -> Result<EcosystemIncentive, PricingError> {
        self.storage.get_ecosystem_incentive(incentive_id).await
    }

    async fn get_all_ecosystem_incentives(&self) -> Result<Vec<EcosystemIncentive>, PricingError> {
        self.storage.get_all_ecosystem_incentives().await
    }

    async fn get_neo_ecosystem_integration(
        &self,
        integration_id: &str,
    ) -> Result<NeoEcosystemIntegration, PricingError> {
        self.storage
            .get_neo_ecosystem_integration(integration_id)
            .await
    }

    async fn get_all_neo_ecosystem_integrations(
        &self,
    ) -> Result<Vec<NeoEcosystemIntegration>, PricingError> {
        self.storage.get_all_neo_ecosystem_integrations().await
    }

    async fn get_user_billing_profile(
        &self,
        user_id: &str,
    ) -> Result<UserBillingProfile, PricingError> {
        self.storage.get_user_billing_profile(user_id).await
    }

    async fn create_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError> {
        self.storage.create_user_billing_profile(profile).await
    }

    async fn update_user_billing_profile(
        &self,
        profile: UserBillingProfile,
    ) -> Result<(), PricingError> {
        self.storage.update_user_billing_profile(profile).await
    }

    async fn subscribe_user(
        &self,
        user_id: &str,
        subscription_id: &str,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Get the subscription model
        let subscription = self.storage.get_subscription_model(subscription_id).await?;

        // Update the user's billing profile
        profile.subscription = Some(subscription_id.to_string());
        profile.tier = subscription.tier;

        // Set subscription dates
        let now = self.get_current_timestamp();
        profile.subscription_start_date = Some(now);

        // Calculate subscription end date based on subscription type
        let end_date = match subscription.subscription_type {
            SubscriptionType::Monthly => now + 30 * 24 * 60 * 60, // 30 days
            SubscriptionType::Annual => now + 365 * 24 * 60 * 60, // 365 days
            SubscriptionType::ReservedCapacity => {
                now + subscription.min_commitment_period as u64 * 30 * 24 * 60 * 60
            } // min_commitment_period months
            SubscriptionType::PayAsYouGo => None,                 // No end date for pay-as-you-go
        };

        profile.subscription_end_date = end_date;

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }

    async fn unsubscribe_user(&self, user_id: &str) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Check if the user has a subscription
        if profile.subscription.is_none() {
            return Err(PricingError::InvalidInput(format!(
                "User {} is not subscribed to any plan",
                user_id
            )));
        }

        // Get the subscription model
        let subscription_id = profile.subscription.as_ref().unwrap();
        let subscription = self.storage.get_subscription_model(subscription_id).await?;

        // Check if the user can unsubscribe (minimum commitment period)
        if let Some(start_date) = profile.subscription_start_date {
            let now = self.get_current_timestamp();
            let subscription_duration = now - start_date;
            let min_commitment_seconds =
                subscription.min_commitment_period as u64 * 30 * 24 * 60 * 60; // min_commitment_period months

            if subscription_duration < min_commitment_seconds {
                // Apply early termination fee
                if let Some(fee) = subscription.early_termination_fee {
                    // Charge the early termination fee
                    let fee_payment = crate::pricing::types::BillingItem {
                        description: "Early termination fee".to_string(),
                        resource_type: None,
                        quantity: 1,
                        unit_price: fee,
                        total_price: fee,
                    };

                    // Create and process billing record for the fee
                    let billing_record = BillingRecord {
                        id: self.generate_id(),
                        date: now,
                        amount: fee,
                        description: format!("Early termination fee for user {}", user_id),
                        items: vec![fee_payment],
                        payment_status: PaymentStatus::Pending,
                        payment_method: None,
                        payment_date: None,
                    };

                    // Store the billing record
                    self.storage.store_billing_record(billing_record).await?;
                }
            }
        }

        // Update the user's billing profile
        profile.subscription = None;
        profile.subscription_start_date = None;
        profile.subscription_end_date = None;

        // Reset to basic tier
        profile.tier = PricingTier::Basic;

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }

    async fn add_value_added_service_to_user(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Get the value-added service
        let service = self.storage.get_value_added_service(service_id).await?;

        // Check if the service is available for the user's tier
        if !service.available_in_tiers.contains(&profile.tier) {
            return Err(PricingError::Unauthorized(format!(
                "Service {} is not available for tier {}",
                service_id, profile.tier
            )));
        }

        // Check if the user already has the service
        if profile
            .value_added_services
            .contains(&service_id.to_string())
        {
            return Err(PricingError::InvalidInput(format!(
                "User {} already has service {}",
                user_id, service_id
            )));
        }

        // Add the service to the user's profile
        profile.value_added_services.push(service_id.to_string());

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }

    async fn remove_value_added_service_from_user(
        &self,
        user_id: &str,
        service_id: &str,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Check if the user has the service
        if !profile
            .value_added_services
            .contains(&service_id.to_string())
        {
            return Err(PricingError::InvalidInput(format!(
                "User {} does not have service {}",
                user_id, service_id
            )));
        }

        // Remove the service from the user's profile
        profile.value_added_services.retain(|s| s != service_id);

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }

    async fn calculate_resource_usage_cost(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
    ) -> Result<f64, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;

        // Get the resource pricing for the user's tier
        let pricing = self
            .storage
            .get_resource_pricing(resource_type, profile.tier)
            .await?;

        // Check if the user has a subscription
        if let Some(subscription_id) = &profile.subscription {
            // Get the subscription model
            let subscription = self.storage.get_subscription_model(subscription_id).await?;

            // Check if the resource is included in the subscription
            if let Some(included_amount) = subscription.included_resources.get(&resource_type) {
                // Get the user's current usage
                let current_usage = profile
                    .resource_usage
                    .get(&resource_type)
                    .cloned()
                    .unwrap_or(0);

                // Calculate the total usage
                let total_usage = current_usage + usage;

                // Check if the usage is within the included amount
                if total_usage <= *included_amount {
                    // Usage is covered by the subscription
                    return Ok(0.0);
                }

                // Calculate the overage
                let overage = total_usage - included_amount;

                // Get the overage pricing
                if let Some(overage_price) = subscription.overage_pricing.get(&resource_type) {
                    // Calculate the cost
                    return Ok(overage as f64 * overage_price);
                }
            }
        }

        // Calculate the cost based on the resource pricing
        let cost = self.calculate_resource_cost(&pricing, usage).await;

        Ok(cost)
    }

    async fn record_resource_usage(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Update the resource usage
        let current_usage = profile
            .resource_usage
            .get(&resource_type)
            .cloned()
            .unwrap_or(0);
        profile
            .resource_usage
            .insert(resource_type, current_usage + usage);

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }

    async fn generate_billing_record(&self, user_id: &str) -> Result<BillingRecord, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;

        // Create a new billing record
        let now = self.get_current_timestamp();
        let mut billing_record = BillingRecord {
            id: self.generate_id(),
            date: now,
            amount: 0.0,
            description: format!("Billing for user {}", user_id),
            items: Vec::new(),
            payment_status: PaymentStatus::Pending,
            payment_method: None,
            payment_date: None,
        };

        // Add subscription fee if applicable
        if let Some(subscription_id) = &profile.subscription {
            // Get the subscription model
            let subscription = self.storage.get_subscription_model(subscription_id).await?;

            // Add subscription fee
            billing_record
                .items
                .push(crate::pricing::types::BillingItem {
                    description: format!("{} subscription", subscription.name),
                    resource_type: None,
                    quantity: 1,
                    unit_price: subscription.base_price,
                    total_price: subscription.base_price,
                });

            billing_record.amount += subscription.base_price;
        }

        // Add resource usage fees
        for (resource_type, usage) in &profile.resource_usage {
            // Calculate the cost
            let cost = self
                .calculate_resource_usage_cost(user_id, *resource_type, *usage)
                .await?;

            if cost > 0.0 {
                // Add resource usage fee
                billing_record
                    .items
                    .push(crate::pricing::types::BillingItem {
                        description: format!("{} usage", resource_type),
                        resource_type: Some(*resource_type),
                        quantity: *usage,
                        unit_price: cost / *usage as f64,
                        total_price: cost,
                    });

                billing_record.amount += cost;
            }
        }

        // Add value-added service fees
        for service_id in &profile.value_added_services {
            // Get the value-added service
            let service = self.storage.get_value_added_service(service_id).await?;

            // Add service fee
            billing_record
                .items
                .push(crate::pricing::types::BillingItem {
                    description: format!("{} service", service.name),
                    resource_type: None,
                    quantity: 1,
                    unit_price: service.price,
                    total_price: service.price,
                });

            billing_record.amount += service.price;
        }

        // Store the billing record
        self.storage
            .create_billing_record(user_id, billing_record.clone())
            .await?;

        Ok(billing_record)
    }

    async fn process_payment(
        &self,
        billing_record_id: &str,
        payment_method_id: &str,
    ) -> Result<PaymentStatus, PricingError> {
        // Get the billing record
        let mut billing_record = self.storage.get_billing_record(billing_record_id).await?;

        // Check if the billing record is already paid
        if billing_record.payment_status == PaymentStatus::Paid {
            return Ok(PaymentStatus::Paid);
        }

        // Process the payment using the specified payment method
        let payment_result = match payment_method_id.split(':').next() {
            Some("crypto") => {
                // Process crypto payment
                log::info!(
                    "Processing crypto payment for billing record {}",
                    billing_record_id
                );
                // In a production environment, this would interact with blockchain services
                // to process the payment using the specified crypto asset
                Ok(())
            }
            Some("fiat") => {
                // Process fiat payment
                log::info!(
                    "Processing fiat payment for billing record {}",
                    billing_record_id
                );
                // In a production environment, this would interact with payment gateways
                // to process the payment using the specified fiat currency
                Ok(())
            }
            Some("credit") => {
                // Process credit payment
                log::info!(
                    "Processing credit payment for billing record {}",
                    billing_record_id
                );
                // In a production environment, this would interact with credit card processors
                // to process the payment using the specified credit card
                Ok(())
            }
            Some("balance") => {
                // Process payment from user balance
                log::info!(
                    "Processing balance payment for billing record {}",
                    billing_record_id
                );
                // In a production environment, this would deduct the amount from the user's balance
                Ok(())
            }
            _ => Err(PricingError::InvalidInput(format!(
                "Invalid payment method: {}",
                payment_method_id
            ))),
        };

        // Update the billing record based on the payment result
        match payment_result {
            Ok(_) => {
                billing_record.payment_status = PaymentStatus::Paid;
                billing_record.payment_method = Some(payment_method_id.to_string());
                billing_record.payment_date = Some(self.get_current_timestamp());
            }
            Err(e) => {
                billing_record.payment_status = PaymentStatus::Failed;
                log::error!("Payment failed: {}", e);
                return Err(e);
            }
        };

        // Update the billing record
        self.storage
            .update_billing_record(billing_record.clone())
            .await?;

        Ok(PaymentStatus::Paid)
    }

    async fn apply_ecosystem_incentive(
        &self,
        user_id: &str,
        incentive_id: &str,
        data: serde_json::Value,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;

        // Get the ecosystem incentive
        let incentive = self.storage.get_ecosystem_incentive(incentive_id).await?;

        // Check if the incentive is enabled
        if !incentive.enabled {
            return Err(PricingError::InvalidInput(format!(
                "Incentive {} is not enabled",
                incentive_id
            )));
        }

        // Validate the incentive conditions
        let incentive = self.storage.get_ecosystem_incentive(incentive_id).await?;

        // Check if the incentive is still active
        let now = self.get_current_timestamp();
        if let Some(end_date) = incentive.end_date {
            if now > end_date {
                return Err(PricingError::InvalidInput(format!(
                    "Incentive {} has expired",
                    incentive_id
                )));
            }
        }

        // Validate the incentive conditions based on the provided data
        for condition in &incentive.conditions {
            if !condition.validate(&data) {
                return Err(PricingError::InvalidInput(format!(
                    "Incentive conditions not met for {}",
                    incentive_id
                )));
            }
        }

        // Create a new earned incentive
        let earned_incentive = crate::pricing::types::EarnedIncentive {
            incentive_id: incentive_id.to_string(),
            earned_date: self.get_current_timestamp(),
            reward_amount: incentive.reward_amount,
            description: incentive.description.clone(),
            status: crate::pricing::types::IncentiveStatus::Approved,
        };

        // Add the earned incentive to the user's profile
        profile.earned_incentives.push(earned_incentive);

        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }
    
    async fn set_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        quota: u64,
    ) -> Result<(), PricingError> {
        // Get the user's billing profile
        let mut profile = self.storage.get_user_billing_profile(user_id).await?;
        
        // Set the resource quota
        profile.resource_quotas.insert(resource_type, quota);
        
        // Update the user's billing profile
        self.storage.update_user_billing_profile(profile).await
    }
    
    async fn get_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
    ) -> Result<u64, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;
        
        // Get the resource quota
        let quota = profile.resource_quotas.get(&resource_type).cloned().unwrap_or(0);
        
        Ok(quota)
    }
    
    async fn check_resource_quota(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        requested_usage: u64,
    ) -> Result<bool, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;
        
        // Get the resource quota
        let quota = profile.resource_quotas.get(&resource_type).cloned().unwrap_or(0);
        
        // If quota is 0, it means unlimited
        if quota == 0 {
            return Ok(true);
        }
        
        // Get the current usage
        let current_usage = profile.resource_usage.get(&resource_type).cloned().unwrap_or(0);
        
        // Check if the requested usage would exceed the quota
        Ok(current_usage + requested_usage <= quota)
    }
    
    async fn track_resource_usage(
        &self,
        user_id: &str,
        resource_type: ResourceType,
        usage: u64,
        function_id: Option<String>,
        service_id: Option<String>,
    ) -> Result<(), PricingError> {
        // Create a new resource usage record
        let record = crate::pricing::types::ResourceUsageRecord {
            id: self.generate_id(),
            user_id: user_id.to_string(),
            resource_type,
            usage,
            timestamp: self.get_current_timestamp(),
            function_id,
            service_id,
        };
        
        // Store the resource usage record
        self.storage.store_resource_usage_record(record).await?;
        
        // Also update the user's billing profile with the usage
        self.record_resource_usage(user_id, resource_type, usage).await
    }
    
    async fn generate_usage_analytics(&self, user_id: &str) -> Result<UsageAnalytics, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;
        
        // Get all resource usage records for the user
        let usage_records = self.storage.get_resource_usage_records(user_id).await?;
        
        // Generate daily usage patterns
        let mut daily_usage = HashMap::new();
        for resource_type in ResourceType::iter() {
            let records = usage_records.iter()
                .filter(|r| r.resource_type == resource_type)
                .collect::<Vec<_>>();
                
            let patterns = self.generate_daily_usage_patterns(&records);
            if !patterns.is_empty() {
                daily_usage.insert(resource_type, patterns);
            }
        }
        
        // Generate monthly usage patterns
        let mut monthly_usage = HashMap::new();
        for resource_type in ResourceType::iter() {
            let records = usage_records.iter()
                .filter(|r| r.resource_type == resource_type)
                .collect::<Vec<_>>();
                
            let patterns = self.generate_monthly_usage_patterns(&records);
            if !patterns.is_empty() {
                monthly_usage.insert(resource_type, patterns);
            }
        }
        
        // Generate usage trends
        let mut usage_trends = HashMap::new();
        for resource_type in ResourceType::iter() {
            if let Some(patterns) = monthly_usage.get(&resource_type) {
                if patterns.len() >= 2 {
                    let trend = self.calculate_usage_trend(&patterns);
                    usage_trends.insert(resource_type, trend);
                }
            }
        }
        
        // Generate optimization recommendations
        let recommendations = self.generate_optimization_recommendations(
            user_id, 
            &profile, 
            &daily_usage, 
            &monthly_usage, 
            &usage_trends
        ).await?;
        
        // Create the usage analytics
        let analytics = UsageAnalytics {
            user_id: user_id.to_string(),
            daily_usage,
            monthly_usage,
            usage_trends,
            optimization_recommendations: recommendations,
            last_updated: self.get_current_timestamp(),
        };
        
        // Store the usage analytics in the user's profile
        let mut profile = profile.clone();
        profile.usage_analytics = Some(analytics.clone());
        self.storage.update_user_billing_profile(profile).await?;
        
        Ok(analytics)
    }
    
    async fn get_optimization_recommendations(
        &self,
        user_id: &str,
    ) -> Result<Vec<OptimizationRecommendation>, PricingError> {
        // Get the user's billing profile
        let profile = self.storage.get_user_billing_profile(user_id).await?;
        
        // Check if the user has usage analytics
        if let Some(analytics) = &profile.usage_analytics {
            // Check if the analytics are recent (less than 7 days old)
            let now = self.get_current_timestamp();
            if now - analytics.last_updated < 7 * 24 * 60 * 60 {
                // Return the existing recommendations
                return Ok(analytics.optimization_recommendations.clone());
            }
        }
        
        // Generate new usage analytics
        let analytics = self.generate_usage_analytics(user_id).await?;
        
        // Return the recommendations
        Ok(analytics.optimization_recommendations)
    }
    
    // Helper methods for usage analytics
    
    fn generate_daily_usage_patterns(
        &self,
        records: &[&ResourceUsageRecord],
    ) -> Vec<DailyUsagePattern> {
        let mut patterns = Vec::new();
        
        // Group records by date
        let mut records_by_date = HashMap::new();
        for record in records {
            let date = chrono::NaiveDateTime::from_timestamp_opt(record.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
                
            records_by_date
                .entry(date)
                .or_insert_with(Vec::new)
                .push(*record);
        }
        
        // Generate patterns for each date
        for (date, date_records) in records_by_date {
            // Initialize hourly usage
            let mut hourly_usage = vec![0; 24];
            
            // Calculate hourly usage
            for record in date_records {
                let hour = chrono::NaiveDateTime::from_timestamp_opt(record.timestamp as i64, 0)
                    .unwrap()
                    .hour() as usize;
                    
                hourly_usage[hour] += record.usage;
            }
            
            // Calculate peak and average usage
            let peak_usage = *hourly_usage.iter().max().unwrap_or(&0);
            let total_usage = hourly_usage.iter().sum::<u64>();
            let average_usage = if hourly_usage.iter().filter(|&&u| u > 0).count() > 0 {
                total_usage as f64 / hourly_usage.iter().filter(|&&u| u > 0).count() as f64
            } else {
                0.0
            };
            
            // Create the pattern
            let pattern = DailyUsagePattern {
                date,
                hourly_usage,
                peak_usage,
                average_usage,
                total_usage,
            };
            
            patterns.push(pattern);
        }
        
        // Sort patterns by date
        patterns.sort_by(|a, b| a.date.cmp(&b.date));
        
        patterns
    }
    
    fn generate_monthly_usage_patterns(
        &self,
        records: &[&ResourceUsageRecord],
    ) -> Vec<MonthlyUsagePattern> {
        let mut patterns = Vec::new();
        
        // Group records by month
        let mut records_by_month = HashMap::new();
        for record in records {
            let month = chrono::NaiveDateTime::from_timestamp_opt(record.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m")
                .to_string();
                
            records_by_month
                .entry(month)
                .or_insert_with(Vec::new)
                .push(*record);
        }
        
        // Generate patterns for each month
        for (month, month_records) in records_by_month {
            // Group records by day
            let mut usage_by_day = HashMap::new();
            for record in month_records {
                let day = chrono::NaiveDateTime::from_timestamp_opt(record.timestamp as i64, 0)
                    .unwrap()
                    .day() as usize;
                    
                *usage_by_day.entry(day).or_insert(0) += record.usage;
            }
            
            // Create daily usage array
            let days_in_month = match month.split('-').nth(1) {
                Some("02") => if self.is_leap_year(month.split('-').next().unwrap_or("2020").parse().unwrap_or(2020)) { 29 } else { 28 },
                Some("04") | Some("06") | Some("09") | Some("11") => 30,
                _ => 31,
            };
            
            let mut daily_usage = vec![0; days_in_month];
            for (day, usage) in usage_by_day {
                if day > 0 && day <= days_in_month {
                    daily_usage[day - 1] = usage;
                }
            }
            
            // Calculate peak and average usage
            let peak_usage = *daily_usage.iter().max().unwrap_or(&0);
            let total_usage = daily_usage.iter().sum::<u64>();
            let average_usage = if daily_usage.iter().filter(|&&u| u > 0).count() > 0 {
                total_usage as f64 / daily_usage.iter().filter(|&&u| u > 0).count() as f64
            } else {
                0.0
            };
            
            // Create the pattern
            let pattern = MonthlyUsagePattern {
                month,
                daily_usage,
                peak_usage,
                average_usage,
                total_usage,
            };
            
            patterns.push(pattern);
        }
        
        // Sort patterns by month
        patterns.sort_by(|a, b| a.month.cmp(&b.month));
        
        patterns
    }
    
    fn calculate_usage_trend(&self, patterns: &[MonthlyUsagePattern]) -> UsageTrend {
        if patterns.len() < 2 {
            return UsageTrend {
                trend_type: TrendType::Stable,
                trend_percentage: 0.0,
                trend_period: 0,
            };
        }
        
        // Calculate the trend over the last 3 months or all available months
        let trend_period = std::cmp::min(3, patterns.len()) as u32;
        let recent_patterns = &patterns[patterns.len() - trend_period as usize..];
        
        // Calculate the percentage change
        let first_usage = recent_patterns.first().unwrap().total_usage;
        let last_usage = recent_patterns.last().unwrap().total_usage;
        
        let trend_percentage = if first_usage > 0 {
            ((last_usage as f64 - first_usage as f64) / first_usage as f64) * 100.0
        } else if last_usage > 0 {
            100.0 // If starting from zero, any increase is 100%
        } else {
            0.0 // No change
        };
        
        // Determine the trend type
        let trend_type = if trend_percentage > 10.0 {
            TrendType::Increasing
        } else if trend_percentage < -10.0 {
            TrendType::Decreasing
        } else {
            // Check for fluctuation
            let mut fluctuating = false;
            for i in 1..recent_patterns.len() {
                let prev_usage = recent_patterns[i - 1].total_usage;
                let curr_usage = recent_patterns[i].total_usage;
                
                if prev_usage > 0 {
                    let change = ((curr_usage as f64 - prev_usage as f64) / prev_usage as f64).abs() * 100.0;
                    if change > 20.0 {
                        fluctuating = true;
                        break;
                    }
                }
            }
            
            if fluctuating {
                TrendType::Fluctuating
            } else {
                TrendType::Stable
            }
        };
        
        UsageTrend {
            trend_type,
            trend_percentage: trend_percentage.abs(),
            trend_period,
        }
    }
    
    async fn generate_optimization_recommendations(
        &self,
        user_id: &str,
        profile: &UserBillingProfile,
        daily_usage: &HashMap<ResourceType, Vec<DailyUsagePattern>>,
        monthly_usage: &HashMap<ResourceType, Vec<MonthlyUsagePattern>>,
        usage_trends: &HashMap<ResourceType, UsageTrend>,
    ) -> Result<Vec<OptimizationRecommendation>, PricingError> {
        let mut recommendations = Vec::new();
        
        // Check for subscription optimization
        if let Some(subscription_id) = &profile.subscription {
            let subscription = self.storage.get_subscription_model(subscription_id).await?;
            
            // Get all subscription models for comparison
            let all_subscriptions = self.storage.get_all_subscription_models().await?;
            
            // Calculate the current monthly cost
            let current_monthly_cost = match subscription.subscription_type {
                SubscriptionType::Monthly => subscription.base_price,
                SubscriptionType::Annual => subscription.base_price / 12.0,
                SubscriptionType::ReservedCapacity => subscription.base_price / subscription.min_commitment_period as f64,
                SubscriptionType::PayAsYouGo => {
                    // Estimate based on recent usage
                    let mut total_cost = 0.0;
                    for (resource_type, patterns) in monthly_usage {
                        if let Some(pattern) = patterns.last() {
                            let resource_pricing = self.storage.get_resource_pricing(*resource_type, profile.tier).await?;
                            total_cost += self.calculate_resource_cost(&resource_pricing, pattern.total_usage).await;
                        }
                    }
                    total_cost
                }
            };
            
            // Check if another subscription would be more cost-effective
            for other_subscription in all_subscriptions {
                if other_subscription.id == *subscription_id {
                    continue;
                }
                
                // Calculate the cost with the other subscription
                let other_monthly_cost = match other_subscription.subscription_type {
                    SubscriptionType::Monthly => other_subscription.base_price,
                    SubscriptionType::Annual => other_subscription.base_price / 12.0,
                    SubscriptionType::ReservedCapacity => other_subscription.base_price / other_subscription.min_commitment_period as f64,
                    SubscriptionType::PayAsYouGo => {
                        // Estimate based on recent usage
                        let mut total_cost = 0.0;
                        for (resource_type, patterns) in monthly_usage {
                            if let Some(pattern) = patterns.last() {
                                // Calculate overage costs
                                let included_amount = other_subscription.included_resources.get(resource_type).cloned().unwrap_or(0);
                                if pattern.total_usage > included_amount {
                                    let overage = pattern.total_usage - included_amount;
                                    if let Some(overage_price) = other_subscription.overage_pricing.get(resource_type) {
                                        total_cost += overage as f64 * overage_price;
                                    } else {
                                        let resource_pricing = self.storage.get_resource_pricing(*resource_type, other_subscription.tier).await?;
                                        total_cost += self.calculate_resource_cost(&resource_pricing, overage).await;
                                    }
                                }
                            }
                        }
                        other_subscription.base_price + total_cost
                    }
                };
                
                // If the other subscription would save at least 10%
                if other_monthly_cost < current_monthly_cost * 0.9 {
                    let savings = current_monthly_cost - other_monthly_cost;
                    recommendations.push(OptimizationRecommendation {
                        id: self.generate_id(),
                        recommendation_type: RecommendationType::SubscriptionChange,
                        description: format!(
                            "Switching from {} to {} could save approximately {} GAS per month",
                            subscription.name, other_subscription.name, savings.round()
                        ),
                        estimated_savings: savings * 12.0, // Annual savings
                        resource_type: None,
                    });
                    
                    // Only recommend one subscription change
                    break;
                }
            }
        }
        
        // Check for resource optimization
        for (resource_type, trend) in usage_trends {
            match trend.trend_type {
                TrendType::Increasing if trend.trend_percentage > 30.0 => {
                    // Recommend resource optimization for rapidly increasing usage
                    recommendations.push(OptimizationRecommendation {
                        id: self.generate_id(),
                        recommendation_type: RecommendationType::ResourceOptimization,
                        description: format!(
                            "{} usage has increased by {:.1}% over the last {} months. Consider optimizing your code or implementing caching.",
                            resource_type, trend.trend_percentage, trend.trend_period
                        ),
                        estimated_savings: 0.0, // Cannot estimate without more data
                        resource_type: Some(*resource_type),
                    });
                },
                TrendType::Fluctuating => {
                    // Recommend reserved capacity for fluctuating usage
                    recommendations.push(OptimizationRecommendation {
                        id: self.generate_id(),
                        recommendation_type: RecommendationType::ReservedCapacity,
                        description: format!(
                            "{} usage is fluctuating significantly. Consider reserved capacity to stabilize costs.",
                            resource_type
                        ),
                        estimated_savings: 0.0, // Cannot estimate without more data
                        resource_type: Some(*resource_type),
                    });
                },
                _ => {}
            }
        }
        
        // Check for usage pattern optimization
        for (resource_type, patterns) in daily_usage {
            if patterns.len() >= 7 {
                // Analyze the last 7 days
                let recent_patterns = &patterns[patterns.len() - 7..];
                
                // Check for peak usage times
                let mut peak_hours = Vec::new();
                for pattern in recent_patterns {
                    for (hour, &usage) in pattern.hourly_usage.iter().enumerate() {
                        if usage > pattern.peak_usage * 0.8 {
                            peak_hours.push(hour);
                        }
                    }
                }
                
                // Count occurrences of each peak hour
                let mut hour_counts = HashMap::new();
                for hour in peak_hours {
                    *hour_counts.entry(hour).or_insert(0) += 1;
                }
                
                // Find the most common peak hours
                let mut common_peak_hours = hour_counts.iter()
                    .filter(|(_, &count)| count >= 3) // At least 3 days with this peak hour
                    .map(|(&hour, _)| hour)
                    .collect::<Vec<_>>();
                common_peak_hours.sort();
                
                if !common_peak_hours.is_empty() {
                    let peak_hours_str = common_peak_hours.iter()
                        .map(|&h| format!("{:02}:00", h))
                        .collect::<Vec<_>>()
                        .join(", ");
                        
                    recommendations.push(OptimizationRecommendation {
                        id: self.generate_id(),
                        recommendation_type: RecommendationType::UsagePatternOptimization,
                        description: format!(
                            "{} usage peaks at {}. Consider scheduling non-critical tasks outside these hours.",
                            resource_type, peak_hours_str
                        ),
                        estimated_savings: 0.0, // Cannot estimate without more data
                        resource_type: Some(*resource_type),
                    });
                }
            }
        }
        
        Ok(recommendations)
    }
    
    // Helper method to check if a year is a leap year
    fn is_leap_year(&self, year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}
