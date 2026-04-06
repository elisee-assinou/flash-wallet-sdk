use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use crate::contexts::wallet::application::use_cases::get_balance::GetBalanceUseCase;

pub async fn get_balance_handler(
    State(use_case): State<Arc<GetBalanceUseCase>>,
) -> impl IntoResponse {
    match use_case.execute().await {
        Ok(balance) => Json(balance).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}
