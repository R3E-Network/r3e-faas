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
import FunctionsPage from './pages/FunctionsPage';
import FunctionDetailPage from './pages/FunctionDetailPage';
import FunctionNewPage from './pages/FunctionNewPage';
import FunctionEditPage from './pages/FunctionEditPage';
import FunctionInvokePage from './pages/FunctionInvokePage';
import GasBankPage from './pages/GasBankPage';

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
                <Route path="/functions" element={<FunctionsPage />} />
                <Route path="/functions/new" element={<FunctionNewPage />} />
                <Route path="/functions/:id" element={<FunctionDetailPage />} />
                <Route path="/functions/:id/edit" element={<FunctionEditPage />} />
                <Route path="/functions/:id/invoke" element={<FunctionInvokePage />} />
                <Route path="/gas-bank" element={<GasBankPage />} />
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
