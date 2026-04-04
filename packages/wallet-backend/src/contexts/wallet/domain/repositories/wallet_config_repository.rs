use async_trait::async_trait;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::entities::wallet_config::WalletConfig;

#[async_trait]
pub trait WalletConfigRepository: Send + Sync {
    async fn save(&self, config: &WalletConfig) -> Result<(), DomainError>;
    async fn find(&self) -> Result<Option<WalletConfig>, DomainError>;
}
