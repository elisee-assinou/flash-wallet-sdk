use chrono::{DateTime, Utc};
use crate::shared::types::EntityId;

#[derive(Debug, Clone)]
pub struct Balance {
    id: EntityId,
    momo_number: String,
    balance_sats: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Balance {
    pub fn new(momo_number: String) -> Self {
        Self {
            id: EntityId::new(),
            momo_number,
            balance_sats: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn from_db(
        id: String,
        momo_number: String,
        balance_sats: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: EntityId::from_string(&id).unwrap(),
            momo_number,
            balance_sats,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &EntityId { &self.id }
    pub fn momo_number(&self) -> &str { &self.momo_number }
    pub fn balance_sats(&self) -> i64 { self.balance_sats }

    pub fn credit(&mut self, sats: i64) {
        self.balance_sats += sats;
        self.updated_at = Utc::now();
    }

    pub fn debit(&mut self, sats: i64) -> Result<(), String> {
        if sats > self.balance_sats {
            return Err(format!(
                "Insufficient balance: {} sats available, {} requested",
                self.balance_sats, sats
            ));
        }
        self.balance_sats -= sats;
        self.updated_at = Utc::now();
        Ok(())
    }
}
