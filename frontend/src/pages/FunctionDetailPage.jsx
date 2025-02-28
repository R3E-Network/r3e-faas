import React from 'react';
import { useParams } from 'react-router-dom';
import FunctionDetail from '../components/FunctionDetail';

const FunctionDetailPage = () => {
  const { id } = useParams();
  
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-2">Function Details</h1>
        <p className="text-gray-600">
          View and manage your function details, logs, and performance metrics.
        </p>
      </div>
      
      <FunctionDetail />
    </div>
  );
};

export default FunctionDetailPage;
