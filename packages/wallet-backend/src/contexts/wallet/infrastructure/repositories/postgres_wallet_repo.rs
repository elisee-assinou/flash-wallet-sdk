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

        Ok(row.map(|r| {
            let momo = MomoNumber::new(r.momo_number).unwrap();
            WalletConfig::new(r.lightning_address, momo, r.convert_ratio)
        }))
    }
}
