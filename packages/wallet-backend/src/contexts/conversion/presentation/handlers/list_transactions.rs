use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use crate::contexts::conversion::application::use_cases::list_transactions::ListTransactionsUseCase;

#[derive(Serialize)]
pub struct TransactionListItem {
    pub id: String,
    pub amount_xof: u64,
    pub amount_sats: u64,
    pub status: String,
}

#[derive(Serialize)]
pub struct TransactionListViewModel {
    pub transactions: Vec<TransactionListItem>,
    pub total: usize,
}

pub async fn list_transactions_handler(
    State(use_case): State<Arc<ListTransactionsUseCase>>,
) -> impl IntoResponse {
    match use_case.execute().await {
        Ok(output) => {
            let total = output.transactions.len();
            let items = output.transactions.into_iter().map(|t| TransactionListItem {
                id: t.id,
                amount_xof: t.amount_xof,
                amount_sats: t.amount_sats,
                status: t.status,
            }).collect();

            let vm = TransactionListViewModel {
                transactions: items,
                total,
            };
            (StatusCode::OK, Json(vm)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
