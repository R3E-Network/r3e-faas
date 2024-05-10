// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use std::fmt::{Debug, Formatter};


#[derive(Debug, Copy, Clone)]
pub enum ParamType {
    Bool,
    I32,
    U32,
    F32,
    F64,
    I64,
    U64,
    String,
    Bytes,
    ArrayBuffer,
}

#[derive(Debug, Copy, Clone)]
pub enum ReturnType {
    Void,
    Bool,
    I32,
    U32,
    F32,
    F64,
    I64,
    U64,
    String,
}


pub type TrivialCall = extern "C" fn(*const v8::FunctionCallbackInfo);

pub struct FastCall {
    pub fast_fn: v8::fast_api::FastFunction,
    pub fn_info: v8::fast_api::CFunctionInfo,
}

impl Debug for FastCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastCall{{fast_fn{{args:{:?},fn:{:?},repr:{:?},return:{:?}}},fn_info(Opaque)}}",
               self.fast_fn.args, self.fast_fn.function, self.fast_fn.repr, self.fast_fn.return_type)
    }
}


#[derive(Debug)]
pub struct JsCall {
    pub name: &'static str,
    pub params: Vec<ParamType>,
    pub return_type: ReturnType,
    pub is_async: bool,
    pub trivial_call: TrivialCall,
    pub fast_call: Option<FastCall>,
}

impl JsCall {
    pub fn params_count(&self) -> usize { self.params.len() }
}
