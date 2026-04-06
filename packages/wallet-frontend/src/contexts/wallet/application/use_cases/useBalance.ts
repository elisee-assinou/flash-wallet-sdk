import { useState, useEffect, useCallback } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';
import { Balance } from '../../domain/entities/Balance';

export const useBalance = () => {
  const [balance, setBalance] = useState<Balance | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchBalance = useCallback(async () => {
    setLoading(true);
    try {
      const result = await walletApi.getBalance();
      setBalance(result);
    } catch {
      setBalance(null);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchBalance();
    const interval = setInterval(fetchBalance, 30000);
    return () => clearInterval(interval);
  }, [fetchBalance]);

  return { balance, loading, fetchBalance };
};
