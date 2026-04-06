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
    flash_transaction_id: Option<String>,
    invoice: Option<String>,
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
            flash_transaction_id: None,
            invoice: None,
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

    pub fn from_db(
        id: String,
        flash_transaction_id: Option<String>,
        invoice: Option<String>,
        transaction_type: String,
        amount_xof: u64,
        amount_sats: u64,
        exchange_rate: u64,
        momo_number: String,
        status: String,
    ) -> Self {
        Self {
            id: EntityId::from_string(&id).unwrap(),
            flash_transaction_id,
            invoice,
            transaction_type: match transaction_type.as_str() {
                "BUY_BITCOIN" => TransactionType::BuyBitcoin,
                _ => TransactionType::SellBitcoin,
            },
            amount_xof: XofAmount::new(amount_xof).unwrap(),
            amount_sats: Satoshis::new(amount_sats.max(1)).unwrap(),
            exchange_rate,
            status: match status.as_str() {
                "COMPLETED" => TransactionStatus::Completed,
                "FAILED" => TransactionStatus::Failed,
                _ => TransactionStatus::Pending,
            },
            momo_number: MomoNumber::new(momo_number).unwrap(),
            payment_hash: None,
            created_at: Utc::now(), // TODO: parse from DB
        }
    }

    // Getters
    pub fn id(&self) -> &EntityId { &self.id }
    pub fn flash_transaction_id(&self) -> Option<&str> { self.flash_transaction_id.as_deref() }
    pub fn invoice(&self) -> Option<&str> { self.invoice.as_deref() }
    pub fn status(&self) -> &TransactionStatus { &self.status }
    pub fn transaction_type(&self) -> &TransactionType { &self.transaction_type }
    pub fn amount_xof(&self) -> &XofAmount { &self.amount_xof }
    pub fn amount_sats(&self) -> &Satoshis { &self.amount_sats }
    pub fn exchange_rate(&self) -> u64 { self.exchange_rate }
    pub fn momo_number(&self) -> &MomoNumber { &self.momo_number }
    pub fn payment_hash(&self) -> Option<&PaymentHash> { self.payment_hash.as_ref() }

    // Setters
    pub fn set_flash_transaction_id(&mut self, id: String) { self.flash_transaction_id = Some(id); }
    pub fn set_invoice(&mut self, invoice: String) { self.invoice = Some(invoice); }

    // Business rules
    pub fn complete(&mut self) { self.status = TransactionStatus::Completed; }
    pub fn fail(&mut self) { self.status = TransactionStatus::Failed; }
    pub fn set_payment_hash(&mut self, hash: PaymentHash) { self.payment_hash = Some(hash); }
    pub fn is_pending(&self) -> bool { self.status == TransactionStatus::Pending }
    pub fn is_completed(&self) -> bool { self.status == TransactionStatus::Completed }
    pub fn is_expired(&self) -> bool {
        // PENDING depuis plus de 24h → expiré
        self.status == TransactionStatus::Pending &&
        (Utc::now() - self.created_at).num_hours() >= 24
    }
    pub fn mark_as_failed(&mut self) { self.status = TransactionStatus::Failed; }
}
