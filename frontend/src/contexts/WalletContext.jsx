import React, { createContext, useContext, useState, useEffect } from 'react';
import { useWeb3Modal } from '@web3modal/react';
import { ethers } from 'ethers';
import { 
  NeoLineN3, 
  O3, 
  OneGate,
  WalletConnect
} from '@rentfuse-labs/neo-wallet-adapter-wallets';
import { 
  WalletProvider as NeoWalletProvider,
  useWallet as useNeoWallet
} from '@rentfuse-labs/neo-wallet-adapter-react';

// Create context
const WalletContext = createContext(null);

// Neo N3 wallet adapters
const wallets = [
  new NeoLineN3(),
  new O3(),
  new OneGate(),
  new WalletConnect({
    options: {
      projectId: 'YOUR_PROJECT_ID', // Replace with your WalletConnect project ID
      relayUrl: 'wss://relay.walletconnect.com',
      metadata: {
        name: 'R3E FaaS Portal',
        description: 'R3E Function as a Service (FaaS) Portal for blockchain services',
        url: window.location.origin,
        icons: [`${window.location.origin}/logo.png`]
      }
    }
  })
];

// Provider component
export const WalletProvider = ({ children }) => {
  // Ethereum wallet state
  const [ethAddress, setEthAddress] = useState(null);
  const [ethProvider, setEthProvider] = useState(null);
  const [ethSigner, setEthSigner] = useState(null);
  const [ethChainId, setEthChainId] = useState(null);
  const [ethConnected, setEthConnected] = useState(false);
  
  // Web3Modal hook
  const { isOpen, open, close } = useWeb3Modal();
  
  // Connect Ethereum wallet
  const connectEthWallet = async () => {
    try {
      // Open Web3Modal
      await open();
      
      // Get provider
      if (window.ethereum) {
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        setEthProvider(provider);
        
        // Get signer
        const signer = provider.getSigner();
        setEthSigner(signer);
        
        // Get address
        const address = await signer.getAddress();
        setEthAddress(address);
        
        // Get chain ID
        const network = await provider.getNetwork();
        setEthChainId(network.chainId);
        
        // Set connected
        setEthConnected(true);
      }
    } catch (error) {
      console.error('Failed to connect Ethereum wallet:', error);
    }
  };
  
  // Disconnect Ethereum wallet
  const disconnectEthWallet = async () => {
    try {
      // Reset state
      setEthAddress(null);
      setEthProvider(null);
      setEthSigner(null);
      setEthChainId(null);
      setEthConnected(false);
    } catch (error) {
      console.error('Failed to disconnect Ethereum wallet:', error);
    }
  };
  
  // Sign message with Ethereum wallet
  const signEthMessage = async (message) => {
    try {
      if (!ethSigner) {
        throw new Error('Ethereum wallet not connected');
      }
      
      // Sign message
      const signature = await ethSigner.signMessage(message);
      
      return {
        message,
        signature,
        address: ethAddress,
      };
    } catch (error) {
      console.error('Failed to sign message with Ethereum wallet:', error);
      throw error;
    }
  };
  
  // Sign typed data with Ethereum wallet (EIP-712)
  const signEthTypedData = async (domain, types, value) => {
    try {
      if (!ethSigner) {
        throw new Error('Ethereum wallet not connected');
      }
      
      // Sign typed data
      const signature = await ethSigner._signTypedData(domain, types, value);
      
      return {
        domain,
        types,
        value,
        signature,
        address: ethAddress,
      };
    } catch (error) {
      console.error('Failed to sign typed data with Ethereum wallet:', error);
      throw error;
    }
  };
  
  // Neo N3 wallet hooks
  const {
    wallet: neoWallet,
    address: neoAddress,
    connected: neoConnected,
    connect: connectNeoWallet,
    disconnect: disconnectNeoWallet,
    signMessage: signNeoMessage,
  } = useNeoWallet();
  
  // Value to provide
  const value = {
    // Ethereum wallet
    ethAddress,
    ethProvider,
    ethSigner,
    ethChainId,
    ethConnected,
    connectEthWallet,
    disconnectEthWallet,
    signEthMessage,
    signEthTypedData,
    
    // Neo N3 wallet
    neoWallet,
    neoAddress,
    neoConnected,
    connectNeoWallet,
    disconnectNeoWallet,
    signNeoMessage,
  };
  
  return (
    <NeoWalletProvider wallets={wallets} autoConnect={false}>
      <WalletContext.Provider value={value}>
        {children}
      </WalletContext.Provider>
    </NeoWalletProvider>
  );
};

// Hook to use wallet context
export const useWallet = () => {
  const context = useContext(WalletContext);
  
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  
  return context;
};
