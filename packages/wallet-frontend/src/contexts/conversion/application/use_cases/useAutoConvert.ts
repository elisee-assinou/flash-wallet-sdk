import { useState } from 'react';
import { transactionApi } from '../../infrastructure/api/transactionApi';
import { AutoConvertInput, Transaction } from '../../domain/entities/Transaction';

export const useAutoConvert = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [transaction, setTransaction] = useState<Transaction | null>(null);

  const execute = async (input: AutoConvertInput) => {
    setLoading(true);
    setError(null);
    try {
      const result = await transactionApi.createSell(input);
      setTransaction(result);
      return result;
    } catch (e: any) {
      setError(e.response?.data || 'Erreur lors de la conversion');
    } finally {
      setLoading(false);
    }
  };

  return { execute, loading, error, transaction };
};
