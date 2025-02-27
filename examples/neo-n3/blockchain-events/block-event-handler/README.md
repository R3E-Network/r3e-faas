# Neo N3 Block Event Handler Example

This example demonstrates how to create a function that is triggered by Neo N3 blockchain block events. The function will be executed whenever a new block is added to the Neo N3 blockchain.

## Overview

The block event handler function is triggered whenever a new block is created on the Neo N3 blockchain. This example demonstrates:

1. How to register a function that responds to Neo N3 block events
2. How to access block data in your function
3. How to process and analyze block information
4. How to log and store results

## Files

- `function.js`: The JavaScript function that will be executed when a new block is created
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

When a new block is created on the Neo N3 blockchain, the FaaS platform detects this event and triggers the function. The function receives the block data as input and can process it as needed.

The example function:
1. Extracts basic information from the block (index, timestamp, transactions)
2. Logs the information
3. Analyzes transaction patterns
4. Stores summary information for later use

## Customization

You can customize this example by:

- Modifying the function to extract different information from blocks
- Changing the trigger configuration to filter specific blocks
- Adding additional processing logic for block data
- Integrating with other services or databases

## Additional Resources

- [Neo N3 Block Structure Documentation](https://docs.neo.org/docs/en-us/reference/rpc/latest-version/api/getblock.html)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
