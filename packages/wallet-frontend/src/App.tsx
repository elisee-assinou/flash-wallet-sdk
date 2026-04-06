import React from 'react';
import { Layout } from './shared/components/Layout';
import { WalletDashboard } from './contexts/wallet/presentation/components/WalletDashboard';

function App() {
  return (
    <Layout>
      <WalletDashboard />
    </Layout>
  );
}

export default App;
