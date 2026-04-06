use std::sync::Arc;
use tokio::sync::Mutex;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

pub struct CreatePaymentInput {
    pub amount_sats: i64,
    pub description: String,
    /// Lightning Address du marchand ex: "elisee@bitcoinflash.xyz"
    pub lightning_address: String,
}

pub struct CreatePaymentOutput {
    pub invoice: String,
    pub amount_sats: i64,
    pub description: String,
    pub merchant: String,
}

#[derive(Clone)]
pub struct CreatePaymentUseCase {
    wallet_repo: Arc<PostgresWalletRepo>,
    lnd_client: Arc<Mutex<LndClient>>,
}

impl CreatePaymentUseCase {
    pub fn new(
        wallet_repo: Arc<PostgresWalletRepo>,
        lnd_client: Arc<Mutex<LndClient>>,
    ) -> Self {
        Self { wallet_repo, lnd_client }
    }

    pub async fn execute(&self, input: CreatePaymentInput) -> Result<CreatePaymentOutput, DomainError> {
        // Extrait le username depuis la Lightning Address
        let username = input.lightning_address
            .split('@')
            .next()
            .ok_or_else(|| DomainError::InvalidValue(
                "Invalid Lightning Address format".to_string()
            ))?;

        // Trouve le wallet du marchand via son username
        let wallet = self.wallet_repo.find_by_username(username).await?
            .ok_or_else(|| DomainError::NotFound(
                format!("Merchant {} not found — configure your wallet first", input.lightning_address)
            ))?;

        // Memo contient le MoMo pour l'auto-convert
        let memo = format!("flash-wallet:{}", wallet.momo_number().value());

        // Génère une invoice Lightning sur notre LND
        let mut lnd = self.lnd_client.lock().await;
        let invoice = lnd.add_invoice(input.amount_sats, &memo).await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        tracing::info!(
            "💳 Merchant payment: {} sats — {} for {} ({})",
            input.amount_sats,
            input.description,
            input.lightning_address,
            wallet.momo_number().value()
        );

        Ok(CreatePaymentOutput {
            invoice,
            amount_sats: input.amount_sats,
            description: input.description,
            merchant: input.lightning_address,
        })
    }
}
