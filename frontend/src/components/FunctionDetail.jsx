import React, { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { useApi } from '../contexts/ApiContext';

const FunctionDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const { loading, error } = useApi();
  
  const [functionData, setFunctionData] = useState(null);
  const [loadingFunction, setLoadingFunction] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  const [logs, setLogs] = useState([]);
  const [loadingLogs, setLoadingLogs] = useState(false);
  
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
        
        // Fetch logs
        fetchLogs();
      } catch (error) {
        console.error('Failed to load function:', error);
        setErrorMessage(error.message);
      } finally {
        setLoadingFunction(false);
      }
    };
    
    fetchFunction();
  }, [id]);
  
  // Fetch function logs
  const fetchLogs = async () => {
    try {
      setLoadingLogs(true);
      
      // Call API to get logs
      const response = await fetch(`/api/functions/${id}/logs`, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch logs: ${response.statusText}`);
      }
      
      const data = await response.json();
      setLogs(data.logs || []);
    } catch (error) {
      console.error('Failed to load logs:', error);
    } finally {
      setLoadingLogs(false);
    }
  };
  
  // Delete function
  const handleDelete = async () => {
    if (!window.confirm('Are you sure you want to delete this function?')) {
      return;
    }
    
    try {
      // Call API to delete function
      const response = await fetch(`/api/functions/${id}`, {
        method: 'DELETE',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
      });
      
      if (!response.ok) {
        throw new Error(`Failed to delete function: ${response.statusText}`);
      }
      
      // Navigate back to functions list
      navigate('/functions');
    } catch (error) {
      console.error('Failed to delete function:', error);
      setErrorMessage(error.message);
    }
  };
  
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
    <div className="space-y-6">
      <div className="flex justify-between items-start">
        <div>
          <h1 className="text-2xl font-bold mb-2">{functionData.name}</h1>
          <p className="text-gray-600">{functionData.description}</p>
        </div>
        
        <div className="flex gap-2">
          <Link
            to={`/functions/${id}/edit`}
            className="btn btn-outline"
          >
            Edit
          </Link>
          <Link
            to={`/functions/${id}/invoke`}
            className="btn btn-primary"
          >
            Invoke
          </Link>
          <button
            onClick={handleDelete}
            className="btn btn-danger"
          >
            Delete
          </button>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="card">
          <h2 className="text-xl font-bold mb-4">Function Details</h2>
          
          <div className="space-y-4">
            <div>
              <h3 className="text-sm font-semibold text-gray-500">Runtime</h3>
              <p>{functionData.runtime}</p>
            </div>
            
            <div>
              <h3 className="text-sm font-semibold text-gray-500">Blockchain Type</h3>
              <p>{functionData.blockchain_type}</p>
            </div>
            
            <div>
              <h3 className="text-sm font-semibold text-gray-500">Status</h3>
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
            
            <div>
              <h3 className="text-sm font-semibold text-gray-500">Created At</h3>
              <p>{new Date(functionData.created_at).toLocaleString()}</p>
            </div>
            
            <div>
              <h3 className="text-sm font-semibold text-gray-500">Last Updated</h3>
              <p>{new Date(functionData.updated_at).toLocaleString()}</p>
            </div>
          </div>
        </div>
        
        <div className="card">
          <h2 className="text-xl font-bold mb-4">Environment Variables</h2>
          
          {Object.keys(functionData.environment_variables || {}).length === 0 ? (
            <p className="text-gray-500 italic">No environment variables defined</p>
          ) : (
            <div className="space-y-2">
              {Object.entries(functionData.environment_variables || {}).map(([key, value]) => (
                <div key={key} className="flex justify-between border-b pb-2">
                  <span className="font-mono text-sm">{key}</span>
                  <span className="font-mono text-sm text-gray-600">{value}</span>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">Function Code</h2>
        
        <pre className="bg-gray-100 p-4 rounded-md overflow-x-auto font-mono text-sm">
          {functionData.code}
        </pre>
      </div>
      
      <div className="card">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold">Function Logs</h2>
          
          <button
            onClick={fetchLogs}
            className="btn btn-sm btn-outline"
            disabled={loadingLogs}
          >
            {loadingLogs ? 'Refreshing...' : 'Refresh Logs'}
          </button>
        </div>
        
        {logs.length === 0 ? (
          <p className="text-gray-500 italic">No logs available</p>
        ) : (
          <div className="space-y-2">
            {logs.map((log, index) => (
              <div key={index} className="border-b pb-2">
                <div className="flex justify-between text-sm">
                  <span className="font-semibold">{new Date(log.timestamp).toLocaleString()}</span>
                  <span className={`px-2 py-0.5 rounded-full text-xs ${
                    log.level === 'error'
                      ? 'bg-red-100 text-red-800'
                      : log.level === 'warn'
                      ? 'bg-yellow-100 text-yellow-800'
                      : 'bg-blue-100 text-blue-800'
                  }`}>
                    {log.level}
                  </span>
                </div>
                <p className="font-mono text-sm mt-1">{log.message}</p>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default FunctionDetail;
