use crate::shared::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct MomoNumber(String);

impl MomoNumber {
    pub fn new(number: String) -> Result<Self, DomainError> {
        if !number.starts_with('+') {
            return Err(DomainError::InvalidValue(
                "MoMo number must start with +".to_string(),
            ));
        }
        let digits = &number[1..];
        if digits.len() < 10 || digits.len() > 15 {
            return Err(DomainError::InvalidValue(
                "MoMo number must be between 10 and 15 digits".to_string(),
            ));
        }
        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::InvalidValue(
                "MoMo number must contain only digits after +".to_string(),
            ));
        }
        Ok(MomoNumber(number))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_momo_number() {
        let number = MomoNumber::new("+2290197245435".to_string()).unwrap();
        assert_eq!(number.value(), "+2290197245435");
    }

    #[test]
    fn test_missing_plus_fails() {
        assert!(MomoNumber::new("2290197245435".to_string()).is_err());
    }

    #[test]
    fn test_too_short_fails() {
        assert!(MomoNumber::new("+229".to_string()).is_err());
    }

    #[test]
    fn test_invalid_chars_fails() {
        assert!(MomoNumber::new("+229abc".to_string()).is_err());
    }
}