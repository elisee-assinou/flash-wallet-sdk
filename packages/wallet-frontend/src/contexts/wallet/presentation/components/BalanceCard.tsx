import React from 'react';
import { RefreshCw, Zap } from 'lucide-react';
import { Balance } from '../../domain/entities/Balance';

interface Props {
  balance: Balance | null;
  loading: boolean;
  onRefresh: () => void;
}

export const BalanceCard: React.FC<Props> = ({ balance, loading, onRefresh }) => {
  return (
    <div className="bg-gray-900 rounded-2xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-bold text-lg flex items-center gap-2">
          <Zap size={18} className="text-yellow-400" />
          Solde en sats
        </h3>
        <button
          onClick={onRefresh}
          className="text-gray-400 hover:text-yellow-400 transition"
        >
          <RefreshCw size={14} />
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
            <span className="text-white text-sm">{balance.balanceBtc.toFixed(8)} BTC</span>
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
