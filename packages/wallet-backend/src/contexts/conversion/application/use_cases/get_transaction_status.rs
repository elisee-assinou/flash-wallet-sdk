use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    ports::flash_gateway::FlashGateway,
    repositories::transaction_repository::TransactionRepository,
};

pub struct GetTransactionStatusInput {
    pub transaction_id: String,
}

pub struct GetTransactionStatusOutput {
    pub transaction_id: String,
    pub status: String,
    pub is_completed: bool,
}

pub struct GetTransactionStatusUseCase {
    flash_gateway: Arc<dyn FlashGateway>,
    transaction_repo: Arc<dyn TransactionRepository>,
}

impl GetTransactionStatusUseCase {
    pub fn new(
        flash_gateway: Arc<dyn FlashGateway>,
        transaction_repo: Arc<dyn TransactionRepository>,
    ) -> Self {
        Self { flash_gateway, transaction_repo }
    }

    pub async fn execute(
        &self,
        input: GetTransactionStatusInput,
    ) -> Result<GetTransactionStatusOutput, DomainError> {
        // 1. Récupère la transaction depuis notre DB
        let mut transaction = self.transaction_repo
            .find_by_id(&input.transaction_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(
                format!("Transaction {} not found", input.transaction_id)
            ))?;

        // 2. Si déjà COMPLETED en DB → retourne directement
        if transaction.is_completed() {
            return Ok(GetTransactionStatusOutput {
                transaction_id: input.transaction_id,
                status: "COMPLETED".to_string(),
                is_completed: true,
            });
        }

        // 3. Interroge Flash API avec le Flash ID
        let flash_id = transaction
            .flash_transaction_id()
            .ok_or_else(|| DomainError::NotFound(
                "Flash transaction ID not found".to_string()
            ))?
            .to_string();

        let status = self.flash_gateway
            .get_transaction_status(&flash_id)
            .await?;

        // 4. Si Flash dit COMPLETED → met à jour notre DB
        if status == "COMPLETED" {
            transaction.complete();
            self.transaction_repo.save(&transaction).await?;
            tracing::info!("Transaction {} marked as COMPLETED", input.transaction_id);
        }

        let is_completed = status == "COMPLETED";

        Ok(GetTransactionStatusOutput {
            transaction_id: input.transaction_id,
            status,
            is_completed,
        })
    }
}
