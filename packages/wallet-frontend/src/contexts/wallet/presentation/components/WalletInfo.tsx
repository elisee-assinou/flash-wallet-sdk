import React, { useState } from 'react';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';
import { ConversionPanel } from '../../../conversion/presentation/components/ConversionPanel';
import { BalanceCard } from './BalanceCard';
import { WalletConfigForm } from './WalletConfigForm';

interface Props {
  wallet: WalletConfig;
  onReconfigure: (input: ConfigureWalletInput) => Promise<any>;
}

export const WalletInfo: React.FC<Props> = ({ wallet, onReconfigure }) => {
  const [showReconfigure, setShowReconfigure] = useState(false);

  // Le username pour la Lightning Address de notre wallet
  const username = wallet.lightningAddress.split('@')[0];
  const ourLightningAddress = `${username}@localhost:8080`;

  return (
    <div className="space-y-6">
      {/* Wallet Card */}
      <div className="bg-gray-900 rounded-2xl p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-bold">Mon Wallet</h2>
          <button
            onClick={() => setShowReconfigure(!showReconfigure)}
            className="text-xs text-gray-400 hover:text-yellow-400 transition"
          >
            ️ Modifier
          </button>
        </div>

        <div className="space-y-3">
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Flash Address</span>
            <span className="text-yellow-400 text-sm font-mono">
              {wallet.lightningAddress}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Lightning Address</span>
            <span className="text-green-400 text-sm font-mono">
              {ourLightningAddress}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">MTN MoMo</span>
            <span className="text-white text-sm">{wallet.momoNumber}</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Auto-convert</span>
            <span className="text-green-400 text-sm font-bold">
              {Math.round(wallet.convertRatio * 100)}%
            </span>
          </div>
        </div>
      </div>

      {/* Reconfiguration */}
      {showReconfigure && (
        <WalletConfigForm
          onConfigure={async (input) => {
            await onReconfigure(input);
            setShowReconfigure(false);
          }}
        />
      )}

      {/* Balance */}
      <BalanceCard />

      {/* Panel conversion */}
      <ConversionPanel wallet={wallet} />
    </div>
  );
};
