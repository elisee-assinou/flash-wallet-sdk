use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::repositories::balance_repository::BalanceRepository;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_balance_repo::PostgresBalanceRepo;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;

pub struct GetBalanceUseCase {
    balance_repo: Arc<PostgresBalanceRepo>,
    wallet_repo: Arc<PostgresWalletRepo>,
}

impl GetBalanceUseCase {
    pub fn new(
        balance_repo: Arc<PostgresBalanceRepo>,
        wallet_repo: Arc<PostgresWalletRepo>,
    ) -> Self {
        Self { balance_repo, wallet_repo }
    }

    pub async fn execute(&self) -> Result<serde_json::Value, DomainError> {
        let wallet = self.wallet_repo.find().await?
            .ok_or_else(|| DomainError::NotFound("Wallet not configured".to_string()))?;

        let balance = self.balance_repo
            .find_by_momo(wallet.momo_number().value())
            .await?;

        let balance_sats = balance.map(|b| b.balance_sats()).unwrap_or(0);

        Ok(serde_json::json!({
            "momo_number": wallet.momo_number().value(),
            "balance_sats": balance_sats,
            "balance_btc": balance_sats as f64 / 100_000_000.0,
        }))
    }
}
