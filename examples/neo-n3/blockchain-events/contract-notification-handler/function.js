/**
 * Neo N3 Contract Notification Event Handler Example
 * 
 * This function is triggered when a smart contract on the Neo N3 blockchain emits a notification.
 * It demonstrates how to access and process notification data.
 * 
 * @param {Object} event - The event object containing notification data
 * @param {Object} context - The execution context
 * @returns {Object} - Processing result
 */

// Import the Neo module from the r3e runtime
import { neo } from 'r3e';
import { runlog } from 'r3e';

/**
 * Main handler function that processes Neo N3 contract notification events
 */
export async function handler(event, context) {
  try {
    // Log the event
    runlog.info('Received Neo N3 contract notification event');
    
    // Extract notification data from the event
    const notificationData = event.data;
    
    // Basic notification information
    const contractHash = notificationData.contract;
    const txHash = notificationData.txid;
    const eventName = notificationData.eventname;
    const state = notificationData.state;
    
    // Log basic notification information
    runlog.info(`Processing notification from contract ${contractHash}`);
    runlog.info(`Transaction hash: ${txHash}`);
    runlog.info(`Event name: ${eventName}`);
    
    // Process notification based on event name and contract
    let notificationDetails = {
      contract: contractHash,
      txid: txHash,
      eventName: eventName,
      processed: false,
      contractType: 'unknown',
      summary: {}
    };
    
    // Get contract information
    const contractInfo = await getContractInfo(contractHash);
    if (contractInfo) {
      notificationDetails.contractType = contractInfo.type;
      notificationDetails.contractName = contractInfo.name;
      runlog.info(`Contract type: ${contractInfo.type}`);
      runlog.info(`Contract name: ${contractInfo.name}`);
    }
    
    // Process NEP-17 Transfer events (token transfers)
    if (eventName === 'Transfer') {
      runlog.info('Processing NEP-17 Transfer event');
      
      // Extract transfer details from state
      const transferDetails = extractTransferDetails(state);
      
      if (transferDetails) {
        notificationDetails.summary = {
          type: 'token_transfer',
          from: transferDetails.from,
          to: transferDetails.to,
          amount: transferDetails.amount,
          asset: notificationDetails.contractName || contractHash
        };
        
        runlog.info(`Token transfer: ${transferDetails.amount} ${notificationDetails.contractName || 'tokens'} from ${transferDetails.from} to ${transferDetails.to}`);
        notificationDetails.processed = true;
      }
    }
    
    // Process NEP-11 Transfer events (NFT transfers)
    else if (eventName === 'Transfer' && contractInfo && contractInfo.type === 'NEP-11') {
      runlog.info('Processing NEP-11 Transfer event (NFT)');
      
      // Extract NFT transfer details from state
      const nftTransferDetails = extractNFTTransferDetails(state);
      
      if (nftTransferDetails) {
        notificationDetails.summary = {
          type: 'nft_transfer',
          from: nftTransferDetails.from,
          to: nftTransferDetails.to,
          tokenId: nftTransferDetails.tokenId,
          asset: notificationDetails.contractName || contractHash
        };
        
        runlog.info(`NFT transfer: Token ID ${nftTransferDetails.tokenId} from ${nftTransferDetails.from} to ${nftTransferDetails.to}`);
        notificationDetails.processed = true;
      }
    }
    
    // Process other common events
    else if (eventName === 'Mint') {
      runlog.info('Processing Mint event');
      
      // Extract mint details from state
      const mintDetails = extractMintDetails(state);
      
      if (mintDetails) {
        notificationDetails.summary = {
          type: 'token_mint',
          to: mintDetails.to,
          amount: mintDetails.amount,
          asset: notificationDetails.contractName || contractHash
        };
        
        runlog.info(`Token mint: ${mintDetails.amount} ${notificationDetails.contractName || 'tokens'} to ${mintDetails.to}`);
        notificationDetails.processed = true;
      }
    }
    
    // Process Burn events
    else if (eventName === 'Burn') {
      runlog.info('Processing Burn event');
      
      // Extract burn details from state
      const burnDetails = extractBurnDetails(state);
      
      if (burnDetails) {
        notificationDetails.summary = {
          type: 'token_burn',
          from: burnDetails.from,
          amount: burnDetails.amount,
          asset: notificationDetails.contractName || contractHash
        };
        
        runlog.info(`Token burn: ${burnDetails.amount} ${notificationDetails.contractName || 'tokens'} from ${burnDetails.from}`);
        notificationDetails.processed = true;
      }
    }
    
    // Process custom events
    else {
      runlog.info(`Processing custom event: ${eventName}`);
      
      // Extract custom event details
      notificationDetails.summary = {
        type: 'custom_event',
        eventName: eventName,
        state: state
      };
      
      runlog.info(`Custom event data:`, state);
      notificationDetails.processed = true;
    }
    
    // Store the notification details for later use
    // This uses the r3e.store module to persist data
    await context.store.set(`notification:${txHash}:${contractHash}:${eventName}`, JSON.stringify(notificationDetails));
    
    // Return the notification details
    return {
      status: 'success',
      message: `Successfully processed notification from contract ${contractHash}`,
      data: notificationDetails
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error processing contract notification event:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error processing contract notification event: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Helper function to get contract information
 * @param {string} contractHash - The contract hash
 * @returns {Object|null} - Contract information or null if not found
 */
async function getContractInfo(contractHash) {
  try {
    // Check if this is a native contract
    if (contractHash === neo.NATIVE_CONTRACT_HASH.NeoToken) {
      return { type: 'NEP-17', name: 'NEO' };
    } else if (contractHash === neo.NATIVE_CONTRACT_HASH.GasToken) {
      return { type: 'NEP-17', name: 'GAS' };
    }
    
    // For other contracts, try to get information from contract
    const contractManifest = await neo.getContractState(contractHash);
    
    if (contractManifest) {
      // Determine contract type based on supported standards
      let type = 'unknown';
      let name = contractHash.substring(0, 10) + '...';
      
      if (contractManifest.manifest && contractManifest.manifest.supportedstandards) {
        if (contractManifest.manifest.supportedstandards.includes('NEP-17')) {
          type = 'NEP-17';
        } else if (contractManifest.manifest.supportedstandards.includes('NEP-11')) {
          type = 'NEP-11';
        }
      }
      
      // Get contract name
      if (contractManifest.manifest && contractManifest.manifest.name) {
        name = contractManifest.manifest.name;
      }
      
      return { type, name };
    }
    
    return null;
  } catch (error) {
    runlog.error('Error getting contract info:', error);
    return null;
  }
}

/**
 * Helper function to extract transfer details from state
 * @param {Array} state - The notification state
 * @returns {Object|null} - Transfer details or null if not a valid transfer
 */
function extractTransferDetails(state) {
  try {
    // Check if state has the expected format for a transfer
    if (Array.isArray(state) && state.length >= 3) {
      // Extract from, to, and amount
      const from = state[0].value;
      const to = state[1].value;
      const amount = parseFloat(state[2].value) / 100000000; // Convert from satoshi to whole tokens
      
      return { from, to, amount };
    }
    
    return null;
  } catch (error) {
    runlog.error('Error extracting transfer details:', error);
    return null;
  }
}

/**
 * Helper function to extract NFT transfer details from state
 * @param {Array} state - The notification state
 * @returns {Object|null} - NFT transfer details or null if not a valid NFT transfer
 */
function extractNFTTransferDetails(state) {
  try {
    // Check if state has the expected format for an NFT transfer
    if (Array.isArray(state) && state.length >= 3) {
      // Extract from, to, and tokenId
      const from = state[0].value;
      const to = state[1].value;
      const tokenId = state[2].value;
      
      return { from, to, tokenId };
    }
    
    return null;
  } catch (error) {
    runlog.error('Error extracting NFT transfer details:', error);
    return null;
  }
}

/**
 * Helper function to extract mint details from state
 * @param {Array} state - The notification state
 * @returns {Object|null} - Mint details or null if not a valid mint
 */
function extractMintDetails(state) {
  try {
    // Check if state has the expected format for a mint
    if (Array.isArray(state) && state.length >= 2) {
      // Extract to and amount
      const to = state[0].value;
      const amount = parseFloat(state[1].value) / 100000000; // Convert from satoshi to whole tokens
      
      return { to, amount };
    }
    
    return null;
  } catch (error) {
    runlog.error('Error extracting mint details:', error);
    return null;
  }
}

/**
 * Helper function to extract burn details from state
 * @param {Array} state - The notification state
 * @returns {Object|null} - Burn details or null if not a valid burn
 */
function extractBurnDetails(state) {
  try {
    // Check if state has the expected format for a burn
    if (Array.isArray(state) && state.length >= 2) {
      // Extract from and amount
      const from = state[0].value;
      const amount = parseFloat(state[1].value) / 100000000; // Convert from satoshi to whole tokens
      
      return { from, amount };
    }
    
    return null;
  } catch (error) {
    runlog.error('Error extracting burn details:', error);
    return null;
  }
}
