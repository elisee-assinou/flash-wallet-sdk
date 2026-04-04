use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::contexts::conversion::application::use_cases::buy_bitcoin::{
    BuyBitcoinUseCase,
    BuyBitcoinInput,
};

#[derive(Deserialize)]
pub struct BuyBitcoinRequest {
    pub amount_xof: u64,
    pub momo_number: String,
    pub lightning_address: String,
}

#[derive(Serialize)]
pub struct BuyBitcoinViewModel {
    pub transaction_id: String,
    pub amount_xof: u64,
    pub status: String,
    pub payment_url: Option<String>,
    pub message: String,
}

pub async fn buy_bitcoin_handler(
    State(use_case): State<Arc<BuyBitcoinUseCase>>,
    Json(body): Json<BuyBitcoinRequest>,
) -> impl IntoResponse {
    let input = BuyBitcoinInput {
        amount_xof: body.amount_xof,
        momo_number: body.momo_number,
        lightning_address: body.lightning_address,
    };

    match use_case.execute(input).await {
        Ok(output) => {
            let vm = BuyBitcoinViewModel {
                transaction_id: output.transaction_id,
                amount_xof: output.amount_xof,
                status: output.status,
                payment_url: output.payment_url,
                message: "Buy transaction created successfully".to_string(),
            };
            (StatusCode::CREATED, Json(vm)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
