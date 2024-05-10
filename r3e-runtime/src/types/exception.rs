// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use crate::types::{ToRustBytesError, ToRustNumberError, ToRustStringError};


pub trait ThrowJsException<'a> {
    fn throw_js_exception(&self, scope: &mut v8::HandleScope<'a>);
}


macro_rules! throw_js_type_error {
    ($ty:ty) => {
        impl<'a> ThrowJsException<'a> for $ty {
            fn throw_js_exception(&self, scope: &mut v8::HandleScope<'a>) {
                let msg = self.to_string();
                let msg = v8::String::new_from_one_byte(scope, msg.as_bytes(), v8::NewStringType::Normal)
                    .expect("`v8::String::new_from_one_byte` should be ok");

                let except = v8::Exception::type_error(scope, msg);
                scope.throw_exception(except);
            }
        }
    };
}

throw_js_type_error!(ToRustNumberError);
throw_js_type_error!(ToRustStringError);
throw_js_type_error!(ToRustBytesError);
