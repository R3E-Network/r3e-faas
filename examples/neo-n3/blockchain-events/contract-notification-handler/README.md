# Neo N3 Contract Notification Event Handler Example

This example demonstrates how to create a function that is triggered by Neo N3 blockchain contract notification events. The function will be executed whenever a smart contract on the Neo N3 blockchain emits a notification.

## Overview

The contract notification event handler function is triggered whenever a smart contract on the Neo N3 blockchain emits a notification. This example demonstrates:

1. How to register a function that responds to Neo N3 contract notification events
2. How to access notification data in your function
3. How to filter notifications based on contract hash and event name
4. How to process and analyze notification information
5. How to log and store results

## Files

- `function.js`: The JavaScript function that will be executed when a contract notification is emitted
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

When a smart contract on the Neo N3 blockchain emits a notification, the FaaS platform detects this event and triggers the function. The function receives the notification data as input and can process it as needed.

The example function:
1. Extracts basic information from the notification (contract hash, event name, arguments)
2. Logs the information
3. Analyzes notification details (e.g., token transfers, contract state changes)
4. Stores summary information for later use

## Notification Filtering

This example demonstrates how to filter notifications based on:

- Contract hash (e.g., only process notifications from specific contracts)
- Event name (e.g., only process "Transfer" events)
- Notification content (e.g., only process notifications with specific arguments)

## Customization

You can customize this example by:

- Modifying the function to extract different information from notifications
- Changing the trigger configuration to filter specific notification types
- Adding additional processing logic for notification data
- Integrating with other services or databases

## Additional Resources

- [Neo N3 Notification Structure Documentation](https://docs.neo.org/docs/en-us/reference/rpc/latest-version/api/getapplicationlog.html)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
