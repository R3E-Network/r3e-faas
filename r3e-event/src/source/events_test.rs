// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;

use crate::source::{self, value};

#[test]
fn test_event_value() {
    let value = value::Value::String("hello".into());
    let value = serde_json::to_string(&value).unwrap();
    assert_eq!(value, r#"{"type":"String","value":"hello"}"#);

    let value = source::Value {
        value: Some(value::Value::String("hello".into())),
    };
    let value = serde_json::to_string(&value).unwrap();
    assert_eq!(value, r#"{"value":{"type":"String","value":"hello"}}"#);
}

#[test]
fn test_event_map() {
    let mut values = HashMap::<String, source::Value>::new();
    values.insert("key1".into(), "value1".into());
    values.insert("key2".into(), "value2".into());
    let map = value::Value::Map(source::Map { values });
    let value = serde_json::to_string(&map).unwrap();

    let got: value::Value = serde_json::from_str(&value).unwrap();
    let value::Value::Map(map) = got else {
        panic!("expected a map");
    };
    assert!(map.values.contains_key("key1"));
    assert!(map.values.contains_key("key2"));

    let value: source::Value = map.into();
    let value = serde_json::to_string(&value).unwrap();
    let got: source::Value = serde_json::from_str(&value).unwrap();
    let Some(value::Value::Map(map)) = got.value else {
        panic!("expected a map");
    };
    assert!(map.values.contains_key("key1"));
    assert!(map.values.contains_key("key2"));
}
