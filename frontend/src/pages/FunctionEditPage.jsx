import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import FunctionForm from '../components/FunctionForm';
import { useApi } from '../contexts/ApiContext';

const FunctionEditPage = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const { loading, error } = useApi();
  
  const [functionData, setFunctionData] = useState(null);
  const [loadingFunction, setLoadingFunction] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  
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
  
  if (loadingFunction) {
    return (
      <div className="text-center py-8">
        <p className="text-gray-600">Loading function...</p>
      </div>
    );
  }
  
  if (errorMessage) {
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
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-2">Edit Function: {functionData.name}</h1>
        <p className="text-gray-600">
          Update your function's code, configuration, and environment variables.
        </p>
      </div>
      
      <FunctionForm editFunction={functionData} />
    </div>
  );
};

export default FunctionEditPage;
