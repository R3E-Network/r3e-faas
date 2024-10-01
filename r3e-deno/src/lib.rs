// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod consts;
pub mod ext;

#[cfg(test)]
pub mod lib_test;

pub use deno_core::op2 as js_op;

use deno_core::error::JsError;
use deno_core::{v8, Extension, JsRuntime as Runtime, RuntimeOptions};
use serde::Serialize;

use r3e_core::make_v8_platform;

pub use {consts::*, ext::*};

const DEFAULT_INIT_HEAP_SIZE: usize = 1 * 1024 * 1024;
const DEFAULT_MAX_HEAP_SIZE: usize = 128 * 1024 * 1024;

#[derive(Debug)]
pub struct RuntimeConfig {
    pub max_heap_size: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_heap_size: DEFAULT_MAX_HEAP_SIZE,
        }
    }
}

pub struct JsRuntime {
    runtime: Runtime,
    // reactor: Arc<tokio::runtime::Runtime>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("exec: on load module: {0}")]
    OnLoad(String),

    #[error("exec: on compile: {0}")]
    OnCompile(&'static str),

    #[error("exec: on execute: {0}")]
    OnExecute(String),
}

impl JsRuntime {
    pub fn new(config: RuntimeConfig) -> Self {
        let allows: Extension = Extension {
            name: "allows",
            middleware_fn: Some(Box::new(op_allowed)),
            ..Default::default()
        };

        // enable jitless for v8
        v8::V8::set_flags_from_string("--jitless");
        let create_params =
            v8::CreateParams::default().heap_limits(DEFAULT_INIT_HEAP_SIZE, config.max_heap_size);

        // let _enter = reactor.enter();
        let runtime = Runtime::new(RuntimeOptions {
            v8_platform: Some(make_v8_platform()),
            extensions: vec![allows, r3e::init_ops_and_esm()],
            create_params: Some(create_params),
            ..Default::default()
        });
        // runtime.add_near_heap_limit_callback(cb);

        Self { runtime }
    }

    // must execute in the tokio context
    pub fn execute(&mut self, code: &str) -> Result<(), ExecError> {
        // let _enter = self.reactor.enter();

        let mut scope = self.runtime.handle_scope();
        let script = v8::String::new(&mut scope, code)
            .ok_or_else(|| ExecError::OnCompile("code too long"))?;

        let script = v8::Script::compile(&mut scope, script, None)
            .ok_or_else(|| ExecError::OnCompile("code compile failed"))?;

        let mut catch = v8::TryCatch::new(&mut scope);
        let _rv = script.run(&mut catch).ok_or_else(|| {
            if let Some(ex) = catch.exception() {
                let js_err = JsError::from_v8_exception(&mut catch, ex);
                return ExecError::OnExecute(js_err.to_string());
            }
            return ExecError::OnExecute("script run failed".into());
        })?;

        Ok(())
    }

    pub async fn load_main_module(&mut self, code: String) -> Result<usize, ExecError> {
        let specifier = deno_core::resolve_url("file://main.js").unwrap();
        // let _enter = self.reactor.enter();
        // let module = self
        //     .reactor
        //     .block_on(self.runtime.load_main_es_module_from_code(&specifier, code))
        //     .map_err(|err| ExecError::OnLoad(err.to_string()))?;

        let module = self
            .runtime
            .load_main_es_module_from_code(&specifier, code)
            .await
            .map_err(|err| ExecError::OnLoad(err.to_string()))?;

        Ok(module)
    }

    pub async fn eval_module(&mut self, module: usize) -> Result<(), ExecError> {
        // let _enter = self.reactor.enter();
        // let _rv = self
        //     .reactor
        //     .block_on(self.runtime.mod_evaluate(module))
        //     .map_err(|err| ExecError::OnExecute(err.to_string()))?;

        let _rv = self
            .runtime
            .mod_evaluate(module)
            .await
            .map_err(|err| ExecError::OnExecute(err.to_string()))?;

        Ok(())
    }

    // call it must after eval_module completed
    pub async fn run_module_default(
        &mut self,
        module: usize,
        args: &[v8::Global<v8::Value>],
    ) -> Result<(), ExecError> {
        let default_fn = {
            let module = self
                .runtime
                .get_module_namespace(module)
                .map_err(|err| ExecError::OnExecute(err.to_string()))?;

            let scope = &mut self.runtime.handle_scope();
            let module = v8::Local::<v8::Object>::new(scope, module);

            let default_name = v8::String::new(scope, "default").unwrap();
            let default_export = module
                .get(scope, default_name.into())
                .ok_or_else(|| ExecError::OnExecute("default export not found".into()))?;

            let default_fn = v8::Local::<v8::Function>::try_from(default_export)
                .map_err(|_err| ExecError::OnExecute("default export is not a function".into()))?;

            v8::Global::new(scope, default_fn)
        };

        // let _enter = self.reactor.enter();
        let options = Default::default();
        let call = self.runtime.call_with_args(&default_fn, args);
        // let _rv = self
        //     .reactor
        //     .block_on(self.runtime.with_event_loop_promise(call, options))
        //     .map_err(|err| ExecError::OnExecute(err.to_string()))?;

        let _rv = self
            .runtime
            .with_event_loop_promise(call, options)
            .await
            .map_err(|err| ExecError::OnExecute(err.to_string()))?;

        Ok(())
    }

    pub fn to_global(
        &mut self,
        value: &impl Serialize,
    ) -> Result<v8::Global<v8::Value>, serde_v8::Error> {
        let scope = &mut self.runtime.handle_scope();
        let value = serde_v8::to_v8(scope, value)?;
        Ok(v8::Global::new(scope, value))
    }

    pub fn heap_stats(&mut self) -> v8::HeapStatistics {
        let mut stats = v8::HeapStatistics::default();
        self.runtime.v8_isolate().get_heap_statistics(&mut stats);
        stats
    }

    #[inline]
    pub fn terminate(&mut self) {
        self.runtime.v8_isolate().terminate_execution();
    }

    pub fn enter(&mut self) {
        let isolate = self.runtime.v8_isolate();
        unsafe { isolate.enter() };
    }

    // pub fn new_context(&mut self) -> v8::Global<v8::Context> {
    //     let scope = &mut self.runtime.handle_scope();
    //     let cx: v8::Local<'_, v8::Context> = v8::Context::new(scope, v8::ContextOptions::default());
    //     v8::Global::new(scope, cx)
    // }
}
