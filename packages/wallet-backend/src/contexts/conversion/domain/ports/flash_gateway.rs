use async_trait::async_trait;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    entities::flash_transaction::FlashTransaction,
    value_objects::{
        xof_amount::XofAmount,
        momo_number::MomoNumber,
    },
};

#[async_trait]
pub trait FlashGateway: Send + Sync {
    async fn create_sell_transaction(
        &self,
        amount_xof: &XofAmount,
        momo_number: &MomoNumber,
    ) -> Result<FlashTransaction, DomainError>;

    async fn create_buy_transaction(
        &self,
        amount_xof: &XofAmount,
        momo_number: &MomoNumber,
        lightning_address: &str,
    ) -> Result<(FlashTransaction, Option<String>), DomainError>;

    async fn get_transaction_status(
        &self,
        transaction_id: &str,
    ) -> Result<String, DomainError>;
}
