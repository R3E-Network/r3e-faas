/**
 * Neo N3 Block Event Handler Example
 * 
 * This function is triggered when a new block is created on the Neo N3 blockchain.
 * It demonstrates how to access and process block data.
 * 
 * @param {Object} event - The event object containing block data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */

// Import the Neo module from the r3e runtime
import { neo } from 'r3e';
import { runlog } from 'r3e';

/**
 * Main handler function that processes Neo N3 block events
 */
export async function handler(event, context) {
  try {
    // Log the event
    runlog.info('Received Neo N3 block event');
    
    // Extract block data from the event
    const blockData = event.data;
    
    // Basic block information
    const blockIndex = blockData.index;
    const blockHash = blockData.hash;
    const blockTime = new Date(blockData.time * 1000); // Convert timestamp to Date
    const txCount = blockData.tx ? blockData.tx.length : 0;
    
    // Log basic block information
    runlog.info(`Processing block #${blockIndex} (${blockHash})`);
    runlog.info(`Block time: ${blockTime.toISOString()}`);
    runlog.info(`Transaction count: ${txCount}`);
    
    // Process transactions if present
    let neoTransfers = 0;
    let gasTransfers = 0;
    let smartContractInvocations = 0;
    
    if (blockData.tx && blockData.tx.length > 0) {
      // Analyze transactions
      for (const tx of blockData.tx) {
        // Check transaction type
        if (tx.type === 'InvocationTransaction') {
          smartContractInvocations++;
          
          // Use Neo module to get more details about the transaction
          const txDetails = await neo.getTransaction(tx.hash);
          
          // Check if this is a NEP-17 transfer (token transfer)
          if (txDetails && txDetails.script.includes('transfer')) {
            // Determine if this is a NEO or GAS transfer
            if (txDetails.script.includes(neo.NATIVE_CONTRACT_HASH.NeoToken)) {
              neoTransfers++;
            } else if (txDetails.script.includes(neo.NATIVE_CONTRACT_HASH.GasToken)) {
              gasTransfers++;
            }
          }
        }
      }
    }
    
    // Create a summary of the block
    const blockSummary = {
      index: blockIndex,
      hash: blockHash,
      timestamp: blockTime.toISOString(),
      transactionCount: txCount,
      neoTransfers: neoTransfers,
      gasTransfers: gasTransfers,
      smartContractInvocations: smartContractInvocations
    };
    
    // Log the summary
    runlog.info('Block summary:', blockSummary);
    
    // Store the block summary for later use
    // This uses the r3e.store module to persist data
    await context.store.set(`block:${blockIndex}`, JSON.stringify(blockSummary));
    
    // Return the block summary
    return {
      status: 'success',
      message: `Successfully processed block #${blockIndex}`,
      data: blockSummary
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error processing block event:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error processing block event: ${error.message}`,
      error: error.stack
    };
  }
}
