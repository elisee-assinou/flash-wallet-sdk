use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::repositories::transaction_repository::TransactionRepository;

pub struct TransactionSummary {
    pub id: String,
    pub amount_xof: u64,
    pub amount_sats: u64,
    pub status: String,
}

pub struct ListTransactionsOutput {
    pub transactions: Vec<TransactionSummary>,
}

pub struct ListTransactionsUseCase {
    transaction_repo: Arc<dyn TransactionRepository>,
}

impl ListTransactionsUseCase {
    pub fn new(transaction_repo: Arc<dyn TransactionRepository>) -> Self {
        Self { transaction_repo }
    }

    pub async fn execute(&self) -> Result<ListTransactionsOutput, DomainError> {
        let transactions = self.transaction_repo.find_all().await?;

        let summaries = transactions.iter().map(|t| TransactionSummary {
            id: t.id().value(),
            amount_xof: t.amount_xof().amount(),
            amount_sats: t.amount_sats().amount(),
            status: format!("{:?}", t.status()),
        }).collect();

        Ok(ListTransactionsOutput { transactions: summaries })
    }
}
