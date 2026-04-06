import React, { useState } from 'react';
import { useBuyBitcoin } from '../../application/use_cases/useBuyBitcoin';
import { WalletConfig } from '../../../wallet/domain/entities/WalletConfig';

interface Props {
  wallet: WalletConfig;
}

export const BuyForm: React.FC<Props> = ({ wallet }) => {
  const [amountXof, setAmountXof] = useState('');
  const { execute, loading, error, transaction } = useBuyBitcoin();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await execute({
      amountXof: parseInt(amountXof),
      momoNumber: wallet.momoNumber,
      lightningAddress: wallet.lightningAddress,
    });
  };

  return (
    <div className="bg-gray-900 rounded-2xl p-6 space-y-4">
      <h3 className="font-bold text-lg">Acheter des sats avec MoMo</h3>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm text-gray-400 mb-1">Montant en XOF</label>
          <input
            type="number"
            value={amountXof}
            onChange={(e) => setAmountXof(e.target.value)}
            placeholder="5000"
            className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
            required
          />
        </div>

        <div className="flex justify-between text-sm text-gray-500">
          <span>Lightning Address</span>
          <span className="text-yellow-400 font-mono text-xs">{wallet.lightningAddress}</span>
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50"
        >
          {loading ? 'Achat en cours...' : 'Acheter des sats ⚡'}
        </button>
      </form>

      {error && (
        <div className="bg-red-900/30 border border-red-500 rounded-lg px-4 py-3 text-red-400 text-sm">
          {error}
        </div>
      )}

      {transaction && (
        <div className="bg-gray-800 rounded-xl p-4 space-y-2">
          <div className="flex justify-between">
            <span className="text-gray-400 text-sm">Transaction</span>
            <span className="text-white text-xs font-mono">{transaction.id.slice(0, 8)}...</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400 text-sm">Statut</span>
            <span className="text-yellow-400 text-sm font-bold">⏳ {transaction.status}</span>
          </div>
          <p className="text-gray-500 text-xs text-center">
            Paiement MoMo en attente de confirmation
          </p>
        </div>
      )}
    </div>
  );
};
