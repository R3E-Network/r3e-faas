// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


pub mod module;
pub mod runtime;
pub mod types;


#[cfg(test)]
mod test {
    use core::ffi::c_void;
    use v8::NewStringType::Normal;


    #[deno_core::op2(fast)]
    pub fn string_call(#[string] name: &str, #[arraybuffer] buffer: &[u8]) -> u32 {
        (name.len() + buffer.len()) as u32
    }

    extern "C" fn near_heap_limit_callback(_: *mut c_void, current: usize, initial: usize) -> usize {
        println!("current {}, initial {}", current, initial);
        current
    }

    fn print(_scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
        println!("arguments print {:?}", args);
    }

    #[test]
    fn test_binding() {
        r3e_core::initialize();

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        isolate.add_near_heap_limit_callback(near_heap_limit_callback, core::ptr::null_mut());

        let mut scope = v8::HandleScope::new(&mut isolate);
        let cx = v8::Context::new(&mut scope);
        let mut scope = v8::ContextScope::new(&mut scope, cx);

        let global = cx.global(&mut scope);
        let r3e = v8::Object::new(&mut scope);
        let r3e_name = v8::String::new_from_one_byte(&mut scope, b"r3e", Normal)
            .expect("v8::String::new should be ok");

        global.set(&mut scope, r3e_name.into(), r3e.into())
            .expect("global.set should be ok");

        let template = v8::FunctionTemplate::new(&mut scope, print);
        let fn_impl = template.get_function(&mut scope)
            .expect("function should be exist");
        let fn_name = v8::String::new_from_one_byte(&mut scope, b"print", Normal)
            .expect("v8::String::new should be ok");

        r3e.set(&mut scope, fn_name.into(), fn_impl.into())
            .expect("r3e.set should be ok");

        let code = r#"JSON.stringify(r3e.print); "#;
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

    #[test]
    fn test_rusty_v8() {
        r3e_core::initialize();

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        let mut scope = v8::HandleScope::new(&mut isolate);

        let cx = v8::Context::new(&mut scope);
        let mut scope = v8::ContextScope::new(&mut scope, cx);
        let code = v8::String::new(&mut scope, "'Hello' + ' World!'")
            .expect("v8::String::new should be ok");

        let script = v8::Script::compile(&mut scope, code, None)
            .expect("v8::Script::compile should be ok");

        let result = script.run(&mut scope)
            .expect("script.run should be ok");

        let result = result.to_string(&mut scope)
            .expect("result.to_string should be ok");
        assert_eq!(result.to_rust_string_lossy(&mut scope).as_str(), "Hello World!");
    }
}