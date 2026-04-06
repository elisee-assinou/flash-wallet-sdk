use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::{
    entities::balance::Balance,
    repositories::balance_repository::BalanceRepository,
};

pub struct PostgresBalanceRepo {
    pool: PgPool,
}

impl PostgresBalanceRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BalanceRepository for PostgresBalanceRepo {
    async fn find_by_momo(&self, momo_number: &str) -> Result<Option<Balance>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT * FROM balances WHERE momo_number = $1"#,
            momo_number
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.map(|r| Balance::from_db(
            r.id.to_string(),
            r.momo_number,
            r.balance_sats,
            r.created_at,
            r.updated_at,
        )))
    }

    async fn save(&self, balance: &Balance) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO balances (id, momo_number, balance_sats)
            VALUES ($1::uuid, $2, $3)
            ON CONFLICT (momo_number) DO UPDATE SET
                balance_sats = EXCLUDED.balance_sats,
                updated_at = NOW()
            "#,
            uuid::Uuid::parse_str(&balance.id().value()).unwrap(),
            balance.momo_number(),
            balance.balance_sats(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(())
    }

    async fn credit(&self, momo_number: &str, sats: i64) -> Result<Balance, DomainError> {
        // Upsert — crée ou met à jour le solde
        let row = sqlx::query!(
            r#"
            INSERT INTO balances (momo_number, balance_sats)
            VALUES ($1, $2)
            ON CONFLICT (momo_number) DO UPDATE SET
                balance_sats = balances.balance_sats + EXCLUDED.balance_sats,
                updated_at = NOW()
            RETURNING *
            "#,
            momo_number,
            sats,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(Balance::from_db(
            row.id.to_string(),
            row.momo_number,
            row.balance_sats,
            row.created_at,
            row.updated_at,
        ))
    }
}
