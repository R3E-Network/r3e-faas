// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Core
//!
//! Core functionality and shared types for the R3E FaaS platform.

pub mod config;
pub mod encoding;
pub mod error;
pub mod types;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

pub use error::{Error, Result};
pub use r3e_proc_macros::BytesLike;
pub use types::Platform;

pub const GIT_VERSION: &str = git_version::git_version!();
pub const BUILD_DATE: &str = compile_time::date_str!();

// Use a function for Platform::current() to make it non-const
fn current_platform_str() -> &'static str {
    Platform::current().to_str()
}

pub const VERSION: &str = "R3E Platform";

/// Create a new V8 platform
#[inline]
pub fn make_v8_platform(config: &config::V8Config) -> v8::SharedRef<v8::Platform> {
    v8::new_default_platform(config.worker_threads as u32, config.background_compilation)
        .make_shared()
}

/// Initialize V8 engine
pub fn v8_initialize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let config = config::V8Config::default();
        let platform = make_v8_platform(&config);
        v8::V8::initialize_platform(platform.clone());
        v8::V8::initialize();
        // cppgc module is not available in this version of v8
        // v8::cppgc::initalize_process(platform);
    });
}

/// Finalize V8 engine
pub fn v8_finalize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        // cppgc module is not available in this version of v8
        // unsafe { v8::cppgc::shutdown_process() };
        unsafe { v8::V8::dispose() };
        v8::V8::dispose_platform();
    })
}

/// Register signal hooks
pub fn signal_hooks(name: &'static str, flag: Arc<AtomicBool>) -> Result<()> {
    unsafe {
        let flag = flag.clone();
        signal_hook::low_level::register(signal_hook::consts::SIGINT, move || {
            log::warn!("{},{} SIGINT received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .map_err(|e| Error::SignalHook(format!("Failed to register SIGINT signal hook: {}", e)))?;
    }

    unsafe {
        let flag = flag.clone();
        signal_hook::low_level::register(signal_hook::consts::SIGTERM, move || {
            log::warn!("{},{} SIGTERM received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .map_err(|e| Error::SignalHook(format!("Failed to register SIGTERM signal hook: {}", e)))?;
    }

    unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGHUP, move || {
            log::warn!("{},{} SIGHUP received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .map_err(|e| Error::SignalHook(format!("Failed to register SIGHUP signal hook: {}", e)))?;
    }

    Ok(())
}
