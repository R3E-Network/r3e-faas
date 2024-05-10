// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use r3e_core::BytesLike;
use crate::types::{ToScopedV8Value, TryToScopedV8Value};


// pub trait BytesLike {
//     fn is_empty(&self) -> bool;
//
//     fn len(&self) -> usize;
//
//     fn as_bytes(&self) -> &[u8];
// }


/// A [u8] wrapper for js ArrayBuffer
#[derive(BytesLike)]
pub struct ArrayBuffer {
    pub(crate) bytes: Box<[u8]>,
}

impl<'a> ToScopedV8Value<'a> for ArrayBuffer {
    #[inline]
    fn to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        if self.is_empty() {
            v8::ArrayBuffer::new(scope, 0).into()
        } else {
            let backing = v8::ArrayBuffer::new_backing_store_from_boxed_slice(self.bytes).make_shared();
            v8::ArrayBuffer::with_backing_store(scope, &backing).into()
        }
    }
}


pub trait ToArrayBuffer {
    fn to_array_buffer(self) -> ArrayBuffer;
}

impl ToArrayBuffer for Vec<u8> {
    fn to_array_buffer(self) -> ArrayBuffer { ArrayBuffer { bytes: self.into_boxed_slice() } }
}

impl ToArrayBuffer for Box<[u8]> {
    fn to_array_buffer(self) -> ArrayBuffer { ArrayBuffer { bytes: self } }
}


/// OneBytesRef for converting [u8] to js String with one-byte(i.e. Latin-1)
#[derive(BytesLike)]
pub struct OneBytesRef<'a> {
    bytes: &'a [u8],
}

impl<'a, 'b> TryToScopedV8Value<'a> for OneBytesRef<'b> {
    #[inline]
    fn try_to_scoped_v8_value(self, scope: &mut v8::HandleScope<'a>) -> serde_v8::Result<v8::Local<'a, v8::Value>> {
        let len = self.len();
        v8::String::new_from_one_byte(scope, self.as_bytes(), v8::NewStringType::Normal)
            .map(|v| v.into())
            .ok_or_else(|| serde_v8::Error::Message(format!("String: alloc len({}) failed", len)))
    }
}

pub trait ToOneBytesRef {
    fn to_one_bytes_ref(&self) -> OneBytesRef;
}

impl<T: AsRef<[u8]>> ToOneBytesRef for T {
    fn to_one_bytes_ref(&self) -> OneBytesRef { OneBytesRef { bytes: self.as_ref() } }
}
