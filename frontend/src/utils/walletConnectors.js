/**
 * Wallet connector utilities for various blockchain wallets
 * Supports Ethereum (Metamask), Neo N3 (NeoLine, O3, OneGate), and Solana wallets
 */

/**
 * Wait for Neo wallet to be fully loaded
 * @param {number} checkIntervalMs - Interval between checks in milliseconds
 * @param {number} timeoutMs - Maximum time to wait before timing out
 * @returns {Promise<void>} - Resolves when wallet is loaded or rejects on timeout
 */
const waitForNeoWallet = (checkIntervalMs = 100, timeoutMs = 10000) => {
  return new Promise((resolve, reject) => {
    if (window.neoWalletLoaded) return resolve();
    
    // Check if any wallet is already available
    if (window.NEOLineN3 || window.OneGate) return resolve();
    
    const startTime = Date.now();
    const interval = setInterval(() => {
      if (window.neoWalletLoaded || window.NEOLineN3 || window.OneGate) {
        clearInterval(interval);
        return resolve();
      }
      
      if (Date.now() - startTime > timeoutMs) {
        clearInterval(interval);
        reject(new Error('Timeout waiting for Neo wallet to load'));
      }
    }, checkIntervalMs);
  });
};

// Neo N3 wallet connectors
const neoWallets = {
  // NeoLine wallet connection
  neoLine: {
    name: 'NeoLine',
    icon: '/images/wallets/neoline.png',
    installed: () => !!window.NEOLineN3,
    connect: async () => {
      try {
        // Wait for wallet to be fully loaded
        await waitForNeoWallet();
        
        // Check if NeoLine is installed
        if (!window.NEOLineN3) {
          throw new Error('NeoLine is not installed. Please install the NeoLine extension.');
        }
        
        // Initialize NeoLine
        const neolineN3 = new window.NEOLineN3.Init();
        
        // Request connection to wallet
        const { address, publicKey } = await neolineN3.getAccount();
        
        if (!address) {
          throw new Error('Failed to get account address from NeoLine. Please ensure you have an account set up.');
        }
        
        return {
          address,
          publicKey,
          blockchain: 'neo_n3',
          provider: neolineN3,
          walletType: 'neoLine',
          sign: async (message) => {
            try {
              // NeoLine's signature method
              const signResult = await neolineN3.signMessage({ message });
              
              if (!signResult || (!signResult.data && !signResult.signature)) {
                throw new Error('NeoLine returned an invalid signature format');
              }
              
              // Return the signature and other details
              return {
                signature: signResult.data || signResult.signature,
                salt: signResult.salt,
                publicKey: signResult.publicKey || publicKey
              };
            } catch (error) {
              console.error('NeoLine signing error:', error);
              throw new Error(`Failed to sign message with NeoLine: ${error.message}`);
            }
          }
        };
      } catch (error) {
        console.error('NeoLine connection error:', error);
        throw new Error(`Failed to connect to NeoLine: ${error.message}`);
      }
    }
  },
  
  // O3 wallet connection
  o3: {
    name: 'O3',
    icon: '/images/wallets/o3.png',
    installed: () => !!window.NeoLineN3 && window.NeoLineN3.isO3,
    connect: async () => {
      try {
        // Wait for wallet to be fully loaded
        await waitForNeoWallet();
        
        // Check if O3 is installed
        if (!window.NeoLineN3 || !window.NeoLineN3.isO3) {
          throw new Error('O3 wallet is not installed.');
        }
        
        // Initialize O3
        const o3 = new window.NeoLineN3.Init();
        
        // Request connection to wallet
        const { address, publicKey } = await o3.getAccount();
        
        if (!address) {
          throw new Error('Failed to get account address from O3. Please ensure you have an account set up.');
        }
        
        return {
          address,
          publicKey,
          blockchain: 'neo_n3',
          provider: o3,
          walletType: 'o3',
          sign: async (message) => {
            try {
              // O3's signature method (uses same interface as NeoLine)
              const signResult = await o3.signMessage({ message });
              
              if (!signResult || (!signResult.data && !signResult.signature)) {
                throw new Error('O3 returned an invalid signature format');
              }
              
              // Return the signature and other details
              return {
                signature: signResult.data || signResult.signature,
                salt: signResult.salt,
                publicKey: signResult.publicKey || publicKey
              };
            } catch (error) {
              console.error('O3 signing error:', error);
              throw new Error(`Failed to sign message with O3: ${error.message}`);
            }
          }
        };
      } catch (error) {
        console.error('O3 connection error:', error);
        throw new Error(`Failed to connect to O3: ${error.message}`);
      }
    }
  },
  
  // OneGate wallet connection
  oneGate: {
    name: 'OneGate',
    icon: '/images/wallets/onegate.png',
    installed: () => !!window.OneGate,
    connect: async () => {
      try {
        // Wait for wallet to be fully loaded
        await waitForNeoWallet();
        
        // Check if OneGate is installed
        if (!window.OneGate) {
          throw new Error('OneGate wallet is not installed.');
        }
        
        // Request connection to wallet
        await window.OneGate.connect();
        const account = await window.OneGate.getAccount();
        const address = account.address;
        
        if (!address) {
          throw new Error('Failed to get account address from OneGate. Please ensure you have an account set up.');
        }
        
        return {
          address,
          publicKey: account.publicKey,
          blockchain: 'neo_n3',
          provider: window.OneGate,
          walletType: 'oneGate',
          sign: async (message) => {
            try {
              // OneGate's signature method
              const signResult = await window.OneGate.signMessage(message);
              
              if (!signResult || (!signResult.signature && !signResult.data)) {
                throw new Error('OneGate returned an invalid signature format');
              }
              
              // Return the signature and other details
              return {
                signature: signResult.signature || signResult.data,
                publicKey: account.publicKey
              };
            } catch (error) {
              console.error('OneGate signing error:', error);
              throw new Error(`Failed to sign message with OneGate: ${error.message}`);
            }
          }
        };
      } catch (error) {
        console.error('OneGate connection error:', error);
        throw new Error(`Failed to connect to OneGate: ${error.message}`);
      }
    }
  }
};

/**
 * Get all available Neo N3 wallets
 * @returns {Array} Array of available wallet options
 */
export const getAvailableNeoWallets = () => {
  return Object.values(neoWallets).filter(wallet => wallet.installed());
};

/**
 * Check if a specific Neo N3 wallet is installed
 * @param {string} walletName - Name of the wallet to check
 * @returns {boolean} True if wallet is installed
 */
export const isNeoWalletInstalled = (walletName) => {
  const wallet = neoWallets[walletName];
  return wallet ? wallet.installed() : false;
};

/**
 * Connect to a specific Neo N3 wallet by name
 * @param {string} walletName - Name of the wallet to connect to
 * @returns {Promise<Object>} Wallet connection instance
 */
export const connectNeoWallet = async (walletName) => {
  const wallet = neoWallets[walletName];
  if (!wallet) {
    throw new Error(`Wallet ${walletName} not supported`);
  }
  
  if (!wallet.installed()) {
    throw new Error(`${wallet.name} is not installed`);
  }
  
  // Store the last wallet used in sessionStorage
  sessionStorage.setItem('lastWalletUsed', walletName);
  
  return await wallet.connect();
};

/**
 * Get the last used wallet, if any
 * @returns {string|null} Last wallet name or null
 */
export const getLastUsedWallet = () => {
  return sessionStorage.getItem('lastWalletUsed');
};

/**
 * Helper function to convert Neo hex string to Base64
 * @param {string} hexString - Hex string to convert
 * @returns {string} Base64 string
 */
export const hexToBase64 = (hexString) => {
  // Remove '0x' prefix if present
  const hex = hexString.startsWith('0x') ? hexString.slice(2) : hexString;
  
  // Convert hex to byte array
  const bytes = [];
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.substr(i, 2), 16));
  }
  
  // Convert byte array to Base64 string
  return btoa(String.fromCharCode.apply(null, bytes));
};

/**
 * Format Neo signature based on wallet type
 * @param {Object} signResult - Result from wallet.sign()
 * @returns {string} - Properly formatted signature
 */
export const formatNeoSignature = (signResult) => {
  if (!signResult) {
    throw new Error('No signature result provided');
  }
  
  // Extract signature based on different wallet formats
  let signature = signResult.signature || signResult.data;
  
  if (!signature && signResult.salt) {
    // Special case for some wallet types that use a different format
    signature = signResult.data;
  }
  
  if (!signature) {
    throw new Error('Wallet returned an invalid signature format');
  }
  
  return signature;
}; 