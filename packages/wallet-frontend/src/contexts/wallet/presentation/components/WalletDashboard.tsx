import React from 'react';
import { useWallet } from '../../application/use_cases/useWallet';
import { WalletConfigForm } from './WalletConfigForm';
import { WalletInfo } from './WalletInfo';

interface Props {
  lightningAddress?: string;
  onLogout: () => void;
}

export const WalletDashboard: React.FC<Props> = ({ lightningAddress, onLogout }) => {
  const { wallet, loading, configuring, error, configure } = useWallet(lightningAddress);

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
        <div className="flex justify-between items-center mb-6">
          <div>
            <h2 className="text-2xl font-bold mb-1">Bienvenue sur Flash Wallet</h2>
            <p className="text-gray-400 text-sm">
              Configurez votre wallet pour commencer à recevoir des sats.
            </p>
          </div>
          <button
            onClick={onLogout}
            className="text-gray-400 hover:text-white text-sm"
          >
            ← Retour
          </button>
        </div>
        <WalletConfigForm
          onConfigure={configure}
          error={error}
          configuring={configuring}
          defaultLightningAddress={lightningAddress}
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
      onLogout={onLogout}
    />
  );
};
