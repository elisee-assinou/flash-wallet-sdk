use serde::Serialize;
use crate::contexts::conversion::application::dtos::auto_convert_dto::AutoConvertOutput;

#[derive(Serialize)]
pub struct TransactionViewModel {
    pub id: String,
    pub amount_sats: u64,
    pub amount_xof: String,
    pub status: String,
    pub message: String,
    pub invoice: Option<String>,
}

impl From<AutoConvertOutput> for TransactionViewModel {
    fn from(output: AutoConvertOutput) -> Self {
        Self {
            id: output.transaction_id,
            amount_sats: output.amount_sats,
            amount_xof: format!("{} XOF", output.amount_xof),
            status: output.status,
            message: "Conversion initiated successfully".to_string(),
            invoice: output.invoice,
        }
    }
}
