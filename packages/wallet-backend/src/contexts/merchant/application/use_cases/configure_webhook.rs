use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;

pub struct ConfigureWebhookInput {
    pub lightning_address: String,
    pub webhook_url: String,
}

#[derive(Clone)]
pub struct ConfigureWebhookUseCase {
    wallet_repo: Arc<PostgresWalletRepo>,
}

impl ConfigureWebhookUseCase {
    pub fn new(wallet_repo: Arc<PostgresWalletRepo>) -> Self {
        Self { wallet_repo }
    }

    pub async fn execute(&self, input: ConfigureWebhookInput) -> Result<(), DomainError> {
        // Vérifie que le wallet existe
        let username = input.lightning_address.split('@').next().unwrap_or("");
        let all = self.wallet_repo.find_all().await?;
        let wallet = all.into_iter()
            .find(|w| w.lightning_address() == input.lightning_address)
            .ok_or_else(|| DomainError::NotFound(
                format!("Wallet not found for {}", input.lightning_address)
            ))?;

        // Valide l'URL
        if !input.webhook_url.starts_with("http://") && !input.webhook_url.starts_with("https://") {
            return Err(DomainError::InvalidValue(
                "Webhook URL must start with http:// or https://".to_string()
            ));
        }

        // Sauvegarde le webhook
        self.wallet_repo.update_webhook(
            wallet.lightning_address(),
            Some(&input.webhook_url)
        ).await.map_err(|e| DomainError::ExternalService(e.to_string()))?;

        tracing::info!(
            " Webhook configured for {}: {}",
            input.lightning_address,
            input.webhook_url
        );

        Ok(())
    }
}
