use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use crate::contexts::wallet::application::use_cases::convert_balance::{
    ConvertBalanceUseCase, ConvertBalanceInput
};

#[derive(Deserialize)]
pub struct ConvertBalanceRequest {
    pub ratio: f64,
    pub lightning_address: Option<String>,
}

pub async fn convert_balance_handler(
    State(use_case): State<Arc<ConvertBalanceUseCase>>,
    Json(body): Json<ConvertBalanceRequest>,
) -> impl IntoResponse {
    let input = ConvertBalanceInput {
        ratio: body.ratio,
        lightning_address: body.lightning_address,
    };

    match use_case.execute(input).await {
        Ok(output) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "sats_converted": output.sats_converted,
                "amount_xof": output.amount_xof,
                "new_balance_sats": output.new_balance_sats,
                "message": "Conversion successful"
            }))
        ).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
