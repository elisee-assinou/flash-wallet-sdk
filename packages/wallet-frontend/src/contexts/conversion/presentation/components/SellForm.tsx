import React, { useState } from 'react';
import { useAutoConvert } from '../../application/use_cases/useAutoConvert';
import { useTransactionStatus } from '../../application/use_cases/useTransactionStatus';
import { WalletConfig } from '../../../wallet/domain/entities/WalletConfig';

interface Props {
  wallet: WalletConfig;
}

export const SellForm: React.FC<Props> = ({ wallet }) => {
  const [amountSats, setAmountSats] = useState('');
  const { execute, loading, error, transaction } = useAutoConvert();
  const { status, connected } = useTransactionStatus(transaction?.id || null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await execute({
      amountSats: parseInt(amountSats),
      momoNumber: wallet.momoNumber,
      convertRatio: wallet.convertRatio,
    });
  };

  const copyInvoice = () => {
    if (transaction?.invoice) {
      navigator.clipboard.writeText(transaction.invoice);
      alert('Invoice copied!');
    }
  };

  return (
      <div className="bg-gray-900 rounded-2xl p-6 space-y-4">
        <h3 className="font-bold text-lg">Vendre des sats → XOF sur MoMo</h3>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm text-gray-400 mb-1">Montant en satoshis</label>
            <input
                type="number"
                value={amountSats}
                onChange={(e) => setAmountSats(e.target.value)}
                placeholder="239999"
                className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
                required
            />
          </div>

          <div className="flex justify-between text-sm text-gray-500">
            <span>MoMo destinataire</span>
            <span className="text-white">{wallet.momoNumber}</span>
          </div>

          <button
              type="submit"
              disabled={loading}
              className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50"
          >
            {loading ? 'Conversion en cours...' : 'Convertir en XOF ⚡'}
          </button>
        </form>

        {error && (
            <div className="bg-red-900/30 border border-red-500 rounded-lg px-4 py-3 text-red-400 text-sm">
              {error}
            </div>
        )}

        {transaction && (
            <div className="bg-gray-800 rounded-xl p-4 space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-400 text-sm">Montant XOF</span>
                <span className="text-yellow-400 font-bold">{transaction.amountXof}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400 text-sm">Statut</span>
                <span className={`text-sm font-bold ${
                    status?.isCompleted ? 'text-green-400' : 'text-yellow-400'
                }`}>
              {status?.isCompleted ? ' COMPLETED' : ' ' + (status?.status || 'PENDING')}
            </span>
              </div>

              {connected && (
                  <div className="text-xs text-gray-500 text-center">
                    🔴 Live
                  </div>
              )}

              {/* Invoice Lightning */}
              {transaction.invoice && !status?.isCompleted && (
                  <div className="border border-yellow-400/30 rounded-lg p-3 space-y-2">
                    <p className="text-xs text-yellow-400 font-bold">⚡ Payez cette invoice Lightning</p>
                    <p className="text-xs text-gray-500">
                      Copiez l'invoice dans votre wallet Lightning (Phoenix, Alby, Zeus...)
                    </p>
                    <div className="bg-gray-900 rounded p-2">
                      <p className="text-xs text-gray-400 font-mono break-all">
                        {transaction.invoice.slice(0, 40)}...
                      </p>
                    </div>
                    <button
                        onClick={copyInvoice}
                        className="w-full bg-yellow-400/10 border border-yellow-400/30 text-yellow-400 text-sm py-2 rounded-lg hover:bg-yellow-400/20 transition"
                    >
                       Copy the invoice
                    </button>
                  </div>
              )}

              {status?.isCompleted && (
                  <div className="bg-green-900/30 border border-green-500 rounded-lg px-4 py-3 text-green-400 text-sm text-center font-bold">
                    {transaction.amountXof} envoyés sur votre MoMo ✅
                  </div>
              )}
            </div>
        )}
      </div>
  );
};