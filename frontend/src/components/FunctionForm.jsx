import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useApi } from '../contexts/ApiContext';
import { useWallet } from '../contexts/WalletContext';

const FunctionForm = ({ editFunction = null }) => {
  const navigate = useNavigate();
  const { loading, error } = useApi();
  const { ethConnected, neoConnected } = useWallet();
  
  const [formData, setFormData] = useState({
    name: editFunction?.name || '',
    description: editFunction?.description || '',
    runtime: editFunction?.runtime || 'node16',
    code: editFunction?.code || '',
    blockchain_type: editFunction?.blockchain_type || 'neo_n3',
    environment_variables: editFunction?.environment_variables || {},
  });
  
  const [formError, setFormError] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  // Available runtimes
  const runtimes = [
    { value: 'node16', label: 'Node.js 16' },
    { value: 'python3.9', label: 'Python 3.9' },
    { value: 'rust1.60', label: 'Rust 1.60' },
    { value: 'deno1.24', label: 'Deno 1.24' },
  ];
  
  // Blockchain types
  const blockchainTypes = [
    { value: 'neo_n3', label: 'Neo N3' },
    { value: 'ethereum', label: 'Ethereum' },
  ];
  
  // Handle form input change
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };
  
  // Handle environment variables change
  const handleEnvVarChange = (key, value) => {
    setFormData((prev) => ({
      ...prev,
      environment_variables: {
        ...prev.environment_variables,
        [key]: value,
      },
    }));
  };
  
  // Add new environment variable
  const handleAddEnvVar = () => {
    const key = `ENV_VAR_${Object.keys(formData.environment_variables).length + 1}`;
    handleEnvVarChange(key, '');
  };
  
  // Remove environment variable
  const handleRemoveEnvVar = (key) => {
    const newEnvVars = { ...formData.environment_variables };
    delete newEnvVars[key];
    
    setFormData((prev) => ({
      ...prev,
      environment_variables: newEnvVars,
    }));
  };
  
  // Handle form submission
  const handleSubmit = async (e) => {
    e.preventDefault();
    
    try {
      setFormError(null);
      setIsSubmitting(true);
      
      // Validate form
      if (!formData.name) {
        throw new Error('Function name is required');
      }
      
      if (!formData.code) {
        throw new Error('Function code is required');
      }
      
      // Check wallet connection
      if (formData.blockchain_type === 'ethereum' && !ethConnected) {
        throw new Error('Please connect your Ethereum wallet first');
      }
      
      if (formData.blockchain_type === 'neo_n3' && !neoConnected) {
        throw new Error('Please connect your Neo N3 wallet first');
      }
      
      // Create request payload
      const payload = {
        name: formData.name,
        description: formData.description,
        runtime: formData.runtime,
        code: formData.code,
        blockchain_type: formData.blockchain_type,
        environment_variables: formData.environment_variables,
      };
      
      // Send request to API
      const url = editFunction
        ? `/api/functions/${editFunction.id}`
        : '/api/functions';
      
      const method = editFunction ? 'PUT' : 'POST';
      
      const response = await fetch(url, {
        method,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
        body: JSON.stringify(payload),
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to deploy function');
      }
      
      const data = await response.json();
      
      // Navigate to function detail page
      navigate(`/functions/${data.id}`);
    } catch (error) {
      console.error('Failed to deploy function:', error);
      setFormError(error.message);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <div className="card">
      <h2 className="text-xl font-bold mb-4">
        {editFunction ? 'Edit Function' : 'Deploy New Function'}
      </h2>
      
      {formError && (
        <div className="bg-red-100 text-red-700 p-3 rounded-md mb-4">
          {formError}
        </div>
      )}
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="name" className="label">
            Function Name
          </label>
          <input
            type="text"
            id="name"
            name="name"
            value={formData.name}
            onChange={handleChange}
            className="input"
            placeholder="my-awesome-function"
            required
          />
        </div>
        
        <div className="mb-4">
          <label htmlFor="description" className="label">
            Description
          </label>
          <textarea
            id="description"
            name="description"
            value={formData.description}
            onChange={handleChange}
            className="input h-20"
            placeholder="Describe what your function does..."
          />
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div>
            <label htmlFor="runtime" className="label">
              Runtime
            </label>
            <select
              id="runtime"
              name="runtime"
              value={formData.runtime}
              onChange={handleChange}
              className="input"
              required
            >
              {runtimes.map((runtime) => (
                <option key={runtime.value} value={runtime.value}>
                  {runtime.label}
                </option>
              ))}
            </select>
          </div>
          
          <div>
            <label htmlFor="blockchain_type" className="label">
              Blockchain Type
            </label>
            <select
              id="blockchain_type"
              name="blockchain_type"
              value={formData.blockchain_type}
              onChange={handleChange}
              className="input"
              required
            >
              {blockchainTypes.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>
          </div>
        </div>
        
        <div className="mb-4">
          <label htmlFor="code" className="label">
            Function Code
          </label>
          <textarea
            id="code"
            name="code"
            value={formData.code}
            onChange={handleChange}
            className="input h-64 font-mono"
            placeholder={`// Write your function code here\n\nexport default async function handler(req, res) {\n  // Your code here\n  return { message: 'Hello, World!' };\n}`}
            required
          />
        </div>
        
        <div className="mb-4">
          <div className="flex justify-between items-center mb-2">
            <label className="label">Environment Variables</label>
            <button
              type="button"
              onClick={handleAddEnvVar}
              className="text-sm text-blue-600 hover:text-blue-800"
            >
              + Add Variable
            </button>
          </div>
          
          {Object.keys(formData.environment_variables).length === 0 ? (
            <div className="text-sm text-gray-500 italic">
              No environment variables defined
            </div>
          ) : (
            <div className="space-y-2">
              {Object.entries(formData.environment_variables).map(([key, value]) => (
                <div key={key} className="flex gap-2">
                  <input
                    type="text"
                    value={key}
                    onChange={(e) => {
                      const newEnvVars = { ...formData.environment_variables };
                      const newKey = e.target.value;
                      
                      if (newKey !== key) {
                        newEnvVars[newKey] = value;
                        delete newEnvVars[key];
                        
                        setFormData((prev) => ({
                          ...prev,
                          environment_variables: newEnvVars,
                        }));
                      }
                    }}
                    className="input flex-1"
                    placeholder="KEY"
                  />
                  <input
                    type="text"
                    value={value}
                    onChange={(e) => handleEnvVarChange(key, e.target.value)}
                    className="input flex-1"
                    placeholder="value"
                  />
                  <button
                    type="button"
                    onClick={() => handleRemoveEnvVar(key)}
                    className="text-red-600 hover:text-red-800"
                  >
                    ✕
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
        
        <div className="mt-6 flex justify-end gap-4">
          <button
            type="button"
            onClick={() => navigate(-1)}
            className="btn btn-outline"
          >
            Cancel
          </button>
          <button
            type="submit"
            className="btn btn-primary"
            disabled={isSubmitting}
          >
            {isSubmitting
              ? 'Deploying...'
              : editFunction
              ? 'Update Function'
              : 'Deploy Function'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default FunctionForm;
