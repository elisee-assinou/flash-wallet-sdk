use std::sync::Arc;
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    value_objects::{
        satoshis::Satoshis,
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

        // 3. Récupère le taux de vente depuis Flash
        let sell_rate = self.flash_gateway.get_sell_rate().await?;

        // 4. Calcule le montant XOF
        let xof_amount = ConversionService::sats_to_xof(&sats_to_convert, sell_rate)?;

        // 5. Crée la transaction Flash
        let transaction = self.flash_gateway
            .create_sell_transaction(&xof_amount, &momo)
            .await?;

        // 6. Sauvegarde
        self.transaction_repo.save(&transaction).await?;

        // 7. Retourne le résultat
        Ok(AutoConvertOutput {
            transaction_id: transaction.id().value(),
            amount_sats: sats_to_convert.amount(),
            amount_xof: xof_amount.amount(),
            status: "PENDING".to_string(),
        })
    }
}
