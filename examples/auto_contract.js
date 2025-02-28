// Example of using automatic smart contract in R3E FaaS

// Function that sets up an automatic smart contract
export function setupAutoContract(args) {
  // Create a time-based trigger for daily execution
  const timeTrigger = {
    triggerType: 'time',
    params: {
      cron: '0 0 * * *', // Daily at midnight
      timezone: 'UTC'
    }
  };
  
  // Create a market-based trigger for price changes
  const marketTrigger = {
    triggerType: 'market',
    params: {
      assetPair: 'NEO/USD',
      condition: 'above',
      price: 50.0
    }
  };
  
  // Create a blockchain-based trigger for contract events
  const blockchainTrigger = {
    triggerType: 'blockchain',
    params: {
      network: 'neo_n3',
      contractAddress: 'NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj',
      eventName: 'Transfer'
    }
  };
  
  // Create automatic smart contracts with different triggers
  const dailyTransfer = r3e.autoContract.create({
    name: 'Daily Token Transfer',
    description: 'Transfer 1 NEO token daily',
    network: 'neo_n3',
    contractAddress: 'NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj',
    method: 'transfer',
    params: ['NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj', 1],
    trigger: timeTrigger
  });
  
  const priceBasedSwap = r3e.autoContract.create({
    name: 'Price-based Token Swap',
    description: 'Swap tokens when NEO price goes above $50',
    network: 'neo_n3',
    contractAddress: 'NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj',
    method: 'swap',
    params: ['NEO', 'GAS', 10],
    trigger: marketTrigger
  });
  
  const eventBasedMint = r3e.autoContract.create({
    name: 'Event-based NFT Mint',
    description: 'Mint an NFT when a Transfer event occurs',
    network: 'neo_n3',
    contractAddress: 'NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj',
    method: 'mint',
    params: ['NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj', 'https://example.com/metadata.json'],
    trigger: blockchainTrigger
  });
  
  return {
    success: true,
    contracts: [
      dailyTransfer,
      priceBasedSwap,
      eventBasedMint
    ]
  };
}

// Function to list and manage automatic smart contracts
export function manageAutoContracts(args) {
  // List all automatic smart contracts
  const contracts = r3e.autoContract.list();
  
  // Get execution history for a specific contract
  const executions = r3e.autoContract.getExecutions(contracts[0].id);
  
  // Disable a contract
  r3e.autoContract.update(contracts[1].id, { enabled: false });
  
  // Delete a contract
  r3e.autoContract.delete(contracts[2].id);
  
  return {
    success: true,
    contracts: contracts,
    executions: executions
  };
}
