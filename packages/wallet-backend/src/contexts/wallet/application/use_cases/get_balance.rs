use std::sync::Arc;
use tokio::sync::Mutex;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::conversion::domain::repositories::transaction_repository::TransactionRepository;
use crate::contexts::conversion::infrastructure::repositories::postgres_transaction_repo::PostgresTransactionRepo;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

pub struct GetBalanceUseCase {
    wallet_repo: Arc<PostgresWalletRepo>,
    transaction_repo: Arc<PostgresTransactionRepo>,
    lnd_client: Arc<Mutex<LndClient>>,
}

impl GetBalanceUseCase {
    pub fn new(
        wallet_repo: Arc<PostgresWalletRepo>,
        transaction_repo: Arc<PostgresTransactionRepo>,
        lnd_client: Arc<Mutex<LndClient>>,
    ) -> Self {
        Self { wallet_repo, transaction_repo, lnd_client }
    }

    pub async fn execute_for(&self, lightning_address: Option<&str>) -> Result<serde_json::Value, DomainError> {
        // Trouve le wallet selon la Lightning Address
        let wallet = match lightning_address {
            Some(address) => {
                let all = self.wallet_repo.find_all().await?;
                all.into_iter().find(|w| w.lightning_address() == address)
                    .ok_or_else(|| DomainError::NotFound(format!("Wallet not found for {}", address)))?
            }
            None => self.wallet_repo.find().await?
                .ok_or_else(|| DomainError::NotFound("Wallet not configured".to_string()))?,
        };

        let momo = wallet.momo_number().value().to_string();

        // Balance depuis LND
        let total_received = {
            let mut lnd = self.lnd_client.lock().await;
            lnd.list_settled_invoices_for_momo(&momo).await
                .map_err(|e| DomainError::ExternalService(e.to_string()))?
        };

        let total_locked = self.transaction_repo.sum_completed_for_momo(&momo).await?;
        let balance_sats = total_received.saturating_sub(total_locked);

        tracing::info!(
            "Balance for {} ({}): {} received - {} locked = {} sats",
            wallet.lightning_address(), momo, total_received, total_locked, balance_sats
        );

        Ok(serde_json::json!({
            "momo_number": momo,
            "balance_sats": balance_sats,
            "balance_btc": balance_sats as f64 / 100_000_000.0,
            "total_received_sats": total_received,
            "total_locked_sats": total_locked,
        }))
    }

    pub async fn execute(&self) -> Result<serde_json::Value, DomainError> {
        self.execute_for(None).await
    }
}
