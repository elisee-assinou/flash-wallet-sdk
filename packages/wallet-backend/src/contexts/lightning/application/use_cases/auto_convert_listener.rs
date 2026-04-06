use std::sync::Arc;
use tokio::sync::Mutex;
use crate::contexts::conversion::application::use_cases::auto_convert::AutoConvertUseCase;
use crate::contexts::conversion::application::dtos::auto_convert_dto::AutoConvertInput;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::wallet::infrastructure::repositories::postgres_balance_repo::PostgresBalanceRepo;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::domain::repositories::balance_repository::BalanceRepository;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

pub struct AutoConvertListener {
    auto_convert_use_case: Arc<AutoConvertUseCase>,
    wallet_repo: Arc<PostgresWalletRepo>,
    balance_repo: Arc<PostgresBalanceRepo>,
    lnd_client: Arc<Mutex<LndClient>>,
}

impl AutoConvertListener {
    pub fn new(
        auto_convert_use_case: Arc<AutoConvertUseCase>,
        wallet_repo: Arc<PostgresWalletRepo>,
        balance_repo: Arc<PostgresBalanceRepo>,
        lnd_client: Arc<Mutex<LndClient>>,
    ) -> Self {
        Self {
            auto_convert_use_case,
            wallet_repo,
            balance_repo,
            lnd_client,
        }
    }

    pub async fn on_invoice_settled(&self, amount_sats: u64, memo: &str) {
        tracing::info!("Invoice settled: {} sats, memo: {}", amount_sats, memo);

        let momo_number = if memo.starts_with("flash-wallet:") {
            memo.trim_start_matches("flash-wallet:").to_string()
        } else {
            tracing::warn!("Invoice memo not recognized: {}", memo);
            return;
        };

        let wallet = match self.wallet_repo.find_by_momo_number(&momo_number).await {
            Ok(Some(w)) => w,
            Ok(None) => {
                tracing::warn!("No wallet found for momo: {}", momo_number);
                return;
            }
            Err(e) => {
                tracing::error!("Error finding wallet: {}", e);
                return;
            }
        };

        let convert_ratio = wallet.convert_ratio();

        // Calcule les sats à convertir et les sats à garder
        let sats_to_convert = (amount_sats as f64 * convert_ratio) as u64;
        let sats_to_keep = amount_sats - sats_to_convert;

        tracing::info!(
            "Split: {} sats total → {} to convert, {} to keep",
            amount_sats, sats_to_convert, sats_to_keep
        );

        // Crédite le solde des sats non convertis
        if sats_to_keep > 0 {
            match self.balance_repo.credit(&momo_number, sats_to_keep as i64).await {
                Ok(balance) => {
                    tracing::info!(
                        " Balance credited: {} sats kept for {} (total: {} sats)",
                        sats_to_keep,
                        momo_number,
                        balance.balance_sats()
                    );
                }
                Err(e) => {
                    tracing::error!("Failed to credit balance: {}", e);
                }
            }
        }

        // Auto-convert les sats selon le ratio
        if sats_to_convert > 0 {
            let input = AutoConvertInput {
                amount_sats: sats_to_convert,
                momo_number: momo_number.clone(),
                convert_ratio: 1.0, // déjà calculé
            };

            match self.auto_convert_use_case.execute(input).await {
                Ok(output) => {
                    tracing::info!(
                        " Auto-convert: {} sats → {} XOF for {}",
                        sats_to_convert,
                        output.amount_xof,
                        momo_number
                    );

                    if let Some(flash_invoice) = output.invoice {
                        tracing::info!("⚡ Paying Flash invoice...");
                        let mut lnd = self.lnd_client.lock().await;
                        match lnd.pay_invoice(&flash_invoice).await {
                            Ok(_) => tracing::info!(" Flash invoice paid --> XOF en route to MoMo"),
                            Err(e) => tracing::error!(" Failed to pay Flash invoice: {}", e),
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Auto-convert failed: {}", e);
                }
            }
        }
    }
}
