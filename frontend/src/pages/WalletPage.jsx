import React from 'react';
import WalletConnect from '../components/WalletConnect';

const WalletPage = () => {
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-4">Wallet Connection</h1>
        <p className="text-gray-600">
          Connect your Ethereum and Neo N3 wallets to access R3E FaaS services.
          You can sign messages, submit meta transactions, and interact with decentralized services.
        </p>
      </div>
      
      <WalletConnect />
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">Supported Wallets</h2>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Ethereum wallets */}
          <div>
            <h3 className="text-lg font-semibold mb-3">Ethereum Wallets</h3>
            <ul className="list-disc list-inside space-y-2 text-gray-700">
              <li>MetaMask</li>
              <li>WalletConnect</li>
              <li>Coinbase Wallet</li>
              <li>Trust Wallet</li>
              <li>Rainbow</li>
            </ul>
          </div>
          
          {/* Neo N3 wallets */}
          <div>
            <h3 className="text-lg font-semibold mb-3">Neo N3 Wallets</h3>
            <ul className="list-disc list-inside space-y-2 text-gray-700">
              <li>NeoLine</li>
              <li>O3</li>
              <li>OneGate</li>
              <li>WalletConnect</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
};

export default WalletPage;
