use async_trait::async_trait;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::entities::balance::Balance;

#[async_trait]
pub trait BalanceRepository: Send + Sync {
    async fn find_by_momo(&self, momo_number: &str) -> Result<Option<Balance>, DomainError>;
    async fn save(&self, balance: &Balance) -> Result<(), DomainError>;
    async fn credit(&self, momo_number: &str, sats: i64) -> Result<Balance, DomainError>;
}
