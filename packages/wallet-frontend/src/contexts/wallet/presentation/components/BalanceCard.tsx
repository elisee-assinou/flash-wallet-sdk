import React from 'react';
import { useBalance } from '../../application/use_cases/useBalance';

export const BalanceCard: React.FC = () => {
  const { balance, loading, fetchBalance } = useBalance();

  return (
    <div className="bg-gray-900 rounded-2xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-bold text-lg"> Solde en sats</h3>
        <button
          onClick={fetchBalance}
          className="text-xs text-gray-400 hover:text-yellow-400 transition"
        >
           Actualiser
        </button>
      </div>

      {loading ? (
        <div className="text-gray-400 text-sm">Chargement...</div>
      ) : balance ? (
        <div className="space-y-3">
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Satoshis</span>
            <span className="text-yellow-400 font-bold text-2xl">
              {balance.balanceSats.toLocaleString()} sats
            </span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Bitcoin</span>
            <span className="text-white text-sm">
              {balance.balanceBtc.toFixed(8)} BTC
            </span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">MoMo</span>
            <span className="text-white text-sm">{balance.momoNumber}</span>
          </div>
        </div>
      ) : (
        <div className="text-gray-500 text-sm">Aucun solde disponible</div>
      )}
    </div>
  );
};
