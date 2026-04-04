use std::sync::Arc;
use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use crate::contexts::conversion::application::use_cases::get_transaction_status::{
    GetTransactionStatusUseCase,
    GetTransactionStatusInput,
};

#[derive(Serialize)]
pub struct TransactionStatusViewModel {
    pub transaction_id: String,
    pub status: String,
    pub is_completed: bool,
}

pub async fn get_transaction_status_handler(
    State(use_case): State<Arc<GetTransactionStatusUseCase>>,
    Path(transaction_id): Path<String>,
) -> impl IntoResponse {
    let input = GetTransactionStatusInput { transaction_id };

    match use_case.execute(input).await {
        Ok(output) => {
            let vm = TransactionStatusViewModel {
                transaction_id: output.transaction_id,
                status: output.status,
                is_completed: output.is_completed,
            };
            (StatusCode::OK, Json(vm)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
