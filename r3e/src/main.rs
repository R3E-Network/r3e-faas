// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::fs::File;
use std::io::Read;

use clap::{Parser, Subcommand};

use crate::worker::WorkerCmd;

mod worker;

#[derive(Parser)]
#[command(author = "R3E Network Team")]
#[command(version = r3e_core::VERSION)]
#[command(about = "A FaaS Framework for Web3")]
struct Cli {
    #[arg(long, help = "The log config file path")]
    log: Option<String>,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run worker")]
    Worker(WorkerCmd),
}

// run worker test mode:
// r3e-faas --log ./config/log.dev.yaml worker --config ./config/r3e-faas-worker.test.yaml
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    #[cfg(not(test))]
    if let Some(log) = &cli.log {
        log4rs::init_file(log, Default::default())?;
    }

    match cli.commands {
        Commands::Worker(cmd) => cmd.run()?,
    }

    Ok(())
}

pub(crate) fn read_file(file: &str) -> anyhow::Result<String> {
    let mut file = File::open(file)?;
    let mut content = String::new();

    let _ = file.read_to_string(&mut content)?;
    Ok(content)
}
