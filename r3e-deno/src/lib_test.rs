// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};

use r3e_event::{Source, Trigger};

use crate::*;

#[test]
fn test_execute() {
    let reactor = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let _enter = reactor.enter();

    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let code = r#"
        const { resources } = Deno.core;
        console.log("resources:", resources());

        // console.log("globalThis:", Object.getOwnPropertyNames(globalThis.__bootstrap));
        // console.log("ops:", Object.getOwnPropertyNames(Deno.core.ops));
    "#;
    runtime
        .execute(code.into())
        .expect("execute script should be ok");

    let code = r#"
        function test(n) {
            if (n == 100) {
                throw new Error("test error: " + n);
            }
            test(n + 1);
        }
        test(0);
    "#;

    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let err = runtime
        .execute(code.into())
        .expect_err("execute script should failed");
    std::println!("throwed err: {}", err);
}

#[tokio::test]
async fn test_load_module_fn() {
    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let code = r#"
        export default function() {
            console.log("hello module, default export");
            return "test";
        }
        console.log("hello module, module inner");
    "#;
    let module = runtime
        .load_main_module(code.into())
        .await
        .expect("load module should be ok");

    let _ = runtime
        .eval_module(module)
        .await
        .expect("eval module should be ok");

    runtime
        .run_module_default(module, &[])
        .await
        .expect("run module should be ok");
}

#[tokio::test]
async fn test_load_module_other() {
    let code = r#"
        export default 42;
    "#;
    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let module = runtime
        .load_main_module(code.into())
        .await
        .expect("load module should be ok");

    let _ = runtime
        .eval_module(module)
        .await
        .expect("eval module should be ok");

    let err = runtime
        .run_module_default(module, &[])
        .await
        .expect_err("run module should be failed");
    std::println!("load export default err: {}", err);
}

#[tokio::test]
async fn test_call_async_js_fn() {
    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let code = r#"
        const delay = (n) => new Promise(r => setTimeout(r, n * 1000));

        export default async function() {
            console.log("hello async module begin:", new Date());
            await delay(1);
            console.log("hello async module end:", new Date());
        }

        console.log("now:", new Date());
        r3e.defer(() => { console.log("defer:", new Date()); });
    "#;
    let module = runtime
        .load_main_module(code.into())
        .await
        .expect("load module should be ok");

    let _ = runtime
        .eval_module(module)
        .await
        .expect("eval module should be ok");

    runtime
        .run_module_default(module, &[])
        .await
        .expect("run module should be ok");
}

#[derive(Serialize, Deserialize)]
struct MockEvent {
    pub trigger: Trigger,
    pub trigger_time: u64,
    pub source: Source,
}

#[tokio::test]
async fn test_function_args() {
    let mut runtime = JsRuntime::new(RuntimeConfig::default());
    let code = r#"
        export default function(event) {
            console.log("event:", typeof event, event);
            assert(typeof event === "object", "event should be object");
            assert(event.trigger === "NewBlock", "event.trigger should be NewBlock");
            assert(event.trigger_time === 100, "event.trigger_time should be 100");
            assert(event.source === "Bitcoin", "event.source should be Bitcoin");
        }

        function assert(condition, msg) {
            if (!condition) {
                throw new Error(msg);
            }
        }
    "#;

    let module = runtime
        .load_main_module(code.into())
        .await
        .expect("load module should be ok");

    let _ = runtime
        .eval_module(module)
        .await
        .expect("eval module should be ok");

    let mock = MockEvent {
        trigger: Trigger::NewBlock,
        trigger_time: 100,
        source: Source::Bitcoin,
    };

    let event = runtime.to_global(&mock).expect("to global should be ok");
    runtime
        .run_module_default(module, &[event.clone()])
        .await
        .expect("run module should be ok");

    let scope = &mut runtime.runtime.handle_scope();
    let event = v8::Local::new(scope, event);
    let got: MockEvent = serde_v8::from_v8(scope, event).expect("from v8 should be ok");
    assert_eq!(got.trigger, Trigger::NewBlock);
    assert_eq!(got.trigger_time, 100);
    assert_eq!(got.source, Source::Bitcoin);
}
