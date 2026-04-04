use crate::shared::errors::DomainError;


#[derive(Debug, Clone, PartialEq)]
pub struct Satoshis(u64);

impl Satoshis {
    pub fn new(amount: u64) -> Result<Self, DomainError> {
        if amount == 0 {
            return Err(DomainError::InvalidValue(
                "Amount must be greater than 0".to_string(),
            ));
        }
        Ok(Satoshis(amount))
    }

    pub fn amount(&self) -> u64 {
         self.0
    }
    pub fn to_millisats(&self) -> u64 {
         self.0 * 1000
    }
    pub fn apply_ratio(&self, ratio: f64) -> Result<Self, DomainError> {
        if ratio <= 0.0 || ratio > 1.0 {
            return Err(DomainError::InvalidValue(
                "Ratio must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(Satoshis((self.0 as f64 * ratio) as u64))
    }
}
