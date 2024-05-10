// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


pub mod encoding;


use std::sync::Once;

pub use r3e_proc_macros::BytesLike;


pub fn initialize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let platform = v8::new_default_platform(1, false).make_shared();
        v8::V8::initialize_platform(platform.clone());
        v8::V8::initialize();
        // v8::cppgc::initalize_process(platform);
    });
}


pub fn finalize() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        // unsafe { v8::cppgc::shutdown_process() };
        unsafe { v8::V8::dispose(); }
        v8::V8::dispose_platform();
    })
}