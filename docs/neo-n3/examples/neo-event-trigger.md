# Neo N3 Event Trigger Example for Neo N3 FaaS Platform

This example demonstrates how to create, deploy, and use a function triggered by Neo N3 blockchain events on the Neo N3 FaaS platform.

## Prerequisites

Before you begin, ensure you have the following:

- Neo N3 FaaS CLI installed (`npm install -g r3e-faas-cli`)
- A Neo N3 FaaS account
- Basic knowledge of JavaScript and Neo N3 blockchain

## Project Setup

1. Create a new project directory:

```bash
mkdir neo-event-monitor
cd neo-event-monitor
```

2. Initialize a new Neo N3 FaaS project:

```bash
r3e-faas-cli init
```

This command creates the following files:

```
neo-event-monitor/
├── functions/
│   └── hello.js
├── r3e.yaml
└── package.json
```

## Function Implementation

Create a new file `functions/block-monitor.js` with the following code:

```javascript
/**
 * A function that monitors Neo N3 blockchain blocks
 * 
 * @param {Object} event - The event object containing blockchain data
 * @param {Object} context - The context object providing access to Neo N3 APIs
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Block event received:', JSON.stringify(event, null, 2));
  
  // Extract block data from the event
  const { blockHeight, blockHash, blockTime, transactions } = event.data;
  
  // Log block information
  console.log(`New block detected at height ${blockHeight}`);
  console.log(`Block hash: ${blockHash}`);
  console.log(`Block time: ${new Date(blockTime).toISOString()}`);
  console.log(`Transaction count: ${transactions.length}`);
  
  // Get additional block information using the Neo N3 API
  let blockDetails;
  try {
    blockDetails = await context.neo.getBlockByHeight(blockHeight);
    console.log('Block details retrieved successfully');
  } catch (error) {
    console.error('Error getting block details:', error);
    blockDetails = { error: error.message };
  }
  
  // Store the block information in the platform storage
  try {
    await context.storage.set(`block:${blockHeight}`, {
      height: blockHeight,
      hash: blockHash,
      time: blockTime,
      transactionCount: transactions.length,
      details: blockDetails
    });
    console.log(`Block ${blockHeight} stored in database`);
  } catch (error) {
    console.error('Error storing block data:', error);
  }
  
  // Return a response with block information
  return {
    blockHeight,
    blockHash,
    blockTime: new Date(blockTime).toISOString(),
    transactionCount: transactions.length,
    processed: true,
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives a Neo N3 block event with block data
2. Logs the block information for debugging
3. Gets additional block details using the Neo N3 API
4. Stores the block information in the platform storage
5. Returns a response with block information

Now, create another file `functions/transaction-monitor.js` with the following code:

```javascript
/**
 * A function that monitors Neo N3 blockchain transactions
 * 
 * @param {Object} event - The event object containing transaction data
 * @param {Object} context - The context object providing access to Neo N3 APIs
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Transaction event received:', JSON.stringify(event, null, 2));
  
  // Extract transaction data from the event
  const { txHash, blockHeight, sender, receiver, asset, amount } = event.data;
  
  // Log transaction information
  console.log(`New transaction detected: ${txHash}`);
  console.log(`Block height: ${blockHeight}`);
  console.log(`Sender: ${sender}`);
  console.log(`Receiver: ${receiver}`);
  console.log(`Asset: ${asset}`);
  console.log(`Amount: ${amount}`);
  
  // Get additional transaction information using the Neo N3 API
  let txDetails;
  try {
    txDetails = await context.neo.getTransaction(txHash);
    console.log('Transaction details retrieved successfully');
  } catch (error) {
    console.error('Error getting transaction details:', error);
    txDetails = { error: error.message };
  }
  
  // Check if this is a specific asset transfer we're interested in
  const isTargetAsset = asset === 'NEO' || asset === 'GAS';
  const isLargeAmount = parseFloat(amount) > 100;
  
  if (isTargetAsset && isLargeAmount) {
    console.log(`Large ${asset} transfer detected: ${amount} ${asset}`);
    
    // Get the current price of the asset
    try {
      const price = await context.oracle.getPrice(asset, 'USD');
      console.log(`Current ${asset} price: $${price} USD`);
      
      // Calculate the total value
      const totalValue = parseFloat(amount) * price;
      console.log(`Total value: $${totalValue.toFixed(2)} USD`);
      
      // Store the large transfer in the platform storage
      await context.storage.set(`large-transfer:${txHash}`, {
        txHash,
        blockHeight,
        sender,
        receiver,
        asset,
        amount,
        price,
        totalValue,
        timestamp: new Date().toISOString()
      });
      
      // You could also send a notification here
      // await context.notification.send({
      //   title: `Large ${asset} Transfer`,
      //   body: `${amount} ${asset} ($${totalValue.toFixed(2)} USD) transferred from ${sender} to ${receiver}`,
      //   data: { txHash, blockHeight, asset, amount }
      // });
    } catch (error) {
      console.error('Error processing large transfer:', error);
    }
  }
  
  // Return a response with transaction information
  return {
    txHash,
    blockHeight,
    sender,
    receiver,
    asset,
    amount,
    isLargeTransfer: isTargetAsset && isLargeAmount,
    processed: true,
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives a Neo N3 transaction event with transaction data
2. Logs the transaction information for debugging
3. Gets additional transaction details using the Neo N3 API
4. Checks if the transaction is a large transfer of NEO or GAS
5. If it's a large transfer, gets the current price using the Oracle service
6. Stores the large transfer information in the platform storage
7. Returns a response with transaction information

Finally, create a file `functions/contract-monitor.js` with the following code:

```javascript
/**
 * A function that monitors Neo N3 smart contract notifications
 * 
 * @param {Object} event - The event object containing contract notification data
 * @param {Object} context - The context object providing access to Neo N3 APIs
 * @returns {Object} - The response object
 */
export default async function(event, context) {
  // Log the event for debugging
  console.log('Contract notification received:', JSON.stringify(event, null, 2));
  
  // Extract contract notification data from the event
  const { contractHash, eventName, params, blockHeight, txHash } = event.data;
  
  // Log contract notification information
  console.log(`Contract notification detected from ${contractHash}`);
  console.log(`Event name: ${eventName}`);
  console.log(`Block height: ${blockHeight}`);
  console.log(`Transaction hash: ${txHash}`);
  console.log(`Parameters:`, params);
  
  // Get contract information using the Neo N3 API
  let contractInfo;
  try {
    contractInfo = await context.neo.getContract(contractHash);
    console.log(`Contract name: ${contractInfo.name}`);
    console.log(`Contract version: ${contractInfo.version}`);
  } catch (error) {
    console.error('Error getting contract information:', error);
    contractInfo = { error: error.message };
  }
  
  // Process specific contract events
  if (eventName === 'Transfer') {
    // Process Transfer event
    const from = params.from;
    const to = params.to;
    const amount = params.amount;
    
    console.log(`Transfer event: ${amount} tokens from ${from} to ${to}`);
    
    // Store the transfer event in the platform storage
    try {
      await context.storage.set(`transfer:${txHash}`, {
        contractHash,
        contractName: contractInfo.name,
        eventName,
        from,
        to,
        amount,
        blockHeight,
        txHash,
        timestamp: new Date().toISOString()
      });
      console.log(`Transfer event ${txHash} stored in database`);
    } catch (error) {
      console.error('Error storing transfer event:', error);
    }
  } else if (eventName === 'Mint' || eventName === 'Burn') {
    // Process Mint or Burn event
    const account = params.account;
    const amount = params.amount;
    
    console.log(`${eventName} event: ${amount} tokens for ${account}`);
    
    // Store the mint/burn event in the platform storage
    try {
      await context.storage.set(`${eventName.toLowerCase()}:${txHash}`, {
        contractHash,
        contractName: contractInfo.name,
        eventName,
        account,
        amount,
        blockHeight,
        txHash,
        timestamp: new Date().toISOString()
      });
      console.log(`${eventName} event ${txHash} stored in database`);
    } catch (error) {
      console.error(`Error storing ${eventName} event:`, error);
    }
  }
  
  // Return a response with contract notification information
  return {
    contractHash,
    contractName: contractInfo.name,
    eventName,
    params,
    blockHeight,
    txHash,
    processed: true,
    timestamp: new Date().toISOString()
  };
}
```

This function:
1. Receives a Neo N3 contract notification event with contract data
2. Logs the contract notification information for debugging
3. Gets contract information using the Neo N3 API
4. Processes specific contract events like Transfer, Mint, and Burn
5. Stores the event information in the platform storage
6. Returns a response with contract notification information

## Configuration

Open the `r3e.yaml` file and update it with the following configuration:

```yaml
project:
  name: neo-event-monitor
  version: 0.1.0

functions:
  block-monitor:
    handler: functions/block-monitor.js
    runtime: javascript
    trigger:
      type: neo
      event: NEW_BLOCK
      network: mainnet
    environment:
      NODE_ENV: production
  
  transaction-monitor:
    handler: functions/transaction-monitor.js
    runtime: javascript
    trigger:
      type: neo
      event: NEW_TRANSACTION
      network: mainnet
    environment:
      NODE_ENV: production
  
  contract-monitor:
    handler: functions/contract-monitor.js
    runtime: javascript
    trigger:
      type: neo
      event: CONTRACT_NOTIFICATION
      contract: 0x1234567890abcdef # Replace with your contract hash
      network: mainnet
    environment:
      NODE_ENV: production
```

This configuration:
1. Sets the project name and version
2. Defines three functions: block-monitor, transaction-monitor, and contract-monitor
3. Specifies the handler file path for each function
4. Sets the runtime to JavaScript for each function
5. Configures Neo N3 blockchain event triggers for each function:
   - block-monitor: Triggered by new blocks
   - transaction-monitor: Triggered by new transactions
   - contract-monitor: Triggered by contract notifications from a specific contract
6. Sets the `NODE_ENV` environment variable to `production` for each function

## Local Testing

Before deploying, test the functions locally using mock events:

### Test Block Monitor

Create a file `events/block-event.json` with the following content:

```json
{
  "data": {
    "blockHeight": 12345,
    "blockHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    "blockTime": 1645678901234,
    "transactions": [
      {
        "txHash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        "sender": "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
        "receiver": "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
        "asset": "NEO",
        "amount": "10"
      },
      {
        "txHash": "0x0987654321fedcba0987654321fedcba0987654321fedcba0987654321fedcba",
        "sender": "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
        "receiver": "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
        "asset": "GAS",
        "amount": "5.5"
      }
    ]
  }
}
```

Test the block-monitor function:

```bash
r3e-faas-cli invoke-local --function block-monitor --event events/block-event.json
```

### Test Transaction Monitor

Create a file `events/transaction-event.json` with the following content:

```json
{
  "data": {
    "txHash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    "blockHeight": 12345,
    "sender": "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
    "receiver": "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
    "asset": "NEO",
    "amount": "150"
  }
}
```

Test the transaction-monitor function:

```bash
r3e-faas-cli invoke-local --function transaction-monitor --event events/transaction-event.json
```

### Test Contract Monitor

Create a file `events/contract-event.json` with the following content:

```json
{
  "data": {
    "contractHash": "0x1234567890abcdef",
    "eventName": "Transfer",
    "params": {
      "from": "NXV7ZhHiyM1aHXwvUBCta7dGNP1tHwfoQG",
      "to": "NZHf1NJvz1tvELGLWZjhpb3NqZJFFUYpxT",
      "amount": "1000"
    },
    "blockHeight": 12345,
    "txHash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
  }
}
```

Test the contract-monitor function:

```bash
r3e-faas-cli invoke-local --function contract-monitor --event events/contract-event.json
```

## Deployment

Deploy the functions to the Neo N3 FaaS platform:

```bash
r3e-faas-cli deploy
```

This command:
1. Packages the function code
2. Uploads it to the Neo N3 FaaS platform
3. Creates the necessary resources
4. Configures the Neo N3 blockchain event triggers

After successful deployment, you should see output similar to:

```
Deploying project: neo-event-monitor
Deploying function: block-monitor... Done!
Deploying function: transaction-monitor... Done!
Deploying function: contract-monitor... Done!
```

## Monitoring

View the function logs:

```bash
r3e-faas-cli logs --function block-monitor
```

To follow the logs in real-time:

```bash
r3e-faas-cli logs --function block-monitor --follow
```

## Accessing Stored Data

You can access the data stored by the functions using the Neo N3 FaaS API:

```bash
# Get stored block data
r3e-faas-cli storage get --key "block:12345"

# Get stored large transfer data
r3e-faas-cli storage get --key "large-transfer:0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"

# Get stored transfer event data
r3e-faas-cli storage get --key "transfer:0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
```

## Advanced Configuration

### Filtering Transactions

You can filter transactions by adding a filter to the transaction-monitor function configuration:

```yaml
transaction-monitor:
  handler: functions/transaction-monitor.js
  runtime: javascript
  trigger:
    type: neo
    event: NEW_TRANSACTION
    network: mainnet
    filter:
      assets: ["NEO", "GAS"]
      minAmount: 10
  environment:
    NODE_ENV: production
```

### Filtering Contract Notifications

You can filter contract notifications by adding a filter to the contract-monitor function configuration:

```yaml
contract-monitor:
  handler: functions/contract-monitor.js
  runtime: javascript
  trigger:
    type: neo
    event: CONTRACT_NOTIFICATION
    contract: 0x1234567890abcdef
    network: mainnet
    filter:
      events: ["Transfer", "Mint", "Burn"]
  environment:
    NODE_ENV: production
```

## Cleaning Up

To remove the functions:

```bash
r3e-faas-cli remove --function block-monitor
r3e-faas-cli remove --function transaction-monitor
r3e-faas-cli remove --function contract-monitor
```

To remove the entire project:

```bash
r3e-faas-cli remove
```

## Next Steps

Now that you've created, deployed, and tested functions triggered by Neo N3 blockchain events, you can:

1. Implement more complex event processing logic
2. Use Oracle services for external data
3. Implement secure computing with TEE services
4. Create a custom dashboard to visualize the blockchain data

For more examples, see:
- [Oracle Service Example](./oracle-service.md)
- [TEE Service Example](./tee-service.md)
- [Service API Example](./service-api.md)

For more information, see the [Documentation](../README.md).
