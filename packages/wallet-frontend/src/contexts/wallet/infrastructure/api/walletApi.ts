import axios from 'axios';
import { WalletConfig, ConfigureWalletInput } from '../../domain/entities/WalletConfig';
import { Balance } from '../../domain/entities/Balance';

const BASE_URL = `${process.env.REACT_APP_API_URL || 'http://localhost:8080'}/api/v1`;

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

  get: async (lightningAddress?: string): Promise<WalletConfig> => {
    const params = lightningAddress ? `?lightning_address=${encodeURIComponent(lightningAddress)}` : '';
    const { data } = await axios.get(`${BASE_URL}/wallet${params}`);
    return mapWallet(data);
  },
  getBalance: async (lightningAddress?: string): Promise<Balance> => {
    const params = lightningAddress ? `?lightning_address=${encodeURIComponent(lightningAddress)}` : '';
    const { data } = await axios.get(`${BASE_URL}/wallet/balance${params}`);
    return mapBalance(data);
  },

  convertBalance: async (ratio: number, lightningAddress?: string): Promise<any> => {
    const { data } = await axios.post(`${BASE_URL}/wallet/balance/convert`, {
      ratio,
      lightning_address: lightningAddress,
    });
    return {
      satsConverted: data.sats_converted,
      amountXof: data.amount_xof,
      newBalanceSats: data.new_balance_sats,
      message: data.message,
    };
  },
};