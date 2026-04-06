import { useState, useEffect } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';

export const useWallet = () => {
  const [wallet, setWallet] = useState<WalletConfig | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchWallet = async () => {
    setLoading(true);
    try {
      const result = await walletApi.get();
      setWallet(result);
    } catch {
      setWallet(null);
    } finally {
      setLoading(false);
    }
  };

  const configure = async (input: ConfigureWalletInput) => {
    setLoading(true);
    setError(null);
    try {
      const result = await walletApi.configure(input);
      setWallet(result);
      return result;
    } catch (e: any) {
      setError(e.response?.data || 'Erreur de configuration');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchWallet();
  }, []);

  return { wallet, loading, error, configure, fetchWallet };
};
