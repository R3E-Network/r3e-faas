import React, { useState } from 'react';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const WalletConnect = () => {
  const {
    ethAddress,
    ethConnected,
    connectEthWallet,
    disconnectEthWallet,
    neoWallet,
    neoAddress,
    neoConnected,
    connectNeoWallet,
    disconnectNeoWallet,
  } = useWallet();
  
  const { connectWallet } = useApi();
  
  const [loading, setLoading] = useState({
    ethereum: false,
    neo: false,
  });
  
  const [error, setError] = useState(null);
  
  // Connect Ethereum wallet
  const handleConnectEthWallet = async () => {
    try {
      setLoading({ ...loading, ethereum: true });
      setError(null);
      
      // Connect wallet
      await connectEthWallet();
      
      // If connected, register with API
      if (ethAddress) {
        // Create message to sign
        const message = `Sign this message to connect your Ethereum wallet to R3E FaaS Portal\n\nWallet: ${ethAddress}\nTimestamp: ${Date.now()}`;
        
        // Sign message
        const { signature } = await signEthMessage(message);
        
        // Connect wallet with API
        await connectWallet({
          blockchain_type: 'ethereum',
          signature_curve: 'secp256k1',
          address: ethAddress,
          message,
          signature,
        });
      }
    } catch (error) {
      console.error('Failed to connect Ethereum wallet:', error);
      setError(error.message);
    } finally {
      setLoading({ ...loading, ethereum: false });
    }
  };
  
  // Connect Neo N3 wallet
  const handleConnectNeoWallet = async () => {
    try {
      setLoading({ ...loading, neo: true });
      setError(null);
      
      // Connect wallet
      await connectNeoWallet();
      
      // If connected, register with API
      if (neoAddress) {
        // Create message to sign
        const message = `Sign this message to connect your Neo N3 wallet to R3E FaaS Portal\n\nWallet: ${neoAddress}\nTimestamp: ${Date.now()}`;
        
        // Sign message
        const { signature } = await signNeoMessage(message);
        
        // Connect wallet with API
        await connectWallet({
          blockchain_type: 'neo_n3',
          signature_curve: 'secp256r1',
          address: neoAddress,
          message,
          signature,
        });
      }
    } catch (error) {
      console.error('Failed to connect Neo N3 wallet:', error);
      setError(error.message);
    } finally {
      setLoading({ ...loading, neo: false });
    }
  };
  
  // Disconnect Ethereum wallet
  const handleDisconnectEthWallet = async () => {
    try {
      await disconnectEthWallet();
    } catch (error) {
      console.error('Failed to disconnect Ethereum wallet:', error);
      setError(error.message);
    }
  };
  
  // Disconnect Neo N3 wallet
  const handleDisconnectNeoWallet = async () => {
    try {
      await disconnectNeoWallet();
    } catch (error) {
      console.error('Failed to disconnect Neo N3 wallet:', error);
      setError(error.message);
    }
  };
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Connect Wallet</h2>
      
      {error && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {error}
        </div>
      )}
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Ethereum wallet */}
        <div className="p-4 border rounded-md">
          <h3 className="text-lg font-semibold mb-2">Ethereum Wallet</h3>
          
          {ethConnected ? (
            <div>
              <p className="mb-2">
                <span className="font-medium">Address:</span> {ethAddress}
              </p>
              
              <button
                onClick={handleDisconnectEthWallet}
                className="btn btn-outline"
                disabled={loading.ethereum}
              >
                Disconnect
              </button>
            </div>
          ) : (
            <div>
              <p className="mb-4 text-sm text-gray-600">
                Connect your Ethereum wallet to use R3E FaaS services.
              </p>
              
              <button
                onClick={handleConnectEthWallet}
                className="btn btn-primary"
                disabled={loading.ethereum}
              >
                {loading.ethereum ? 'Connecting...' : 'Connect Ethereum Wallet'}
              </button>
            </div>
          )}
        </div>
        
        {/* Neo N3 wallet */}
        <div className="p-4 border rounded-md">
          <h3 className="text-lg font-semibold mb-2">Neo N3 Wallet</h3>
          
          {neoConnected ? (
            <div>
              <p className="mb-2">
                <span className="font-medium">Address:</span> {neoAddress}
              </p>
              <p className="mb-2">
                <span className="font-medium">Wallet:</span> {neoWallet?.name || 'Unknown'}
              </p>
              
              <button
                onClick={handleDisconnectNeoWallet}
                className="btn btn-outline"
                disabled={loading.neo}
              >
                Disconnect
              </button>
            </div>
          ) : (
            <div>
              <p className="mb-4 text-sm text-gray-600">
                Connect your Neo N3 wallet to use R3E FaaS services.
              </p>
              
              <button
                onClick={handleConnectNeoWallet}
                className="btn btn-secondary"
                disabled={loading.neo}
              >
                {loading.neo ? 'Connecting...' : 'Connect Neo N3 Wallet'}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default WalletConnect;
