import { useState, useEffect } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';

export const useWallet = () => {
  const [wallet, setWallet] = useState<WalletConfig | null>(null);
  const [initialLoading, setInitialLoading] = useState(true);
  const [configuring, setConfiguring] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchWallet = async () => {
    try {
      const result = await walletApi.get();
      setWallet(result);
    } catch {
      setWallet(null);
    } finally {
      setInitialLoading(false);
    }
  };

  const configure = async (input: ConfigureWalletInput) => {
    setConfiguring(true);
    setError(null);
    try {
      const result = await walletApi.configure(input);
      setWallet(result);
      return result;
    } catch (e: any) {
      setError(e.response?.data || 'Erreur de configuration');
      return null;
    } finally {
      setConfiguring(false);
    }
  };

  useEffect(() => {
    fetchWallet();
  }, []);

  return {
    wallet,
    loading: initialLoading,  // seulement pour le chargement initial
    configuring,              // pour le bouton "Configuration..."
    error,
    configure,
    fetchWallet,
  };
};
