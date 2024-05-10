// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


pub mod buffer;
pub mod exception;
pub mod to_v8_value;
pub mod to_v8_return;
pub mod to_rust_number;
pub mod to_rust_value;


pub use {buffer::*, exception::*, to_v8_value::*, to_v8_return::*, to_rust_number::*, to_rust_value::*};


#[inline(always)]
pub(crate) fn from_v8_value<T>(value: &v8::Value) -> &T {
    unsafe { std::mem::transmute::<&v8::Value, &T>(value) }
}