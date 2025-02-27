/**
 * Neo N3 Custom Event Filter Example
 * 
 * This function demonstrates how to use custom event filters to process
 * specific Neo N3 blockchain events based on complex filtering criteria.
 * 
 * @param {Object} event - The event object containing blockchain event data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */

// Import the Neo module from the r3e runtime
import { neo } from 'r3e';
import { runlog } from 'r3e';
import { filters } from './filters';

/**
 * Main handler function that processes filtered Neo N3 blockchain events
 */
export async function handler(event, context) {
  try {
    // Log the event
    runlog.info('Received Neo N3 blockchain event');
    runlog.info(`Event type: ${event.type}`);
    
    // Apply custom JavaScript filters (in addition to the declarative filters in config.yaml)
    if (!applyCustomFilters(event)) {
      runlog.info('Event filtered out by custom JavaScript filters');
      return {
        status: 'filtered',
        message: 'Event did not match custom JavaScript filters',
        eventType: event.type
      };
    }
    
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
        runlog.warn(`Unknown event type: ${event.type}`);
        return {
          status: 'error',
          message: `Unknown event type: ${event.type}`
        };
    }
    
    // Return the processing result
    return {
      status: 'success',
      message: `Successfully processed filtered ${event.type} event`,
      data: result
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error processing filtered event:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error processing filtered event: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Apply custom JavaScript filters to the event
 * @param {Object} event - The event object
 * @returns {boolean} - True if the event passes all filters, false otherwise
 */
function applyCustomFilters(event) {
  // Example: Process only every 10th block
  if (event.type === 'block' && event.data.index % 10 !== 0) {
    return false;
  }
  
  // Example: Process only large transactions (> 500 bytes)
  if (event.type === 'transaction' && event.data.size <= 500) {
    return false;
  }
  
  // Example: Process only Transfer and Mint notifications
  if (event.type === 'notification' && 
      !['Transfer', 'Mint'].includes(event.data.eventname)) {
    return false;
  }
  
  // Example: Process only events from specific contracts
  if (event.type === 'notification') {
    const whitelistedContracts = [
      neo.NATIVE_CONTRACT_HASH.NeoToken,
      neo.NATIVE_CONTRACT_HASH.GasToken,
      // Add other contract hashes as needed
    ];
    
    if (!whitelistedContracts.includes(event.data.contract)) {
      return false;
    }
  }
  
  // Example: Use a custom filter from the filters module
  if (filters && typeof filters.isHighValueTransaction === 'function') {
    if (event.type === 'transaction' && !filters.isHighValueTransaction(event.data)) {
      return false;
    }
  }
  
  // All filters passed
  return true;
}

/**
 * Process a filtered block event
 * @param {Object} blockData - The block data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processBlockEvent(blockData, context) {
  runlog.info(`Processing filtered block #${blockData.index}`);
  
  // Extract relevant block information
  const blockSummary = {
    index: blockData.index,
    hash: blockData.hash,
    timestamp: new Date(blockData.time * 1000).toISOString(),
    transactionCount: blockData.tx ? blockData.tx.length : 0,
    size: blockData.size
  };
  
  // Store the block summary
  await context.store.set(`filtered_block:${blockData.index}`, JSON.stringify(blockSummary));
  
  // Return the block summary
  return blockSummary;
}

/**
 * Process a filtered transaction event
 * @param {Object} txData - The transaction data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processTransactionEvent(txData, context) {
  runlog.info(`Processing filtered transaction ${txData.hash}`);
  
  // Extract relevant transaction information
  const txSummary = {
    hash: txData.hash,
    type: txData.type,
    size: txData.size,
    version: txData.version,
    processed: true
  };
  
  // Store the transaction summary
  await context.store.set(`filtered_tx:${txData.hash}`, JSON.stringify(txSummary));
  
  // Return the transaction summary
  return txSummary;
}

/**
 * Process a filtered notification event
 * @param {Object} notificationData - The notification data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */
async function processNotificationEvent(notificationData, context) {
  runlog.info(`Processing filtered notification from contract ${notificationData.contract}`);
  
  // Extract relevant notification information
  const notificationSummary = {
    contract: notificationData.contract,
    txid: notificationData.txid,
    eventName: notificationData.eventname,
    state: notificationData.state,
    processed: true
  };
  
  // Store the notification summary
  await context.store.set(
    `filtered_notification:${notificationData.txid}:${notificationData.contract}:${notificationData.eventname}`,
    JSON.stringify(notificationSummary)
  );
  
  // Return the notification summary
  return notificationSummary;
}
