// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use libc::pid_t;
use log::{debug, error, info, warn};
use signal_hook::consts::SIGINT;
use tokio::sync::mpsc;

use r3e_built_in_services::balance::{BalanceService, MemoryBalanceStorage};
use r3e_built_in_services::gas_bank::GasBankServiceTrait;
use r3e_event::source::TaskSource;

use crate::{RunHandle, Runner, Stopper, TaskConfig, TaskSourceBuilder, WorkerConfig};

pub struct Worker {
    config: WorkerConfig,
    stop: Arc<AtomicBool>,
    runners: Arc<Mutex<HashMap<pid_t, RunHandle>>>,
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let runners = Arc::new(Mutex::new(HashMap::new()));

        Self {
            config,
            stop,
            runners,
        }
    }

    pub fn run(&self) {
        let (tx, mut rx) = mpsc::channel::<pid_t>(self.config.max_pending as usize);

        // Register signal handler
        let stop = self.stop.clone();
        let _ = signal_hook::flag::register(SIGINT, Arc::clone(&stop));

        // Spawn runner manager
        let runners = self.runners.clone();
        let stop2 = self.stop.clone();
        let max_runners = self.config.max_runners();
        let max_runtimes = self.config.max_runtimes_per_runner;
        let task_config = self.config.tasks.clone();

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("worker: build reactor");

            rt.block_on(async move {
                let mut uid: u64 = 0;
                while !stop2.load(Ordering::Relaxed) {
                    if runners.lock().unwrap().len() >= max_runners as usize {
                        // Wait for a runner to exit
                        match rx.recv().await {
                            Some(pid) => {
                                debug!("worker: runner {} exited", pid);
                                runners.lock().unwrap().remove(&pid);
                            }
                            None => {
                                error!("worker: runner channel closed");
                                break;
                            }
                        }
                    }

                    // Spawn a new runner
                    uid += 1;
                    let task_source = TaskSourceBuilder::new(task_config.clone()).build();

                    // Create a balance service
                    let balance_storage = Arc::new(MemoryBalanceStorage::new());

                    // Get the gas bank service from configuration
                    let gas_bank_service = match &self.config.gas_bank_service {
                        Some(service) => service.clone(),
                        None => {
                            warn!("No gas bank service configured, using mock implementation");
                            Arc::new(MockGasBankService::new())
                        }
                    };

                    let balance_service =
                        Arc::new(BalanceService::new(balance_storage, gas_bank_service));

                    // Get the sandbox configuration
                    let sandbox_config = self.config.sandbox.clone();

                    let runner = Runner::new(uid, max_runtimes, task_source)
                        .with_balance_service(balance_service)
                        .with_sandbox_config(sandbox_config);

                    let stop = stop2.clone();
                    let tx = tx.clone();
                    let pid = unsafe { libc::fork() };
                    match pid {
                        -1 => {
                            error!("worker: fork failed");
                            break;
                        }
                        0 => {
                            // Child process
                            drop(tx);
                            drop(runners);
                            runner.run(stop);
                            std::process::exit(0);
                        }
                        _ => {
                            // Parent process
                            info!("worker: spawned runner {} with pid {}", uid, pid);
                            runners
                                .lock()
                                .unwrap()
                                .insert(pid, RunHandle::new(pid, true));
                        }
                    }
                }
            });
        });

        // Wait for stop signal
        while !self.stop.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }

        info!("worker: stopping");

        // Wait for all runners to exit
        let graceful = self.config.graceful;
        let start = std::time::Instant::now();
        while start.elapsed() < graceful {
            if self.runners.lock().unwrap().is_empty() {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Kill remaining runners
        let mut runners = self.runners.lock().unwrap();
        for (pid, _) in runners.iter() {
            unsafe {
                libc::kill(*pid, SIGINT);
            }
        }
        runners.clear();

        // Wait for runner manager to exit
        let _ = handle.join();

        info!("worker: stopped");
    }
}

// Mock implementation of GasBankServiceTrait for testing
struct MockGasBankService;

impl MockGasBankService {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl GasBankServiceTrait for MockGasBankService {
    async fn get_account(
        &self,
        address: &str,
    ) -> Result<
        Option<r3e_built_in_services::gas_bank::GasBankAccount>,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(Some(r3e_built_in_services::gas_bank::GasBankAccount {
            address: address.to_string(),
            balance: 1000000,
            fee_model: r3e_built_in_services::gas_bank::FeeModel::Fixed(100),
            credit_limit: 0,
            used_credit: 0,
            updated_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        }))
    }

    async fn create_account(
        &self,
        address: &str,
        fee_model: r3e_built_in_services::gas_bank::FeeModel,
        credit_limit: u64,
    ) -> Result<
        r3e_built_in_services::gas_bank::GasBankAccount,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(r3e_built_in_services::gas_bank::GasBankAccount {
            address: address.to_string(),
            balance: 0,
            fee_model,
            credit_limit,
            used_credit: 0,
            updated_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        })
    }

    async fn deposit(
        &self,
        tx_hash: &str,
        address: &str,
        amount: u64,
    ) -> Result<
        r3e_built_in_services::gas_bank::GasBankDeposit,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(r3e_built_in_services::gas_bank::GasBankDeposit {
            tx_hash: tx_hash.to_string(),
            address: address.to_string(),
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        })
    }

    async fn withdraw(
        &self,
        address: &str,
        amount: u64,
    ) -> Result<
        r3e_built_in_services::gas_bank::GasBankWithdrawal,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(r3e_built_in_services::gas_bank::GasBankWithdrawal {
            tx_hash: format!("0x{:016x}", chrono::Utc::now().timestamp()),
            address: address.to_string(),
            amount,
            fee: 100,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        })
    }

    async fn pay_gas_for_transaction(
        &self,
        tx_hash: &str,
        address: &str,
        amount: u64,
    ) -> Result<
        r3e_built_in_services::gas_bank::GasBankTransaction,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(r3e_built_in_services::gas_bank::GasBankTransaction {
            tx_hash: tx_hash.to_string(),
            address: address.to_string(),
            tx_type: "gas_payment".to_string(),
            amount,
            fee: 100,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: "confirmed".to_string(),
        })
    }

    async fn get_gas_price(&self) -> Result<u64, r3e_built_in_services::gas_bank::Error> {
        Ok(1000)
    }

    async fn estimate_gas(
        &self,
        tx_data: &[u8],
    ) -> Result<u64, r3e_built_in_services::gas_bank::Error> {
        Ok(21000 + (tx_data.len() as u64 * 68))
    }

    async fn get_balance(
        &self,
        address: &str,
    ) -> Result<u64, r3e_built_in_services::gas_bank::Error> {
        Ok(1000000)
    }

    async fn get_transactions(
        &self,
        address: &str,
    ) -> Result<
        Vec<r3e_built_in_services::gas_bank::GasBankTransaction>,
        r3e_built_in_services::gas_bank::Error,
    > {
        Ok(vec![])
    }
}
