use std::sync::Arc;
use axum::{extract::{State, Query}, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use crate::contexts::wallet::application::use_cases::get_wallet::GetWalletUseCase;

#[derive(Deserialize)]
pub struct GetWalletQuery {
    pub lightning_address: Option<String>,
}

#[derive(Serialize)]
pub struct WalletViewModel {
    pub wallet_id: String,
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}

pub async fn get_wallet_handler(
    State(use_case): State<Arc<GetWalletUseCase>>,
    Query(query): Query<GetWalletQuery>,
) -> impl IntoResponse {
    match use_case.execute(query.lightning_address.as_deref()).await {
        Ok(Some(output)) => (
            StatusCode::OK,
            Json(WalletViewModel {
                wallet_id: output.wallet_id,
                lightning_address: output.lightning_address,
                momo_number: output.momo_number,
                convert_ratio: output.convert_ratio,
            })
        ).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Wallet not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
