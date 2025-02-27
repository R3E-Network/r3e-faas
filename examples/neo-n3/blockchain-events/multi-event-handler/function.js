/**
 * Neo N3 Multi-Event Handler Example
 * 
 * This function demonstrates how to handle multiple types of Neo N3 blockchain events
 * in a single function, with coordinated processing across event types.
 * 
 * @param {Object} event - The event object containing blockchain event data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */

// Import the Neo module from the r3e runtime
import { neo } from 'r3e';
import { runlog } from 'r3e';

// Global state for cross-event coordination
// Note: This state persists only within the same function instance
// For persistent state across instances, use context.store
const state = {
  // Track the latest block we've seen
  latestBlock: null,
  
  // Track transactions we've processed
  processedTransactions: new Set(),
  
  // Track notifications by transaction
  notificationsByTx: new Map(),
  
  // Track statistics
  stats: {
    blocksProcessed: 0,
    transactionsProcessed: 0,
    notificationsProcessed: 0,
    tokenTransfers: 0,
    contractInvocations: 0
  }
};

/**
 * Main handler function that processes multiple Neo N3 blockchain event types
 */
export async function handler(event, context) {
  try {
    // Log the event
    runlog.info(`Received Neo N3 blockchain event of type: ${event.type}`);
    
    // Process the event based on its type
    let result;
    
    switch (event.type) {
      case 'block':
        result = await processBlockEvent(event.data, context);
        break;
      case 'transaction':
        result = await processTransactionEvent(event.data, context);
        break;
      case 'notification':
        result = await processNotificationEvent(event.data, context);
        break;
      default:
        runlog.error(`Unknown event type: ${event.type}`);
        throw new Error(`Unknown event type: ${event.type}`);
    }
    
    // Perform cross-event coordination
    await coordinateEvents(event.type, event.data, context);
    
    // Return the processing result
    return {
      status: 'success',
      message: `Successfully processed ${event.type} event`,
      data: result,
      stats: state.stats
    };
    
  } catch (error) {
    // Log any errors
    runlog.error(`Error processing ${event.type} event:`, error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error processing ${event.type} event: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Process a block event
 * @param {Object} blockData - The block data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processBlockEvent(blockData, context) {
  runlog.info(`Processing block #${blockData.index}`);
  
  // Update our state with the latest block
  state.latestBlock = {
    index: blockData.index,
    hash: blockData.hash,
    timestamp: new Date(blockData.time * 1000).toISOString(),
    transactionCount: blockData.tx ? blockData.tx.length : 0,
    size: blockData.size
  };
  
  // Update statistics
  state.stats.blocksProcessed++;
  
  // Store the block summary
  await context.store.set(`block:${blockData.index}`, JSON.stringify(state.latestBlock));
  
  // Return the block summary
  return state.latestBlock;
}

/**
 * Process a transaction event
 * @param {Object} txData - The transaction data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processTransactionEvent(txData, context) {
  runlog.info(`Processing transaction ${txData.hash}`);
  
  // Check if we've already processed this transaction
  if (state.processedTransactions.has(txData.hash)) {
    runlog.info(`Transaction ${txData.hash} already processed, skipping`);
    return { skipped: true, hash: txData.hash };
  }
  
  // Extract relevant transaction information
  const txSummary = {
    hash: txData.hash,
    type: txData.type,
    size: txData.size,
    version: txData.version,
    blockIndex: txData.blockindex,
    blockTime: txData.blocktime ? new Date(txData.blocktime * 1000).toISOString() : null
  };
  
  // Add to our processed transactions set
  state.processedTransactions.add(txData.hash);
  
  // Update statistics
  state.stats.transactionsProcessed++;
  
  // For InvocationTransactions, track contract invocations
  if (txData.type === 'InvocationTransaction') {
    state.stats.contractInvocations++;
  }
  
  // Store the transaction summary
  await context.store.set(`tx:${txData.hash}`, JSON.stringify(txSummary));
  
  // Return the transaction summary
  return txSummary;
}

/**
 * Process a notification event
 * @param {Object} notificationData - The notification data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processNotificationEvent(notificationData, context) {
  runlog.info(`Processing notification from contract ${notificationData.contract}`);
  
  // Extract relevant notification information
  const notificationSummary = {
    contract: notificationData.contract,
    txid: notificationData.txid,
    eventName: notificationData.eventname,
    state: notificationData.state,
    blockIndex: notificationData.blockindex,
    contractName: await getContractName(notificationData.contract)
  };
  
  // Update statistics
  state.stats.notificationsProcessed++;
  
  // Track token transfers
  if (notificationData.eventname === 'Transfer') {
    state.stats.tokenTransfers++;
  }
  
  // Store the notification in our map, grouped by transaction
  if (!state.notificationsByTx.has(notificationData.txid)) {
    state.notificationsByTx.set(notificationData.txid, []);
  }
  state.notificationsByTx.get(notificationData.txid).push(notificationSummary);
  
  // Store the notification summary
  await context.store.set(
    `notification:${notificationData.txid}:${notificationData.contract}:${notificationData.eventname}`,
    JSON.stringify(notificationSummary)
  );
  
  // Return the notification summary
  return notificationSummary;
}

/**
 * Coordinate processing across different event types
 * @param {string} eventType - The type of event that triggered this coordination
 * @param {Object} eventData - The event data
 * @param {Object} context - The execution context
 */
async function coordinateEvents(eventType, eventData, context) {
  // Example coordination: When we receive a block, check if we have processed
  // all transactions in that block and all notifications from those transactions
  if (eventType === 'block' && state.latestBlock) {
    runlog.info(`Coordinating events for block #${state.latestBlock.index}`);
    
    // Get all transactions in this block
    const blockTransactions = eventData.tx || [];
    
    // Check if we've processed all transactions in this block
    const processedAllTx = blockTransactions.every(tx => 
      state.processedTransactions.has(tx.hash || tx.txid)
    );
    
    if (processedAllTx) {
      runlog.info(`Processed all ${blockTransactions.length} transactions in block #${state.latestBlock.index}`);
      
      // Create a block summary with transaction details
      const blockSummary = {
        ...state.latestBlock,
        transactions: blockTransactions.map(tx => tx.hash || tx.txid),
        processedAt: new Date().toISOString()
      };
      
      // Store the complete block summary
      await context.store.set(`complete_block:${state.latestBlock.index}`, JSON.stringify(blockSummary));
    }
  }
  
  // Example coordination: When we receive a transaction, check if we have processed
  // all notifications from that transaction
  if (eventType === 'transaction') {
    const txHash = eventData.hash;
    
    // Check if we have notifications for this transaction
    if (state.notificationsByTx.has(txHash)) {
      const notifications = state.notificationsByTx.get(txHash);
      
      runlog.info(`Transaction ${txHash} has ${notifications.length} notifications`);
      
      // Create a transaction summary with notification details
      const txSummary = {
        hash: txHash,
        type: eventData.type,
        notifications: notifications.map(n => ({
          contract: n.contract,
          eventName: n.eventName,
          contractName: n.contractName
        })),
        processedAt: new Date().toISOString()
      };
      
      // Store the complete transaction summary
      await context.store.set(`complete_tx:${txHash}`, JSON.stringify(txSummary));
    }
  }
  
  // Store the current statistics
  await context.store.set('event_stats', JSON.stringify(state.stats));
}

/**
 * Helper function to get contract name
 * @param {string} contractHash - The contract hash
 * @returns {string} - The contract name or a shortened hash
 */
async function getContractName(contractHash) {
  try {
    // Check if this is a native contract
    if (contractHash === neo.NATIVE_CONTRACT_HASH.NeoToken) {
      return 'NEO';
    } else if (contractHash === neo.NATIVE_CONTRACT_HASH.GasToken) {
      return 'GAS';
    }
    
    // For other contracts, try to get the name from contract manifest
    const contractState = await neo.getContractState(contractHash);
    
    if (contractState && contractState.manifest && contractState.manifest.name) {
      return contractState.manifest.name;
    }
    
    // Return a shortened hash if name not available
    return contractHash.substring(0, 10) + '...';
  } catch (error) {
    runlog.error('Error getting contract name:', error);
    return contractHash.substring(0, 10) + '...';
  }
}
