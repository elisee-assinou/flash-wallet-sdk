use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use crate::contexts::wallet::application::use_cases::get_wallet::GetWalletUseCase;

#[derive(Serialize)]
pub struct WalletViewModel {
    pub wallet_id: String,
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}

pub async fn get_wallet_handler(
    State(use_case): State<Arc<GetWalletUseCase>>,
) -> impl IntoResponse {
    match use_case.execute().await {
        Ok(output) => {
            let vm = WalletViewModel {
                wallet_id: output.wallet_id,
                lightning_address: output.lightning_address,
                momo_number: output.momo_number,
                convert_ratio: output.convert_ratio,
            };
            (StatusCode::OK, Json(vm)).into_response()
        }
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response()
    }
}
