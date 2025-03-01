// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Logging utilities for the FaaS service.

use std::sync::Once;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
};

static INIT: Once = Once::new();

/// Initialize the logging system
pub fn init_logging(service_name: &str, log_level: Option<&str>) {
    INIT.call_once(|| {
        let env_filter = match log_level {
            Some(level) => EnvFilter::new(level),
            None => EnvFilter::from_default_env(),
        };

        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(env_filter)
            .with_span_events(FmtSpan::CLOSE)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_ansi(true)
            .json()
            .finish();

        init_subscriber(subscriber);

        tracing::info!(
            service = service_name,
            version = env!("CARGO_PKG_VERSION"),
            "Service starting"
        );
    });
}

/// Initialize the subscriber
fn init_subscriber<S>(subscriber: S)
where
    S: Subscriber + Send + Sync + 'static,
{
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}

/// Log an error with context
#[macro_export]
macro_rules! log_error {
    ($err:expr, $($field:tt)+) => {
        tracing::error!(
            error = $err.to_string(),
            error_type = std::any::type_name_of_val(&$err),
            $($field)+
        )
    };
}

/// Log a warning with context
#[macro_export]
macro_rules! log_warn {
    ($err:expr, $($field:tt)+) => {
        tracing::warn!(
            error = $err.to_string(),
            error_type = std::any::type_name_of_val(&$err),
            $($field)+
        )
    };
}

/// Create a span for function execution
#[macro_export]
macro_rules! function_span {
    ($name:expr) => {
        let _span = tracing::info_span!("function", name = $name).entered();
    };
    ($name:expr, $($field:tt)+) => {
        let _span = tracing::info_span!("function", name = $name, $($field)+).entered();
    };
}

/// Create a span for service execution
#[macro_export]
macro_rules! service_span {
    ($name:expr) => {
        let _span = tracing::info_span!("service", name = $name).entered();
    };
    ($name:expr, $($field:tt)+) => {
        let _span = tracing::info_span!("service", name = $name, $($field)+).entered();
    };
}

/// Create a span for database operations
#[macro_export]
macro_rules! db_span {
    ($operation:expr) => {
        let _span = tracing::info_span!("database", operation = $operation).entered();
    };
    ($operation:expr, $($field:tt)+) => {
        let _span = tracing::info_span!("database", operation = $operation, $($field)+).entered();
    };
}
