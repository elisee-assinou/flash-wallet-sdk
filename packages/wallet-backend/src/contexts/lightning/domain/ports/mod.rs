use async_trait::async_trait;
use crate::shared::errors::DomainError;

#[async_trait]
pub trait LndGateway: Send + Sync {
    async fn add_invoice(&self, amount_sats: i64, memo: &str) -> Result<String, DomainError>;
    async fn pay_invoice(&self, payment_request: &str) -> Result<(), DomainError>;
}
