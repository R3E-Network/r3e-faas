// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;

use crate::source::{self, value};

impl From<value::Value> for source::Value {
    #[inline]
    fn from(value: value::Value) -> Self {
        source::Value { value: Some(value) }
    }
}

impl From<String> for source::Value {
    #[inline]
    fn from(value: String) -> Self {
        source::Value {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<&str> for source::Value {
    #[inline]
    fn from(value: &str) -> Self {
        source::Value {
            value: Some(value::Value::String(value.into())),
        }
    }
}

impl From<bool> for source::Value {
    #[inline]
    fn from(value: bool) -> Self {
        source::Value {
            value: Some(value::Value::Bool(value)),
        }
    }
}

impl From<i64> for source::Value {
    #[inline]
    fn from(value: i64) -> Self {
        source::Value {
            value: Some(value::Value::Int64(value)),
        }
    }
}

impl From<source::Map> for source::Value {
    #[inline]
    fn from(map: source::Map) -> Self {
        source::Value {
            value: Some(value::Value::Map(map)),
        }
    }
}

impl From<source::List> for source::Value {
    #[inline]
    fn from(list: source::List) -> Self {
        source::Value {
            value: Some(value::Value::List(list)),
        }
    }
}

impl From<HashMap<String, source::Value>> for source::Value {
    #[inline]
    fn from(values: HashMap<String, source::Value>) -> Self {
        source::Value {
            value: Some(value::Value::Map(source::Map { values })),
        }
    }
}

impl From<Vec<source::Value>> for source::Value {
    #[inline]
    fn from(values: Vec<source::Value>) -> Self {
        source::Value {
            value: Some(value::Value::List(source::List { values })),
        }
    }
}

impl From<HashMap<String, source::Value>> for value::Value {
    #[inline]
    fn from(values: HashMap<String, source::Value>) -> Self {
        value::Value::Map(source::Map { values })
    }
}

impl From<Vec<source::Value>> for value::Value {
    #[inline]
    fn from(values: Vec<source::Value>) -> Self {
        value::Value::List(source::List { values })
    }
}

impl From<source::Map> for value::Value {
    #[inline]
    fn from(map: source::Map) -> Self {
        value::Value::Map(map)
    }
}

impl From<source::List> for value::Value {
    #[inline]
    fn from(list: source::List) -> Self {
        value::Value::List(list)
    }
}

impl From<String> for value::Value {
    #[inline]
    fn from(value: String) -> Self {
        value::Value::String(value)
    }
}

impl From<&str> for value::Value {
    #[inline]
    fn from(value: &str) -> Self {
        value::Value::String(value.into())
    }
}

impl From<bool> for value::Value {
    #[inline]
    fn from(value: bool) -> Self {
        value::Value::Bool(value)
    }
}

impl From<i64> for value::Value {
    #[inline]
    fn from(value: i64) -> Self {
        value::Value::Int64(value)
    }
}
