import React, { useState } from 'react';
import { Settings, Zap, Smartphone, RefreshCw, CheckCircle } from 'lucide-react';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';
import { ConversionPanel } from '../../../conversion/presentation/components/ConversionPanel';
import { BalanceCard } from './BalanceCard';
import { WalletConfigForm } from './WalletConfigForm';
import { useBalance } from '../../application/use_cases/useBalance';

interface Props {
  wallet: WalletConfig;
  onReconfigure: (input: ConfigureWalletInput) => Promise<any>;
  reconfigureError?: string | null;
  configuring?: boolean;
  onLogout?: () => void;
}

export const WalletInfo: React.FC<Props> = ({ wallet, onReconfigure, reconfigureError, configuring, onLogout }) => {
  const [showReconfigure, setShowReconfigure] = useState(false);
  const { balance, loading, fetchBalance } = useBalance(wallet.lightningAddress);

  const username = wallet.lightningAddress.split('@')[0];
  const ourLightningAddress = `${username}@localhost:8080`;

  const handleReconfigure = async (input: ConfigureWalletInput) => {
    const result = await onReconfigure(input);
    // Ferme le formulaire SEULEMENT si la config a réussi
    if (result) {
      setShowReconfigure(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="bg-gray-900 rounded-2xl p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-bold">Mon Wallet</h2>
          <div className="flex items-center gap-3">
            <button
              onClick={() => setShowReconfigure(!showReconfigure)}
              className="text-gray-400 hover:text-yellow-400 transition"
            >
              <Settings size={16} />
            </button>
            {onLogout && (
              <button
                onClick={onLogout}
                className="text-gray-400 hover:text-red-400 transition text-xs"
              >
                Déconnexion
              </button>
            )}
          </div>
        </div>

        <div className="space-y-3">
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm flex items-center gap-2">
              <Zap size={14} className="text-yellow-400" />
              Flash Address
            </span>
            <span className="text-yellow-400 text-sm font-mono">{wallet.lightningAddress}</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm flex items-center gap-2">
              <Zap size={14} className="text-green-400" />
              Lightning Address
            </span>
            <span className="text-green-400 text-sm font-mono">{ourLightningAddress}</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm flex items-center gap-2">
              <Smartphone size={14} />
              MTN MoMo
            </span>
            <span className="text-white text-sm">{wallet.momoNumber}</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm flex items-center gap-2">
              <RefreshCw size={14} />
              Auto-convert
            </span>
            <span className="text-green-400 text-sm font-bold flex items-center gap-1">
              <CheckCircle size={14} />
              {Math.round(wallet.convertRatio * 100)}%
            </span>
          </div>
        </div>
      </div>

      {showReconfigure && (
        <WalletConfigForm
          onConfigure={handleReconfigure}
          error={reconfigureError}
        />
      )}

      <BalanceCard balance={balance} loading={loading} onRefresh={fetchBalance} />
      <ConversionPanel wallet={wallet} balance={balance} onConvertSuccess={fetchBalance} />
    </div>
  );
};
