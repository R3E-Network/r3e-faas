// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::{mpsc, Arc};
use std::time::Duration;

use crate::{Action, Stopper, Worker};

#[derive(Debug, Clone, Copy)]
pub struct Assign {
    pub uid: u64,
    pub action: Action,
}

pub struct Assigner {
    worker: Arc<Worker>,
    tx: mpsc::SyncSender<Assign>,
}

#[derive(Debug, thiserror::Error)]
pub enum AssignError {
    #[error("assign: send action to worker failed: {0:?}")]
    SendError(Assign),
}

impl Assigner {
    pub fn new(tx: mpsc::SyncSender<Assign>, worker: Arc<Worker>) -> Self {
        Self { tx, worker }
    }

    // TODO:
    pub fn run(&self, stopper: impl Stopper) -> Result<(), AssignError> {
        while !stopper.stopped() {
            let uid = 1;
            if self.worker.runners_of(uid) == 0 {
                let assign = Assign {
                    uid,
                    action: Action::Spawn,
                };
                self.tx
                    .send(assign)
                    .map_err(|_err| AssignError::SendError(assign))?;
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        Ok(())
    }
}
