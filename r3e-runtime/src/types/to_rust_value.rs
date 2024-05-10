// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use std::ops::Deref;
use crate::types::from_v8_value;


#[derive(Debug, Clone, thiserror::Error)]
pub enum ToRustStringError {
    #[error("to-rust-string: expected '{0}', but got '{1}'")]
    InvalidType(&'static str, &'static str),
}


pub trait TryToRustString<T: Deref<Target=str>> {
    fn try_to_rust_string(&self, scope: &mut v8::Isolate) -> Result<T, ToRustStringError>;
}


impl TryToRustString<String> for v8::Value {
    fn try_to_rust_string(&self, scope: &mut v8::Isolate) -> Result<String, ToRustStringError> {
        if !self.is_string() {
            return Err(ToRustStringError::InvalidType("string", self.type_repr()));
        }

        let value = from_v8_value::<v8::String>(self)
            .to_rust_string_lossy(scope);
        Ok(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ToRustBytesError {
    #[error("to-rust-bytes: expected '{0}', but got '{1}'")]
    InvalidType(&'static str, &'static str),
}

pub trait TryToRustBytes<T: Deref<Target=[u8]>> {
    fn try_to_rust_bytes(&self) -> Result<T, ToRustBytesError>;
}


impl TryToRustBytes<serde_v8::V8Slice<u8>> for v8::Value {
    fn try_to_rust_bytes(&self) -> Result<serde_v8::V8Slice<u8>, ToRustBytesError> {
        let (store, range) = if self.is_array_buffer() {
            let buf = from_v8_value::<v8::ArrayBuffer>(self);
            let store = buf.get_backing_store();
            (store, 0..buf.byte_length())
        } else if self.is_array_buffer_view() {
            let buf = from_v8_value::<v8::ArrayBufferView>(self);
            let store = buf.get_backing_store()
                .ok_or(ToRustBytesError::InvalidType("ArrayBuffer/ArrayBufferView", "NoBackingStoreArrayBufferView"))?;

            let offset = buf.byte_offset();
            (store, offset..offset + buf.byte_length())
        } else {
            return Err(ToRustBytesError::InvalidType("ArrayBuffer/ArrayBufferView", self.type_repr()));
        };

        Ok(unsafe { serde_v8::V8Slice::from_parts(store, range) })
    }
}


/// Must ensure that the returned slice does not outlive the `ArrayBuffer`
pub unsafe fn try_to_rust_bytes(value: &v8::Value) -> Result<&mut [u8], ToRustBytesError> {
    let (pointer, len) = if value.is_array_buffer_view() {
        let view = from_v8_value::<v8::ArrayBufferView>(value);
        (view.data(), view.byte_length())
    } else if value.is_array_buffer() {
        let buffer = from_v8_value::<v8::ArrayBuffer>(value);
        (buffer.data().map(|v| v.as_ptr()).unwrap_or(std::ptr::null_mut()), buffer.byte_length())
    } else {
        return Err(ToRustBytesError::InvalidType("ArrayBufferView or ArrayBuffer", value.type_repr()));
    };

    if !pointer.is_null() {
        Ok(std::slice::from_raw_parts_mut(pointer as _, len))
    } else {
        Ok(&mut [])
    }
}
