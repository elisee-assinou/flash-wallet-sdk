use std::sync::Arc;
use crate::contexts::conversion::domain::repositories::transaction_repository::TransactionRepository;

pub struct ExpirePendingUseCase {
    transaction_repo: Arc<dyn TransactionRepository>,
}

impl ExpirePendingUseCase {
    pub fn new(transaction_repo: Arc<dyn TransactionRepository>) -> Self {
        Self { transaction_repo }
    }

    pub async fn run_forever(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; // toutes les heures

            match self.transaction_repo.find_pending().await {
                Ok(pending) => {
                    for mut tx in pending {
                        if tx.is_expired() {
                            tx.mark_as_failed();
                            if let Err(e) = self.transaction_repo.save(&tx).await {
                                tracing::error!("Failed to expire transaction {}: {}", tx.id().value(), e);
                            } else {
                                tracing::info!("Transaction {} expired → FAILED", tx.id().value());
                            }
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to fetch pending transactions: {}", e),
            }
        }
    }
}
