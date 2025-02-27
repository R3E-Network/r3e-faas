# Neo N3 Custom Event Filter Example

This example demonstrates how to create custom event filters for Neo N3 blockchain events in the FaaS platform. Custom event filters allow you to process only specific events that match your criteria, reducing unnecessary function executions and improving performance.

## Overview

The custom event filter example shows how to:

1. Define complex filtering criteria for Neo N3 blockchain events
2. Combine multiple filter conditions using logical operators
3. Filter events based on transaction properties, contract properties, and event content
4. Create reusable filter components
5. Optimize event processing with efficient filters

## Files

- `function.js`: The JavaScript function that processes filtered events
- `config.yaml`: Configuration file with custom filter definitions
- `register.js`: Script to register the function with the FaaS platform
- `filters/`: Directory containing reusable filter components

## Prerequisites

- Neo N3 FaaS platform installed and running
- Access to a Neo N3 node (testnet or mainnet)
- Node.js installed for running the registration script

## Setup

1. Configure your Neo N3 node connection in `config.yaml`
2. Customize the filter definitions in `config.yaml` as needed
3. Register the function using the registration script:

```bash
node register.js
```

## How It Works

The custom event filter example demonstrates several filtering techniques:

### 1. Value-Based Filtering

Filter events based on specific values in the event data:

```yaml
filters:
  - type: value
    field: block.index
    operator: ">="
    value: 1000000
```

### 2. Range Filtering

Filter events that fall within a specific range:

```yaml
filters:
  - type: range
    field: transaction.size
    min: 100
    max: 1000
```

### 3. Pattern Matching

Filter events using regular expressions:

```yaml
filters:
  - type: pattern
    field: notification.eventname
    pattern: "^Transfer|^Mint"
```

### 4. Compound Filtering

Combine multiple filters with logical operators:

```yaml
filters:
  - type: compound
    operator: and
    conditions:
      - type: value
        field: contract.hash
        operator: "=="
        value: "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"
      - type: value
        field: notification.eventname
        operator: "=="
        value: "Transfer"
```

### 5. Custom JavaScript Filters

Define custom filtering logic in JavaScript:

```javascript
function customFilter(event) {
  // Custom filtering logic
  if (event.type === 'block' && event.data.index % 10 === 0) {
    return true; // Process only every 10th block
  }
  return false;
}
```

## Customization

You can customize this example by:

- Modifying the filter definitions in `config.yaml`
- Creating new filter components in the `filters/` directory
- Implementing custom JavaScript filter functions
- Combining different filter types for complex filtering scenarios

## Performance Considerations

- Use the most restrictive filters first to minimize processing
- Avoid complex regex patterns for high-volume events
- Consider using indexed fields for faster filtering
- Test filter performance with historical data before deployment

## Additional Resources

- [Neo N3 Event Structure Documentation](https://docs.neo.org/docs/en-us/reference/rpc/latest-version/api.html)
- [Neo N3 FaaS Platform Documentation](../../docs/neo-n3/README.md)
- [JavaScript Function Development Guide](../../docs/neo-n3/guides/function-development.md)
