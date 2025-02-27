/**
 * Neo N3 Transaction Event Handler Example
 * 
 * This function is triggered when a new transaction is processed on the Neo N3 blockchain.
 * It demonstrates how to access and process transaction data.
 * 
 * @param {Object} event - The event object containing transaction data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */

// Import the Neo module from the r3e runtime
import { neo } from 'r3e';
import { runlog } from 'r3e';

/**
 * Main handler function that processes Neo N3 transaction events
 */
export async function handler(event, context) {
  try {
    // Log the event
    runlog.info('Received Neo N3 transaction event');
    
    // Extract transaction data from the event
    const txData = event.data;
    
    // Basic transaction information
    const txHash = txData.hash;
    const txType = txData.type;
    const txSize = txData.size;
    const txVersion = txData.version;
    
    // Log basic transaction information
    runlog.info(`Processing transaction ${txHash}`);
    runlog.info(`Transaction type: ${txType}`);
    runlog.info(`Transaction size: ${txSize} bytes`);
    runlog.info(`Transaction version: ${txVersion}`);
    
    // Process transaction based on type
    let txDetails = {
      hash: txHash,
      type: txType,
      size: txSize,
      version: txVersion,
      processed: false,
      transfers: [],
      contractInvocations: []
    };
    
    // Process InvocationTransaction (smart contract invocation)
    if (txType === 'InvocationTransaction') {
      runlog.info('Processing InvocationTransaction');
      
      // Get detailed transaction information using the Neo module
      const detailedTx = await neo.getTransaction(txHash);
      
      if (detailedTx && detailedTx.script) {
        // Process script to identify contract invocations
        const script = detailedTx.script;
        
        // Check if this is a NEP-17 transfer (token transfer)
        if (script.includes('transfer')) {
          // Extract transfer details
          let transferInfo = await extractTransferInfo(txHash);
          
          if (transferInfo) {
            txDetails.transfers.push(transferInfo);
            runlog.info(`Detected token transfer: ${transferInfo.amount} ${transferInfo.asset} from ${transferInfo.from} to ${transferInfo.to}`);
          }
        }
        
        // Check for other contract invocations
        const contractHash = await extractContractHash(script);
        if (contractHash) {
          txDetails.contractInvocations.push({
            contractHash: contractHash,
            method: await extractMethodName(script)
          });
          
          runlog.info(`Detected contract invocation: ${contractHash}`);
        }
        
        txDetails.processed = true;
      }
    }
    
    // Process ClaimTransaction (GAS claim)
    else if (txType === 'ClaimTransaction') {
      runlog.info('Processing ClaimTransaction');
      
      // Get claim details
      const claimDetails = await neo.getClaimDetails(txHash);
      
      if (claimDetails) {
        txDetails.claimAmount = claimDetails.amount;
        txDetails.claimAddress = claimDetails.address;
        
        runlog.info(`GAS claim: ${claimDetails.amount} to ${claimDetails.address}`);
        txDetails.processed = true;
      }
    }
    
    // Process other transaction types
    else {
      runlog.info(`Transaction type ${txType} processing not implemented in this example`);
    }
    
    // Store the transaction details for later use
    // This uses the r3e.store module to persist data
    await context.store.set(`tx:${txHash}`, JSON.stringify(txDetails));
    
    // Return the transaction details
    return {
      status: 'success',
      message: `Successfully processed transaction ${txHash}`,
      data: txDetails
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error processing transaction event:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error processing transaction event: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Helper function to extract transfer information from a transaction
 * @param {string} txHash - The transaction hash
 * @returns {Object|null} - Transfer information or null if not a transfer
 */
async function extractTransferInfo(txHash) {
  try {
    // Get application log for the transaction
    const appLog = await neo.getApplicationLog(txHash);
    
    if (appLog && appLog.executions && appLog.executions.length > 0) {
      // Look for NEP-17 transfer notifications
      for (const execution of appLog.executions) {
        if (execution.notifications) {
          for (const notification of execution.notifications) {
            if (notification.contract && notification.state && notification.state.type === 'Array') {
              // Check if this is a transfer notification
              const values = notification.state.value;
              if (values.length >= 3 && values[0].value === 'transfer') {
                // Extract transfer details
                return {
                  asset: await getAssetSymbol(notification.contract),
                  from: values[1].value,
                  to: values[2].value,
                  amount: values.length >= 4 ? parseFloat(values[3].value) / 100000000 : 0 // Convert from satoshi to whole tokens
                };
              }
            }
          }
        }
      }
    }
    
    return null;
  } catch (error) {
    runlog.error('Error extracting transfer info:', error);
    return null;
  }
}

/**
 * Helper function to get asset symbol from contract hash
 * @param {string} contractHash - The contract hash
 * @returns {string} - Asset symbol
 */
async function getAssetSymbol(contractHash) {
  try {
    // Check if this is a native asset
    if (contractHash === neo.NATIVE_CONTRACT_HASH.NeoToken) {
      return 'NEO';
    } else if (contractHash === neo.NATIVE_CONTRACT_HASH.GasToken) {
      return 'GAS';
    }
    
    // For other tokens, try to get the symbol from contract
    const tokenSymbol = await neo.invokeFunction(contractHash, 'symbol', []);
    if (tokenSymbol && tokenSymbol.state === 'HALT' && tokenSymbol.stack && tokenSymbol.stack.length > 0) {
      return tokenSymbol.stack[0].value;
    }
    
    // Return contract hash if symbol cannot be determined
    return contractHash.substring(0, 10) + '...';
  } catch (error) {
    runlog.error('Error getting asset symbol:', error);
    return contractHash.substring(0, 10) + '...';
  }
}

/**
 * Helper function to extract contract hash from script
 * @param {string} script - The transaction script
 * @returns {string|null} - Contract hash or null if not found
 */
async function extractContractHash(script) {
  try {
    // This is a simplified example - in a real implementation,
    // you would need to parse the script properly
    const contractHashMatch = script.match(/([0-9a-fA-F]{40})/);
    if (contractHashMatch) {
      return '0x' + contractHashMatch[1];
    }
    return null;
  } catch (error) {
    runlog.error('Error extracting contract hash:', error);
    return null;
  }
}

/**
 * Helper function to extract method name from script
 * @param {string} script - The transaction script
 * @returns {string} - Method name or 'unknown'
 */
async function extractMethodName(script) {
  try {
    // This is a simplified example - in a real implementation,
    // you would need to parse the script properly
    const commonMethods = ['transfer', 'balanceOf', 'decimals', 'totalSupply', 'name', 'symbol'];
    
    for (const method of commonMethods) {
      if (script.includes(method)) {
        return method;
      }
    }
    
    return 'unknown';
  } catch (error) {
    runlog.error('Error extracting method name:', error);
    return 'unknown';
  }
}
