use crate::shared::types::EntityId;
use crate::contexts::conversion::domain::value_objects::momo_number::MomoNumber;

#[derive(Debug, Clone)]
pub struct WalletConfig {
    id: EntityId,
    lightning_address: String,
    momo_number: MomoNumber,
    convert_ratio: f64,
    is_auto_convert: bool,
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
        }
    }

    pub fn id(&self) -> &EntityId { &self.id }
    pub fn lightning_address(&self) -> &str { &self.lightning_address }
    pub fn momo_number(&self) -> &MomoNumber { &self.momo_number }
    pub fn convert_ratio(&self) -> f64 { self.convert_ratio }
    pub fn is_auto_convert(&self) -> bool { self.is_auto_convert }
}