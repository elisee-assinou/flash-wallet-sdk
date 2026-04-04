use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;

pub struct GetWalletOutput {
    pub wallet_id: String,
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}

pub struct GetWalletUseCase {
    wallet_repo: Arc<dyn WalletConfigRepository>,
}

impl GetWalletUseCase {
    pub fn new(wallet_repo: Arc<dyn WalletConfigRepository>) -> Self {
        Self { wallet_repo }
    }

    pub async fn execute(&self) -> Result<GetWalletOutput, DomainError> {
        let config = self.wallet_repo
            .find()
            .await?
            .ok_or_else(|| DomainError::NotFound("Wallet not configured".to_string()))?;

        Ok(GetWalletOutput {
            wallet_id: config.id().value(),
            lightning_address: config.lightning_address().to_string(),
            momo_number: config.momo_number().value().to_string(),
            convert_ratio: config.convert_ratio(),
        })
    }
}
