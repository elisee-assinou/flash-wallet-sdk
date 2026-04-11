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

    fn validate_lightning_address(address: &str) -> Result<(), DomainError> {
        let parts: Vec<&str> = address.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::InvalidValue(
                "Invalid Lightning Address format. Expected: username@domain".to_string()
            ));
        }
        Ok(())
    }

    pub async fn execute(
        &self,
        input: ConfigureWalletInput,
    ) -> Result<ConfigureWalletOutput, DomainError> {
        // 1. Valide le format Lightning Address
        Self::validate_lightning_address(&input.lightning_address)?;

        // 2. Valide le format MoMo (domain rule)
        let momo = MomoNumber::new(input.momo_number.clone())?;

        // 3. Vérifie si le MoMo est déjà utilisé par une autre Lightning Address
        if let Ok(Some(existing)) = self.wallet_repo.find_by_momo_number(momo.value()).await {
            if existing.lightning_address() != input.lightning_address {
                return Err(DomainError::InvalidValue(
                    format!("Le numéro MoMo {} est déjà associé à une autre adresse Lightning", momo.value())
                ));
            }
        }

        // 4. Vérifie si la Lightning Address est déjà utilisée par un autre MoMo
        let username = input.lightning_address.split('@').next().unwrap_or("");
        if let Ok(Some(existing)) = self.wallet_repo.find_by_username(username).await {
            if existing.momo_number().value() != momo.value() {
                return Err(DomainError::InvalidValue(
                    format!("La Lightning Address {} est déjà associée à un autre numéro MoMo", input.lightning_address)
                ));
            }
        }

        // 5. Crée et sauvegarde (ON CONFLICT met à jour le ratio)
        let config = WalletConfig::new(
            input.lightning_address.clone(),
            momo,
            input.convert_ratio,
        );

        self.wallet_repo.save(&config).await?;

        tracing::info!(
            " Wallet configured: {} → {} (ratio: {}%)",
            input.lightning_address,
            input.momo_number,
            (input.convert_ratio * 100.0) as u32
        );

        Ok(ConfigureWalletOutput {
            wallet_id: config.id().value(),
            lightning_address: input.lightning_address,
            momo_number: input.momo_number,
            convert_ratio: input.convert_ratio,
        })
    }
}
