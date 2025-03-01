// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::{mem::*, *};

#[test]
fn test_mem_store() {
    let store = MemKvStore::new();
    let table = "test_table";
    let key = "test_key";
    let value = "test_value";

    // Test put
    let put_input = PutInput {
        key: key.as_bytes(),
        value: value.as_bytes(),
        if_not_exists: false,
    };

    let put = store.put(table, put_input);
    assert!(put.is_ok());

    // Test get
    let got = store.get(table, key.as_bytes());
    assert!(got.is_ok());
    assert_eq!(got.unwrap(), value.as_bytes().to_vec());

    // Test multi_put
    let puts = vec![
        (
            table,
            PutInput {
                key: "key1".as_bytes(),
                value: "value1".as_bytes(),
                if_not_exists: false,
            },
        ),
        (
            table,
            PutInput {
                key: "key2".as_bytes(),
                value: "value2".as_bytes(),
                if_not_exists: false,
            },
        ),
    ];
    let puts = store.multi_put(&puts);
    assert!(puts.is_ok());

    // Test multi_get
    let keys = vec![(table, "key1".as_bytes()), (table, "key2".as_bytes())];
    let got = store.multi_get(&keys);
    assert!(got.is_ok());
    assert_eq!(
        got.unwrap(),
        vec![
            Some("value1".as_bytes().to_vec()),
            Some("value2".as_bytes().to_vec())
        ]
    );

    // Test multi_delete
    let keys = vec![(table, "key1".as_bytes()), (table, "key2".as_bytes())];
    let deleted = store.multi_delete(&keys);
    assert!(deleted.is_ok());
    assert_eq!(
        deleted.unwrap(),
        vec![
            Some("value1".as_bytes().to_vec()),
            Some("value2".as_bytes().to_vec())
        ]
    );

    let scanned = store.scan(
        table,
        ScanInput {
            start_key: &[],
            start_exclusive: false,
            end_key: &[],
            end_inclusive: false,
            max_count: 10,
        },
    );
    assert!(scanned.is_ok());

    let scanned = scanned.unwrap();
    assert_eq!(scanned.kvs.len(), 1);
    assert_eq!(scanned.kvs[0].0, "test_key".as_bytes().to_vec());
    assert_eq!(scanned.kvs[0].1, "test_value".as_bytes().to_vec());
    assert_eq!(scanned.has_more, false);
}
