import React, { useState } from 'react';
import { ConfigureWalletInput } from '../../domain/entities/WalletConfig';

interface Props {
  onConfigure: (input: ConfigureWalletInput) => Promise<any>;
}

export const WalletConfigForm: React.FC<Props> = ({ onConfigure }) => {
  const [form, setForm] = useState({
    lightningAddress: '',
    momoNumber: '',
    convertRatio: 1.0,
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    await onConfigure(form);
    setLoading(false);
  };

  return (
    <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl p-6 space-y-4">
      <div>
        <label className="block text-sm text-gray-400 mb-1">Lightning Address</label>
        <input
          type="text"
          value={form.lightningAddress}
          onChange={(e) => setForm({ ...form, lightningAddress: e.target.value })}
          placeholder="toi@bitcoinflash.xyz"
          className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
          required
        />
      </div>

      <div>
        <label className="block text-sm text-gray-400 mb-1">Numéro MTN MoMo</label>
        <input
          type="text"
          value={form.momoNumber}
          onChange={(e) => setForm({ ...form, momoNumber: e.target.value })}
          placeholder="+2290197245435"
          className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
          required
        />
      </div>

      <div>
        <label className="block text-sm text-gray-400 mb-2">
          Ratio de conversion — {Math.round(form.convertRatio * 100)}%
        </label>
        <input
          type="range"
          min="0.1"
          max="1"
          step="0.1"
          value={form.convertRatio}
          onChange={(e) => setForm({ ...form, convertRatio: parseFloat(e.target.value) })}
          className="w-full accent-yellow-400"
        />
        <div className="flex justify-between text-xs text-gray-500 mt-1">
          <span>10%</span>
          <span>50%</span>
          <span>100%</span>
        </div>
      </div>

      <button
        type="submit"
        disabled={loading}
        className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50"
      >
        {loading ? 'Configuration...' : 'Configurer mon wallet ⚡'}
      </button>
    </form>
  );
};
