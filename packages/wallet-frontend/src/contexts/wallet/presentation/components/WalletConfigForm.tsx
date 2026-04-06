import React, { useState } from 'react';
import { Zap, Smartphone, Sliders, Loader2, AlertCircle } from 'lucide-react';
import { ConfigureWalletInput } from '../../domain/entities/WalletConfig';

interface Props {
  onConfigure: (input: ConfigureWalletInput) => Promise<any>;
  error?: string | null;
  configuring?: boolean;
}

export const WalletConfigForm: React.FC<Props> = ({ onConfigure, error, configuring }) => {
  const [form, setForm] = useState({
    lightningAddress: '',
    momoNumber: '',
    convertRatio: 1.0,
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await onConfigure(form);
  };

  return (
    <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl p-6 space-y-5">
      <h3 className="font-bold text-lg flex items-center gap-2">
        <Zap size={20} className="text-yellow-400" />
        Configurer mon wallet
      </h3>

      <div className="space-y-1">
        <label className="text-sm text-gray-400 flex items-center gap-2">
          <Zap size={14} className="text-yellow-400" />
          Flash Lightning Address
        </label>
        <input
          type="text"
          value={form.lightningAddress}
          onChange={(e) => setForm({ ...form, lightningAddress: e.target.value })}
          placeholder="toi@bitcoinflash.xyz"
          className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
          required
        />
      </div>

      <div className="space-y-1">
        <label className="text-sm text-gray-400 flex items-center gap-2">
          <Smartphone size={14} />
          Numéro MTN MoMo
        </label>
        <input
          type="text"
          value={form.momoNumber}
          onChange={(e) => setForm({ ...form, momoNumber: e.target.value })}
          placeholder="+2290197245435"
          className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
          required
        />
      </div>

      <div className="space-y-2">
        <label className="text-sm text-gray-400 flex items-center gap-2">
          <Sliders size={14} />
          Ratio auto-convert —
          <span className="text-yellow-400 font-bold">
            {Math.round(form.convertRatio * 100)}%
          </span>
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
        <div className="flex justify-between text-xs text-gray-500">
          <span>10%</span>
          <span>50%</span>
          <span>100%</span>
        </div>
        <div className="bg-gray-800 rounded-lg px-4 py-3 text-sm text-gray-400">
          {Math.round(form.convertRatio * 100)}% de chaque paiement reçu sera
          automatiquement converti en XOF sur votre MoMo.
          {form.convertRatio < 1 && (
            <span className="text-yellow-400">
              {' '}Les {Math.round((1 - form.convertRatio) * 100)}% restants seront gardés en sats.
            </span>
          )}
        </div>
      </div>

      {error && (
        <div className="bg-red-900/30 border border-red-500 rounded-lg px-4 py-3 text-red-400 text-sm flex items-start gap-2">
          <AlertCircle size={16} className="mt-0.5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      <button
        type="submit"
        disabled={configuring}
        className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50 flex items-center justify-center gap-2"
      >
        {configuring ? (
          <>
            <Loader2 size={18} className="animate-spin" />
            Configuration...
          </>
        ) : (
          <>
            <Zap size={18} />
            Configurer mon wallet
          </>
        )}
      </button>
    </form>
  );
};
