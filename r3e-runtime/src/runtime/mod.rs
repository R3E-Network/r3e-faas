// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


pub mod event;
pub mod extension;
pub mod jscall;
pub mod stats;


pub use {event::*, extension::*, jscall::*, stats::*};


pub struct JsRuntimeConfig {
    pub extensions: Vec<Extension>,
}


#[allow(dead_code)]
pub struct JsRuntime {
    isolate: v8::OwnedIsolate,
    extensions: Vec<Extension>,
}


impl JsRuntime {
    pub fn new(config: JsRuntimeConfig) -> Self {
        r3e_core::initialize();

        let params = v8::CreateParams::default();
        let mut isolate = v8::Isolate::new(params);
        let global = new_global_context(&mut isolate);
        {
            let mut scope = v8::HandleScope::with_context(&mut isolate, &global);
            let cx = v8::Local::new(&mut scope, &global);
            for ext in config.extensions.iter() {
                ext.register(&mut scope, &cx);
            }
        }

        Self { isolate, extensions: config.extensions }
    }
}


#[inline]
pub fn new_global_context(isolate: &mut v8::OwnedIsolate) -> v8::Global<v8::Context> {
    let scope = &mut v8::HandleScope::new(isolate);
    let tpl = v8::ObjectTemplate::new(scope);
    let cx = v8::Context::new_from_template(scope, tpl);

    // let scope = &mut v8::ContextScope::new(scope, cx);
    v8::Global::new(scope, cx)
}
