// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod ethereum;
pub mod event_filter;
pub mod event_processor;
pub mod event_processor_service;
pub mod events;
pub mod events_ext;
pub mod mock;
pub mod neo;
pub mod service;

#[cfg(test)]
mod events_test;

#[allow(unused_imports)]
pub use {
    ethereum::*, event_filter::*, event_processor::*, event_processor_service::*,
    events::*, events_ext::*, mock::*, neo::*, service::*,
};

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("task: no such uid: {0}")]
    NoSuchUid(u64),

    #[error("task: no more task: {0}")]
    NoMoreTask(u64),
    
    #[error("task: error: {0}")]
    Error(String),
}

#[derive(Debug, thiserror::Error)]
pub enum FuncError {
    #[error("func: no such uid: {0}")]
    NoSuchUid(u64),

    // uid, fid
    #[error("func: no such func: {0},{1}")]
    NoSuchFunc(u64, u64),
    
    #[error("func: error: {0}")]
    Error(String),
}

pub struct Task {
    pub uid: u64,
    pub fid: u64,
    pub event: event::Event,
}

impl Task {
    #[inline]
    pub fn new(uid: u64, fid: u64, event: event::Event) -> Self {
        Self { uid, fid, event }
    }
}

#[async_trait::async_trait]
pub trait TaskSource: Send + Sync {
    async fn acquire_task(&mut self, uid: u64, fid_hint: u64) -> Result<Task, TaskError>;

    async fn acquire_fn(&mut self, uid: u64, fid: u64) -> Result<Func, FuncError>;
}

pub struct TaskSourceClient {
    inner: task_source_client::TaskSourceClient<tonic::transport::Channel>,
}

impl TaskSourceClient {
    pub async fn connect<Endpoint>(addr: Endpoint) -> Result<Self, tonic::transport::Error>
    where
        Endpoint: TryInto<tonic::transport::Endpoint>,
        Endpoint::Error: Into<tonic::codegen::StdError>,
    {
        Ok(Self {
            inner: task_source_client::TaskSourceClient::connect(addr).await?,
        })
    }
}

#[async_trait::async_trait]
impl TaskSource for TaskSourceClient {
    async fn acquire_task(&mut self, uid: u64, fid_hint: u64) -> Result<Task, TaskError> {
        let mut res = self
            .inner
            .acquire_task(AcquireTaskInput { uid, fid_hint })
            .await
            .map_err(|err| {
                if err.code() == tonic::Code::NotFound {
                    TaskError::NoSuchUid(uid)
                } else {
                    TaskError::NoMoreTask(uid)
                }
            })?;

        let out = res.get_mut();
        let Some(event) = out.event.as_mut().and_then(|x| x.event.take()) else {
            return Err(TaskError::NoMoreTask(uid));
        };
        Ok(Task {
            uid: out.uid,
            fid: out.fid,
            event: event,
        })
    }

    async fn acquire_fn(&mut self, uid: u64, fid: u64) -> Result<Func, FuncError> {
        let mut res = self
            .inner
            .acquire_func(AcquireFuncInput { uid, fid })
            .await
            .map_err(|err| {
                if err.code() == tonic::Code::NotFound {
                    FuncError::NoSuchFunc(uid, fid)
                } else {
                    FuncError::NoSuchUid(uid)
                }
            })?;

        let out = res.get_mut();
        let Some(func) = out.func.take() else {
            return Err(FuncError::NoSuchFunc(uid, fid));
        };
        Ok(Func {
            version: func.version,
            code: func.code,
        })
    }
}
