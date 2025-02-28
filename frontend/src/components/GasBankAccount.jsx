import React, { useState, useEffect } from 'react';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const GasBankAccount = () => {
  const { ethAddress, ethConnected, neoAddress, neoConnected } = useWallet();
  const { loading, error } = useApi();
  
  const [account, setAccount] = useState(null);
  const [transactions, setTransactions] = useState([]);
  const [loadingAccount, setLoadingAccount] = useState(false);
  const [loadingTransactions, setLoadingTransactions] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  
  // Load account data
  useEffect(() => {
    const fetchAccount = async () => {
      try {
        if (!ethConnected && !neoConnected) {
          return;
        }
        
        setLoadingAccount(true);
        setErrorMessage(null);
        
        // Determine address to use
        const address = ethConnected ? ethAddress : neoAddress;
        const blockchainType = ethConnected ? 'ethereum' : 'neo_n3';
        
        // Call API to get account
        const response = await fetch(`/api/gas-bank/account/${blockchainType}/${address}`, {
          headers: {
            'Authorization': `Bearer ${localStorage.getItem('token')}`,
          },
        });
        
        if (!response.ok) {
          if (response.status === 404) {
            // Account doesn't exist yet
            setAccount(null);
            return;
          }
          
          throw new Error(`Failed to fetch account: ${response.statusText}`);
        }
        
        const data = await response.json();
        setAccount(data);
        
        // Fetch transactions
        fetchTransactions(blockchainType, address);
      } catch (error) {
        console.error('Failed to load account:', error);
        setErrorMessage(error.message);
      } finally {
        setLoadingAccount(false);
      }
    };
    
    fetchAccount();
  }, [ethAddress, ethConnected, neoAddress, neoConnected]);
  
  // Fetch transactions
  const fetchTransactions = async (blockchainType, address) => {
    try {
      setLoadingTransactions(true);
      
      // Call API to get transactions
      const response = await fetch(`/api/gas-bank/transactions/${blockchainType}/${address}`, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch transactions: ${response.statusText}`);
      }
      
      const data = await response.json();
      setTransactions(data.transactions || []);
    } catch (error) {
      console.error('Failed to load transactions:', error);
    } finally {
      setLoadingTransactions(false);
    }
  };
  
  // Format amount
  const formatAmount = (amount, decimals = 8) => {
    return parseFloat(amount).toFixed(decimals);
  };
  
  if (!ethConnected && !neoConnected) {
    return (
      <div className="card">
        <h2 className="text-xl font-bold mb-4">Gas Bank Account</h2>
        
        <div className="text-center py-8">
          <p className="text-gray-600 mb-4">
            Please connect your wallet to view your Gas Bank account.
          </p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Gas Bank Account</h2>
      
      {errorMessage && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {errorMessage}
        </div>
      )}
      
      {loadingAccount ? (
        <div className="text-center py-8">
          <p className="text-gray-600">Loading account...</p>
        </div>
      ) : !account ? (
        <div className="text-center py-8">
          <p className="text-gray-600 mb-4">
            You don't have a Gas Bank account yet. Deposit funds to create one.
          </p>
        </div>
      ) : (
        <div className="space-y-6">
          <div className="bg-blue-50 p-4 rounded-md">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Address</h3>
                <p className="font-mono text-sm">{account.address}</p>
              </div>
              
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Blockchain</h3>
                <p>{account.blockchain_type}</p>
              </div>
              
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Balance</h3>
                <p className="text-2xl font-bold text-blue-600">
                  {formatAmount(account.balance)} {account.blockchain_type === 'ethereum' ? 'ETH' : 'GAS'}
                </p>
              </div>
              
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Created At</h3>
                <p>{new Date(account.created_at).toLocaleString()}</p>
              </div>
            </div>
          </div>
          
          <div>
            <h3 className="text-lg font-semibold mb-3">Transaction History</h3>
            
            {loadingTransactions ? (
              <div className="text-center py-4">
                <p className="text-gray-600">Loading transactions...</p>
              </div>
            ) : transactions.length === 0 ? (
              <div className="text-center py-4">
                <p className="text-gray-600">No transactions yet</p>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="bg-gray-100">
                      <th className="px-4 py-2 text-left">Type</th>
                      <th className="px-4 py-2 text-left">Amount</th>
                      <th className="px-4 py-2 text-left">Status</th>
                      <th className="px-4 py-2 text-left">Timestamp</th>
                      <th className="px-4 py-2 text-left">Transaction Hash</th>
                    </tr>
                  </thead>
                  <tbody>
                    {transactions.map((tx) => (
                      <tr key={tx.id} className="border-b">
                        <td className="px-4 py-2">
                          <span className={`inline-block px-2 py-1 rounded-full text-xs ${
                            tx.type === 'deposit'
                              ? 'bg-green-100 text-green-800'
                              : 'bg-red-100 text-red-800'
                          }`}>
                            {tx.type}
                          </span>
                        </td>
                        <td className="px-4 py-2">
                          {formatAmount(tx.amount)} {account.blockchain_type === 'ethereum' ? 'ETH' : 'GAS'}
                        </td>
                        <td className="px-4 py-2">
                          <span className={`inline-block px-2 py-1 rounded-full text-xs ${
                            tx.status === 'completed'
                              ? 'bg-green-100 text-green-800'
                              : tx.status === 'pending'
                              ? 'bg-yellow-100 text-yellow-800'
                              : 'bg-red-100 text-red-800'
                          }`}>
                            {tx.status}
                          </span>
                        </td>
                        <td className="px-4 py-2">
                          {new Date(tx.timestamp).toLocaleString()}
                        </td>
                        <td className="px-4 py-2">
                          <a
                            href={`${tx.blockchain_type === 'ethereum' ? 'https://etherscan.io/tx/' : 'https://explorer.neo.org/transaction/'}${tx.tx_hash}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:text-blue-800 font-mono text-xs"
                          >
                            {tx.tx_hash.slice(0, 10)}...{tx.tx_hash.slice(-8)}
                          </a>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default GasBankAccount;
