// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::hash::Hash;
use std::num::NonZero;
use std::sync::Arc;
use std::time::{Duration, Instant};

use libc::pid_t;
use lru::LruCache;
use uuid::Uuid;

use r3e_built_in_services::balance::{BalanceServiceTrait, TransactionType};
use r3e_deno::{ExecError, JsRuntime, RuntimeConfig, sandbox::SandboxConfig};
use r3e_event::source::{Task, TaskSource};

use crate::Stopper;

pub struct Runner {
    uid: u64,
    max_runtimes: u32,
    tasks: Box<dyn TaskSource>,
    // Sandbox configuration
    sandbox_config: SandboxConfig,
    // Balance service
    balance_service: Option<Arc<dyn BalanceServiceTrait>>,
}

struct RunContext {
    module: usize,
    version: u64,
    runtime: JsRuntime,
}

impl Runner {
    pub fn new(uid: u64, max_runtimes: u32, tasks: Box<dyn TaskSource>) -> Self {
        // Default sandbox configuration
        let sandbox_config = SandboxConfig {
            initial_heap_size: 1 * 1024 * 1024, // 1MB
            max_heap_size: 128 * 1024 * 1024,   // 128MB
            max_execution_time: Duration::from_secs(10),
            enable_jit: false,
            allow_net: false,
            allow_fs: false,
            allow_env: false,
            allow_run: false,
            allow_hrtime: false,
        };
        
        Self {
            uid,
            tasks,
            max_runtimes,
            sandbox_config,
            balance_service: None,
        }
    }
    
    pub fn with_balance_service(mut self, balance_service: Arc<dyn BalanceServiceTrait>) -> Self {
        self.balance_service = Some(balance_service);
        self
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
            
            // Charge for execution if balance service is available
            if let Some(balance_service) = &self.balance_service {
                let user_id = uid.to_string();
                let function_id = fid.to_string();
                
                // Calculate gas amount based on execution time
                // This is a simple calculation, in a real implementation this would be more sophisticated
                let gas_amount = (elapsed.as_millis() as u64) * 10; // 10 gas per millisecond
                
                match balance_service.charge_for_execution(&user_id, &function_id, gas_amount).await {
                    Ok(transaction) => {
                        log::info!(
                            "runner: {},{} charged {} gas for execution, transaction ID: {}",
                            uid, fid, gas_amount, transaction.id
                        );
                    },
                    Err(err) => {
                        log::error!(
                            "runner: {},{} failed to charge for execution: {}",
                            uid, fid, err
                        );
                    }
                }
            }
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
        // Check if user has enough balance to run the function
        if let Some(balance_service) = &self.balance_service {
            let user_id = self.uid.to_string();
            let balance = match balance_service.get_balance(&user_id).await {
                Ok(balance) => balance,
                Err(err) => {
                    return Err(ExecError::OnLoad(format!("Failed to get user balance: {}", err)));
                }
            };
            
            // Check if user has enough GAS balance
            // This is a simple check, in a real implementation this would be more sophisticated
            if balance.gas_balance < 1000 { // Minimum required balance
                return Err(ExecError::OnLoad(format!(
                    "Insufficient GAS balance to run function: {} < 1000",
                    balance.gas_balance
                )));
            }
        }
        
        // Create a new runtime with sandbox configuration
        let runtime_config = RuntimeConfig {
            max_heap_size: self.sandbox_config.max_heap_size,
            sandbox_config: Some(self.sandbox_config.clone()),
        };
        
        let mut runtime = JsRuntime::new(runtime_config);
        
        let fn_code = self
            .tasks
            .acquire_fn(self.uid, fid)
            .await
            .map_err(|err| ExecError::OnLoad(err.to_string()))?;

        log::info!("runner: {} load fn for {} in sandbox", self.uid, fid);
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
