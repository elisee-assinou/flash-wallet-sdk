use crate::shared::types::EntityId;
use crate::contexts::conversion::domain::value_objects::momo_number::MomoNumber;

#[derive(Debug, Clone)]
pub struct WalletConfig {
    id: EntityId,
    lightning_address: String,
    momo_number: MomoNumber,
    convert_ratio: f64,
    is_auto_convert: bool,
    webhook_url: Option<String>,
}

impl WalletConfig {
    pub fn new(
        lightning_address: String,
        momo_number: MomoNumber,
        convert_ratio: f64,
    ) -> Self {
        Self {
            id: EntityId::new(),
            lightning_address,
            momo_number,
            convert_ratio,
            is_auto_convert: true,
            webhook_url: None,
        }
    }

    pub fn with_webhook(mut self, webhook_url: Option<String>) -> Self {
        self.webhook_url = webhook_url;
        self
    }

    pub fn id(&self) -> &EntityId { &self.id }
    pub fn lightning_address(&self) -> &str { &self.lightning_address }
    pub fn momo_number(&self) -> &MomoNumber { &self.momo_number }
    pub fn convert_ratio(&self) -> f64 { self.convert_ratio }
    pub fn is_auto_convert(&self) -> bool { self.is_auto_convert }
    pub fn webhook_url(&self) -> Option<&str> { self.webhook_url.as_deref() }
}
