#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcBlock {
    #[prost(uint32, tag = "1")]
    pub height: u32,
    #[prost(uint32, tag = "2")]
    pub magic: u32,
    #[prost(uint32, tag = "3")]
    pub size: u32,
    #[prost(message, optional, tag = "4")]
    pub header: ::core::option::Option<BtcBlockHeader>,
    #[prost(message, repeated, tag = "5")]
    pub txs: ::prost::alloc::vec::Vec<BtcTx>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcBlockHeader {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(string, tag = "2")]
    pub prev_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub merkle_root: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub time: u64,
    #[prost(uint32, tag = "5")]
    pub bits: u32,
    #[prost(uint64, tag = "6")]
    pub nonce: u64,
}
/// TODO: Implement
///
/// string hash = 1;
/// uint32 version = 2;
/// uint32 lock_time = 3;
/// uint32 size = 4;
/// repeated BtcVin vin = 5;
/// repeated BtcVout vout = 6;
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcTx {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoBlock {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<NeoBlockHeader>,
    #[prost(message, repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<NeoTx>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoBlockHeader {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub version: u32,
    #[prost(string, tag = "3")]
    pub prev_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub merkle_root: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub time: u64,
    #[prost(uint64, tag = "6")]
    pub nonce: u64,
    #[prost(uint32, tag = "7")]
    pub height: u32,
    #[prost(uint32, tag = "8")]
    pub primary: u32,
    #[prost(string, tag = "9")]
    pub next_consensus: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "10")]
    pub witnesses: ::prost::alloc::vec::Vec<NeoWitness>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoWitness {
    #[prost(string, tag = "1")]
    pub invocation_script: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub verification_script: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoTx {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub size: u32,
    #[prost(uint32, tag = "3")]
    pub version: u32,
    #[prost(uint32, tag = "4")]
    pub nonce: u32,
    #[prost(uint64, tag = "5")]
    pub sysfee: u64,
    #[prost(uint64, tag = "6")]
    pub netfee: u64,
    #[prost(uint32, tag = "7")]
    pub valid_until_block: u32,
    #[prost(message, repeated, tag = "8")]
    pub signers: ::prost::alloc::vec::Vec<NeoSigner>,
    #[prost(message, repeated, tag = "9")]
    pub attributes: ::prost::alloc::vec::Vec<NeoTxAttr>,
    #[prost(string, tag = "10")]
    pub script: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "11")]
    pub witnesses: ::prost::alloc::vec::Vec<NeoWitness>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoSigner {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub scopes: u32,
    #[prost(string, repeated, tag = "3")]
    pub allowed_contract: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub allowed_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub rules: ::prost::alloc::vec::Vec<NeoWitnessRule>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoWitnessRule {
    #[prost(enumeration = "NeoWitnessAction", tag = "1")]
    pub action: i32,
    #[prost(message, optional, tag = "2")]
    pub condition: ::core::option::Option<NeoWitnessCondition>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoWitnessCondition {
    #[prost(
        oneof = "neo_witness_condition::Condition",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub condition: ::core::option::Option<neo_witness_condition::Condition>,
}
/// Nested message and enum types in `NeoWitnessCondition`.
pub mod neo_witness_condition {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(bool, tag = "1")]
        Boolean(bool),
        #[prost(string, tag = "2")]
        Not(::prost::alloc::string::String),
        #[prost(message, tag = "3")]
        And(super::AndCondition),
        #[prost(message, tag = "4")]
        Or(super::OrCondition),
        #[prost(string, tag = "5")]
        ScriptHash(::prost::alloc::string::String),
        #[prost(string, tag = "6")]
        Group(::prost::alloc::string::String),
        #[prost(bool, tag = "7")]
        CalledByEntry(bool),
        #[prost(string, tag = "8")]
        CalledByContract(::prost::alloc::string::String),
        #[prost(string, tag = "9")]
        CalledByGroup(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndCondition {
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<NeoWitnessCondition>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrCondition {
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<NeoWitnessCondition>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoTxAttr {
    #[prost(oneof = "neo_tx_attr::Attr", tags = "1, 2, 3, 4")]
    pub attr: ::core::option::Option<neo_tx_attr::Attr>,
}
/// Nested message and enum types in `NeoTxAttr`.
pub mod neo_tx_attr {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type", content = "value")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Attr {
        #[prost(bool, tag = "1")]
        HighPriority(bool),
        #[prost(message, tag = "2")]
        OracleResponse(super::NeoOracleResponse),
        #[prost(message, tag = "3")]
        NotValidBefore(super::NeoNotValidBefore),
        #[prost(message, tag = "4")]
        Conflicts(super::NeoConflicts),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoOracleResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(enumeration = "NeoOracleCode", tag = "2")]
    pub code: i32,
    #[prost(string, tag = "3")]
    pub result: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoNotValidBefore {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoConflicts {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NeoWitnessAction {
    Deny = 0,
    Allow = 1,
}
impl NeoWitnessAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NeoWitnessAction::Deny => "DENY",
            NeoWitnessAction::Allow => "ALLOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DENY" => Some(Self::Deny),
            "ALLOW" => Some(Self::Allow),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NeoOracleCode {
    Success = 0,
    ProtocolNotSupported = 16,
    ConsensusUnreachable = 18,
    NotFound = 20,
    Timeout = 22,
    Forbidden = 24,
    ResponseTooLarge = 26,
    InsufficientFunds = 28,
    ContentTypeNotSupported = 31,
    Error = 255,
}
impl NeoOracleCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NeoOracleCode::Success => "SUCCESS",
            NeoOracleCode::ProtocolNotSupported => "PROTOCOL_NOT_SUPPORTED",
            NeoOracleCode::ConsensusUnreachable => "CONSENSUS_UNREACHABLE",
            NeoOracleCode::NotFound => "NOT_FOUND",
            NeoOracleCode::Timeout => "TIMEOUT",
            NeoOracleCode::Forbidden => "FORBIDDEN",
            NeoOracleCode::ResponseTooLarge => "RESPONSE_TOO_LARGE",
            NeoOracleCode::InsufficientFunds => "INSUFFICIENT_FUNDS",
            NeoOracleCode::ContentTypeNotSupported => "CONTENT_TYPE_NOT_SUPPORTED",
            NeoOracleCode::Error => "ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "PROTOCOL_NOT_SUPPORTED" => Some(Self::ProtocolNotSupported),
            "CONSENSUS_UNREACHABLE" => Some(Self::ConsensusUnreachable),
            "NOT_FOUND" => Some(Self::NotFound),
            "TIMEOUT" => Some(Self::Timeout),
            "FORBIDDEN" => Some(Self::Forbidden),
            "RESPONSE_TOO_LARGE" => Some(Self::ResponseTooLarge),
            "INSUFFICIENT_FUNDS" => Some(Self::InsufficientFunds),
            "CONTENT_TYPE_NOT_SUPPORTED" => Some(Self::ContentTypeNotSupported),
            "ERROR" => Some(Self::Error),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MockEvent {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    #[prost(message, repeated, tag = "1")]
    #[serde(flatten)]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Map {
    #[prost(map = "string, message", tag = "1")]
    #[serde(flatten)]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type", content = "value")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(int64, tag = "2")]
        Int64(i64),
        #[prost(bool, tag = "3")]
        Bool(bool),
        #[prost(message, tag = "4")]
        List(super::List),
        #[prost(message, tag = "5")]
        Map(super::Map),
    }
}
/// Protobuf event message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct Event {
    #[prost(bytes, tag = "1")]
    pub serialized_event: Vec<u8>,
    
    #[serde(skip)]
    #[prost(skip)]
    #[serde(flatten)]
    pub event: event::Event,
}

impl Event {
    pub fn new(event: event::Event) -> Self {
        let serialized = serde_json::to_vec(&event).unwrap_or_default();
        Self {
            serialized_event: serialized,
            event,
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            serialized_event: vec![],
            event: event::Event::None,
        }
    }
}

/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
    pub enum Event {
        /// None event
        #[serde(rename = "none")]
        None,
        /// Mock event
        #[serde(rename = "mock")]
        Mock(super::MockEvent),
        /// Bitcoin block
        #[serde(rename = "btc_block")]
        BtcBlock(serde_json::Value),
        /// Neo block
        #[serde(rename = "neo_block")]
        NeoBlock(super::NeoBlock),
        /// Neo contract notification
        #[serde(rename = "neo_contract_notification")]
        NeoContractNotification(super::NeoContractNotification),
        /// Neo application log
        #[serde(rename = "neo_application_log")]
        NeoApplicationLog(super::NeoApplicationLog),
        /// NEAR event
        #[serde(rename = "near_block")]
        NearBlock(serde_json::Value),
        /// NEAR account change
        #[serde(rename = "near_account_change")]
        NearAccountChange(serde_json::Value),
        /// NEAR transaction
        #[serde(rename = "near_transaction")]
        NearTransaction(serde_json::Value),
        /// Ethereum block
        #[serde(rename = "ethereum_block")]
        EthereumBlock(serde_json::Value),
        /// Ethereum transaction
        #[serde(rename = "ethereum_transaction")]
        EthereumTransaction(serde_json::Value),
        /// Ethereum contract event
        #[serde(rename = "ethereum_contract_event")]
        EthereumContractEvent {
            contract_address: String,
            events: Vec<serde_json::Value>,
        },
    }

    impl Default for Event {
        fn default() -> Self {
            Self::None
        }
    }
}

/// Neo Application Log
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoApplicationLog {
    #[prost(string, tag = "1")]
    pub tx_hash: String,
    #[prost(string, tag = "2")]
    pub application_log: String,
}

/// Neo Contract Notification
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeoContractNotification {
    #[prost(string, tag = "1")]
    pub tx_hash: String,
    #[prost(string, tag = "2")]
    pub notifications: String,
}
