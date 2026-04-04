use crate::shared::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentHash(String);

impl PaymentHash {
    pub fn new(hash: String) -> Result<Self, DomainError> {
        // validation 1 : longueur doit être 64
        if hash.len() != 64 {
            return Err(DomainError::InvalidValue(
                "Payment hash must be 64 characters long".to_string(),
            ));
        }
        // validation 2 : tous les caractères doivent être hex (0-9, a-f)
        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidValue(
                "Payment hash must contain only hexadecimal characters".to_string(),
            ));
        }
        Ok(PaymentHash(hash))
    }

    pub fn value(&self) -> &str {
        // retourne le hash
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hash() {
        let valid_hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let payment_hash = PaymentHash::new(valid_hash.to_string()).unwrap();
        assert_eq!(payment_hash.value(), valid_hash);
    }

    #[test]
    fn test_invalid_length_fails() {
        let invalid_hash = "too short";
        assert!(PaymentHash::new(invalid_hash.to_string()).is_err());
    }

    #[test]
    fn test_invalid_characters_fails() {
        // hash avec des caractères non hex
        let invalid_hash = "invalid-characters";
        assert!(PaymentHash::new(invalid_hash.to_string()).is_err());
    }
}