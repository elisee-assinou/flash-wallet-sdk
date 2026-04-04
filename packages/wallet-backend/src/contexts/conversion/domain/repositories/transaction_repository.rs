use async_trait::async_trait;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::entities::flash_transaction::FlashTransaction;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn save(&self, transaction: &FlashTransaction) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<FlashTransaction>, DomainError>;
    async fn find_pending(&self) -> Result<Vec<FlashTransaction>, DomainError>;
}
