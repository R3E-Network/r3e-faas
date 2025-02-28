// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod mem;
pub mod rocksdb;

#[cfg(test)]
pub mod mem_test;

pub const MAX_TABLE_NAME_SIZE: usize = 128; // 128 bytes
pub const MAX_KEY_SIZE: usize = 1024; // 1 KB
pub const MAX_VALUE_SIZE: usize = 4 * 1024 * 1024; // 4 MB

pub struct PutInput<'k, 'v> {
    pub key: &'k [u8],
    pub value: &'v [u8],
    pub if_not_exists: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum PutError {
    #[error("kv-put: key already exists")]
    AlreadyExists,

    #[error("kv-put: invalid table name")]
    InvalidTable,

    #[error("kv-put: key is too large")]
    TooLargeKey,

    #[error("kv-put: value is too large")]
    TooLargeValue,
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("kv-get: no such key")]
    NoSuchKey,

    #[error("kv-get: invalid table name")]
    InvalidTable,

    #[error("kv-get: key is too large")]
    TooLargeKey,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("kv-delete: key is too large")]
    TooLargeKey,

    #[error("kv-delete: invalid table name")]
    InvalidTable,
}

pub trait KvStore {
    fn put(&self, table: &str, input: PutInput) -> Result<(), PutError>;

    fn get(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, GetError>;

    fn delete(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, DeleteError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("kv-scan: key is too large")]
    TooLargeKey,

    #[error("kv-scan: invalid table name")]
    InvalidTable,
}

pub struct ScanInput<'k, 'v> {
    // empty means from the start of the table
    pub start_key: &'k [u8],

    // if true, the start key is excluded, otherwise included
    pub start_exclusive: bool,

    // empty means to the end of the table
    pub end_key: &'v [u8],

    // if true, the end key is included, otherwise excluded
    pub end_inclusive: bool,

    // 0 means 100
    pub max_count: u32,
}

impl<'k, 'v> ScanInput<'k, 'v> {
    pub fn max_count(&self) -> usize {
        if self.max_count == 0 {
            100
        } else {
            self.max_count as usize
        }
    }
}

pub struct ScanOutput {
    // .0 = key, .1 = value
    pub kvs: Vec<(Vec<u8>, Vec<u8>)>,
    pub has_more: bool,
}

pub trait SortedKvStore: KvStore {
    fn scan(&self, table: &str, input: ScanInput) -> Result<ScanOutput, ScanError>;
}

#[derive(Debug, thiserror::Error)]
pub enum MultiPutError {
    #[error("kv-multi-put: key already exists")]
    AlreadyExists,

    #[error("kv-multi-put: invalid table name")]
    InvalidTable,

    #[error("kv-multi-put: key is too large")]
    TooLargeKey,

    #[error("kv-multi-put: value is too large")]
    TooLargeValue,
}

#[derive(Debug, thiserror::Error)]
pub enum MultiDeleteError {
    #[error("kv-multi-delete: key is too large")]
    TooLargeKey,

    #[error("kv-multi-delete: invalid table name")]
    InvalidTable,
}

#[derive(Debug, thiserror::Error)]
pub enum MultiGetError {
    #[error("kv-multi-get: key is too large")]
    TooLargeKey,

    #[error("kv-multi-get: invalid table name")]
    InvalidTable,
}

pub trait BatchKvStore: KvStore {
    fn multi_put(&self, inputs: &[(&str, PutInput)]) -> Result<(), MultiPutError>;

    fn multi_get(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiGetError>;

    fn multi_delete(&self, inputs: &[(&str, &[u8])]) -> Result<Vec<Option<Vec<u8>>>, MultiDeleteError>;
}


// pub trait DocStore {
//     fn put_doc(&self, table: &str, doc_key: &[u8], doc: &[u8]) -> Result<(), PutDocError>;
//
//     fn get_doc(&self, table: &str, doc_key: &[u8]) -> Result<Option<Vec<u8>>, GetDocError>;
//
//     fn delete_doc(&self, table: &str, doc_key: &[u8]) -> Result<Option<Vec<u8>>, DeleteDocError>;
// }
