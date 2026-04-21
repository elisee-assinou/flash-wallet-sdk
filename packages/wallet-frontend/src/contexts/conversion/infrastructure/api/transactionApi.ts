import axios from 'axios';
import { AutoConvertInput, BuyBitcoinInput, Transaction, TransactionStatusResult } from '../../domain/entities/Transaction';

const BASE_URL = `${process.env.REACT_APP_API_URL || 'http://localhost:8080'}/api/v1`;

const mapTransaction = (data: any): Transaction => ({
  id: data.id,
  amountSats: data.amount_sats,
  amountXof: data.amount_xof,
  status: data.status?.toUpperCase() as any,
  message: data.message,
});

export const transactionApi = {
  createSell: async (input: AutoConvertInput): Promise<Transaction> => {
    const { data } = await axios.post(`${BASE_URL}/transactions/convert`, {
      amount_sats: input.amountSats,
      momo_number: input.momoNumber,
      convert_ratio: input.convertRatio,
    });
    return mapTransaction(data);
  },

  createBuy: async (input: BuyBitcoinInput): Promise<Transaction> => {
    const { data } = await axios.post(`${BASE_URL}/transactions/buy`, {
      amount_xof: input.amountXof,
      momo_number: input.momoNumber,
      lightning_address: input.lightningAddress,
    });
    return mapTransaction(data);
  },

  getStatus: async (id: string): Promise<TransactionStatusResult> => {
    const { data } = await axios.get(`${BASE_URL}/transactions/${id}/status`);
    return {
      transactionId: data.transaction_id,
      status: data.status,
      isCompleted: data.is_completed,
    };
  },

  listAll: async (): Promise<{ transactions: Transaction[]; total: number }> => {
    const { data } = await axios.get(`${BASE_URL}/transactions`);
    return {
      transactions: data.transactions.map(mapTransaction),
      total: data.total,
    };
  },
};
