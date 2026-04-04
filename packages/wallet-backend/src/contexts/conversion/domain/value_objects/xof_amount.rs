use crate::shared::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct XofAmount(u64);

impl XofAmount {
    pub fn new(amount: u64) -> Result<Self, DomainError> {
        if amount == 0 {
            return Err(DomainError::InvalidValue(
                "Amount must be greater than 0".to_string(),
            ));
        }
        Ok(XofAmount(amount))
    }
    pub fn amount(&self) -> u64 {
         self.0
    }
    pub fn display_amount(&self) -> String {
        format!("{} XOF", self.0)
    }
}

impl std::fmt::Display for XofAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} XOF", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_xof_amount() {
        let amount = XofAmount::new(100).unwrap();
        assert_eq!(amount.amount(), 100);
    }
    #[test]
    fn test_display_xof_amount() {
        let amount = XofAmount::new(100).unwrap();
        assert_eq!(amount.display_amount(), "100 XOF");
    }
    #[test]
    fn test_amount_zero_fails() {
        let result = XofAmount::new(0);
        assert!(result.is_err());
    }

}