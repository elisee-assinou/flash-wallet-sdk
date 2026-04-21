import { useState } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';

export interface ConvertBalanceOutput {
  satsConverted: number;
  amountXof: number;
  newBalanceSats: number;
  message: string;
}

export const useConvertBalance = (lightningAddress?: string) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<ConvertBalanceOutput | null>(null);

  const execute = async (ratio: number) => {
    setLoading(true);
    setError(null);
    try {
      const output = await walletApi.convertBalance(ratio, lightningAddress);
      setResult(output);
      return output;
    } catch (e: any) {
      const msg = e.response?.data?.error
        || e.response?.data
        || e.message
        || 'Erreur lors de la conversion';
      setError(msg);
      return null;
    } finally {
      setLoading(false);
    }
  };

  return { execute, loading, error, result };
};
