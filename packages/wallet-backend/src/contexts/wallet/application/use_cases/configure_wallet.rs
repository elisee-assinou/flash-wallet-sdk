use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::value_objects::momo_number::MomoNumber;
use crate::contexts::wallet::domain::{
    entities::wallet_config::WalletConfig,
    repositories::wallet_config_repository::WalletConfigRepository,
};
use crate::contexts::wallet::application::dtos::configure_wallet_dto::{
    ConfigureWalletInput,
    ConfigureWalletOutput,
};

pub struct ConfigureWalletUseCase {
    wallet_repo: Arc<dyn WalletConfigRepository>,
}

impl ConfigureWalletUseCase {
    pub fn new(wallet_repo: Arc<dyn WalletConfigRepository>) -> Self {
        Self { wallet_repo }
    }

    pub async fn execute(
        &self,
        input: ConfigureWalletInput,
    ) -> Result<ConfigureWalletOutput, DomainError> {
        let momo = MomoNumber::new(input.momo_number.clone())?;

        let config = WalletConfig::new(
            input.lightning_address.clone(),
            momo,
            input.convert_ratio,
        );

        self.wallet_repo.save(&config).await?;

        Ok(ConfigureWalletOutput {
            wallet_id: config.id().value(),
            lightning_address: input.lightning_address,
            momo_number: input.momo_number,
            convert_ratio: input.convert_ratio,
        })
    }
}
