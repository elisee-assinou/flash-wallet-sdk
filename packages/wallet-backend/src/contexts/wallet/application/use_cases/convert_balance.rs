use std::sync::Arc;
use tokio::sync::Mutex;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    value_objects::{xof_amount::XofAmount, momo_number::MomoNumber},
    ports::flash_gateway::FlashGateway,
    repositories::transaction_repository::TransactionRepository,
};
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

pub struct ConvertBalanceInput {
    pub ratio: f64,
}

pub struct ConvertBalanceOutput {
    pub sats_converted: u64,
    pub amount_xof: u64,
    pub new_balance_sats: i64,
}

#[derive(Clone)]
pub struct ConvertBalanceUseCase {
    wallet_repo: Arc<PostgresWalletRepo>,
    flash_gateway: Arc<dyn FlashGateway + Send + Sync>,
    transaction_repo: Arc<dyn TransactionRepository + Send + Sync>,
    lnd_client: Arc<Mutex<LndClient>>,
}

impl ConvertBalanceUseCase {
    pub fn new(
        wallet_repo: Arc<PostgresWalletRepo>,
        flash_gateway: Arc<dyn FlashGateway + Send + Sync>,
        transaction_repo: Arc<dyn TransactionRepository + Send + Sync>,
        lnd_client: Arc<Mutex<LndClient>>,
    ) -> Self {
        Self { wallet_repo, flash_gateway, transaction_repo, lnd_client }
    }

    pub async fn execute(&self, input: ConvertBalanceInput) -> Result<ConvertBalanceOutput, DomainError> {
        let wallet = self.wallet_repo.find().await?
            .ok_or_else(|| DomainError::NotFound("Wallet not configured".to_string()))?;

        let momo = wallet.momo_number().value().to_string();

        // Balance réelle depuis LND
        let total_received = {
            let mut lnd = self.lnd_client.lock().await;
            lnd.list_settled_invoices_for_momo(&momo).await
                .map_err(|e| DomainError::ExternalService(e.to_string()))?
        };

        let total_locked = self.transaction_repo.sum_completed_for_momo(&momo).await?;
        let available_sats = total_received.saturating_sub(total_locked);

        if available_sats == 0 {
            return Err(DomainError::InvalidValue("Balance is 0 sats".to_string()));
        }

        let sats_to_convert = (available_sats as f64 * input.ratio) as u64;

        if sats_to_convert == 0 {
            return Err(DomainError::InvalidValue("Amount to convert is 0".to_string()));
        }

        tracing::info!(
            "💱 Convert balance: {} sats ({}%) → XOF for {}",
            sats_to_convert,
            (input.ratio * 100.0) as u32,
            momo
        );

        let estimated_xof = XofAmount::new(
            (sats_to_convert as f64 / 100_000_000.0 * 36_456_183.0) as u64
        )?;

        let momo_number = MomoNumber::new(momo.clone())?;

        let mut transaction = self.flash_gateway
            .create_sell_transaction(&estimated_xof, &momo_number)
            .await?;

        self.transaction_repo.save(&transaction).await?;

        if let Some(invoice) = transaction.invoice() {
            tracing::info!("⚡ Paying Flash invoice...");
            let mut lnd = self.lnd_client.lock().await;
            match lnd.pay_invoice(invoice).await {
                Ok(_) => {
                    tracing::info!("✅ Flash invoice paid → XOF en route to MoMo");
                    transaction.complete();
                    self.transaction_repo.save(&transaction).await?;

                    let new_balance = available_sats as i64 - sats_to_convert as i64;
                    tracing::info!("✅ New balance: {} sats", new_balance);

                    return Ok(ConvertBalanceOutput {
                        sats_converted: sats_to_convert,
                        amount_xof: transaction.amount_xof().amount(),
                        new_balance_sats: new_balance,
                    });
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    tracing::error!("❌ Failed to pay Flash invoice: {}", err_msg);
                    transaction.fail();
                    self.transaction_repo.save(&transaction).await?;
                    return Err(DomainError::ExternalService(
                        format!("Paiement échoué: {}", err_msg)
                    ));
                }
            }
        }

        transaction.fail();
        self.transaction_repo.save(&transaction).await?;
        Err(DomainError::ExternalService("Flash n'a pas retourné d'invoice".to_string()))
    }
}
