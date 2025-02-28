import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useApi } from '../contexts/ApiContext';

const FunctionList = () => {
  const { loading, error } = useApi();
  
  const [functions, setFunctions] = useState([]);
  const [loadingFunctions, setLoadingFunctions] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  
  // Load functions
  useEffect(() => {
    const fetchFunctions = async () => {
      try {
        setLoadingFunctions(true);
        setErrorMessage(null);
        
        // Call API to get functions
        const response = await fetch('/api/functions', {
          headers: {
            'Authorization': `Bearer ${localStorage.getItem('token')}`,
          },
        });
        
        if (!response.ok) {
          throw new Error(`Failed to fetch functions: ${response.statusText}`);
        }
        
        const data = await response.json();
        setFunctions(data.functions || []);
      } catch (error) {
        console.error('Failed to load functions:', error);
        setErrorMessage(error.message);
      } finally {
        setLoadingFunctions(false);
      }
    };
    
    fetchFunctions();
  }, []);
  
  return (
    <div className="card">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Your Functions</h2>
        <Link to="/functions/new" className="btn btn-primary">
          Deploy New Function
        </Link>
      </div>
      
      {errorMessage && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {errorMessage}
        </div>
      )}
      
      {loadingFunctions ? (
        <div className="text-center py-8">
          <p className="text-gray-600">Loading functions...</p>
        </div>
      ) : functions.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-600">No functions deployed yet.</p>
          <Link to="/functions/new" className="btn btn-outline mt-4">
            Deploy Your First Function
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-4">
          {functions.map((func) => (
            <div
              key={func.id}
              className="border rounded-md p-4 hover:shadow-md transition-shadow"
            >
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="text-lg font-semibold mb-1">{func.name}</h3>
                  <p className="text-sm text-gray-600 mb-2">{func.description}</p>
                  
                  <div className="flex flex-wrap gap-2 mb-3">
                    <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded-full">
                      {func.runtime}
                    </span>
                    <span className="text-xs bg-green-100 text-green-800 px-2 py-1 rounded-full">
                      {func.status}
                    </span>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <Link
                    to={`/functions/${func.id}`}
                    className="btn btn-sm btn-outline"
                  >
                    View
                  </Link>
                  <Link
                    to={`/functions/${func.id}/invoke`}
                    className="btn btn-sm btn-primary"
                  >
                    Invoke
                  </Link>
                </div>
              </div>
              
              <div className="text-xs text-gray-500 mt-2">
                <span>Created: {new Date(func.created_at).toLocaleString()}</span>
                <span className="mx-2">•</span>
                <span>Last Updated: {new Date(func.updated_at).toLocaleString()}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default FunctionList;
