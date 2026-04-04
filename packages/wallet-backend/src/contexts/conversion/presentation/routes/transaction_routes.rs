use std::sync::Arc;
use axum::{routing::{post, get}, Router};
use crate::contexts::conversion::{
    application::use_cases::{
        auto_convert::AutoConvertUseCase,
        get_transaction_status::GetTransactionStatusUseCase,
        buy_bitcoin::BuyBitcoinUseCase,
        list_transactions::ListTransactionsUseCase,
    },
    presentation::handlers::{
        create_transaction::create_transaction_handler,
        get_transaction_status::get_transaction_status_handler,
        buy_bitcoin::buy_bitcoin_handler,
        transaction_ws::transaction_ws_handler,
        list_transactions::list_transactions_handler,
    },
};

pub fn transaction_router(
    auto_convert: Arc<AutoConvertUseCase>,
    get_status: Arc<GetTransactionStatusUseCase>,
    buy_bitcoin: Arc<BuyBitcoinUseCase>,
    list_transactions: Arc<ListTransactionsUseCase>,
) -> Router {
    Router::new()
        .route("/api/v1/transactions/convert",
            post(create_transaction_handler).with_state(auto_convert))
        .route("/api/v1/transactions/:id/status",
            get(get_transaction_status_handler).with_state(get_status.clone()))
        .route("/api/v1/transactions/buy",
            post(buy_bitcoin_handler).with_state(buy_bitcoin))
        .route("/api/v1/transactions",
            get(list_transactions_handler).with_state(list_transactions))
        .route("/ws/transactions/:id",
            get(transaction_ws_handler).with_state(get_status))
}
