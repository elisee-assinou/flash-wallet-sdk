import React, { useState } from 'react';
import { Layout } from './shared/components/Layout';
import { WalletDashboard } from './contexts/wallet/presentation/components/WalletDashboard';
import { LoginPage } from './contexts/wallet/presentation/components/LoginPage';

function App() {
  const [lightningAddress, setLightningAddress] = useState<string | null>(null);

  if (lightningAddress === null) {
    return <LoginPage onLogin={(addr) => setLightningAddress(addr)} />;
  }

  return (
    <Layout>
      <WalletDashboard
        lightningAddress={lightningAddress || undefined}
        onLogout={() => setLightningAddress(null)}
      />
    </Layout>
  );
}

export default App;
