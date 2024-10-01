// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc};

use r3e_worker::{Assigner, Worker, WorkerConfig};

#[derive(clap::Args)]
pub struct WorkerCmd {
    #[arg(long, help = "The worker config file path")]
    config: String,
}

impl WorkerCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let config = crate::read_file(&self.config)?;

        let config: WorkerConfig = serde_yaml::from_str(&config)?;
        let graceful = config.graceful;
        let worker = Arc::new(Worker::new(config));

        let stopper = Arc::new(AtomicBool::new(false));
        r3e_core::signal_hooks("worker", stopper.clone());

        let (tx, rx) = mpsc::sync_channel(1);
        {
            let worker = worker.clone();
            let stopper = stopper.clone();
            std::thread::spawn(move || {
                if let Err(err) = worker.run(rx, stopper) {
                    log::error!("worker run error: {}", err);
                }
            });
        }

        let assigner = Assigner::new(tx, worker);
        if let Err(err) = assigner.run(stopper) {
            log::error!("assigner run error: {}", err);
        }

        // wait for gracefully exit
        std::thread::sleep(graceful);
        log::warn!("node exited, pid {}", std::process::id());
        Ok(())
    }
}
