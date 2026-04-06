use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use crate::contexts::merchant::application::use_cases::create_payment::{
    CreatePaymentUseCase, CreatePaymentInput
};

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub amount_sats: i64,
    pub description: String,
    pub lightning_address: String,
}

pub async fn create_payment_handler(
    State(use_case): State<Arc<CreatePaymentUseCase>>,
    Json(body): Json<CreatePaymentRequest>,
) -> impl IntoResponse {
    let input = CreatePaymentInput {
        amount_sats: body.amount_sats,
        description: body.description,
        lightning_address: body.lightning_address,
    };

    match use_case.execute(input).await {
        Ok(output) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "invoice": output.invoice,
                "amount_sats": output.amount_sats,
                "description": output.description,
                "merchant": output.merchant,
            }))
        ).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
