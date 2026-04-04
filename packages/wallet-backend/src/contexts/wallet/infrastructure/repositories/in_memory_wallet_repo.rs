use async_trait::async_trait;
use std::sync::Mutex;
use crate::shared::errors::DomainError;
use crate::contexts::wallet::domain::{
    entities::wallet_config::WalletConfig,
    repositories::wallet_config_repository::WalletConfigRepository,
};

pub struct InMemoryWalletRepo {
    config: Mutex<Option<WalletConfig>>,
}

impl InMemoryWalletRepo {
    pub fn new() -> Self {
        Self { config: Mutex::new(None) }
    }
}

#[async_trait]
impl WalletConfigRepository for InMemoryWalletRepo {
    async fn save(&self, config: &WalletConfig) -> Result<(), DomainError> {
        let mut c = self.config.lock().unwrap();
        *c = Some(config.clone());
        Ok(())
    }

    async fn find(&self) -> Result<Option<WalletConfig>, DomainError> {
        let c = self.config.lock().unwrap();
        Ok(c.clone())
    }
}
