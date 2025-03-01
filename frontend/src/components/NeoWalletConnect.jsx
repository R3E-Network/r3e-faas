import React, { useState, useEffect, useCallback } from 'react';
import { 
  getAvailableNeoWallets, 
  connectNeoWallet, 
  formatNeoSignature,
  getLastUsedWallet,
  isNeoWalletInstalled
} from '../utils/walletConnectors';
import { requestChallenge, verifyChallenge, logout } from '../services/authService';

// Styling constants
const BUTTON_STYLE = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  padding: '12px 16px',
  borderRadius: '8px',
  border: '1px solid #e0e0e0',
  background: '#ffffff',
  cursor: 'pointer',
  transition: 'all 0.2s ease',
  margin: '8px 0',
  width: '100%',
};

const BUTTON_HOVER_STYLE = {
  background: '#f5f5f5',
  boxShadow: '0 2px 5px rgba(0, 0, 0, 0.1)',
};

const BUTTON_DISABLED_STYLE = {
  opacity: 0.6,
  cursor: 'not-allowed',
};

const ICON_STYLE = {
  width: '24px',
  height: '24px',
  marginRight: '12px',
};

const STATUS_INDICATOR_STYLE = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  padding: '8px 16px',
  borderRadius: '4px',
  margin: '8px 0',
  fontSize: '14px',
  width: '100%',
};

/**
 * Connection status values
 */
const CONNECTION_STATUS = {
  IDLE: 'idle',
  CONNECTING: 'connecting',
  CONNECTED: 'connected',
  SIGNING: 'signing',
  AUTHENTICATING: 'authenticating',
  SUCCESS: 'success',
  ERROR: 'error',
};

/**
 * Neo Wallet Connect Component
 * 
 * A component that allows users to connect to Neo N3 wallets
 * like NeoLine, O3, and OneGate.
 * 
 * @param {Object} props
 * @param {Function} props.onConnect - Callback function called when wallet connects successfully
 * @param {Function} props.onAuthenticate - Callback function called when user authenticates successfully
 * @param {Function} props.onError - Callback function called when an error occurs
 * @param {boolean} props.autoConnect - Try to auto-connect to last used wallet
 * @param {boolean} props.pollForWallets - Whether to poll for newly installed wallets
 * @param {number} props.pollingInterval - Polling interval in ms (default: 3000)
 */
const NeoWalletConnect = ({ 
  onConnect, 
  onAuthenticate, 
  onError,
  autoConnect = true,
  pollForWallets = true,
  pollingInterval = 3000
}) => {
  const [availableWallets, setAvailableWallets] = useState([]);
  const [status, setStatus] = useState(CONNECTION_STATUS.IDLE);
  const [wallet, setWallet] = useState(null);
  const [buttonHover, setButtonHover] = useState(null);
  const [error, setError] = useState('');
  const [lastAttemptedWallet, setLastAttemptedWallet] = useState(null);
  const [challenge, setChallenge] = useState(null);
  const [retryCount, setRetryCount] = useState(0);

  // Check for available wallets
  const checkWallets = useCallback(() => {
    const wallets = getAvailableNeoWallets();
    
    // Only update if the list has changed
    if (JSON.stringify(wallets.map(w => w.name)) !== 
        JSON.stringify(availableWallets.map(w => w.name))) {
      setAvailableWallets(wallets);
    }
    
    return wallets;
  }, [availableWallets]);

  // Initialize component
  useEffect(() => {
    // Initial wallet check
    const initializeWallets = async () => {
      // Small delay to ensure wallet extensions have loaded
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const wallets = checkWallets();
      
      // Auto-connect to last used wallet if enabled
      if (autoConnect && wallets.length > 0) {
        const lastWallet = getLastUsedWallet();
        if (lastWallet && isNeoWalletInstalled(lastWallet)) {
          console.log(`Auto-connecting to last used wallet: ${lastWallet}`);
          handleConnect(lastWallet, true);
        }
      }
    };

    initializeWallets();
    
    // Set up polling for wallet detection if enabled
    let walletPollInterval;
    if (pollForWallets) {
      walletPollInterval = setInterval(() => {
        checkWallets();
      }, pollingInterval);
    }

    return () => {
      if (walletPollInterval) {
        clearInterval(walletPollInterval);
      }
    };
  }, [autoConnect, checkWallets, pollForWallets, pollingInterval]);

  // Handle wallet connection
  const handleConnect = async (walletName, isAutoConnect = false) => {
    try {
      // Don't reconnect if already connected
      if (status === CONNECTION_STATUS.CONNECTED || 
          status === CONNECTION_STATUS.SIGNING || 
          status === CONNECTION_STATUS.AUTHENTICATING ||
          status === CONNECTION_STATUS.SUCCESS) {
        return;
      }
      
      setError('');
      setStatus(CONNECTION_STATUS.CONNECTING);
      setLastAttemptedWallet(walletName);
      
      // Connect to wallet
      const walletInstance = await connectNeoWallet(walletName);
      setWallet(walletInstance);
      setStatus(CONNECTION_STATUS.CONNECTED);
      
      // Call onConnect callback
      if (onConnect) {
        onConnect(walletInstance);
      }
      
      // Start authentication process (unless this is from auto-connect and we're already authenticated)
      if (!isAutoConnect || !sessionStorage.getItem('authToken')) {
        await authenticateWallet(walletInstance);
      }
    } catch (error) {
      console.error('Wallet connection error:', error);
      setStatus(CONNECTION_STATUS.ERROR);
      setError(error.message || 'Failed to connect wallet');
      
      if (onError) {
        onError(error.message);
      }
    }
  };

  // Authenticate wallet with backend
  const authenticateWallet = async (walletInstance) => {
    try {
      setStatus(CONNECTION_STATUS.AUTHENTICATING);
      
      // Request challenge from backend
      const challengeResponse = await requestChallenge(
        walletInstance.address,
        walletInstance.blockchain
      );
      
      setChallenge(challengeResponse);
      setStatus(CONNECTION_STATUS.SIGNING);
      
      // Sign the challenge message
      const signResult = await walletInstance.sign(challengeResponse.challenge);
      
      // Format the signature appropriately
      const signature = formatNeoSignature(signResult);
      
      if (!signature) {
        throw new Error('Failed to get a valid signature from wallet');
      }
      
      // Update status for verification phase
      setStatus(CONNECTION_STATUS.AUTHENTICATING);
      
      // Verify the signed challenge
      const authResult = await verifyChallenge(
        challengeResponse.challenge_id,
        walletInstance.address,
        walletInstance.blockchain,
        signature,
        "secp256r1" // Neo uses secp256r1
      );
      
      // Success!
      setStatus(CONNECTION_STATUS.SUCCESS);
      setError('');
      setRetryCount(0);
      
      // Call onAuthenticate callback
      if (onAuthenticate) {
        onAuthenticate(authResult);
      }
    } catch (error) {
      console.error('Authentication error:', error);
      setStatus(CONNECTION_STATUS.ERROR);
      
      // Set appropriate error message
      if (error.message.includes('sign') || error.message.includes('signature')) {
        setError('Failed to sign message with wallet. Please try again.');
      } else if (error.message.includes('challenge')) {
        setError('Failed to get authentication challenge from server.');
      } else if (error.message.includes('verify')) {
        setError('Failed to verify signature with server.');
      } else {
        setError(error.message || 'Authentication failed');
      }
      
      if (onError) {
        onError(error.message);
      }
    }
  };

  // Handle wallet disconnection
  const handleDisconnect = () => {
    setWallet(null);
    setStatus(CONNECTION_STATUS.IDLE);
    setError('');
    
    // Clear authentication data
    logout();
    
    // Clear the challenge
    setChallenge(null);
  };

  // Handle retry attempt
  const handleRetry = () => {
    setRetryCount(retryCount + 1);
    setError('');
    
    if (wallet) {
      // If we already have a wallet instance, just retry authentication
      authenticateWallet(wallet);
    } else if (lastAttemptedWallet) {
      // Otherwise retry connection
      handleConnect(lastAttemptedWallet);
    }
  };

  // Render status indicator
  const renderStatusIndicator = () => {
    switch (status) {
      case CONNECTION_STATUS.CONNECTING:
        return (
          <div style={{...STATUS_INDICATOR_STYLE, backgroundColor: '#e3f2fd', color: '#0d47a1'}}>
            Connecting to wallet...
          </div>
        );
      case CONNECTION_STATUS.SIGNING:
        return (
          <div style={{...STATUS_INDICATOR_STYLE, backgroundColor: '#fff8e1', color: '#ff6f00'}}>
            Please sign the message in your wallet to authenticate
          </div>
        );
      case CONNECTION_STATUS.AUTHENTICATING:
        return (
          <div style={{...STATUS_INDICATOR_STYLE, backgroundColor: '#e8f5e9', color: '#1b5e20'}}>
            Authenticating with server...
          </div>
        );
      case CONNECTION_STATUS.SUCCESS:
        return (
          <div style={{...STATUS_INDICATOR_STYLE, backgroundColor: '#e8f5e9', color: '#1b5e20'}}>
            Successfully authenticated!
          </div>
        );
      case CONNECTION_STATUS.ERROR:
        return (
          <div style={{...STATUS_INDICATOR_STYLE, backgroundColor: '#ffebee', color: '#c62828'}}>
            Error: {error}
            <button 
              onClick={handleRetry} 
              style={{
                marginLeft: '10px',
                padding: '4px 8px',
                borderRadius: '4px',
                border: '1px solid #c62828',
                background: 'none',
                color: '#c62828',
                cursor: 'pointer',
              }}
            >
              Retry
            </button>
          </div>
        );
      default:
        return null;
    }
  };

  // If no wallets are available
  if (availableWallets.length === 0) {
    return (
      <div>
        <h3>No Neo N3 wallets found</h3>
        <p>Please install one of the following wallets:</p>
        <ul>
          <li>
            <a 
              href="https://neoline.io/" 
              target="_blank" 
              rel="noopener noreferrer"
              style={{ color: '#336699', textDecoration: 'none', fontWeight: 'bold' }}
            >
              NeoLine Wallet
            </a>
            <span> - The most popular Neo N3 wallet extension</span>
          </li>
          <li>
            <a 
              href="https://o3.network/" 
              target="_blank" 
              rel="noopener noreferrer"
              style={{ color: '#336699', textDecoration: 'none', fontWeight: 'bold' }}
            >
              O3 Wallet
            </a>
            <span> - Full featured Neo ecosystem wallet</span>
          </li>
          <li>
            <a 
              href="https://onegate.space/" 
              target="_blank" 
              rel="noopener noreferrer"
              style={{ color: '#336699', textDecoration: 'none', fontWeight: 'bold' }}
            >
              OneGate Wallet
            </a>
            <span> - Web3 gateway for Neo N3</span>
          </li>
        </ul>
        
        <button 
          onClick={() => checkWallets()}
          style={BUTTON_STYLE}
        >
          Check for wallets
        </button>
      </div>
    );
  }

  // If wallet is connected
  if (wallet && [
    CONNECTION_STATUS.CONNECTED, 
    CONNECTION_STATUS.SIGNING,
    CONNECTION_STATUS.AUTHENTICATING,
    CONNECTION_STATUS.SUCCESS
  ].includes(status)) {
    return (
      <div>
        <h3>Connected to {wallet.walletType}</h3>
        <p style={{ 
          fontFamily: 'monospace', 
          padding: '8px', 
          background: '#f5f5f5', 
          borderRadius: '4px',
          wordBreak: 'break-all'
        }}>
          Address: {wallet.address}
        </p>
        
        {renderStatusIndicator()}
        
        <button
          onClick={handleDisconnect}
          style={{
            ...BUTTON_STYLE,
            marginTop: '16px',
            border: '1px solid #e57373',
            color: '#c62828'
          }}
          disabled={status === CONNECTION_STATUS.SIGNING || status === CONNECTION_STATUS.AUTHENTICATING}
        >
          Disconnect Wallet
        </button>
      </div>
    );
  }

  // Render wallet connect options
  return (
    <div>
      <h3>Connect your Neo N3 wallet</h3>
      
      {renderStatusIndicator()}
      
      {availableWallets.map((walletOption) => (
        <button
          key={walletOption.name}
          onClick={() => handleConnect(walletOption.name.toLowerCase())}
          disabled={status === CONNECTION_STATUS.CONNECTING}
          style={{
            ...BUTTON_STYLE,
            ...(buttonHover === walletOption.name ? BUTTON_HOVER_STYLE : {}),
            ...(status === CONNECTION_STATUS.CONNECTING ? BUTTON_DISABLED_STYLE : {}),
          }}
          onMouseEnter={() => setButtonHover(walletOption.name)}
          onMouseLeave={() => setButtonHover(null)}
        >
          {walletOption.icon && (
            <img
              src={walletOption.icon}
              alt={`${walletOption.name} icon`}
              style={ICON_STYLE}
            />
          )}
          Connect with {walletOption.name}
        </button>
      ))}
      
      {error && (
        <div style={{marginTop: '16px', color: '#c62828', fontSize: '14px'}}>
          {error}
        </div>
      )}
    </div>
  );
};

export default NeoWalletConnect; 