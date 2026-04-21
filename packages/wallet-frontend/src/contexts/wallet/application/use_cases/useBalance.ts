import { useState, useEffect, useCallback, useRef } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';
import { Balance } from '../../domain/entities/Balance';

export const useBalance = (lightningAddress?: string) => {
  const [balance, setBalance] = useState<Balance | null>(null);
  const [loading, setLoading] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const addressRef = useRef(lightningAddress);

  const fetchBalance = useCallback(async () => {
    setLoading(true);
    try {
      const result = await walletApi.getBalance(addressRef.current);
      setBalance(result);
    } catch {
      setBalance(null);
    } finally {
      setLoading(false);
    }
  }, []); // pas de dépendance → stable

  useEffect(() => {
    addressRef.current = lightningAddress;
    fetchBalance();

    const wsBase = process.env.REACT_APP_API_URL
      ?.replace('http://', '')
      .replace('https://', '') || 'localhost:8080';

    const wsUrl = lightningAddress
      ? `ws://${wsBase}/ws/balance?lightning_address=${encodeURIComponent(lightningAddress)}`
      : `ws://${wsBase}/ws/balance`;

    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.balance_sats !== undefined) {
          setBalance({
            momoNumber: data.momo_number,
            balanceSats: data.balance_sats,
            balanceBtc: data.balance_btc,
          });
        }
      } catch {}
    };

    return () => {
      ws.close();
    };
  }, [lightningAddress]); // seulement quand lightningAddress change

  return { balance, loading, fetchBalance };
};
