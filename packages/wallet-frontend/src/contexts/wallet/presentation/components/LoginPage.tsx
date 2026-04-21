import React, { useState } from 'react';
import { Zap, LogIn, AlertCircle, Loader2 } from 'lucide-react';
import { walletApi } from '../../infrastructure/api/walletApi';

interface Props {
  onLogin: (lightningAddress: string) => void;
}

export const LoginPage: React.FC<Props> = ({ onLogin }) => {
  const [address, setAddress] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const parts = address.split('@');
    if (parts.length !== 2 || !parts[0] || !parts[1]) {
      setError('Format invalide. Exemple: toi@bitcoinflash.xyz');
      return;
    }

    setError(null);
    setLoading(true);

    try {
      // Vérifie que le wallet existe avec cette adresse EXACTE
      await walletApi.get(address);
      onLogin(address);
    } catch {
      setError(`Aucun wallet trouvé pour ${address}. Vérifiez votre adresse ou créez un wallet.`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center px-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <div className="flex items-center justify-center gap-2 mb-4">
            <Zap size={40} className="text-yellow-400" />
            <h1 className="text-3xl font-bold">Flash Wallet</h1>
          </div>
          <p className="text-gray-400">
            Entrez votre Lightning Address Flash pour accéder à votre wallet
          </p>
        </div>

        <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl p-6 space-y-4">
          <div className="space-y-1">
            <label className="text-sm text-gray-400 flex items-center gap-2">
              <Zap size={14} className="text-yellow-400" />
              Lightning Address Flash
            </label>
            <input
              type="text"
              value={address}
              onChange={(e) => { setAddress(e.target.value); setError(null); }}
              placeholder="toi@bitcoinflash.xyz"
              className="w-full bg-gray-800 rounded-lg px-4 py-3 text-white placeholder-gray-600 outline-none focus:ring-2 focus:ring-yellow-400"
              required
            />
          </div>

          {error && (
            <div className="bg-red-900/30 border border-red-500 rounded-lg px-4 py-3 text-red-400 text-sm flex items-start gap-2">
              <AlertCircle size={16} className="mt-0.5 shrink-0" />
              <span>{error}</span>
            </div>
          )}

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-yellow-400 text-gray-950 font-bold py-3 rounded-lg hover:bg-yellow-300 transition disabled:opacity-50 flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <Loader2 size={18} className="animate-spin" />
                Vérification...
              </>
            ) : (
              <>
                <LogIn size={18} />
                Accéder à mon wallet
              </>
            )}
          </button>

          <p className="text-center text-gray-500 text-sm">
            Première fois ?{' '}
            <button
              type="button"
              onClick={() => onLogin('')}
              className="text-yellow-400 hover:underline"
            >
              Créer un wallet
            </button>
          </p>
        </form>
      </div>
    </div>
  );
};
