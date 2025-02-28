import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { EthereumProvider } from '@web3modal/ethereum';
import { Web3Modal } from '@web3modal/react';
import { WalletProvider } from './contexts/WalletContext';
import { ApiProvider } from './contexts/ApiContext';

// Pages
import HomePage from './pages/HomePage';
import ServicesPage from './pages/ServicesPage';
import WalletPage from './pages/WalletPage';
import MetaTxPage from './pages/MetaTxPage';

// Components
import Navbar from './components/Navbar';
import Footer from './components/Footer';

// Web3Modal configuration
const ethereumConfig = {
  appName: 'R3E FaaS Portal',
  autoConnect: false,
};

const App = () => {
  return (
    <>
      <ApiProvider>
        <WalletProvider>
          <div className="min-h-screen flex flex-col">
            <Navbar />
            <main className="flex-grow container mx-auto px-4 py-8">
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/services" element={<ServicesPage />} />
                <Route path="/wallet" element={<WalletPage />} />
                <Route path="/meta-tx" element={<MetaTxPage />} />
              </Routes>
            </main>
            <Footer />
          </div>
        </WalletProvider>
      </ApiProvider>
      
      {/* Web3Modal configuration */}
      <Web3Modal
        ethereumConfig={ethereumConfig}
        themeMode="light"
        themeColor="blue"
      />
    </>
  );
};

export default App;
