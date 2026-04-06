import axios from 'axios';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';
import { Balance } from '../../domain/entities/Balance';

const BASE_URL = 'http://localhost:8080/api/v1';

const mapWallet = (data: any): WalletConfig => ({
  walletId: data.wallet_id,
  lightningAddress: data.lightning_address,
  momoNumber: data.momo_number,
  convertRatio: data.convert_ratio,
});

const mapBalance = (data: any): Balance => ({
  momoNumber: data.momo_number,
  balanceSats: data.balance_sats,
  balanceBtc: data.balance_btc,
});

export const walletApi = {
  configure: async (input: ConfigureWalletInput): Promise<WalletConfig> => {
    const { data } = await axios.post(`${BASE_URL}/wallet/configure`, {
      lightning_address: input.lightningAddress,
      momo_number: input.momoNumber,
      convert_ratio: input.convertRatio,
    });
    return mapWallet(data);
  },

  get: async (): Promise<WalletConfig> => {
    const { data } = await axios.get(`${BASE_URL}/wallet`);
    return mapWallet(data);
  },
  getBalance: async (): Promise<Balance> => {
    const { data } = await axios.get(`${BASE_URL}/wallet/balance`);
    return mapBalance(data);
  },

  convertBalance: async (ratio: number): Promise<any> => {
    const { data } = await axios.post(`${BASE_URL}/wallet/balance/convert`, {
      ratio,
    });
    return {
      satsConverted: data.sats_converted,
      amountXof: data.amount_xof,
      newBalanceSats: data.new_balance_sats,
      message: data.message,
    };
  },
};