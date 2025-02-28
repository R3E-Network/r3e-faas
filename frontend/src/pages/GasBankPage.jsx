import React, { useState } from 'react';
import GasBankAccount from '../components/GasBankAccount';
import GasBankDeposit from '../components/GasBankDeposit';
import GasBankWithdrawal from '../components/GasBankWithdrawal';

const GasBankPage = () => {
  const [activeTab, setActiveTab] = useState('account');
  
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-2">Gas Bank</h1>
        <p className="text-gray-600">
          Manage your Gas Bank account, deposit funds, and withdraw funds.
          The Gas Bank allows you to pay for function execution and meta transactions.
        </p>
      </div>
      
      <div className="card">
        <div className="border-b mb-4">
          <div className="flex">
            <button
              className={`px-4 py-2 font-medium ${
                activeTab === 'account'
                  ? 'border-b-2 border-blue-600 text-blue-600'
                  : 'text-gray-500 hover:text-gray-700'
              }`}
              onClick={() => setActiveTab('account')}
            >
              Account
            </button>
            
            <button
              className={`px-4 py-2 font-medium ${
                activeTab === 'deposit'
                  ? 'border-b-2 border-blue-600 text-blue-600'
                  : 'text-gray-500 hover:text-gray-700'
              }`}
              onClick={() => setActiveTab('deposit')}
            >
              Deposit
            </button>
            
            <button
              className={`px-4 py-2 font-medium ${
                activeTab === 'withdraw'
                  ? 'border-b-2 border-blue-600 text-blue-600'
                  : 'text-gray-500 hover:text-gray-700'
              }`}
              onClick={() => setActiveTab('withdraw')}
            >
              Withdraw
            </button>
          </div>
        </div>
        
        {activeTab === 'account' && <GasBankAccount />}
        {activeTab === 'deposit' && <GasBankDeposit />}
        {activeTab === 'withdraw' && <GasBankWithdrawal />}
      </div>
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">About Gas Bank</h2>
        <p className="text-gray-600 mb-4">
          The Gas Bank is a service that allows you to deposit and withdraw funds for use with R3E FaaS services.
          It supports both Ethereum and Neo N3 blockchains, allowing you to pay for function execution and meta transactions.
        </p>
        
        <h3 className="text-lg font-semibold mb-2">Features:</h3>
        <ul className="list-disc list-inside space-y-2 text-gray-700 mb-4">
          <li>Deposit funds from your wallet to your Gas Bank account</li>
          <li>Withdraw funds from your Gas Bank account to your wallet</li>
          <li>View your Gas Bank account balance and transaction history</li>
          <li>Pay for function execution and meta transactions</li>
          <li>Support for both Ethereum and Neo N3 blockchains</li>
        </ul>
        
        <h3 className="text-lg font-semibold mb-2">How It Works:</h3>
        <ol className="list-decimal list-inside space-y-2 text-gray-700">
          <li>Connect your wallet</li>
          <li>Deposit funds to your Gas Bank account</li>
          <li>Use your Gas Bank account to pay for function execution and meta transactions</li>
          <li>Withdraw funds when needed</li>
        </ol>
      </div>
    </div>
  );
};

export default GasBankPage;
