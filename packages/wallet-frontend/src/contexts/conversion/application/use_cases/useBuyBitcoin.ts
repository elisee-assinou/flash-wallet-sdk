import { useState } from 'react';
import { transactionApi } from '../../infrastructure/api/transactionApi';
import { BuyBitcoinInput, Transaction } from '../../domain/entities/Transaction';

export const useBuyBitcoin = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [transaction, setTransaction] = useState<Transaction | null>(null);

  const execute = async (input: BuyBitcoinInput) => {
    setLoading(true);
    setError(null);
    try {
      const result = await transactionApi.createBuy(input);
      setTransaction(result);
      return result;
    } catch (e: any) {
      setError(e.response?.data || 'Erreur lors de l\'achat');
    } finally {
      setLoading(false);
    }
  };

  return { execute, loading, error, transaction };
};
