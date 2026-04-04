use chrono::{DateTime, Utc};
use crate::shared::types::EntityId;
use crate::contexts::conversion::domain::value_objects::{
    xof_amount::XofAmount,
    satoshis::Satoshis,
    momo_number::MomoNumber,
    payment_hash::PaymentHash,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    SellBitcoin,
    BuyBitcoin,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct FlashTransaction {
    id: EntityId,
    transaction_type: TransactionType,
    amount_xof: XofAmount,
    amount_sats: Satoshis,
    exchange_rate: u64,
    status: TransactionStatus,
    momo_number: MomoNumber,
    payment_hash: Option<PaymentHash>,
    created_at: DateTime<Utc>,
}

impl FlashTransaction {
    pub fn new(
        transaction_type: TransactionType,
        amount_xof: XofAmount,
        amount_sats: Satoshis,
        exchange_rate: u64,
        momo_number: MomoNumber,
    ) -> Self {
        Self {
            id: EntityId::new(),
            transaction_type,
            amount_xof,
            amount_sats,
            exchange_rate,
            status: TransactionStatus::Pending,
            momo_number,
            payment_hash: None,
            created_at: Utc::now(),
        }
    }

    // Getters
    pub fn id(&self) -> &EntityId { &self.id }
    pub fn status(&self) -> &TransactionStatus { &self.status }
    pub fn amount_xof(&self) -> &XofAmount { &self.amount_xof }
    pub fn amount_sats(&self) -> &Satoshis { &self.amount_sats }
    pub fn momo_number(&self) -> &MomoNumber { &self.momo_number }
    pub fn payment_hash(&self) -> Option<&PaymentHash> { self.payment_hash.as_ref() }

    // Business rules
    pub fn complete(&mut self) {
        self.status = TransactionStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.status = TransactionStatus::Failed;
    }

    pub fn set_payment_hash(&mut self, hash: PaymentHash) {
        self.payment_hash = Some(hash);
    }

    pub fn is_pending(&self) -> bool {
        self.status == TransactionStatus::Pending
    }

    pub fn is_completed(&self) -> bool {
        self.status == TransactionStatus::Completed
    }
}
