use axum::{extract::{State, Query}, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use crate::contexts::wallet::application::use_cases::get_balance::GetBalanceUseCase;

#[derive(Deserialize)]
pub struct BalanceQuery {
    pub lightning_address: Option<String>,
}

pub async fn get_balance_handler(
    State(use_case): State<Arc<GetBalanceUseCase>>,
    Query(query): Query<BalanceQuery>,
) -> impl IntoResponse {
    match use_case.execute_for(query.lightning_address.as_deref()).await {
        Ok(balance) => Json(balance).into_response(),
        Err(e) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    }
}
