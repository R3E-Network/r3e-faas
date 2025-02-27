# Neo N3 Transaction Event Handler Example

This example demonstrates how to create a function that is triggered by Neo N3 blockchain transaction events. The function will be executed whenever a new transaction is added to the Neo N3 blockchain.

## Overview

The transaction event handler function is triggered whenever a new transaction is processed on the Neo N3 blockchain. This example demonstrates:

1. How to register a function that responds to Neo N3 transaction events
2. How to access transaction data in your function
3. How to filter transactions based on type or content
4. How to process and analyze transaction information
5. How to log and store results

## Files

- `function.js`: The JavaScript function that will be executed when a new transaction is processed
- `config.yaml`: Configuration file for the function
- `register.js`: Script to register the function with the FaaS platform

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script

## Setup

1. Configure your Neo N3 node connection in `config.yaml`
2. Register the function using the registration script:

```bash
node register.js
```

## How It Works

When a new transaction is processed on the Neo N3 blockchain, the FaaS platform detects this event and triggers the function. The function receives the transaction data as input and can process it as needed.

The example function:
1. Extracts basic information from the transaction (hash, type, sender, receiver)
2. Logs the information
3. Analyzes transaction details (e.g., token transfers, smart contract invocations)
4. Stores summary information for later use

## Transaction Filtering

This example demonstrates how to filter transactions based on:

- Transaction type (e.g., only process InvocationTransactions)
- Contract hash (e.g., only process transactions involving a specific contract)
- Sender/receiver addresses (e.g., only process transactions from/to specific addresses)
- Transaction content (e.g., only process transactions with specific script contents)

## Customization

You can customize this example by:

- Modifying the function to extract different information from transactions
- Changing the trigger configuration to filter specific transaction types
- Adding additional processing logic for transaction data
- Integrating with other services or databases

## Additional Resources

- [Neo N3 Transaction Structure Documentation](https://docs.neo.org/docs/en-us/reference/rpc/latest-version/api/getrawtransaction.html)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
