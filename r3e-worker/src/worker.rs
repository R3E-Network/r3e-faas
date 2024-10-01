// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use libc::pid_t;

use crate::*;

struct Inner {
    runners: HashMap<u64, HashSet<RunHandle>>,
    processes: HashMap<pid_t, u64>,
}

pub struct Worker {
    inner: Mutex<Inner>,
    config: WorkerConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Spawn,
    Destroy,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkError {
    #[error("worker: failed to spawn runner: {0}")]
    SpawnError(i32),
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self {
            inner: Mutex::new(Inner {
                processes: HashMap::new(),
                runners: HashMap::new(),
            }),
            config,
        }
    }

    #[inline]
    pub fn runners(&self) -> usize {
        self.inner.lock().unwrap().runners.len()
    }

    pub fn runners_of(&self, uid: u64) -> usize {
        self.inner
            .lock()
            .unwrap()
            .runners
            .get(&uid)
            .map_or(0, |v| v.len())
    }

    #[inline]
    pub fn max_runners(&self) -> usize {
        self.config.max_runners() as usize
    }

    pub fn run(
        &self,
        chan: mpsc::Receiver<Assign>,
        stopper: Arc<AtomicBool>,
    ) -> Result<(), WorkError> {
        let on_recv = |assign: Assign| {
            match assign.action {
                Action::Spawn => {
                    let pid = self.spawn(assign.uid, stopper.clone())?;
                    let mut inner = self.inner.lock().unwrap();
                    inner.processes.insert(pid, assign.uid);
                    inner
                        .runners
                        .entry(assign.uid)
                        .or_default()
                        .insert(RunHandle::new(pid, true));
                }
                Action::Destroy => {
                    let mut inner = self.inner.lock().unwrap();
                    if let Some(workers) = inner.runners.remove(&assign.uid) {
                        for worker in workers {
                            let _ = inner.processes.remove(&worker.pid);
                        }
                    }
                }
            }
            Ok(())
        };

        while !stopper.load(Ordering::SeqCst) {
            match chan.recv_timeout(Duration::from_secs(1)) {
                Ok(assign) => on_recv(assign)?,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            }

            self.wait();
        }
        Ok(())
    }

    pub(crate) fn wait(&self) {
        let mut status = 0;
        let pid = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
        if pid == -1 {
            log::error!(
                "incubate: failed to wait pid: {}",
                errno::errno().to_string()
            );
            return;
        }

        if pid == 0 {
            return;
        }

        let mut inner = self.inner.lock().unwrap();
        if let Some(uid) = inner.processes.remove(&pid) {
            if let Some(workers) = inner.runners.get_mut(&uid) {
                workers.remove(&RunHandle::new(pid, false));
            }
        }
        drop(inner);

        log::info!("incubate: worker {} exited: {}", pid, status);
    }

    // non-Linux platform just for test and development. TODO: use clone instead in Linux
    // #[cfg(not(target_os = "linux"))]
    pub fn spawn(&self, uid: u64, stopper: Arc<AtomicBool>) -> Result<pid_t, WorkError> {
        let pid = unsafe { libc::fork() };
        if pid < 0 {
            return Err(WorkError::SpawnError(errno::errno().0));
        }

        if pid > 0 {
            return Ok(pid);
        }

        let tasks = TaskSourceBuilder::new(self.config.tasks.clone()).build();
        let runner = Runner::new(uid, self.config.max_runtimes_per_runner, tasks);
        runner.run(stopper);
        std::process::exit(0);
        // std::unreachable!();
    }
}
