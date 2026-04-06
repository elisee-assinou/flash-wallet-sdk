import { useState } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';

export interface ConvertBalanceOutput {
  satsConverted: number;
  amountXof: number;
  newBalanceSats: number;
  message: string;
}

export const useConvertBalance = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<ConvertBalanceOutput | null>(null);

  const execute = async (ratio: number) => {
    setLoading(true);
    setError(null);
    try {
      const output = await walletApi.convertBalance(ratio);
      setResult(output);
      return output;
    } catch (e: any) {
      setError(e.response?.data?.error || 'Erreur lors de la conversion');
    } finally {
      setLoading(false);
    }
  };

  return { execute, loading, error, result };
};
