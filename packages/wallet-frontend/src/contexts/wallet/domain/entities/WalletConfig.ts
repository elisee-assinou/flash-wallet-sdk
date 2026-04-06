export interface WalletConfig {
  walletId: string;
  lightningAddress: string;
  momoNumber: string;
  convertRatio: number;
}

export interface ConfigureWalletInput {
  lightningAddress: string;
  momoNumber: string;
  convertRatio: number;
}
