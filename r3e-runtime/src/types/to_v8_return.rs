// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use crate::types::TryToScopedV8Value;


pub trait SetV8Value<T> {
    fn set_v8_value(&mut self, value: T);
}

pub trait TrySetV8Value<'a, T> {
    fn try_set_v8_value(&mut self, scope: &mut v8::HandleScope<'a>, value: T) -> serde_v8::Result<()>;
}


macro_rules! set_v8_value {
    ($ty:ty : |$rv:ident, $value:ident| $expr:expr) => {
        impl<'a> SetV8Value<$ty> for v8::ReturnValue<'a> {
             #[inline]
             fn set_v8_value(&mut self, value: $ty) {
                 let $rv = self;
                 let $value = value;
                 $expr;
             }
        }
    }
}

set_v8_value!((): |rv, _value| rv.set_null());
set_v8_value!(bool: |rv, value| rv.set_bool(value));

set_v8_value!(i8: |rv, value| rv.set_int32(value as i32));
set_v8_value!(i16: |rv, value| rv.set_int32(value as i32));
set_v8_value!(i32: |rv, value| rv.set_int32(value));

set_v8_value!(u8: |rv, value| rv.set_uint32(value as u32));
set_v8_value!(u16: |rv, value| rv.set_uint32(value as u32));
set_v8_value!(u32: |rv, value| rv.set_uint32(value));

set_v8_value!(f32: |rv, value| rv.set_double(value as f64));
set_v8_value!(f64: |rv, value| rv.set_double(value));

// set_v8_value!(i64: |rv, value| rv.set_double(value as f64));
// set_v8_value!(u64: |rv, value| rv.set_double(value as f64));


impl<'a, T: SetV8Value<T>> SetV8Value<Option<T>> for v8::ReturnValue<'a>
    where v8::ReturnValue<'a>: SetV8Value<T>
{
    #[inline]
    fn set_v8_value(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.set_v8_value(value);
        } else {
            self.set_null();
        }
    }
}

impl<'a> SetV8Value<v8::Local<'a, v8::Value>> for v8::ReturnValue<'a> {
    #[inline]
    fn set_v8_value(&mut self, value: v8::Local<'a, v8::Value>) { self.set(value) }
}


// === TrySetV8Value

impl<'a> TrySetV8Value<'a, i64> for v8::ReturnValue<'a> {
    fn try_set_v8_value(&mut self, scope: &mut v8::HandleScope<'a>, value: i64) -> serde_v8::Result<()> {
        self.set(v8::BigInt::new_from_i64(scope, value).into());
        Ok(())
    }
}

impl<'a> TrySetV8Value<'a, u64> for v8::ReturnValue<'a> {
    fn try_set_v8_value(&mut self, scope: &mut v8::HandleScope<'a>, value: u64) -> serde_v8::Result<()> {
        self.set(v8::BigInt::new_from_u64(scope, value).into());
        Ok(())
    }
}

impl<'a> TrySetV8Value<'a, &str> for v8::ReturnValue<'a> {
    fn try_set_v8_value(&mut self, scope: &mut v8::HandleScope<'a>, value: &str) -> serde_v8::Result<()> {
        if value.is_empty() {
            self.set_empty_string();
        } else {
            self.set(value.try_to_scoped_v8_value(scope)?)
        }
        Ok(())
    }
}

impl<'a, T: TrySetV8Value<'a, T>> TrySetV8Value<'a, Option<T>> for v8::ReturnValue<'a>
    where v8::ReturnValue<'a>: TrySetV8Value<'a, T>
{
    fn try_set_v8_value(&mut self, scope: &mut v8::HandleScope<'a>, value: Option<T>) -> serde_v8::Result<()> {
        if let Some(value) = value {
            self.try_set_v8_value(scope, value)?;
        } else {
            self.set_null();
        }
        Ok(())
    }
}
