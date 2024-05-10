// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use crate::types::from_v8_value;


/// Number.MAX_SAFE_INTEGER in js
pub const MAX_SAFE_INT: i64 = 9007199254740991;

/// Number.MIN_SAFE_INTEGER in js
pub const MIN_SAFE_INT: i64 = -9007199254740991;


pub trait ToRustNumber<T> {
    fn to_rust_number(&self) -> T;
}

macro_rules! to_rust_number {
    ($src:ty: $($dst:ty),+) => { $(
        impl ToRustNumber<$dst> for $src {
            #[inline]
            fn to_rust_number(&self) -> $dst { self.value() as $dst }
        }
    )+};
}

to_rust_number!(v8::Int32: i32, i64);
to_rust_number!(v8::Uint32: u32, u64);
to_rust_number!(v8::Number: f32, f64);


macro_rules! to_rust_number64 {
    ($src:ty: $($dst:ty),+) => { $(
        impl ToRustNumber<$dst> for $src {
            #[inline]
            fn to_rust_number(&self) -> $dst { self.u64_value().0 as $dst }
        }
    )+};
}

to_rust_number64!(v8::BigInt: i64, u64, isize, usize);


#[derive(Debug, Clone, thiserror::Error)]
pub enum ToRustNumberError {
    #[error("to-rust-number: expected '{0}', but got '{1}'")]
    InvalidType(&'static str, &'static str)
}


pub trait TryToRustNumber<T> {
    fn try_to_rust_number(&self) -> Result<T, ToRustNumberError>;
}

macro_rules! try_to_rust_int {
    ($ty:ty) => {
        impl TryToRustNumber<$ty> for v8::Value {
            fn try_to_rust_number(&self) -> Result<$ty, ToRustNumberError> {
                if self.is_int32() {
                    Ok(from_v8_value::<v8::Int32>(self).value() as $ty)
                } else if self.is_uint32() {
                    Ok(from_v8_value::<v8::Uint32>(self).value() as $ty)
                } else if self.is_number() {
                    Ok(from_v8_value::<v8::Number>(self).value() as $ty)
                } else if self.is_big_int() {
                    Ok(from_v8_value::<v8::BigInt>(self).u64_value().0 as $ty)
                } else {
                    Err(ToRustNumberError::InvalidType(std::stringify!($ty), self.type_repr()))
                }
            }
        }
    };
}

try_to_rust_int!(i32);
try_to_rust_int!(u32);
try_to_rust_int!(i64);
try_to_rust_int!(u64);
try_to_rust_int!(isize);
try_to_rust_int!(usize);


macro_rules! try_to_rust_float {
    ($ty:ty) => {
        impl TryToRustNumber<$ty> for v8::Value {
            fn try_to_rust_number(&self) -> Result<$ty, ToRustNumberError> {
                if self.is_number() {
                    Ok(from_v8_value::<v8::Number>(self).value() as $ty)
                } else if self.is_big_int() {
                    Ok(from_v8_value::<v8::BigInt>(self).i64_value().0 as $ty)
                } else {
                    Err(ToRustNumberError::InvalidType(std::stringify!($ty), self.type_repr()))
                }
            }
        }
    };
}

try_to_rust_float!(f32);
try_to_rust_float!(f64);
