// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PricingError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
}

/// Pricing tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PricingTier {
    /// Basic tier (limited resources, suitable for development and testing)
    Basic,

    /// Standard tier (moderate resources for production workloads)
    Standard,

    /// Premium tier (high resources for demanding applications)
    Premium,

    /// Enterprise tier (custom resources and dedicated support)
    Enterprise,
}

impl std::fmt::Display for PricingTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PricingTier::Basic => write!(f, "basic"),
            PricingTier::Standard => write!(f, "standard"),
            PricingTier::Premium => write!(f, "premium"),
            PricingTier::Enterprise => write!(f, "enterprise"),
        }
    }
}

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// Execution time (in milliseconds)
    ExecutionTime,

    /// Memory usage (in MB)
    MemoryUsage,

    /// Storage usage (in MB)
    StorageUsage,

    /// Network usage (in MB)
    NetworkUsage,

    /// TEE usage (in milliseconds)
    TeeUsage,

    /// API calls
    ApiCalls,

    /// Oracle requests
    OracleRequests,

    /// Gas bank operations
    GasBankOperations,

    /// Identity operations
    IdentityOperations,

    /// Indexing operations
    IndexingOperations,

    /// Bridge operations
    BridgeOperations,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::ExecutionTime => write!(f, "execution_time"),
            ResourceType::MemoryUsage => write!(f, "memory_usage"),
            ResourceType::StorageUsage => write!(f, "storage_usage"),
            ResourceType::NetworkUsage => write!(f, "network_usage"),
            ResourceType::TeeUsage => write!(f, "tee_usage"),
            ResourceType::ApiCalls => write!(f, "api_calls"),
            ResourceType::OracleRequests => write!(f, "oracle_requests"),
            ResourceType::GasBankOperations => write!(f, "gas_bank_operations"),
            ResourceType::IdentityOperations => write!(f, "identity_operations"),
            ResourceType::IndexingOperations => write!(f, "indexing_operations"),
            ResourceType::BridgeOperations => write!(f, "bridge_operations"),
        }
    }
}

/// Resource pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePricing {
    /// Resource type
    pub resource_type: ResourceType,

    /// Pricing tier
    pub tier: PricingTier,

    /// Base price (in GAS)
    pub base_price: f64,

    /// Price per unit (in GAS)
    pub price_per_unit: f64,

    /// Free tier limit
    pub free_tier_limit: Option<u64>,

    /// Minimum billable units
    pub min_billable_units: u64,

    /// Maximum billable units
    pub max_billable_units: Option<u64>,

    /// Volume discounts
    pub volume_discounts: Vec<VolumeDiscount>,
}

/// Volume discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDiscount {
    /// Threshold (in units)
    pub threshold: u64,

    /// Discount percentage
    pub discount_percentage: f64,
}

/// Subscription type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionType {
    /// Pay-as-you-go (traditional usage-based billing)
    PayAsYouGo,

    /// Monthly subscription (fixed monthly fee for a certain amount of resources)
    Monthly,

    /// Annual subscription (discounted annual fee for committed usage)
    Annual,

    /// Reserved capacity (discounted rates for reserved capacity)
    ReservedCapacity,
}

impl std::fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionType::PayAsYouGo => write!(f, "pay_as_you_go"),
            SubscriptionType::Monthly => write!(f, "monthly"),
            SubscriptionType::Annual => write!(f, "annual"),
            SubscriptionType::ReservedCapacity => write!(f, "reserved_capacity"),
        }
    }
}

/// Subscription model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionModel {
    /// Subscription ID
    pub id: String,

    /// Subscription name
    pub name: String,

    /// Subscription type
    pub subscription_type: SubscriptionType,

    /// Pricing tier
    pub tier: PricingTier,

    /// Base price (in GAS)
    pub base_price: f64,

    /// Included resources
    pub included_resources: HashMap<ResourceType, u64>,

    /// Overage pricing
    pub overage_pricing: HashMap<ResourceType, f64>,

    /// Discount percentage
    pub discount_percentage: f64,

    /// Minimum commitment period (in months)
    pub min_commitment_period: u32,

    /// Early termination fee (in GAS)
    pub early_termination_fee: Option<f64>,
}

/// Value-added service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueAddedService {
    /// Service ID
    pub id: String,

    /// Service name
    pub name: String,

    /// Service description
    pub description: String,

    /// Service price (in GAS)
    pub price: f64,

    /// Pricing model
    pub pricing_model: ValueAddedServicePricingModel,

    /// Available in tiers
    pub available_in_tiers: Vec<PricingTier>,

    /// Is the service enabled?
    pub enabled: bool,
}

/// Value-added service pricing model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueAddedServicePricingModel {
    /// One-time fee
    OneTime(f64),

    /// Recurring fee
    Recurring {
        /// Fee amount
        amount: f64,

        /// Billing period (in days)
        period_days: u32,
    },

    /// Usage-based fee
    UsageBased {
        /// Base fee
        base_fee: f64,

        /// Fee per unit
        fee_per_unit: f64,

        /// Unit type
        unit_type: String,
    },
}

/// Ecosystem incentive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemIncentive {
    /// Incentive ID
    pub id: String,

    /// Incentive name
    pub name: String,

    /// Incentive description
    pub description: String,

    /// Incentive type
    pub incentive_type: EcosystemIncentiveType,

    /// Reward amount (in GAS)
    pub reward_amount: f64,

    /// Reward conditions
    pub reward_conditions: serde_json::Value,

    /// Is the incentive enabled?
    pub enabled: bool,
}

/// Ecosystem incentive type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EcosystemIncentiveType {
    /// Developer rewards
    DeveloperRewards,

    /// Referral program
    ReferralProgram,

    /// Community contributions
    CommunityContributions,

    /// Staking rewards
    StakingRewards,
}

impl std::fmt::Display for EcosystemIncentiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcosystemIncentiveType::DeveloperRewards => write!(f, "developer_rewards"),
            EcosystemIncentiveType::ReferralProgram => write!(f, "referral_program"),
            EcosystemIncentiveType::CommunityContributions => write!(f, "community_contributions"),
            EcosystemIncentiveType::StakingRewards => write!(f, "staking_rewards"),
        }
    }
}

/// Neo ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoEcosystemIntegration {
    /// Integration ID
    pub id: String,

    /// Integration name
    pub name: String,

    /// Integration description
    pub description: String,

    /// Integration type
    pub integration_type: NeoEcosystemIntegrationType,

    /// Integration details
    pub details: serde_json::Value,

    /// Is the integration enabled?
    pub enabled: bool,
}

/// Neo ecosystem integration type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeoEcosystemIntegrationType {
    /// Neo N3 governance
    Governance,

    /// Neo N3 DeFi
    DeFi,

    /// Neo N3 NFTs
    NFTs,

    /// Neo N3 dApps
    DApps,
}

impl std::fmt::Display for NeoEcosystemIntegrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeoEcosystemIntegrationType::Governance => write!(f, "governance"),
            NeoEcosystemIntegrationType::DeFi => write!(f, "defi"),
            NeoEcosystemIntegrationType::NFTs => write!(f, "nfts"),
            NeoEcosystemIntegrationType::DApps => write!(f, "dapps"),
        }
    }
}

/// User billing profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBillingProfile {
    /// User ID
    pub user_id: String,

    /// Pricing tier
    pub tier: PricingTier,

    /// Subscription model
    pub subscription: Option<String>,

    /// Subscription start date
    pub subscription_start_date: Option<u64>,

    /// Subscription end date
    pub subscription_end_date: Option<u64>,

    /// Resource usage
    pub resource_usage: HashMap<ResourceType, u64>,

    /// Billing history
    pub billing_history: Vec<BillingRecord>,

    /// Payment methods
    pub payment_methods: Vec<PaymentMethod>,

    /// Default payment method
    pub default_payment_method: Option<String>,

    /// Value-added services
    pub value_added_services: Vec<String>,

    /// Earned incentives
    pub earned_incentives: Vec<EarnedIncentive>,

    /// Neo ecosystem integrations
    pub neo_integrations: Vec<String>,
}

/// Billing record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingRecord {
    /// Record ID
    pub id: String,

    /// Billing date
    pub date: u64,

    /// Billing amount (in GAS)
    pub amount: f64,

    /// Billing description
    pub description: String,

    /// Billing items
    pub items: Vec<BillingItem>,

    /// Payment status
    pub payment_status: PaymentStatus,

    /// Payment method
    pub payment_method: Option<String>,

    /// Payment date
    pub payment_date: Option<u64>,
}

/// Billing item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingItem {
    /// Item description
    pub description: String,

    /// Resource type
    pub resource_type: Option<ResourceType>,

    /// Quantity
    pub quantity: u64,

    /// Unit price (in GAS)
    pub unit_price: f64,

    /// Total price (in GAS)
    pub total_price: f64,
}

/// Payment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Pending payment
    Pending,

    /// Paid
    Paid,

    /// Failed
    Failed,

    /// Refunded
    Refunded,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Pending => write!(f, "pending"),
            PaymentStatus::Paid => write!(f, "paid"),
            PaymentStatus::Failed => write!(f, "failed"),
            PaymentStatus::Refunded => write!(f, "refunded"),
        }
    }
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Method ID
    pub id: String,

    /// Method type
    pub method_type: PaymentMethodType,

    /// Method details
    pub details: serde_json::Value,

    /// Is this method enabled?
    pub enabled: bool,
}

/// Payment method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentMethodType {
    /// Neo wallet
    NeoWallet,

    /// Ethereum wallet
    EthereumWallet,

    /// Credit card
    CreditCard,

    /// Bank transfer
    BankTransfer,
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethodType::NeoWallet => write!(f, "neo_wallet"),
            PaymentMethodType::EthereumWallet => write!(f, "ethereum_wallet"),
            PaymentMethodType::CreditCard => write!(f, "credit_card"),
            PaymentMethodType::BankTransfer => write!(f, "bank_transfer"),
        }
    }
}

/// Earned incentive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnedIncentive {
    /// Incentive ID
    pub incentive_id: String,

    /// Earned date
    pub earned_date: u64,

    /// Reward amount (in GAS)
    pub reward_amount: f64,

    /// Reward description
    pub description: String,

    /// Reward status
    pub status: IncentiveStatus,
}

/// Incentive status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncentiveStatus {
    /// Pending
    Pending,

    /// Approved
    Approved,

    /// Rejected
    Rejected,

    /// Paid
    Paid,
}

impl std::fmt::Display for IncentiveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncentiveStatus::Pending => write!(f, "pending"),
            IncentiveStatus::Approved => write!(f, "approved"),
            IncentiveStatus::Rejected => write!(f, "rejected"),
            IncentiveStatus::Paid => write!(f, "paid"),
        }
    }
}
