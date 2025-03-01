// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::{BTreeMap, HashMap};
use std::ops::Bound;
use std::sync::Mutex;

use crate::*;

pub struct MemKvStore {
    tables: Mutex<HashMap<String, BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl MemKvStore {
    pub fn new() -> Self {
        Self {
            tables: Mutex::new(HashMap::new()),
        }
    }
}

impl KvStore for MemKvStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(PutError::InvalidTable);
        }

        if input.key.len() > MAX_KEY_SIZE {
            return Err(PutError::TooLargeKey);
        }

        if input.value.len() > MAX_VALUE_SIZE {
            return Err(PutError::TooLargeValue);
        }

        let mut tables = self.tables.lock().unwrap();

        let entry = tables.entry(table.to_string()).or_default();
        if input.if_not_exists && entry.contains_key(input.key) {
            return Err(PutError::AlreadyExists);
        }

        entry.insert(input.key.to_vec(), input.value.to_vec());
        Ok(())
    }

    fn get(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, GetError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(GetError::InvalidTable);
        }

        if key.len() > MAX_KEY_SIZE {
            return Err(GetError::TooLargeKey);
        }

        let tables = self.tables.lock().unwrap();
        let table = tables.get(table).ok_or(GetError::NoSuchKey)?;
        let value = table.get(key).ok_or(GetError::NoSuchKey)?;
        Ok(value.to_vec())
    }

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(DeleteError::InvalidTable);
        }

        if key.len() > MAX_KEY_SIZE {
            return Err(DeleteError::TooLargeKey);
        }

        let mut tables = self.tables.lock().unwrap();
        let value = tables.get_mut(table).and_then(|table| table.remove(key));
        Ok(value)
    }
}

impl SortedKvStore for MemKvStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError> {
        if table.len() > MAX_TABLE_NAME_SIZE {
            return Err(ScanError::InvalidTable);
        }

        if input.start_key.len() > MAX_KEY_SIZE {
            return Err(ScanError::TooLargeKey);
        }

        if input.end_key.len() > MAX_KEY_SIZE {
            return Err(ScanError::TooLargeKey);
        }

        let tables = self.tables.lock().unwrap();
        let Some(table) = tables.get(table) else {
            return Ok(ScanOutput {
                kvs: vec![],
                has_more: false,
            });
        };

        let start_bound = if input.start_key.is_empty() {
            Bound::Unbounded
        } else if input.start_exclusive {
            Bound::Excluded(input.start_key.to_vec())
        } else {
            Bound::Included(input.start_key.to_vec())
        };

        let end_bound = if input.end_key.is_empty() {
            Bound::Unbounded
        } else if input.end_inclusive {
            Bound::Included(input.end_key.to_vec())
        } else {
            Bound::Excluded(input.end_key.to_vec())
        };

        let max_count = input.max_count();
        let range = table.range((start_bound, end_bound));
        let values: Vec<(Vec<u8>, Vec<u8>)> = range
            .take(max_count)
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();

        let has_more = values.len() == max_count;
        Ok(ScanOutput {
            kvs: values,
            has_more,
        })
    }
}

impl BatchKvStore for MemKvStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError> {
        for (table, input) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiPutError::InvalidTable);
            }

            if input.key.len() > MAX_KEY_SIZE {
                return Err(MultiPutError::TooLargeKey);
            }

            if input.value.len() > MAX_VALUE_SIZE {
                return Err(MultiPutError::TooLargeValue);
            }
        }

        let mut tables = self.tables.lock().unwrap();
        for (table, input) in inputs {
            let entry = tables.entry(table.to_string()).or_default();

            if input.if_not_exists && entry.contains_key(input.key) {
                continue;
                // return Err(MultiPutError::AlreadyExists);
            }

            entry.insert(input.key.to_vec(), input.value.to_vec());
        }

        Ok(())
    }

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError> {
        for (table, key) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiGetError::InvalidTable);
            }

            if key.len() > MAX_KEY_SIZE {
                return Err(MultiGetError::TooLargeKey);
            }
        }

        let tables = self.tables.lock().unwrap();
        let mut results = Vec::with_capacity(inputs.len());
        for (table, key) in inputs {
            let value = tables.get(*table).and_then(|table| table.get(*key));
            results.push(value.cloned());
        }

        Ok(results)
    }

    fn multi_delete(
        &self,
        inputs: &[(&str, &[u8])],
    ) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError> {
        for (table, key) in inputs {
            if table.len() > MAX_TABLE_NAME_SIZE {
                return Err(MultiDeleteError::InvalidTable);
            }

            if key.len() > MAX_KEY_SIZE {
                return Err(MultiDeleteError::TooLargeKey);
            }
        }

        let mut tables = self.tables.lock().unwrap();
        let mut results = Vec::with_capacity(inputs.len());
        for (table, key) in inputs {
            let value = tables.get_mut(*table).and_then(|table| table.remove(*key));
            results.push(value);
        }

        Ok(results)
    }
}
