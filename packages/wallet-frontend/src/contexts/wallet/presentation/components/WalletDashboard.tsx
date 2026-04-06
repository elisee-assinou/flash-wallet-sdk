import React from 'react';
import { useWallet } from '../../application/use_cases/useWallet';
import { WalletConfigForm } from './WalletConfigForm';
import { WalletInfo } from './WalletInfo';

export const WalletDashboard: React.FC = () => {
  const { wallet, loading, configuring, error, configure } = useWallet();

  if (loading) {
    return (
      <div className="flex items-center justify-center py-20">
        <div className="text-yellow-400 text-lg">Chargement...</div>
      </div>
    );
  }

  if (!wallet) {
    return (
      <div>
        <h2 className="text-2xl font-bold mb-2">Bienvenue sur Flash Wallet</h2>
        <p className="text-gray-400 mb-8">
          Configurez votre wallet pour commencer à recevoir des sats et les convertir en XOF sur MTN MoMo.
        </p>
        <WalletConfigForm
          onConfigure={configure}
          error={error}
          configuring={configuring}
        />
      </div>
    );
  }

  return (
    <WalletInfo
      wallet={wallet}
      onReconfigure={configure}
      reconfigureError={error}
      configuring={configuring}
    />
  );
};
