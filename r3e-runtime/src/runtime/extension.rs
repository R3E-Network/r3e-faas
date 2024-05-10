// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use v8::NewStringType::Normal;
use crate::runtime::JsCall;


pub struct Extension {
    pub name: &'static str,
    pub calls: Vec<JsCall>,
}

pub struct NamedV8Fn<'a> {
    pub name: v8::Local<'a, v8::String>,
    pub v8_fn: v8::Local<'a, v8::Function>,
}

impl Extension {
    pub fn build_v8_fn<'a>(&self, scope: &mut v8::HandleScope<'a>) -> Vec<NamedV8Fn<'a>> {
        self.calls.iter()
            .map(|call| build_v8_fn(scope, call))
            .collect()
    }

    pub fn register<'a>(&self, scope: &mut v8::HandleScope<'a>, cx: &v8::Local<'a, v8::Context>) {
        let v8_fns = self.build_v8_fn(scope);
        let global = cx.global(scope);

        let ext = v8::Object::new(scope);
        let ext_name = v8::String::new_from_one_byte(scope, self.name.as_bytes(), Normal)
            .expect("v8::String::new_from_one_byte should be ok");

        for f in v8_fns {
            ext.set(scope, f.name.into(), f.v8_fn.into())
                .expect("ext.set should be ok");
        }

        global.set(scope, ext_name.into(), ext.into())
            .expect("global.set should be ok");
    }
}

pub fn build_v8_fn<'a>(scope: &mut v8::HandleScope<'a>, call: &JsCall) -> NamedV8Fn<'a> {
    // let state = call as *const JsCall as *mut core::ffi::c_void;
    // let state = v8::External::new(scope, state);
    let builder = v8::FunctionTemplate::builder_raw(call.trivial_call)
        // .data(state.into())
        .length(call.params_count() as i32);

    let template = if let Some(fc) = call.fast_call.as_ref() {
        builder.build_fast(scope, &fc.fast_fn, Some(&fc.fn_info), None, None)
    } else {
        builder.build(scope)
    };

    let v8_fn = template.get_function(scope)
        .expect("template.get_function should be ok");

    let name = v8::String::new_from_one_byte(scope, call.name.as_bytes(), Normal)
        .expect("v8::String::new_from_one_byte should be ok");

    v8_fn.set_name(name);
    NamedV8Fn { name, v8_fn }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::runtime::{new_global_context, ReturnType};
    use crate::types::TrySetV8Value;

    extern "C" fn print(info: *const v8::FunctionCallbackInfo) {
        let info = unsafe { &*info };
        let mut rv = v8::ReturnValue::from_function_callback_info(info);
        let mut scope = unsafe { v8::CallbackScope::new(info) };
        rv.try_set_v8_value(&mut scope, r#"{"Hello": "World"}"#)
            .expect("try_set_v8_value should be ok");
        println!("info {:?}", info);
    }


    #[test]
    fn test_extension() {
        r3e_core::initialize();

        let call = JsCall {
            name: "print",
            params: Vec::new(),
            return_type: ReturnType::Void,
            is_async: false,
            trivial_call: print,
            fast_call: None,
        };

        let ext = Extension {
            name: "r4e",
            calls: vec![call],
        };

        let params = v8::CreateParams::default();
        let mut isolate = v8::Isolate::new(params);
        let global = new_global_context(&mut isolate);

        let mut scope = v8::HandleScope::with_context(&mut isolate, &global);
        let cx = v8::Local::new(&mut scope, &global);

        ext.register(&mut scope, &cx);

        let code = r#"
            let v = JSON.parse(r4e.print("xxx"));
            v["First"] = 1;
            JSON.stringify(v);
        "#;
        let code = v8::String::new(&mut scope, code)
            .expect("v8::String::new should be ok");

        let script = v8::Script::compile(&mut scope, code, None)
            .expect("v8::Script::compile should be ok");

        let result = script.run(&mut scope)
            .expect("script.run should be ok");

        let result = result.to_string(&mut scope)
            .expect("result.to_string should be ok");
        std::println!("result {}", result.to_rust_string_lossy(&mut scope).as_str());
    }
}
