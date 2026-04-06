export type TransactionStatus = 'PENDING' | 'COMPLETED' | 'FAILED';
export type TransactionType = 'SELL_BITCOIN' | 'BUY_BITCOIN';

export interface Transaction {
  id: string;
  amountSats: number;
  amountXof: string;
  status: TransactionStatus;
  message: string;
  invoice?: string;
}

export interface TransactionStatusResult {
  transactionId: string;
  status: TransactionStatus;
  isCompleted: boolean;
}

export interface AutoConvertInput {
  amountSats: number;
  momoNumber: string;
  convertRatio: number;
}

export interface BuyBitcoinInput {
  amountXof: number;
  momoNumber: string;
  lightningAddress: string;
}
