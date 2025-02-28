import React, { useState } from 'react';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const GasBankDeposit = () => {
  const { ethAddress, ethConnected, ethSigner, neoAddress, neoConnected, neoWallet } = useWallet();
  const { loading, error } = useApi();
  
  const [formData, setFormData] = useState({
    amount: '',
  });
  
  const [formError, setFormError] = useState(null);
  const [txHash, setTxHash] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  // Handle form input change
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
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
      
      // Check wallet connection
      if (!ethConnected && !neoConnected) {
        throw new Error('Please connect your wallet first');
      }
      
      // Determine blockchain type
      const blockchainType = ethConnected ? 'ethereum' : 'neo_n3';
      const address = ethConnected ? ethAddress : neoAddress;
      
      // Get deposit address
      const depositAddressResponse = await fetch(`/api/gas-bank/deposit-address/${blockchainType}/${address}`, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
      });
      
      if (!depositAddressResponse.ok) {
        const errorData = await depositAddressResponse.json();
        throw new Error(errorData.message || 'Failed to get deposit address');
      }
      
      const { deposit_address } = await depositAddressResponse.json();
      
      // Send transaction
      let hash;
      
      if (blockchainType === 'ethereum') {
        // Send Ethereum transaction
        const tx = await ethSigner.sendTransaction({
          to: deposit_address,
          value: ethers.utils.parseEther(formData.amount),
        });
        
        hash = tx.hash;
      } else {
        // Send Neo N3 transaction
        const tx = await neoWallet.send({
          fromAddress: neoAddress,
          toAddress: deposit_address,
          asset: 'GAS',
          amount: formData.amount,
        });
        
        hash = tx.txid;
      }
      
      // Notify API about the deposit
      const notifyResponse = await fetch('/api/gas-bank/deposit', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
        body: JSON.stringify({
          blockchain_type: blockchainType,
          address,
          amount: formData.amount,
          tx_hash: hash,
        }),
      });
      
      if (!notifyResponse.ok) {
        const errorData = await notifyResponse.json();
        throw new Error(errorData.message || 'Failed to notify about deposit');
      }
      
      // Set transaction hash
      setTxHash(hash);
      
      // Reset form
      setFormData({ amount: '' });
    } catch (error) {
      console.error('Failed to deposit:', error);
      setFormError(error.message);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Deposit Funds</h2>
      
      {formError && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {formError}
        </div>
      )}
      
      {txHash && (
        <div className="bg-green-100 text-green-800 p-3 rounded-md mb-4">
          <h3 className="font-semibold mb-2">Deposit Submitted</h3>
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
            Your deposit will be credited to your Gas Bank account once it's confirmed on the blockchain.
          </p>
        </div>
      )}
      
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
            required
          />
        </div>
        
        <div className="mt-6">
          <button
            type="submit"
            className="btn btn-primary w-full"
            disabled={isSubmitting || (!ethConnected && !neoConnected)}
          >
            {isSubmitting ? 'Depositing...' : 'Deposit'}
          </button>
        </div>
      </form>
      
      {!ethConnected && !neoConnected && (
        <div className="mt-4 text-center text-sm text-gray-600">
          Please connect your wallet to make a deposit.
        </div>
      )}
    </div>
  );
};

export default GasBankDeposit;
