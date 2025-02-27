# Neo N3 Multi-Event Handler Example

This example demonstrates how to create a function that handles multiple types of Neo N3 blockchain events in a single function. This approach is useful for applications that need to respond to different types of blockchain events in a coordinated way.

## Overview

The multi-event handler example shows how to:

1. Configure a function to receive multiple types of Neo N3 blockchain events
2. Process different event types with a single handler function
3. Coordinate responses across different event types
4. Maintain state between event invocations
5. Implement complex event-driven workflows

## Files

- `function.js`: The JavaScript function that processes multiple event types
- `config.yaml`: Configuration file for the multi-event handler
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

The multi-event handler function is configured to receive multiple types of Neo N3 blockchain events:

- **Block events**: Triggered when a new block is added to the blockchain
- **Transaction events**: Triggered when a new transaction is processed
- **Contract notification events**: Triggered when a smart contract emits a notification

The function uses a single handler to process all event types, with type-specific processing logic:

```javascript
export async function handler(event, context) {
  // Determine the event type
  switch (event.type) {
    case 'block':
      return processBlockEvent(event.data, context);
    case 'transaction':
      return processTransactionEvent(event.data, context);
    case 'notification':
      return processNotificationEvent(event.data, context);
    default:
      throw new Error(`Unknown event type: ${event.type}`);
  }
}
```

## Coordinated Event Processing

This example demonstrates how to coordinate processing across different event types:

1. **State Aggregation**: Collect data from different event types to build a comprehensive view
2. **Cross-Event Correlation**: Link related events (e.g., transactions in a block, notifications from a transaction)
3. **Sequential Processing**: Process events in a specific order to implement complex workflows
4. **Conditional Logic**: Execute different logic based on the combination of events received

## Use Cases

The multi-event handler approach is useful for:

- **Analytics Applications**: Aggregate data from multiple event types
- **Monitoring Systems**: Track blockchain activity across different levels
- **Complex Workflows**: Implement multi-step processes triggered by different events
- **Coordinated Responses**: Execute actions that depend on multiple event types

## Customization

You can customize this example by:

- Modifying the event types to monitor in `config.yaml`
- Implementing different processing logic for each event type
- Adding additional event correlation logic
- Extending the state management to support your specific use case

## Additional Resources

- [Neo N3 Event Structure Documentation](https://docs.neo.org/docs/en-us/reference/rpc/latest-version/api.html)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
