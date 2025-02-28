import React, { useState, useEffect } from 'react';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const MetaTxForm = () => {
  const {
    ethAddress,
    ethConnected,
    signEthTypedData,
    neoAddress,
    neoConnected,
    signNeoMessage,
  } = useWallet();
  
  const {
    submitMetaTx,
    getNextNonce,
    loading,
    error,
  } = useApi();
  
  const [formData, setFormData] = useState({
    blockchain_type: 'ethereum',
    target_contract: '',
    tx_data: '',
    nonce: 0,
    deadline: Math.floor(Date.now() / 1000) + 3600, // 1 hour from now
  });
  
  const [formError, setFormError] = useState(null);
  const [txResult, setTxResult] = useState(null);
  
  // Load nonce when wallet is connected
  useEffect(() => {
    const fetchNonce = async () => {
      try {
        if (ethConnected && ethAddress) {
          const nonce = await getNextNonce(ethAddress);
          setFormData((prev) => ({ ...prev, nonce }));
        } else if (neoConnected && neoAddress) {
          const nonce = await getNextNonce(neoAddress);
          setFormData((prev) => ({ ...prev, nonce }));
        }
      } catch (error) {
        console.error('Failed to fetch nonce:', error);
        setFormError(error.message);
      }
    };
    
    fetchNonce();
  }, [ethConnected, ethAddress, neoConnected, neoAddress, getNextNonce]);
  
  // Handle form input change
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };
  
  // Handle blockchain type change
  const handleBlockchainTypeChange = (e) => {
    const { value } = e.target;
    setFormData((prev) => ({ ...prev, blockchain_type: value }));
  };
  
  // Handle form submission
  const handleSubmit = async (e) => {
    e.preventDefault();
    
    try {
      setFormError(null);
      setTxResult(null);
      
      // Validate form
      if (!formData.target_contract) {
        throw new Error('Target contract is required');
      }
      
      if (!formData.tx_data) {
        throw new Error('Transaction data is required');
      }
      
      // Create meta transaction request
      const metaTxRequest = {
        blockchain_type: formData.blockchain_type === 'ethereum' ? 'ethereum' : 'neo_n3',
        signature_curve: formData.blockchain_type === 'ethereum' ? 'secp256k1' : 'secp256r1',
        sender: formData.blockchain_type === 'ethereum' ? ethAddress : neoAddress,
        target_contract: formData.target_contract,
        tx_data: formData.tx_data,
        nonce: formData.nonce,
        deadline: formData.deadline,
        timestamp: Math.floor(Date.now() / 1000),
      };
      
      // Sign meta transaction
      let signature;
      
      if (formData.blockchain_type === 'ethereum') {
        // Sign with Ethereum wallet (EIP-712)
        const domain = {
          name: 'R3E Meta Transaction',
          version: '1',
          chainId: 1, // Mainnet
          verifyingContract: formData.target_contract,
        };
        
        const types = {
          MetaTransaction: [
            { name: 'sender', type: 'address' },
            { name: 'target_contract', type: 'address' },
            { name: 'tx_data', type: 'bytes' },
            { name: 'nonce', type: 'uint256' },
            { name: 'deadline', type: 'uint256' },
            { name: 'timestamp', type: 'uint256' },
          ],
        };
        
        const value = {
          sender: metaTxRequest.sender,
          target_contract: metaTxRequest.target_contract,
          tx_data: metaTxRequest.tx_data,
          nonce: metaTxRequest.nonce,
          deadline: metaTxRequest.deadline,
          timestamp: metaTxRequest.timestamp,
        };
        
        const result = await signEthTypedData(domain, types, value);
        signature = result.signature;
      } else {
        // Sign with Neo N3 wallet
        const message = JSON.stringify({
          sender: metaTxRequest.sender,
          target_contract: metaTxRequest.target_contract,
          tx_data: metaTxRequest.tx_data,
          nonce: metaTxRequest.nonce,
          deadline: metaTxRequest.deadline,
          timestamp: metaTxRequest.timestamp,
        });
        
        const result = await signNeoMessage(message);
        signature = result.signature;
      }
      
      // Submit meta transaction
      metaTxRequest.signature = signature;
      const response = await submitMetaTx(metaTxRequest);
      
      // Set result
      setTxResult(response);
    } catch (error) {
      console.error('Failed to submit meta transaction:', error);
      setFormError(error.message);
    }
  };
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Submit Meta Transaction</h2>
      
      {formError && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {formError}
        </div>
      )}
      
      {txResult && (
        <div className="bg-green-100 text-green-800 p-3 rounded-md mb-4">
          <h3 className="font-semibold mb-2">Transaction Submitted</h3>
          <p><span className="font-medium">Request ID:</span> {txResult.request_id}</p>
          <p><span className="font-medium">Original Hash:</span> {txResult.original_hash}</p>
          <p><span className="font-medium">Relayed Hash:</span> {txResult.relayed_hash}</p>
          <p><span className="font-medium">Status:</span> {txResult.status}</p>
        </div>
      )}
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="blockchain_type" className="label">
            Blockchain Type
          </label>
          <select
            id="blockchain_type"
            name="blockchain_type"
            value={formData.blockchain_type}
            onChange={handleBlockchainTypeChange}
            className="input"
            required
          >
            <option value="ethereum">Ethereum</option>
            <option value="neo_n3">Neo N3</option>
          </select>
        </div>
        
        <div className="mb-4">
          <label htmlFor="target_contract" className="label">
            Target Contract
          </label>
          <input
            type="text"
            id="target_contract"
            name="target_contract"
            value={formData.target_contract}
            onChange={handleChange}
            className="input"
            placeholder={formData.blockchain_type === 'ethereum' ? '0x...' : 'N...'}
            required
          />
        </div>
        
        <div className="mb-4">
          <label htmlFor="tx_data" className="label">
            Transaction Data
          </label>
          <textarea
            id="tx_data"
            name="tx_data"
            value={formData.tx_data}
            onChange={handleChange}
            className="input h-32"
            placeholder={formData.blockchain_type === 'ethereum' ? '0x...' : 'Base64 encoded...'}
            required
          />
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div>
            <label htmlFor="nonce" className="label">
              Nonce
            </label>
            <input
              type="number"
              id="nonce"
              name="nonce"
              value={formData.nonce}
              onChange={handleChange}
              className="input"
              required
            />
          </div>
          
          <div>
            <label htmlFor="deadline" className="label">
              Deadline (Unix Timestamp)
            </label>
            <input
              type="number"
              id="deadline"
              name="deadline"
              value={formData.deadline}
              onChange={handleChange}
              className="input"
              required
            />
          </div>
        </div>
        
        <div className="mt-6">
          <button
            type="submit"
            className="btn btn-primary w-full"
            disabled={loading || (!ethConnected && !neoConnected)}
          >
            {loading ? 'Submitting...' : 'Submit Meta Transaction'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default MetaTxForm;
