// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use crate::types::ToArrayBuffer;


pub trait ToScopedV8Value<'a> {
    fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value>;
}

pub trait ToV8Value<'a> {
    fn to_v8_value(self) -> v8::Local<'a, v8::Value>;
}

pub trait TryToScopedV8Value<'a> {
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>>;
}

pub trait TryToV8Value<'a> {
    fn try_to_v8_value(self) -> serde_v8::Result<v8::Local<'a, v8::Value>>;
}


macro_rules! to_scoped_v8_value {
    ($ty:ty : |$scope:ident, $value:ident| $expr:expr) => {
        impl<'a> ToScopedV8Value<'a> for $ty {
            #[inline]
            fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
                let $value = self;
                let $scope = scope;
                ($expr).into()
            }
        }
    }
}

to_scoped_v8_value!((): |scope, _value| v8::null(scope));
to_scoped_v8_value!(bool: |scope, value| v8::Boolean::new(scope, value));

to_scoped_v8_value!(i8: |scope, value| v8::Integer::new(scope, value as i32));
to_scoped_v8_value!(i16: |scope, value| v8::Integer::new(scope, value as i32));
to_scoped_v8_value!(i32: |scope, value| v8::Integer::new(scope, value));

to_scoped_v8_value!(u8: |scope, value| v8::Integer::new_from_unsigned(scope, value as u32));
to_scoped_v8_value!(u16: |scope, value| v8::Integer::new_from_unsigned(scope, value as u32));
to_scoped_v8_value!(u32: |scope, value| v8::Integer::new_from_unsigned(scope, value));

to_scoped_v8_value!(f32: |scope, value| v8::Number::new(scope, value as f64));
to_scoped_v8_value!(f64: |scope, value| v8::Number::new(scope, value));

to_scoped_v8_value!(i64: |scope, value| v8::BigInt::new_from_i64(scope, value));
to_scoped_v8_value!(u64: |scope, value| v8::BigInt::new_from_u64(scope, value));

to_scoped_v8_value!(isize: |scope, value| v8::BigInt::new_from_i64(scope, value as i64));
to_scoped_v8_value!(usize: |scope, value| v8::BigInt::new_from_u64(scope, value as u64));


impl<'a, T> ToScopedV8Value<'a> for v8::Global<T>
    where v8::Local<'a, v8::Value>: From<v8::Local<'a, T>>
{
    #[inline]
    fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        v8::Local::new(scope, self).into()
    }
}

impl<'a, T> ToV8Value<'a> for v8::Local<'a, T>
    where v8::Local<'a, v8::Value>: From<v8::Local<'a, T>>
{
    #[inline]
    fn to_v8_value(self) -> v8::Local<'a, v8::Value> { self.into() }
}

impl<'a> ToScopedV8Value<'a> for serde_v8::V8Slice<u8> {
    #[inline]
    fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        self.into_v8_unsliced_arraybuffer_local(scope).into()
    }
}

impl<'a, T: ToScopedV8Value<'a>> ToScopedV8Value<'a> for Option<T> {
    #[inline]
    fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        self.map(|v| v.to_scoped_v8_value(scope))
            .unwrap_or_else(|| v8::null(scope).into())
    }
}


// implements for TryToScopedV8Value


impl<'a> TryToScopedV8Value<'a> for &str {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        let len = self.len();
        v8::String::new(scope, self)
            .map(|v| v.into())
            .ok_or_else(|| serde_v8::Error::Message(format!("String: alloc len({}) failed", len)))
    }
}

impl<'a> TryToScopedV8Value<'a> for &String {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        self.as_str().try_to_scoped_v8_value(scope)
    }
}

impl<'a> TryToScopedV8Value<'a> for Box<[u8]> {
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        let len = self.len();
        let buffer = unsafe {
            v8::Local::cast(self.to_array_buffer().to_scoped_v8_value(scope))
        };

        v8::Uint8Array::new(scope, buffer, 0, len)
            .map(|v| v.into())
            .ok_or_else(|| serde_v8::Error::Message(format!("Uint8Array: alloc len({}) failed", len)))
    }
}

impl<'a> TryToScopedV8Value<'a> for Vec<u8> {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        self.into_boxed_slice().try_to_scoped_v8_value(scope)
    }
}

impl<'a> TryToScopedV8Value<'a> for serde_v8::V8Slice<u8> {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        let len = self.len();
        self.into_v8_local(scope)
            .map(|v| v.into())
            .ok_or_else(|| serde_v8::Error::Message(format!("Uint8Array: alloc len({}) failed", len)))
    }
}

impl<'a, T: TryToScopedV8Value<'a>> TryToScopedV8Value<'a> for Option<T> {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        self.map(|v| v.try_to_scoped_v8_value(scope))
            .unwrap_or_else(|| Ok(v8::null(scope).into()))
    }
}


pub struct V8Serialize<'a, T: serde::Serialize> {
    value: &'a T,
}

pub trait ToV8Serialize<T: serde::Serialize> {
    fn to_v8_serialize(&self) -> V8Serialize<T>;
}

impl<T: serde::Serialize> ToV8Serialize<T> for T {
    fn to_v8_serialize(&self) -> V8Serialize<T> { V8Serialize { value: self } }
}

impl<'a, 'b, T: serde::Serialize> TryToScopedV8Value<'a> for V8Serialize<'b, T> {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        serde_v8::to_v8(scope, self.value)
    }
}


// for js Number

pub trait ToV8Number<'a> {
    fn to_v8_number(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Number>;
}

macro_rules! to_v8_number {
   ($ty:ty : |$scope:ident, $value:ident| $expr:expr) => {
        impl<'a> ToV8Number<'a> for $ty {
            fn to_v8_number(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Number> {
                let $scope = scope;
                let $value = self;
                ($expr).into()
            }
        }
   }
}

to_v8_number!(f64: |scope, value| v8::Number::new(scope, value));
to_v8_number!(f32: |scope, value| v8::Number::new(scope, value as f64));

to_v8_number!(i8: |scope, value| v8::Integer::new(scope, value as i32));
to_v8_number!(i16: |scope, value| v8::Integer::new(scope, value as i32));
to_v8_number!(i32: |scope, value| v8::Integer::new(scope, value));

to_v8_number!(u8: |scope, value| v8::Integer::new_from_unsigned(scope, value as u32));
to_v8_number!(u16: |scope, value| v8::Integer::new_from_unsigned(scope, value as u32));
to_v8_number!(u32: |scope, value| v8::Integer::new_from_unsigned(scope, value));


#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Test {
        pub first: u32,
        pub second: bool,
    }

    #[test]
    fn test_v8_serialize() {
        r3e_core::initialize();

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        let mut scope = v8::HandleScope::new(&mut isolate);
        let cx = v8::Context::new(&mut scope);
        let mut scope = v8::ContextScope::new(&mut scope, cx);

        let t = Test { first: 123, second: true };
        let v = t.to_v8_serialize();
        let v = v.try_to_scoped_v8_value(&mut scope)
            .expect("try_to_scoped_v8_value should be ok");
        println!("serialize value {:?}", v);
    }
}