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
}
