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

    pub async fn execute(&self, lightning_address: Option<&str>) -> Result<Option<GetWalletOutput>, DomainError> {
        let config = match lightning_address {
            Some(address) => {
                // Cherche par adresse EXACTE — pas juste le username
                let all = self.wallet_repo.find_all().await?;
                all.into_iter().find(|w| w.lightning_address() == address)
            }
            None => self.wallet_repo.find().await?,
        };

        Ok(config.map(|c| GetWalletOutput {
            wallet_id: c.id().value(),
            lightning_address: c.lightning_address().to_string(),
            momo_number: c.momo_number().value().to_string(),
            convert_ratio: c.convert_ratio(),
        }))
    }
}
