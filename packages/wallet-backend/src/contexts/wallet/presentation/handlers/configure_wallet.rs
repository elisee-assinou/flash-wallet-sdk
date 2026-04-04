use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::contexts::wallet::application::{
    use_cases::configure_wallet::ConfigureWalletUseCase,
    dtos::configure_wallet_dto::ConfigureWalletInput,
};

#[derive(Deserialize)]
pub struct ConfigureWalletRequest {
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
}

#[derive(Serialize)]
pub struct ConfigureWalletResponse {
    pub wallet_id: String,
    pub lightning_address: String,
    pub momo_number: String,
    pub convert_ratio: f64,
    pub message: String,
}

pub async fn configure_wallet_handler(
    State(use_case): State<Arc<ConfigureWalletUseCase>>,
    Json(body): Json<ConfigureWalletRequest>,
) -> impl IntoResponse {
    let input = ConfigureWalletInput {
        lightning_address: body.lightning_address,
        momo_number: body.momo_number,
        convert_ratio: body.convert_ratio,
    };

    match use_case.execute(input).await {
        Ok(output) => {
            let response = ConfigureWalletResponse {
                wallet_id: output.wallet_id,
                lightning_address: output.lightning_address,
                momo_number: output.momo_number,
                convert_ratio: output.convert_ratio,
                message: "Wallet configured successfully".to_string(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
