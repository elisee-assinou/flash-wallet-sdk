use async_trait::async_trait;
use std::sync::Mutex;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    entities::flash_transaction::FlashTransaction,
    repositories::transaction_repository::TransactionRepository,
};

pub struct InMemoryTransactionRepo {
    transactions: Mutex<Vec<FlashTransaction>>,
}

impl InMemoryTransactionRepo {
    pub fn new() -> Self {
        Self { transactions: Mutex::new(vec![]) }
    }
}

#[async_trait]
impl TransactionRepository for InMemoryTransactionRepo {
    async fn save(&self, transaction: &FlashTransaction) -> Result<(), DomainError> {
        let mut txs = self.transactions.lock().unwrap();
        txs.push(transaction.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<FlashTransaction>, DomainError> {
        let txs = self.transactions.lock().unwrap();
        Ok(txs.iter().find(|t| t.id().value() == id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<FlashTransaction>, DomainError> {
        let txs = self.transactions.lock().unwrap();
        Ok(txs.clone())
    }

    async fn find_pending(&self) -> Result<Vec<FlashTransaction>, DomainError> {
        let txs = self.transactions.lock().unwrap();
        Ok(txs.iter().filter(|t| t.is_pending()).cloned().collect())
    }
}
