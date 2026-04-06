import React, { useEffect, useState } from 'react';
import { CheckCircle, XCircle, Clock, Zap, ArrowRight, RefreshCw } from 'lucide-react';
import { transactionApi } from '../../infrastructure/api/transactionApi';
import { Transaction } from '../../domain/entities/Transaction';

const StatusIcon: React.FC<{ status: string }> = ({ status }) => {
  if (status === 'COMPLETED') return <CheckCircle size={16} className="text-green-400" />;
  if (status === 'FAILED') return <XCircle size={16} className="text-red-400" />;
  return <Clock size={16} className="text-yellow-400" />;
};

const statusColor = (status: string) => {
  if (status === 'COMPLETED') return 'text-green-400';
  if (status === 'FAILED') return 'text-red-400';
  return 'text-yellow-400';
};

export const TransactionList: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchTransactions = () => {
    setLoading(true);
    transactionApi.listAll()
      .then((res) => setTransactions(res.transactions))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    fetchTransactions();
  }, []);

  if (loading) {
    return (
      <div className="bg-gray-900 rounded-2xl p-8 flex items-center justify-center gap-2 text-gray-400">
        <RefreshCw size={16} className="animate-spin" />
        Chargement...
      </div>
    );
  }

  if (transactions.length === 0) {
    return (
      <div className="bg-gray-900 rounded-2xl p-8 text-center">
        <Zap size={32} className="text-gray-600 mx-auto mb-3" />
        <p className="text-gray-500">Aucune transaction pour l'instant.</p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between px-1">
        <span className="text-gray-400 text-sm">{transactions.length} transaction(s)</span>
        <button
          onClick={fetchTransactions}
          className="text-gray-400 hover:text-yellow-400 transition"
        >
          <RefreshCw size={14} />
        </button>
      </div>

      {transactions.map((tx) => (
        <div key={tx.id} className="bg-gray-900 rounded-xl p-4 space-y-2">
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-2">
              <Zap size={14} className="text-yellow-400" />
              <span className="text-gray-500 text-xs font-mono">{tx.id.slice(0, 8)}...</span>
            </div>
            <div className={`flex items-center gap-1 text-xs font-bold ${statusColor(tx.status)}`}>
              <StatusIcon status={tx.status} />
              {tx.status}
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-sm">
              <span className="text-white font-bold">{tx.amountSats.toLocaleString()} sats</span>
              <ArrowRight size={14} className="text-gray-500" />
              <span className="text-yellow-400 font-bold">{tx.amountXof} XOF</span>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
