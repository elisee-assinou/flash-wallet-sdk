use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    entities::flash_transaction::{FlashTransaction, TransactionType, TransactionStatus},
    repositories::transaction_repository::TransactionRepository,
};

pub struct PostgresTransactionRepo {
    pool: PgPool,
}

impl PostgresTransactionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepo {
    async fn save(&self, transaction: &FlashTransaction) -> Result<(), DomainError> {
        let id = Uuid::parse_str(&transaction.id().value())
            .map_err(|e| DomainError::InvalidValue(e.to_string()))?;

        let transaction_type = match transaction.transaction_type() {
            TransactionType::SellBitcoin => "SELL_BITCOIN",
            TransactionType::BuyBitcoin => "BUY_BITCOIN",
        };

        let status = match transaction.status() {
            TransactionStatus::Pending => "PENDING",
            TransactionStatus::Completed => "COMPLETED",
            TransactionStatus::Failed => "FAILED",
        };

        sqlx::query!(
            r#"
            INSERT INTO transactions 
                (id, flash_transaction_id, invoice, transaction_type, amount_xof, amount_sats, exchange_rate, momo_number, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                flash_transaction_id = EXCLUDED.flash_transaction_id,
                invoice = EXCLUDED.invoice,
                updated_at = NOW()
            "#,
            id,
            transaction.flash_transaction_id(),
            transaction.invoice(),
            transaction_type,
            transaction.amount_xof().amount() as i64,
            transaction.amount_sats().amount() as i64,
            transaction.exchange_rate() as i64,
            transaction.momo_number().value(),
            status,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<FlashTransaction>, DomainError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| DomainError::InvalidValue(e.to_string()))?;

        let row = sqlx::query!(
            r#"SELECT * FROM transactions WHERE id = $1"#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.map(|r| FlashTransaction::from_db(
            r.id.to_string(),
            r.flash_transaction_id,
            r.invoice,
            r.transaction_type,
            r.amount_xof as u64,
            r.amount_sats as u64,
            r.exchange_rate as u64,
            r.momo_number,
            r.status,
        )))
    }

    async fn find_all(&self) -> Result<Vec<FlashTransaction>, DomainError> {
        let rows = sqlx::query!(
            r#"SELECT * FROM transactions ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(rows.into_iter().map(|r| FlashTransaction::from_db(
            r.id.to_string(),
            r.flash_transaction_id,
            r.invoice,
            r.transaction_type,
            r.amount_xof as u64,
            r.amount_sats as u64,
            r.exchange_rate as u64,
            r.momo_number,
            r.status,
        )).collect())
    }

    async fn find_pending(&self) -> Result<Vec<FlashTransaction>, DomainError> {
        let rows = sqlx::query!(
            r#"SELECT * FROM transactions WHERE status = 'PENDING' ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(rows.into_iter().map(|r| FlashTransaction::from_db(
            r.id.to_string(),
            r.flash_transaction_id,
            r.invoice,
            r.transaction_type,
            r.amount_xof as u64,
            r.amount_sats as u64,
            r.exchange_rate as u64,
            r.momo_number,
            r.status,
        )).collect())
    }

    async fn sum_completed_for_momo(&self, momo_number: &str) -> Result<u64, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(amount_sats), 0)::bigint as "total!: i64"
            FROM transactions
            WHERE momo_number = $1
            AND status IN ('COMPLETED', 'PENDING')
            AND transaction_type = 'SELL_BITCOIN'
            "#,
            momo_number
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.total as u64)
    }
}
