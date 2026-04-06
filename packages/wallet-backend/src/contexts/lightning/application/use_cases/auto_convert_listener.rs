use std::sync::Arc;
use tokio::sync::Mutex;
use crate::contexts::conversion::application::use_cases::auto_convert::AutoConvertUseCase;
use crate::contexts::conversion::application::dtos::auto_convert_dto::AutoConvertInput;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

pub struct AutoConvertListener {
    auto_convert_use_case: Arc<AutoConvertUseCase>,
    wallet_repo: Arc<PostgresWalletRepo>,
    lnd_client: Arc<Mutex<LndClient>>,
}

impl AutoConvertListener {
    pub fn new(
        auto_convert_use_case: Arc<AutoConvertUseCase>,
        wallet_repo: Arc<PostgresWalletRepo>,
        lnd_client: Arc<Mutex<LndClient>>,
    ) -> Self {
        Self {
            auto_convert_use_case,
            wallet_repo,
            lnd_client,
        }
    }

    pub async fn on_invoice_settled(&self, amount_sats: u64, memo: &str) {
        tracing::info!("Invoice settled: {} sats, memo: {}", amount_sats, memo);

        // Format memo: "flash-wallet:+2290197245435"
        let momo_number = if memo.starts_with("flash-wallet:") {
            memo.trim_start_matches("flash-wallet:").to_string()
        } else {
            tracing::warn!("Invoice memo not recognized: {}", memo);
            return;
        };

        // Trouve le wallet par numéro MoMo
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

        let input = AutoConvertInput {
            amount_sats,
            momo_number: momo_number.clone(),
            convert_ratio: wallet.convert_ratio(),
        };

        match self.auto_convert_use_case.execute(input).await {
            Ok(output) => {
                tracing::info!(
                    " Auto-convert: {} sats → {} XOF for {}",
                    amount_sats,
                    output.amount_xof,
                    momo_number
                );

                // Paie l'invoice Flash avec notre LND
                if let Some(flash_invoice) = output.invoice {
                    tracing::info!(" Paying Flash invoice...");
                    let mut lnd = self.lnd_client.lock().await;
                    match lnd.pay_invoice(&flash_invoice).await {
                        Ok(_) => {
                            tracing::info!(" Flash invoice paid → XOF en route to MoMo");
                        }
                        Err(e) => {
                            tracing::error!(" Failed to pay Flash invoice: {}", e);
                        }
                    }
                } else {
                    tracing::warn!("No Flash invoice returned");
                }
            }
            Err(e) => {
                tracing::error!("Auto-convert failed: {}", e);
            }
        }
    }
}
