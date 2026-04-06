import { useState, useEffect, useCallback, useRef } from 'react';
import { walletApi } from '../../infrastructure/api/walletApi';
import { Balance } from '../../domain/entities/Balance';

export const useBalance = () => {
  const [balance, setBalance] = useState<Balance | null>(null);
  const [loading, setLoading] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const fetchBalance = useCallback(async () => {
    setLoading(true);
    try {
      const result = await walletApi.getBalance();
      setBalance(result);
    } catch {
      setBalance(null);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    // Fetch initial
    fetchBalance();

    // WebSocket pour les mises à jour en temps réel
    const ws = new WebSocket('ws://localhost:8080/ws/balance');
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

    ws.onerror = () => {
      // Fallback — polling toutes les 10s si WS échoue
      const interval = setInterval(fetchBalance, 10000);
      return () => clearInterval(interval);
    };

    return () => {
      ws.close();
    };
  }, [fetchBalance]);

  return { balance, loading, fetchBalance };
};
