use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    value_objects::{
        xof_amount::XofAmount,
        momo_number::MomoNumber,
    },
    repositories::transaction_repository::TransactionRepository,
    ports::flash_gateway::FlashGateway,
};

pub struct BuyBitcoinInput {
    pub amount_xof: u64,
    pub momo_number: String,
    pub lightning_address: String,
}

pub struct BuyBitcoinOutput {
    pub transaction_id: String,
    pub amount_xof: u64,
    pub status: String,
    pub payment_url: Option<String>,
}

pub struct BuyBitcoinUseCase {
    transaction_repo: Arc<dyn TransactionRepository>,
    flash_gateway: Arc<dyn FlashGateway>,
}

impl BuyBitcoinUseCase {
    pub fn new(
        transaction_repo: Arc<dyn TransactionRepository>,
        flash_gateway: Arc<dyn FlashGateway>,
    ) -> Self {
        Self { transaction_repo, flash_gateway }
    }

    pub async fn execute(
        &self,
        input: BuyBitcoinInput,
    ) -> Result<BuyBitcoinOutput, DomainError> {
        let xof = XofAmount::new(input.amount_xof)?;
        let momo = MomoNumber::new(input.momo_number)?;

        let (transaction, payment_url) = self.flash_gateway
            .create_buy_transaction(&xof, &momo, &input.lightning_address)
            .await?;

        self.transaction_repo.save(&transaction).await?;

        Ok(BuyBitcoinOutput {
            transaction_id: transaction.id().value(),
            amount_xof: xof.amount(),
            status: "PENDING".to_string(),
            payment_url,
        })
    }
}
