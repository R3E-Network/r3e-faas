import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useWallet } from '../contexts/WalletContext';
import { useApi } from '../contexts/ApiContext';

const FunctionInvoke = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const { ethAddress, ethConnected, neoAddress, neoConnected } = useWallet();
  const { loading, error } = useApi();
  
  const [functionData, setFunctionData] = useState(null);
  const [formData, setFormData] = useState({
    params: '{}',
  });
  
  const [loadingFunction, setLoadingFunction] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [result, setResult] = useState(null);
  
  // Load function data
  useEffect(() => {
    const fetchFunction = async () => {
      try {
        setLoadingFunction(true);
        setErrorMessage(null);
        
        // Call API to get function
        const response = await fetch(`/api/functions/${id}`, {
          headers: {
            'Authorization': `Bearer ${localStorage.getItem('token')}`,
          },
        });
        
        if (!response.ok) {
          throw new Error(`Failed to fetch function: ${response.statusText}`);
        }
        
        const data = await response.json();
        setFunctionData(data);
      } catch (error) {
        console.error('Failed to load function:', error);
        setErrorMessage(error.message);
      } finally {
        setLoadingFunction(false);
      }
    };
    
    fetchFunction();
  }, [id]);
  
  // Handle form input change
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };
  
  // Handle form submission
  const handleSubmit = async (e) => {
    e.preventDefault();
    
    try {
      setErrorMessage(null);
      setResult(null);
      setIsSubmitting(true);
      
      // Validate form
      let params;
      try {
        params = JSON.parse(formData.params);
      } catch (error) {
        throw new Error('Invalid JSON parameters');
      }
      
      // Check wallet connection
      if (functionData.blockchain_type === 'ethereum' && !ethConnected) {
        throw new Error('Please connect your Ethereum wallet first');
      }
      
      if (functionData.blockchain_type === 'neo_n3' && !neoConnected) {
        throw new Error('Please connect your Neo N3 wallet first');
      }
      
      // Determine address to use
      const address = functionData.blockchain_type === 'ethereum' ? ethAddress : neoAddress;
      
      // Submit invocation request
      const response = await fetch(`/api/functions/${id}/invoke`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
        body: JSON.stringify({
          blockchain_type: functionData.blockchain_type,
          address,
          params,
        }),
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to invoke function');
      }
      
      const data = await response.json();
      
      // Set result
      setResult(data.result);
    } catch (error) {
      console.error('Failed to invoke function:', error);
      setErrorMessage(error.message);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  if (loadingFunction) {
    return (
      <div className="text-center py-8">
        <p className="text-gray-600">Loading function...</p>
      </div>
    );
  }
  
  if (errorMessage && !functionData) {
    return (
      <div className="bg-red-100 text-red-700 p-4 rounded-md">
        <h2 className="text-lg font-semibold mb-2">Error</h2>
        <p>{errorMessage}</p>
        <button
          onClick={() => navigate('/functions')}
          className="mt-4 btn btn-outline"
        >
          Back to Functions
        </button>
      </div>
    );
  }
  
  if (!functionData) {
    return null;
  }
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">Invoke Function: {functionData.name}</h2>
      
      {errorMessage && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {errorMessage}
        </div>
      )}
      
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">Function Details</h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <h4 className="text-sm font-semibold text-gray-500">Runtime</h4>
            <p>{functionData.runtime}</p>
          </div>
          
          <div>
            <h4 className="text-sm font-semibold text-gray-500">Blockchain Type</h4>
            <p>{functionData.blockchain_type}</p>
          </div>
          
          <div>
            <h4 className="text-sm font-semibold text-gray-500">Status</h4>
            <p>
              <span className={`inline-block px-2 py-1 rounded-full text-xs ${
                functionData.status === 'active'
                  ? 'bg-green-100 text-green-800'
                  : functionData.status === 'error'
                  ? 'bg-red-100 text-red-800'
                  : 'bg-yellow-100 text-yellow-800'
              }`}>
                {functionData.status}
              </span>
            </p>
          </div>
        </div>
      </div>
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="params" className="label">
            Parameters (JSON)
          </label>
          <textarea
            id="params"
            name="params"
            value={formData.params}
            onChange={handleChange}
            className="input h-32 font-mono"
            placeholder="{}"
            required
          />
          
          <div className="mt-1 text-xs text-gray-500">
            Enter parameters as a valid JSON object.
          </div>
        </div>
        
        <div className="mt-6">
          <button
            type="submit"
            className="btn btn-primary w-full"
            disabled={
              isSubmitting ||
              (functionData.blockchain_type === 'ethereum' && !ethConnected) ||
              (functionData.blockchain_type === 'neo_n3' && !neoConnected)
            }
          >
            {isSubmitting ? 'Invoking...' : 'Invoke Function'}
          </button>
        </div>
      </form>
      
      {!ethConnected && !neoConnected && (
        <div className="mt-4 text-center text-sm text-gray-600">
          Please connect your wallet to invoke this function.
        </div>
      )}
      
      {result && (
        <div className="mt-6">
          <h3 className="text-lg font-semibold mb-2">Result</h3>
          
          <pre className="bg-gray-100 p-4 rounded-md overflow-x-auto font-mono text-sm">
            {typeof result === 'object' ? JSON.stringify(result, null, 2) : result.toString()}
          </pre>
        </div>
      )}
    </div>
  );
};

export default FunctionInvoke;
