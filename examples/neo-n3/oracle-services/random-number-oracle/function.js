/**
 * Neo N3 Random Number Oracle Example
 * 
 * This function demonstrates how to implement a random number oracle service
 * that generates cryptographically secure random numbers and provides them
 * to Neo N3 smart contracts in a verifiable way.
 * 
 * @param {Object} event - The event object (schedule or request)
 * @param {Object} context - The execution context
 * @returns {Object} - Random number data and verification proof
 */

// Import the Neo and Oracle modules from the r3e runtime
import { neo } from 'r3e';
import { oracle } from 'r3e';
import { runlog } from 'r3e';
import { crypto } from 'r3e';

// Define supported random number types
const RANDOM_TYPES = {
  INTEGER: 'integer',
  BYTES: 'bytes',
  UUID: 'uuid',
  BOOLEAN: 'boolean',
  FLOAT: 'float'
};

// Define entropy sources
const ENTROPY_SOURCES = {
  HARDWARE: 'hardware',
  CRYPTO: 'crypto',
  BLOCKCHAIN: 'blockchain',
  EXTERNAL: 'external',
  TIME: 'time'
};

// Define verification methods
const VERIFICATION_METHODS = {
  VRF: 'vrf',
  COMMIT_REVEAL: 'commit-reveal',
  MULTI_PARTY: 'multi-party',
  SIGNATURE: 'signature'
};

/**
 * Main handler function for the random number oracle
 */
export async function handler(event, context) {
  try {
    runlog.info('Random Number Oracle function triggered');
    
    // Determine if this is a scheduled generation or a direct request
    const isScheduled = event.type === 'schedule';
    
    // Parse request parameters
    const requestParams = isScheduled 
      ? { type: RANDOM_TYPES.INTEGER, min: 1, max: 100 } 
      : parseRequestParams(event.data);
    
    // Validate request parameters
    validateRequestParams(requestParams);
    
    // Generate random number with proof
    const result = await generateRandomWithProof(requestParams, context);
    
    // If this is a scheduled generation, store the result
    if (isScheduled) {
      await storeRandomResult(result, context);
    }
    
    // Return the result
    return {
      status: 'success',
      timestamp: Date.now(),
      data: result
    };
    
  } catch (error) {
    // Log any errors
    runlog.error('Error in random number oracle:', error);
    
    // Return error information
    return {
      status: 'error',
      message: `Error in random number oracle: ${error.message}`,
      error: error.stack
    };
  }
}

/**
 * Parse request parameters from the event data
 * @param {Object} data - The event data
 * @returns {Object} - Parsed request parameters
 */
function parseRequestParams(data) {
  // Default parameters
  const defaultParams = {
    type: RANDOM_TYPES.INTEGER,
    min: 1,
    max: 100,
    count: 1,
    bytes: 32,
    verification: VERIFICATION_METHODS.SIGNATURE,
    entropy: [ENTROPY_SOURCES.CRYPTO, ENTROPY_SOURCES.BLOCKCHAIN]
  };
  
  // Merge with provided parameters
  return { ...defaultParams, ...data };
}

/**
 * Validate request parameters
 * @param {Object} params - The request parameters
 * @throws {Error} - If parameters are invalid
 */
function validateRequestParams(params) {
  // Check if random type is supported
  if (!Object.values(RANDOM_TYPES).includes(params.type)) {
    throw new Error(`Unsupported random type: ${params.type}`);
  }
  
  // Validate type-specific parameters
  switch (params.type) {
    case RANDOM_TYPES.INTEGER:
      if (typeof params.min !== 'number' || typeof params.max !== 'number') {
        throw new Error('Min and max must be numbers for integer random type');
      }
      if (params.min >= params.max) {
        throw new Error('Min must be less than max for integer random type');
      }
      break;
      
    case RANDOM_TYPES.BYTES:
      if (typeof params.bytes !== 'number' || params.bytes <= 0 || params.bytes > 1024) {
        throw new Error('Bytes must be a number between 1 and 1024');
      }
      break;
      
    case RANDOM_TYPES.FLOAT:
      if (typeof params.min !== 'number' || typeof params.max !== 'number') {
        throw new Error('Min and max must be numbers for float random type');
      }
      if (params.min >= params.max) {
        throw new Error('Min must be less than max for float random type');
      }
      break;
  }
  
  // Validate count
  if (typeof params.count !== 'number' || params.count <= 0 || params.count > 100) {
    throw new Error('Count must be a number between 1 and 100');
  }
  
  // Validate verification method
  if (!Object.values(VERIFICATION_METHODS).includes(params.verification)) {
    throw new Error(`Unsupported verification method: ${params.verification}`);
  }
  
  // Validate entropy sources
  if (!Array.isArray(params.entropy) || params.entropy.length === 0) {
    throw new Error('At least one entropy source must be specified');
  }
  
  for (const source of params.entropy) {
    if (!Object.values(ENTROPY_SOURCES).includes(source)) {
      throw new Error(`Unsupported entropy source: ${source}`);
    }
  }
}

/**
 * Generate random number with cryptographic proof
 * @param {Object} params - The request parameters
 * @param {Object} context - The execution context
 * @returns {Object} - Random number with proof
 */
async function generateRandomWithProof(params, context) {
  // Collect entropy from specified sources
  const entropy = await collectEntropy(params.entropy);
  
  // Generate random number based on type
  const randomValue = await generateRandom(params, entropy);
  
  // Generate proof based on verification method
  const proof = await generateProof(randomValue, params, entropy);
  
  // Return random number with proof
  return {
    request_id: crypto.randomUUID(),
    timestamp: Date.now(),
    type: params.type,
    value: randomValue,
    proof: proof,
    entropy_sources: params.entropy,
    verification_method: params.verification
  };
}

/**
 * Collect entropy from specified sources
 * @param {Array} sources - The entropy sources to use
 * @returns {Object} - Collected entropy
 */
async function collectEntropy(sources) {
  const entropy = {};
  
  // Collect entropy from each source
  for (const source of sources) {
    switch (source) {
      case ENTROPY_SOURCES.HARDWARE:
        entropy.hardware = await collectHardwareEntropy();
        break;
        
      case ENTROPY_SOURCES.CRYPTO:
        entropy.crypto = await collectCryptoEntropy();
        break;
        
      case ENTROPY_SOURCES.BLOCKCHAIN:
        entropy.blockchain = await collectBlockchainEntropy();
        break;
        
      case ENTROPY_SOURCES.EXTERNAL:
        entropy.external = await collectExternalEntropy();
        break;
        
      case ENTROPY_SOURCES.TIME:
        entropy.time = collectTimeEntropy();
        break;
    }
  }
  
  return entropy;
}

/**
 * Collect entropy from hardware random number generator
 * @returns {string} - Hardware entropy
 */
async function collectHardwareEntropy() {
  try {
    // Use hardware RNG if available
    const hardwareRandom = await crypto.getRandomValues(new Uint8Array(32));
    return Buffer.from(hardwareRandom).toString('hex');
  } catch (error) {
    runlog.warn('Hardware entropy not available:', error);
    // Fall back to crypto entropy
    return await collectCryptoEntropy();
  }
}

/**
 * Collect entropy from cryptographic PRNG
 * @returns {string} - Crypto entropy
 */
async function collectCryptoEntropy() {
  // Generate 32 bytes of cryptographically secure random data
  const cryptoRandom = await crypto.randomBytes(32);
  return cryptoRandom.toString('hex');
}

/**
 * Collect entropy from blockchain data
 * @returns {Object} - Blockchain entropy
 */
async function collectBlockchainEntropy() {
  try {
    // Get latest block from Neo N3 blockchain
    const latestBlock = await neo.getLatestBlock();
    
    // Get some recent transactions
    const transactions = await neo.getRecentTransactions(5);
    
    // Combine block and transaction data for entropy
    return {
      block_hash: latestBlock.hash,
      block_index: latestBlock.index,
      block_time: latestBlock.time,
      transaction_hashes: transactions.map(tx => tx.hash)
    };
  } catch (error) {
    runlog.warn('Blockchain entropy not available:', error);
    // Return empty object if blockchain data is not available
    return {};
  }
}

/**
 * Collect entropy from external sources
 * @returns {Object} - External entropy
 */
async function collectExternalEntropy() {
  try {
    // Try to get entropy from NIST Randomness Beacon
    const nistResponse = await oracle.fetch('https://beacon.nist.gov/beacon/2.0/pulse/last');
    const nistData = await nistResponse.json();
    
    // Try to get entropy from another external source
    const drandResponse = await oracle.fetch('https://drand.cloudflare.com/public/latest');
    const drandData = await drandResponse.json();
    
    return {
      nist: nistData.pulse.outputValue,
      drand: drandData.randomness
    };
  } catch (error) {
    runlog.warn('External entropy not available:', error);
    // Return empty object if external entropy is not available
    return {};
  }
}

/**
 * Collect entropy from time-based sources
 * @returns {Object} - Time entropy
 */
function collectTimeEntropy() {
  // Get current time in various formats
  const now = new Date();
  
  return {
    timestamp: now.getTime(),
    iso: now.toISOString(),
    high_resolution: performance.now()
  };
}

/**
 * Generate random number based on type and entropy
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {any} - Random value
 */
async function generateRandom(params, entropy) {
  // Combine all entropy sources into a single seed
  const seed = await createSeedFromEntropy(entropy);
  
  // Generate random value based on type
  switch (params.type) {
    case RANDOM_TYPES.INTEGER:
      return generateRandomInteger(params.min, params.max, seed, params.count);
      
    case RANDOM_TYPES.BYTES:
      return generateRandomBytes(params.bytes, seed);
      
    case RANDOM_TYPES.UUID:
      return generateRandomUUID(seed, params.count);
      
    case RANDOM_TYPES.BOOLEAN:
      return generateRandomBoolean(seed, params.count);
      
    case RANDOM_TYPES.FLOAT:
      return generateRandomFloat(params.min, params.max, seed, params.count);
      
    default:
      throw new Error(`Unsupported random type: ${params.type}`);
  }
}

/**
 * Create a seed from collected entropy
 * @param {Object} entropy - The collected entropy
 * @returns {string} - Seed for random generation
 */
async function createSeedFromEntropy(entropy) {
  // Serialize entropy object to string
  const entropyString = JSON.stringify(entropy);
  
  // Hash the entropy string to create a seed
  const seed = await crypto.sha256(entropyString);
  
  return seed;
}

/**
 * Generate random integer in range [min, max]
 * @param {number} min - Minimum value (inclusive)
 * @param {number} max - Maximum value (inclusive)
 * @param {string} seed - Seed for random generation
 * @param {number} count - Number of random values to generate
 * @returns {number|Array} - Random integer or array of integers
 */
async function generateRandomInteger(min, max, seed, count) {
  // Generate random bytes
  const randomBytes = await crypto.deriveRandomBytes(seed, count * 4);
  
  // Convert to integers in the specified range
  const result = [];
  for (let i = 0; i < count; i++) {
    const value = randomBytes.readUInt32LE(i * 4);
    const scaled = min + (value % (max - min + 1));
    result.push(scaled);
  }
  
  // Return single value or array based on count
  return count === 1 ? result[0] : result;
}

/**
 * Generate random bytes
 * @param {number} length - Number of bytes to generate
 * @param {string} seed - Seed for random generation
 * @returns {string} - Hex-encoded random bytes
 */
async function generateRandomBytes(length, seed) {
  // Generate random bytes
  const randomBytes = await crypto.deriveRandomBytes(seed, length);
  
  // Return hex-encoded bytes
  return randomBytes.toString('hex');
}

/**
 * Generate random UUID
 * @param {string} seed - Seed for random generation
 * @param {number} count - Number of UUIDs to generate
 * @returns {string|Array} - Random UUID or array of UUIDs
 */
async function generateRandomUUID(seed, count) {
  // Generate random bytes for UUIDs
  const randomBytes = await crypto.deriveRandomBytes(seed, count * 16);
  
  // Convert to UUIDs
  const result = [];
  for (let i = 0; i < count; i++) {
    const uuidBytes = randomBytes.slice(i * 16, (i + 1) * 16);
    
    // Set version (4) and variant bits
    uuidBytes[6] = (uuidBytes[6] & 0x0f) | 0x40;
    uuidBytes[8] = (uuidBytes[8] & 0x3f) | 0x80;
    
    // Format as UUID string
    const uuid = [
      uuidBytes.slice(0, 4).toString('hex'),
      uuidBytes.slice(4, 6).toString('hex'),
      uuidBytes.slice(6, 8).toString('hex'),
      uuidBytes.slice(8, 10).toString('hex'),
      uuidBytes.slice(10, 16).toString('hex')
    ].join('-');
    
    result.push(uuid);
  }
  
  // Return single value or array based on count
  return count === 1 ? result[0] : result;
}

/**
 * Generate random boolean
 * @param {string} seed - Seed for random generation
 * @param {number} count - Number of booleans to generate
 * @returns {boolean|Array} - Random boolean or array of booleans
 */
async function generateRandomBoolean(seed, count) {
  // Generate random bytes
  const randomBytes = await crypto.deriveRandomBytes(seed, count);
  
  // Convert to booleans
  const result = [];
  for (let i = 0; i < count; i++) {
    result.push((randomBytes[i] & 1) === 1);
  }
  
  // Return single value or array based on count
  return count === 1 ? result[0] : result;
}

/**
 * Generate random float in range [min, max)
 * @param {number} min - Minimum value (inclusive)
 * @param {number} max - Maximum value (exclusive)
 * @param {string} seed - Seed for random generation
 * @param {number} count - Number of random values to generate
 * @returns {number|Array} - Random float or array of floats
 */
async function generateRandomFloat(min, max, seed, count) {
  // Generate random bytes
  const randomBytes = await crypto.deriveRandomBytes(seed, count * 4);
  
  // Convert to floats in the specified range
  const result = [];
  for (let i = 0; i < count; i++) {
    const value = randomBytes.readUInt32LE(i * 4);
    const normalized = value / 0xFFFFFFFF; // Convert to [0, 1)
    const scaled = min + normalized * (max - min);
    result.push(scaled);
  }
  
  // Return single value or array based on count
  return count === 1 ? result[0] : result;
}

/**
 * Generate proof for the random value
 * @param {any} value - The random value
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {Object} - Proof of randomness
 */
async function generateProof(value, params, entropy) {
  // Generate proof based on verification method
  switch (params.verification) {
    case VERIFICATION_METHODS.VRF:
      return await generateVRFProof(value, params, entropy);
      
    case VERIFICATION_METHODS.COMMIT_REVEAL:
      return await generateCommitRevealProof(value, params, entropy);
      
    case VERIFICATION_METHODS.MULTI_PARTY:
      return await generateMultiPartyProof(value, params, entropy);
      
    case VERIFICATION_METHODS.SIGNATURE:
      return await generateSignatureProof(value, params, entropy);
      
    default:
      throw new Error(`Unsupported verification method: ${params.verification}`);
  }
}

/**
 * Generate VRF proof
 * @param {any} value - The random value
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {Object} - VRF proof
 */
async function generateVRFProof(value, params, entropy) {
  try {
    // Create input for VRF
    const input = JSON.stringify({
      type: params.type,
      params: params,
      entropy_sources: Object.keys(entropy)
    });
    
    // Generate VRF proof
    const vrfResult = await crypto.vrf.prove('oracle-random-number', input);
    
    return {
      type: 'vrf',
      input: input,
      proof: vrfResult.proof,
      public_key: vrfResult.publicKey
    };
  } catch (error) {
    runlog.warn('VRF proof generation failed:', error);
    // Fall back to signature proof
    return await generateSignatureProof(value, params, entropy);
  }
}

/**
 * Generate commit-reveal proof
 * @param {any} value - The random value
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {Object} - Commit-reveal proof
 */
async function generateCommitRevealProof(value, params, entropy) {
  // Create commit data
  const commitData = {
    value: value,
    entropy: entropy,
    timestamp: Date.now()
  };
  
  // Hash the commit data
  const commitHash = await crypto.sha256(JSON.stringify(commitData));
  
  // Create nonce
  const nonce = await crypto.randomBytes(32);
  const nonceHex = nonce.toString('hex');
  
  // Create salted commit hash
  const saltedCommit = await crypto.sha256(commitHash + nonceHex);
  
  return {
    type: 'commit-reveal',
    commit: saltedCommit,
    reveal: {
      value: value,
      nonce: nonceHex,
      timestamp: commitData.timestamp
    }
  };
}

/**
 * Generate multi-party computation proof
 * @param {any} value - The random value
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {Object} - Multi-party proof
 */
async function generateMultiPartyProof(value, params, entropy) {
  // In a real implementation, this would involve multiple parties
  // For this example, we simulate it with multiple entropy sources
  
  // Create contributions from different entropy sources
  const contributions = {};
  
  for (const source in entropy) {
    if (typeof entropy[source] === 'string') {
      contributions[source] = entropy[source];
    } else if (typeof entropy[source] === 'object') {
      contributions[source] = await crypto.sha256(JSON.stringify(entropy[source]));
    }
  }
  
  // Combine contributions
  const combinedContribution = Object.values(contributions).join('');
  const finalHash = await crypto.sha256(combinedContribution);
  
  return {
    type: 'multi-party',
    contributions: contributions,
    combined_hash: finalHash
  };
}

/**
 * Generate signature proof
 * @param {any} value - The random value
 * @param {Object} params - The request parameters
 * @param {Object} entropy - The collected entropy
 * @returns {Object} - Signature proof
 */
async function generateSignatureProof(value, params, entropy) {
  // Create data to sign
  const signData = {
    value: value,
    type: params.type,
    timestamp: Date.now(),
    entropy_sources: Object.keys(entropy)
  };
  
  // Convert to string
  const dataString = JSON.stringify(signData);
  
  // Sign the data
  const signature = await crypto.sign(dataString, 'oracle-random-number');
  
  // Hash the entropy for verification
  const entropyHash = await crypto.sha256(JSON.stringify(entropy));
  
  return {
    type: 'signature',
    data: signData,
    entropy_hash: entropyHash,
    signature: signature
  };
}

/**
 * Store random result in the persistent storage
 * @param {Object} result - The random result to store
 * @param {Object} context - The execution context
 */
async function storeRandomResult(result, context) {
  try {
    // Store the result
    const key = `random:${result.request_id}`;
    await context.store.set(key, JSON.stringify(result));
    
    // Update the random history index
    const historyKey = 'random:history';
    const historyJson = await context.store.get(historyKey) || '[]';
    const history = JSON.parse(historyJson);
    
    // Add the new request ID to the history
    history.push(result.request_id);
    
    // Keep only the last 100 entries
    if (history.length > 100) {
      history.shift();
    }
    
    // Save the updated history
    await context.store.set(historyKey, JSON.stringify(history));
    
    runlog.info('Random result stored successfully');
  } catch (error) {
    runlog.error('Error storing random result:', error);
    throw error;
  }
}
