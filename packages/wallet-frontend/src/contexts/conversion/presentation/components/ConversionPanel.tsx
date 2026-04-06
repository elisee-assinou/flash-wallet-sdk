import React, { useState } from 'react';
import { WalletConfig } from '../../../wallet/domain/entities/WalletConfig';
import { SellForm } from './SellForm';
import { BuyForm } from './BuyForm';
import { TransactionList } from './TransactionList';

interface Props {
  wallet: WalletConfig;
}

export const ConversionPanel: React.FC<Props> = ({ wallet }) => {
  const [tab, setTab] = useState<'sell' | 'buy' | 'history'>('sell');

  return (
    <div className="space-y-4">
      {/* Tabs */}
      <div className="flex bg-gray-900 rounded-xl p-1">
        {(['sell', 'buy', 'history'] as const).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`flex-1 py-2 rounded-lg text-sm font-medium transition ${
              tab === t
                ? 'bg-yellow-400 text-gray-950'
                : 'text-gray-400 hover:text-white'
            }`}
          >
            {t === 'sell' ? '️ Vendre' : t === 'buy' ? '️ Acheter' : '📋 History'}
          </button>
        ))}
      </div>

      {tab === 'sell' && <SellForm wallet={wallet} />}
      {tab === 'buy' && <BuyForm wallet={wallet} />}
      {tab === 'history' && <TransactionList />}
    </div>
  );
};
