// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod encoding;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

pub use r3e_proc_macros::BytesLike;

pub const GIT_VERSION: &str = git_version::git_version!();
pub const BUILD_DATE: &str = compile_time::date_str!();

pub const VERSION: &str =
    const_format::concatcp!("(", platform(), "; ", BUILD_DATE, "_", GIT_VERSION, ")");

#[inline]
pub fn make_v8_platform() -> v8::SharedRef<v8::Platform> {
    v8::new_default_platform(0, false).make_shared()
}

pub fn v8_initialize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let platform = make_v8_platform();
        v8::V8::initialize_platform(platform.clone());
        v8::V8::initialize();
        v8::cppgc::initalize_process(platform);
    });
}

pub fn v8_finalize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        unsafe { v8::cppgc::shutdown_process() };
        unsafe { v8::V8::dispose() };
        v8::V8::dispose_platform();
    })
}

pub fn signal_hooks(name: &'static str, flag: Arc<AtomicBool>) {
    unsafe {
        let flag = flag.clone();
        signal_hook::low_level::register(signal_hook::consts::SIGINT, move || {
            log::warn!("{},{} SIGINT received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .expect("register SIGINT signal hook");
    }

    unsafe {
        let flag = flag.clone();
        signal_hook::low_level::register(signal_hook::consts::SIGTERM, move || {
            log::warn!("{},{} SIGTERM received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .expect("register SIGTERM signal hook");
    }

    unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGHUP, move || {
            log::warn!("{},{} SIGHUP received", name, std::process::id());
            flag.store(true, Ordering::SeqCst);
        })
        .expect("register SIGHUP signal hook");
    }
}

pub const fn platform() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "wasi") {
        "wasi"
    } else {
        "unknown"
    }
}
