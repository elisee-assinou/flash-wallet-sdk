import React, { useEffect, useState } from 'react';
import { transactionApi } from '../../infrastructure/api/transactionApi';
import { Transaction } from '../../domain/entities/Transaction';

export const TransactionList: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    transactionApi.listAll()
      .then((res) => setTransactions(res.transactions))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="text-center text-gray-400 py-8">Chargement...</div>;

  if (transactions.length === 0) {
    return (
      <div className="bg-gray-900 rounded-2xl p-8 text-center text-gray-500">
        Aucune transaction pour l'instant.
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {transactions.map((tx) => (
        <div key={tx.id} className="bg-gray-900 rounded-xl p-4 flex justify-between items-center">
          <div>
            <div className="text-white text-sm font-mono">{tx.id.slice(0, 8)}...</div>
            <div className="text-gray-400 text-xs">{tx.amountSats} sats</div>
          </div>
          <div className="text-right">
            <div className="text-yellow-400 font-bold">{tx.amountXof}</div>
            <div className={`text-xs font-bold ${
              tx.status === 'COMPLETED' ? 'text-green-400' :
              tx.status === 'FAILED' ? 'text-red-400' : 'text-yellow-400'
            }`}>
              {tx.status === 'COMPLETED' ? '✅' : tx.status === 'FAILED' ? '❌' : '⏳'} {tx.status}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
