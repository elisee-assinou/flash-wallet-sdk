use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::contexts::conversion::application::{
    use_cases::auto_convert::AutoConvertUseCase,
    dtos::auto_convert_dto::AutoConvertInput,
};
use crate::contexts::conversion::presentation::view_models::transaction_view_model::TransactionViewModel;
use crate::shared::errors::DomainError;

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub amount_sats: u64,
    pub momo_number: String,
    pub convert_ratio: f64,
}

pub async fn create_transaction_handler(
    State(use_case): State<Arc<AutoConvertUseCase>>,
    Json(body): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    let input = AutoConvertInput {
        amount_sats: body.amount_sats,
        momo_number: body.momo_number,
        convert_ratio: body.convert_ratio,
    };

    let result: Result<_, DomainError> = use_case.execute(input).await;

    match result {
        Ok(output) => {
            let view_model = TransactionViewModel::from(output);
            (StatusCode::CREATED, Json(view_model)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
