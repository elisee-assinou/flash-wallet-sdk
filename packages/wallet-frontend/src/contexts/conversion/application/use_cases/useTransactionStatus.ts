import { useState, useEffect } from 'react';
import { TransactionStatusResult } from '../../domain/entities/Transaction';

export const useTransactionStatus = (transactionId: string | null) => {
  const [status, setStatus] = useState<TransactionStatusResult | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    if (!transactionId) return;

    const ws = new WebSocket(`ws://localhost:8080/ws/transactions/${transactionId}`);

    ws.onopen = () => setConnected(true);

    ws.onmessage = (event) => {
      const data: TransactionStatusResult = JSON.parse(event.data);
      setStatus(data);
      if (data.isCompleted) {
        ws.close();
        setConnected(false);
      }
    };

    ws.onerror = () => setConnected(false);
    ws.onclose = () => setConnected(false);

    return () => ws.close();
  }, [transactionId]);

  return { status, connected };
};
