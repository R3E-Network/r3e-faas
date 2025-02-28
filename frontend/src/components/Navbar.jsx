import React from 'react';
import { Link } from 'react-router-dom';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const Navbar = () => {
  const { ethAddress, ethConnected, neoAddress, neoConnected } = useWallet();
  const { logout } = useApi();
  
  // Format address for display
  const formatAddress = (address) => {
    if (!address) return '';
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };
  
  return (
    <nav className="bg-white shadow-md">
      <div className="container mx-auto px-4 py-3">
        <div className="flex justify-between items-center">
          {/* Logo and brand */}
          <div className="flex items-center space-x-2">
            <Link to="/" className="text-xl font-bold text-blue-600">
              R3E FaaS
            </Link>
          </div>
          
          {/* Navigation links */}
          <div className="hidden md:flex space-x-6">
            <Link to="/" className="text-gray-700 hover:text-blue-600">
              Home
            </Link>
            <Link to="/services" className="text-gray-700 hover:text-blue-600">
              Services
            </Link>
            <Link to="/functions" className="text-gray-700 hover:text-blue-600">
              Functions
            </Link>
            <Link to="/gas-bank" className="text-gray-700 hover:text-blue-600">
              Gas Bank
            </Link>
            <Link to="/wallet" className="text-gray-700 hover:text-blue-600">
              Wallet
            </Link>
            <Link to="/meta-tx" className="text-gray-700 hover:text-blue-600">
              Meta Tx
            </Link>
          </div>
          
          {/* Wallet status */}
          <div className="flex items-center space-x-4">
            {ethConnected && (
              <div className="text-sm bg-blue-100 text-blue-800 px-3 py-1 rounded-full">
                ETH: {formatAddress(ethAddress)}
              </div>
            )}
            
            {neoConnected && (
              <div className="text-sm bg-green-100 text-green-800 px-3 py-1 rounded-full">
                NEO: {formatAddress(neoAddress)}
              </div>
            )}
            
            {(ethConnected || neoConnected) && (
              <button
                onClick={logout}
                className="text-sm text-red-600 hover:text-red-800"
              >
                Disconnect
              </button>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
