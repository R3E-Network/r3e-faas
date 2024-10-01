# R3E FaaS: A FaaS Platform for Web3

## Components:

* r3e-core: core & common functions;
* r3e-deno: js-runtime based on deno-core;
* r3e-event: defines events and fetch events from various sources;
* r3e-proc-marco: some proc-macros;
* r3e-runlog: catches and stores user function run logs;
* r3e-scheduler: scheduler logical, i.e. r3e-master, schedules worker nodes and a user function when/where to run;
* r3e-stock: for querying a blockchain history/stock data, like previous block data, etc.
* r3e-store: stores event and others;
* r3e-worker: how to run user function in a worker node;

## Architecture
