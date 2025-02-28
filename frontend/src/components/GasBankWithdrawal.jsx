import React, { useState, useEffect } from 'react';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const GasBankWithdrawal = () => {
  const { ethAddress, ethConnected, neoAddress, neoConnected } = useWallet();
  const { loading, error } = useApi();
  
  const [formData, setFormData] = useState({
    amount: '',
    destination_address: '',
  });
  
  const [account, setAccount] = useState(null);
  const [formError, setFormError] = useState(null);
  const [txHash, setTxHash] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [loadingAccount, setLoadingAccount] = useState(false);
  
  // Load account data
  useEffect(() => {
    const fetchAccount = async () => {
      try {
        if (!ethConnected && !neoConnected) {
          return;
        }
        
        setLoadingAccount(true);
        setFormError(null);
        
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
        
        // Set destination address to connected wallet by default
        setFormData((prev) => ({
          ...prev,
          destination_address: address,
        }));
      } catch (error) {
        console.error('Failed to load account:', error);
        setFormError(error.message);
      } finally {
        setLoadingAccount(false);
      }
    };
    
    fetchAccount();
  }, [ethAddress, ethConnected, neoAddress, neoConnected]);
  
  // Handle form input change
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };
  
  // Format amount
  const formatAmount = (amount, decimals = 8) => {
    return parseFloat(amount).toFixed(decimals);
  };
  
  // Handle form submission
  const handleSubmit = async (e) => {
    e.preventDefault();
    
    try {
      setFormError(null);
      setTxHash(null);
      setIsSubmitting(true);
      
      // Validate form
      if (!formData.amount || parseFloat(formData.amount) <= 0) {
        throw new Error('Please enter a valid amount');
      }
      
      if (!formData.destination_address) {
        throw new Error('Please enter a destination address');
      }
      
      // Check wallet connection
      if (!ethConnected && !neoConnected) {
        throw new Error('Please connect your wallet first');
      }
      
      // Check if account exists
      if (!account) {
        throw new Error('You need to have a Gas Bank account to withdraw');
      }
      
      // Check if amount is less than or equal to balance
      if (parseFloat(formData.amount) > parseFloat(account.balance)) {
        throw new Error('Insufficient balance');
      }
      
      // Determine blockchain type
      const blockchainType = ethConnected ? 'ethereum' : 'neo_n3';
      const address = ethConnected ? ethAddress : neoAddress;
      
      // Submit withdrawal request
      const response = await fetch('/api/gas-bank/withdraw', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
        body: JSON.stringify({
          blockchain_type: blockchainType,
          address,
          amount: formData.amount,
          destination_address: formData.destination_address,
        }),
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to submit withdrawal');
      }
      
      const data = await response.json();
      
      // Set transaction hash
      setTxHash(data.tx_hash);
      
      // Reset form
      setFormData({
        amount: '',
        destination_address: address,
      });
      
      // Refresh account data
      fetchAccount();
    } catch (error) {
      console.error('Failed to withdraw:', error);
      setFormError(error.message);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  // Fetch account data
  const fetchAccount = async () => {
    try {
      if (!ethConnected && !neoConnected) {
        return;
      }
      
      setLoadingAccount(true);
      
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
    } catch (error) {
      console.error('Failed to load account:', error);
    } finally {
      setLoadingAccount(false);
    }
  };
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Withdraw Funds</h2>
      
      {formError && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {formError}
        </div>
      )}
      
      {txHash && (
        <div className="bg-green-100 text-green-800 p-3 rounded-md mb-4">
          <h3 className="font-semibold mb-2">Withdrawal Submitted</h3>
          <p>
            <span className="font-medium">Transaction Hash:</span>{' '}
            <a
              href={`${ethConnected ? 'https://etherscan.io/tx/' : 'https://explorer.neo.org/transaction/'}${txHash}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-blue-600 hover:text-blue-800 break-all"
            >
              {txHash}
            </a>
          </p>
          <p className="mt-2 text-sm">
            Your withdrawal has been submitted and will be processed shortly.
          </p>
        </div>
      )}
      
      {loadingAccount ? (
        <div className="text-center py-4">
          <p className="text-gray-600">Loading account...</p>
        </div>
      ) : !account ? (
        <div className="text-center py-4">
          <p className="text-gray-600 mb-2">
            You don't have a Gas Bank account yet. Deposit funds to create one.
          </p>
        </div>
      ) : (
        <>
          <div className="bg-blue-50 p-4 rounded-md mb-4">
            <div className="flex justify-between items-center">
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Available Balance</h3>
                <p className="text-2xl font-bold text-blue-600">
                  {formatAmount(account.balance)} {account.blockchain_type === 'ethereum' ? 'ETH' : 'GAS'}
                </p>
              </div>
              
              <div>
                <h3 className="text-sm font-semibold text-gray-500">Blockchain</h3>
                <p>{account.blockchain_type}</p>
              </div>
            </div>
          </div>
          
          <form onSubmit={handleSubmit}>
            <div className="mb-4">
              <label htmlFor="amount" className="label">
                Amount ({ethConnected ? 'ETH' : 'GAS'})
              </label>
              <input
                type="number"
                id="amount"
                name="amount"
                value={formData.amount}
                onChange={handleChange}
                className="input"
                placeholder="0.0"
                step="0.000001"
                min="0"
                max={account.balance}
                required
              />
              
              <div className="flex justify-between mt-1">
                <span className="text-xs text-gray-500">
                  Min: 0.000001
                </span>
                <span className="text-xs text-gray-500">
                  Max: {formatAmount(account.balance)}
                </span>
              </div>
            </div>
            
            <div className="mb-4">
              <label htmlFor="destination_address" className="label">
                Destination Address
              </label>
              <input
                type="text"
                id="destination_address"
                name="destination_address"
                value={formData.destination_address}
                onChange={handleChange}
                className="input"
                placeholder={ethConnected ? '0x...' : 'N...'}
                required
              />
              
              <div className="mt-1">
                <button
                  type="button"
                  onClick={() => setFormData((prev) => ({
                    ...prev,
                    destination_address: ethConnected ? ethAddress : neoAddress,
                  }))}
                  className="text-xs text-blue-600 hover:text-blue-800"
                >
                  Use connected wallet address
                </button>
              </div>
            </div>
            
            <div className="mt-6">
              <button
                type="submit"
                className="btn btn-primary w-full"
                disabled={isSubmitting || (!ethConnected && !neoConnected) || !account}
              >
                {isSubmitting ? 'Withdrawing...' : 'Withdraw'}
              </button>
            </div>
          </form>
        </>
      )}
      
      {!ethConnected && !neoConnected && (
        <div className="mt-4 text-center text-sm text-gray-600">
          Please connect your wallet to make a withdrawal.
        </div>
      )}
    </div>
  );
};

export default GasBankWithdrawal;
