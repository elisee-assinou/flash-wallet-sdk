use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    value_objects::{
        satoshis::Satoshis,
        xof_amount::XofAmount,
        momo_number::MomoNumber,
    },
    services::conversion_service::ConversionService,
    repositories::transaction_repository::TransactionRepository,
    ports::flash_gateway::FlashGateway,
};
use crate::contexts::conversion::application::dtos::auto_convert_dto::{
    AutoConvertInput,
    AutoConvertOutput,
};

pub struct AutoConvertUseCase {
    transaction_repo: Arc<dyn TransactionRepository>,
    flash_gateway: Arc<dyn FlashGateway>,
}

impl AutoConvertUseCase {
    pub fn new(
        transaction_repo: Arc<dyn TransactionRepository>,
        flash_gateway: Arc<dyn FlashGateway>,
    ) -> Self {
        Self { transaction_repo, flash_gateway }
    }

    pub async fn execute(
        &self,
        input: AutoConvertInput,
    ) -> Result<AutoConvertOutput, DomainError> {
        // 1. Valide les données d'entrée
        let sats = Satoshis::new(input.amount_sats)?;
        let momo = MomoNumber::new(input.momo_number)?;

        // 2. Applique le ratio de conversion
        let sats_to_convert = sats.apply_ratio(input.convert_ratio)?;

        // 3. Estime le montant XOF (taux fixe pour l'estimation)
        // Le taux réel sera dans la réponse Flash
        let estimated_xof = XofAmount::new(
            (sats_to_convert.amount() as f64 / 100_000_000.0 * 36_456_183.0) as u64
        )?;

        // 4. Crée la transaction Flash
        let transaction = self.flash_gateway
            .create_sell_transaction(&estimated_xof, &momo)
            .await?;

        // 5. Sauvegarde
        self.transaction_repo.save(&transaction).await?;

        // 6. Retourne le résultat
        Ok(AutoConvertOutput {
            transaction_id: transaction.id().value(),
            amount_sats: sats_to_convert.amount(),
            amount_xof: transaction.amount_xof().amount(),
            status: "PENDING".to_string(),
            invoice: transaction.invoice().map(|s| s.to_string()),
        })
    }
}
