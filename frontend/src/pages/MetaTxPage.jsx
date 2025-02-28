import React from 'react';
import MetaTxForm from '../components/MetaTxForm';

const MetaTxPage = () => {
  return (
    <div className="space-y-8">
      <div className="bg-white p-6 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-4">Meta Transactions</h1>
        <p className="text-gray-600">
          Submit meta transactions to interact with blockchain contracts without paying gas fees.
          Connect your wallet to sign and submit transactions through the R3E FaaS service.
        </p>
      </div>
      
      <MetaTxForm />
      
      <div className="card">
        <h2 className="text-xl font-bold mb-4">About Meta Transactions</h2>
        <p className="text-gray-600 mb-4">
          Meta transactions allow users to interact with blockchain contracts without directly paying gas fees.
          Instead, a relayer (in this case, the R3E FaaS service) submits the transaction on behalf of the user.
        </p>
        
        <h3 className="text-lg font-semibold mb-2">How it works:</h3>
        <ol className="list-decimal list-inside space-y-2 text-gray-700 mb-4">
          <li>User signs a message containing transaction details with their wallet</li>
          <li>The signed message is sent to the R3E FaaS service</li>
          <li>The service verifies the signature and submits the transaction to the blockchain</li>
          <li>The service pays the gas fees and may charge a fee in tokens</li>
          <li>The transaction is executed on the blockchain</li>
        </ol>
        
        <h3 className="text-lg font-semibold mb-2">Benefits:</h3>
        <ul className="list-disc list-inside space-y-2 text-gray-700">
          <li>Users don't need to hold native tokens (ETH, GAS) for gas fees</li>
          <li>Improved user experience with fewer transaction steps</li>
          <li>Reduced friction for onboarding new users</li>
          <li>Support for both Ethereum and Neo N3 blockchains</li>
        </ul>
      </div>
    </div>
  );
};

export default MetaTxPage;
