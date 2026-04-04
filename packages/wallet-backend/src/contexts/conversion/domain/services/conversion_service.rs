use crate::contexts::conversion::domain::value_objects::{
    satoshis::Satoshis,
    xof_amount::XofAmount,
};
use crate::shared::errors::DomainError;

pub struct ConversionService;

impl ConversionService {
    /// Convertit des sats en XOF selon le taux Flash
    /// sell_rate = taux de vente en XOF par BTC
    pub fn sats_to_xof(sats: &Satoshis, sell_rate: u64) -> Result<XofAmount, DomainError> {
        let xof = (sats.amount() as f64 / 100_000_000.0 * sell_rate as f64) as u64;
        XofAmount::new(xof)
    }

    /// Convertit du XOF en sats selon le taux Flash
    /// buy_rate = taux d'achat en XOF par BTC
    pub fn xof_to_sats(xof: &XofAmount, buy_rate: u64) -> Result<Satoshis, DomainError> {
        let sats = (xof.amount() as f64 / buy_rate as f64 * 100_000_000.0) as u64;
        Satoshis::new(sats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sats_to_xof() {
        // 239999 sats au taux 36_044_280 XOF/BTC
        // = 239999 / 100_000_000 * 36_044_280 = ~86_506 XOF
        let sats = Satoshis::new(239_999).unwrap();
        let xof = ConversionService::sats_to_xof(&sats, 36_044_280).unwrap();
        assert_eq!(xof.amount(), 86_505); // arrondi bas
    }

    #[test]
    fn test_xof_to_sats() {
        let xof = XofAmount::new(5000).unwrap();
        let sats = ConversionService::xof_to_sats(&xof, 39_838_415).unwrap();
        assert!(sats.amount() > 0);
    }
}
