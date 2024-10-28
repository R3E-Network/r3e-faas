// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::hash::Hash;
use std::num::NonZero;
use std::time::Instant;

use libc::pid_t;
use lru::LruCache;

use r3e_deno::{ExecError, JsRuntime, RuntimeConfig};
use r3e_event::source::{Task, TaskSource};

use crate::Stopper;

pub struct Runner {
    uid: u64,
    max_runtimes: u32,
    tasks: Box<dyn TaskSource>,
    // reactor: tokio::runtime::Runtime,
}

struct RunContext {
    module: usize,
    version: u64,
    runtime: JsRuntime,
}

impl Runner {
    pub fn new(uid: u64, max_runtimes: u32, tasks: Box<dyn TaskSource>) -> Self {
        Self {
            uid,
            tasks,
            max_runtimes,
        }
    }

    pub fn run(mut self, stop: impl Stopper) {
        let reactor = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runner: build reactor");
        let _enter = reactor.enter();
        reactor.block_on(self.do_run(stop));
    }

    async fn do_run(&mut self, stop: impl Stopper) {
        let uid = self.uid;
        let max_runtimes = NonZero::new(self.max_runtimes as usize)
            .unwrap_or(unsafe { NonZero::new_unchecked(16) });

        let mut fid = 0;
        let mut runtimes = LruCache::<u64, RunContext>::new(max_runtimes);
        while !stop.stopped() {
            let task = match self.tasks.acquire_task(uid, fid).await {
                Ok(task) => task,
                Err(err) => {
                    log::error!("runner: {} acquire task failed: {}", uid, err);
                    break;
                }
            };
            log::info!("runner: {} acquire task for {}", uid, task.fid);

            fid = task.fid;
            let run_cx = match runtimes.get_mut(&fid) {
                Some(run_cx) => run_cx,
                None => match self.load_runtime(fid, &mut runtimes).await {
                    Ok(run_cx) => run_cx,
                    Err(_err) => continue,
                },
            };

            let start = Instant::now();
            if let Err(err) = self.run_task(run_cx, task).await {
                log::error!("runner: {} run task failed: {}", uid, err);
            }

            let elapsed = start.elapsed();
            log::info!("runner: {},{} run task cost: {:?}", uid, fid, elapsed);
        }

        log::info!(
            "runner: {},{} with stopped({}) exited",
            uid,
            std::process::id(),
            stop.stopped()
        );
    }

    async fn run_task(&self, run_cx: &mut RunContext, task: Task) -> Result<(), ExecError> {
        let event = run_cx
            .runtime
            .to_global(&task.event)
            .map_err(|err| ExecError::OnExecute(err.to_string()))?;

        let _ = run_cx
            .runtime
            .run_module_default(run_cx.module, &[event])
            .await?;
        Ok(())
    }

    async fn load_runtime<'a>(
        &mut self,
        fid: u64,
        runtimes: &'a mut LruCache<u64, RunContext>,
    ) -> Result<&'a mut RunContext, ExecError> {
        // if let Some(run_cx) = runtimes.get_mut(&fid) {
        //     return Ok(run_cx);
        // }

        let run_cx = match self.load_fn(fid).await {
            Ok(run_cx) => run_cx,
            Err(err) => {
                log::error!("runner: {} load fn failed: {}", self.uid, err);
                return Err(err);
            }
        };

        let run_cx = runtimes.get_or_insert_mut(fid, || run_cx);
        Ok(run_cx)
    }

    async fn load_fn(&mut self, fid: u64) -> Result<RunContext, ExecError> {
        let mut runtime = JsRuntime::new(RuntimeConfig::default());
        let fn_code = self
            .tasks
            .acquire_fn(self.uid, fid)
            .await
            .map_err(|err| ExecError::OnLoad(err.to_string()))?;

        log::info!("runner: {} load fn for {}", self.uid, fid);
        let module = runtime.load_main_module(fn_code.code).await?;

        let _ = runtime.eval_module(module).await?;

        Ok(RunContext {
            module,
            version: fn_code.version,
            runtime,
        })
    }
}

#[derive(Debug)]
pub(crate) struct RunHandle {
    pub(crate) pid: pid_t,
    pub(crate) kill_on_drop: bool,
}

impl PartialEq for RunHandle {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for RunHandle {}

impl Hash for RunHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pid.hash(state);
    }
}

impl RunHandle {
    pub fn new(pid: pid_t, kill_on_drop: bool) -> Self {
        Self { pid, kill_on_drop }
    }
}

impl Drop for RunHandle {
    fn drop(&mut self) {
        if !self.kill_on_drop {
            return;
        }

        let pid = self.pid;
        let rv = unsafe { libc::kill(pid, libc::SIGINT) };
        if rv == -1 {
            log::error!("worker: kill {} error: {}", pid, errno::errno().to_string());
        } else {
            log::info!("worker: killed {}", pid);
        }
    }
}
