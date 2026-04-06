import React, { useState } from 'react';
import { Zap, Smartphone, Loader2, CheckCircle, Clock, AlertCircle } from 'lucide-react';
import { useConvertBalance } from '../../../wallet/application/use_cases/useConvertBalance';
import { Balance } from '../../../wallet/domain/entities/Balance';
import { WalletConfig } from '../../../wallet/domain/entities/WalletConfig';

interface Props {
  wallet: WalletConfig;
  balance: Balance | null;
  onConvertSuccess?: () => void;
}

export const SellForm: React.FC<Props> = ({ wallet, balance, onConvertSuccess }) => {
  const { execute, loading, error, result } = useConvertBalance();
  const [ratio, setRatio] = useState(100);

  const availableSats = balance?.balanceSats ?? 0;
  const satsToConvert = Math.floor(availableSats * ratio / 100);
  const satsRemaining = availableSats - satsToConvert;

  const handleConvert = async () => {
    if (availableSats === 0 || satsToConvert === 0) return;
    const output = await execute(ratio / 100);
    if (output && onConvertSuccess) {
      onConvertSuccess(); // rafraîchit la balance
    }
  };

  return (
    <div className="bg-gray-900 rounded-2xl p-6 space-y-5">
      <h3 className="font-bold text-lg flex items-center gap-2">
        <Zap size={20} className="text-yellow-400" />
        Convertir mes sats en XOF
      </h3>

      <div className="bg-gray-800 rounded-xl p-4">
        <p className="text-gray-400 text-sm mb-1">Solde disponible</p>
        <p className="text-yellow-400 font-bold text-2xl">
          {availableSats.toLocaleString()} sats
        </p>
      </div>

      <div className="space-y-3">
        <div className="flex justify-between items-center">
          <label className="text-sm text-gray-400">Montant à convertir</label>
          <span className="text-yellow-400 font-bold">{ratio}%</span>
        </div>
        <input
          type="range"
          min="10"
          max="100"
          step="10"
          value={ratio}
          onChange={(e) => setRatio(parseInt(e.target.value))}
          className="w-full accent-yellow-400"
        />
        <div className="flex justify-between text-xs text-gray-500">
          <span>10%</span>
          <span>50%</span>
          <span>100%</span>
        </div>

        <div className="bg-gray-800 rounded-xl p-4 space-y-2">
          <div className="flex justify-between text-sm">
            <span className="text-gray-400">Sats à convertir</span>
            <span className="text-white font-bold">{satsToConvert.toLocaleString()} sats</span>
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-gray-400">Sats restants</span>
            <span className="text-gray-400">{satsRemaining.toLocaleString()} sats</span>
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-gray-400 flex items-center gap-1">
              <Smartphone size={12} />
              MoMo
            </span>
            <span className="text-white">{wallet.momoNumber}</span>
          </div>
        </div>
      </div>

      <button
        onClick={handleConvert}
        disabled={loading || availableSats === 0 || satsToConvert === 0}
        className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50 flex items-center justify-center gap-2"
      >
        {loading ? (
          <>
            <Loader2 size={18} className="animate-spin" />
            Conversion en cours...
          </>
        ) : (
          <>
            <Zap size={18} />
            Convertir {satsToConvert.toLocaleString()} sats en XOF
          </>
        )}
      </button>

      {error && (
        <div className="bg-red-900/30 border border-red-500 rounded-lg px-4 py-3 text-red-400 text-sm flex items-start gap-2">
          <AlertCircle size={16} className="mt-0.5 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {result && (
        <div className="bg-gray-800 rounded-xl p-4 space-y-3">
          <div className="flex justify-between">
            <span className="text-gray-400 text-sm">Sats convertis</span>
            <span className="text-white font-bold">{result.satsConverted.toLocaleString()} sats</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400 text-sm">Montant XOF</span>
            <span className="text-yellow-400 font-bold">{result.amountXof.toLocaleString()} XOF</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-gray-400 text-sm">Statut</span>
            <span className="text-yellow-400 text-sm font-bold flex items-center gap-1">
              <Clock size={14} /> En attente Flash...
            </span>
          </div>
          <div className="bg-green-900/30 border border-green-500 rounded-lg px-4 py-3 text-green-400 text-sm text-center font-bold flex items-center justify-center gap-2">
            <CheckCircle size={16} />
            Conversion initiée — XOF en route sur votre MoMo
          </div>
        </div>
      )}
    </div>
  );
};
