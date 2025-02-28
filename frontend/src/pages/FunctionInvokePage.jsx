import React from 'react';
import { useParams } from 'react-router-dom';
import FunctionInvoke from '../components/FunctionInvoke';

const FunctionInvokePage = () => {
  const { id } = useParams();
  
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-2">Invoke Function</h1>
        <p className="text-gray-600">
          Execute your function with custom parameters and view the results.
        </p>
      </div>
      
      <FunctionInvoke />
    </div>
  );
};

export default FunctionInvokePage;
