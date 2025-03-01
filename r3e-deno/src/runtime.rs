// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::error::JsError;
use deno_core::{v8, Extension, JsRuntime as Runtime, RuntimeOptions};
use serde::Serialize;

use crate::ext::op_allowed;
use crate::sandbox::{create_v8_flags, create_v8_params, SandboxConfig, SandboxContext};
use r3e_core::make_v8_platform;

#[derive(Debug)]
pub struct RuntimeConfig {
    pub max_heap_size: usize,
    pub sandbox_config: Option<SandboxConfig>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_heap_size: 128 * 1024 * 1024, // 128MB
            sandbox_config: None,
        }
    }
}

pub struct JsRuntime {
    runtime: Runtime,
    sandbox_context: Option<SandboxContext>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("exec: on load module: {0}")]
    OnLoad(String),

    #[error("exec: on compile: {0}")]
    OnCompile(&'static str),

    #[error("exec: on execute: {0}")]
    OnExecute(String),

    #[error("exec: sandbox violation: {0}")]
    SandboxViolation(String),

    #[error("exec: timeout: execution exceeded time limit")]
    Timeout,
}

impl JsRuntime {
    pub fn new(config: RuntimeConfig) -> Self {
        let allows: Extension = Extension {
            name: "allows",
            middleware_fn: Some(Box::new(op_allowed)),
            ..Default::default()
        };

        // Set up sandbox if configured
        let sandbox_config = config
            .sandbox_config
            .clone()
            .unwrap_or_else(|| SandboxConfig::default());

        // Set V8 flags based on sandbox configuration
        let v8_flags = create_v8_flags(&sandbox_config);
        v8::V8::set_flags_from_string(&v8_flags);

        // Create V8 parameters
        let create_params = create_v8_params(&sandbox_config);

        // Create runtime
        let mut runtime = Runtime::new(RuntimeOptions {
            v8_platform: Some(make_v8_platform()),
            extensions: vec![allows, crate::r3e::init_ops_and_esm()],
            create_params: Some(create_params),
            ..Default::default()
        });

        // Create sandbox context if needed
        let sandbox_context = if config.sandbox_config.is_some() {
            Some(SandboxContext::new(sandbox_config, runtime.v8_isolate()))
        } else {
            None
        };

        Self {
            runtime,
            sandbox_context,
        }
    }

    // must execute in the tokio context
    pub fn execute(&mut self, code: &str) -> Result<(), ExecError> {
        let mut scope = self.runtime.handle_scope();
        let script = v8::String::new(&mut scope, code)
            .ok_or_else(|| ExecError::OnCompile("code too long"))?;

        let script = v8::Script::compile(&mut scope, script, None)
            .ok_or_else(|| ExecError::OnCompile("code compile failed"))?;

        let mut catch = v8::TryCatch::new(&mut scope);
        let _rv = script.run(&mut catch).ok_or_else(|| {
            if let Some(ex) = catch.exception() {
                let js_err = JsError::from_v8_exception(&mut catch, ex);

                // Check if this is a termination exception (timeout)
                if catch.is_execution_terminating() {
                    return ExecError::Timeout;
                }

                return ExecError::OnExecute(js_err.to_string());
            }
            return ExecError::OnExecute("script run failed".into());
        })?;

        Ok(())
    }

    pub async fn load_main_module(&mut self, code: String) -> Result<usize, ExecError> {
        let specifier = deno_core::resolve_url("file://main.js").unwrap();
        let module = self
            .runtime
            .load_main_es_module_from_code(&specifier, code)
            .await
            .map_err(|err| ExecError::OnLoad(err.to_string()))?;

        Ok(module)
    }

    pub async fn eval_module(&mut self, module: usize) -> Result<(), ExecError> {
        let result = self.runtime.mod_evaluate(module).await.map_err(|err| {
            // Check if this is a termination exception (timeout)
            if err.to_string().contains("execution terminated") {
                return ExecError::Timeout;
            }
            ExecError::OnExecute(err.to_string())
        })?;

        Ok(result)
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

        let options = Default::default();
        let call = self.runtime.call_with_args(&default_fn, args);
        let result = self
            .runtime
            .with_event_loop_promise(call, options)
            .await
            .map_err(|err| {
                // Check if this is a termination exception (timeout)
                if err.to_string().contains("execution terminated") {
                    return ExecError::Timeout;
                }
                ExecError::OnExecute(err.to_string())
            })?;

        Ok(result)
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
}
