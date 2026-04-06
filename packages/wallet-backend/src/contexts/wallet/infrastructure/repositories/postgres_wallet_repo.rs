use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::value_objects::momo_number::MomoNumber;
use crate::contexts::wallet::domain::{
    entities::wallet_config::WalletConfig,
    repositories::wallet_config_repository::WalletConfigRepository,
};

pub struct PostgresWalletRepo {
    pool: PgPool,
}

impl PostgresWalletRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(lightning_address: String, momo_number: String, convert_ratio: f64) -> WalletConfig {
        let momo = MomoNumber::new(momo_number).unwrap();
        WalletConfig::new(lightning_address, momo, convert_ratio)
    }
}

#[async_trait]
impl WalletConfigRepository for PostgresWalletRepo {
    async fn save(&self, config: &WalletConfig) -> Result<(), DomainError> {
        let id = Uuid::parse_str(&config.id().value())
            .map_err(|e| DomainError::InvalidValue(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO wallet_config
                (id, lightning_address, momo_number, convert_ratio, is_auto_convert)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                lightning_address = EXCLUDED.lightning_address,
                momo_number = EXCLUDED.momo_number,
                convert_ratio = EXCLUDED.convert_ratio,
                updated_at = NOW()
            "#,
            id,
            config.lightning_address(),
            config.momo_number().value(),
            config.convert_ratio(),
            true,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(())
    }

    async fn find(&self) -> Result<Option<WalletConfig>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT * FROM wallet_config ORDER BY created_at DESC LIMIT 1"#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.map(|r| Self::map_row(r.lightning_address, r.momo_number, r.convert_ratio)))
    }

    async fn find_all(&self) -> Result<Vec<WalletConfig>, DomainError> {
        let rows = sqlx::query!(
            r#"SELECT * FROM wallet_config ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(rows.into_iter()
            .map(|r| Self::map_row(r.lightning_address, r.momo_number, r.convert_ratio))
            .collect())
    }

    async fn find_by_momo_number(&self, momo_number: &str) -> Result<Option<WalletConfig>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT * FROM wallet_config WHERE momo_number = $1 LIMIT 1"#,
            momo_number
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.map(|r| Self::map_row(r.lightning_address, r.momo_number, r.convert_ratio)))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<WalletConfig>, DomainError> {
        let pattern = format!("{}@%", username);
        let row = sqlx::query!(
            r#"SELECT * FROM wallet_config WHERE lightning_address LIKE $1 LIMIT 1"#,
            pattern
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        Ok(row.map(|r| Self::map_row(r.lightning_address, r.momo_number, r.convert_ratio)))
    }
}
