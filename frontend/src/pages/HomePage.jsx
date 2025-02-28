import React from 'react';
import { Link } from 'react-router-dom';
import WalletConnect from '../components/WalletConnect';

const HomePage = () => {
  return (
    <div className="space-y-8">
      {/* Hero section */}
      <div className="bg-gradient-to-r from-blue-500 to-green-500 text-white rounded-lg p-8 shadow-lg">
        <h1 className="text-3xl md:text-4xl font-bold mb-4">
          R3E Function as a Service (FaaS)
        </h1>
        <p className="text-lg md:text-xl mb-6">
          Access blockchain services seamlessly with your Ethereum and Neo N3 wallets.
          Sign messages, submit meta transactions, and interact with decentralized services.
        </p>
        <div className="flex flex-wrap gap-4">
          <Link to="/services" className="btn bg-white text-blue-600 hover:bg-gray-100">
            Explore Services
          </Link>
          <Link to="/wallet" className="btn bg-blue-700 text-white hover:bg-blue-800">
            Connect Wallet
          </Link>
        </div>
      </div>
      
      {/* Features section */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="card">
          <h2 className="text-xl font-bold mb-3">Multi-Chain Support</h2>
          <p className="text-gray-600 mb-4">
            Connect your Ethereum and Neo N3 wallets to access a wide range of blockchain services.
          </p>
          <Link to="/wallet" className="text-blue-600 hover:text-blue-800">
            Connect Now →
          </Link>
        </div>
        
        <div className="card">
          <h2 className="text-xl font-bold mb-3">Meta Transactions</h2>
          <p className="text-gray-600 mb-4">
            Submit gasless transactions through our meta transaction service.
            Pay fees in tokens instead of native gas.
          </p>
          <Link to="/meta-tx" className="text-blue-600 hover:text-blue-800">
            Try Meta Tx →
          </Link>
        </div>
        
        <div className="card">
          <h2 className="text-xl font-bold mb-3">Serverless Functions</h2>
          <p className="text-gray-600 mb-4">
            Deploy and execute serverless functions that interact with blockchain networks.
            Build decentralized applications without managing infrastructure.
          </p>
          <Link to="/services" className="text-blue-600 hover:text-blue-800">
            View Services →
          </Link>
        </div>
      </div>
      
      {/* Wallet connection section */}
      <WalletConnect />
    </div>
  );
};

export default HomePage;
