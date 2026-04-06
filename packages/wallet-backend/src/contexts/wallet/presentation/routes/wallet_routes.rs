use std::sync::Arc;
use axum::{routing::{post, get}, Router};
use crate::contexts::wallet::{
    application::use_cases::{
        configure_wallet::ConfigureWalletUseCase,
        get_wallet::GetWalletUseCase,
        get_balance::GetBalanceUseCase,
    },
    presentation::handlers::{
        configure_wallet::configure_wallet_handler,
        get_wallet::get_wallet_handler,
        get_balance::get_balance_handler,
    },
};

pub fn wallet_router(
    configure_use_case: Arc<ConfigureWalletUseCase>,
    get_use_case: Arc<GetWalletUseCase>,
    get_balance_use_case: Arc<GetBalanceUseCase>,
) -> Router {
    Router::new()
        .route("/api/v1/wallet/configure",
            post(configure_wallet_handler).with_state(configure_use_case))
        .route("/api/v1/wallet",
            get(get_wallet_handler).with_state(get_use_case))
        .route("/api/v1/wallet/balance",
            get(get_balance_handler).with_state(get_balance_use_case))
}
