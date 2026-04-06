use std::sync::Arc;
use axum::{routing::{post, get}, Router, Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use crate::contexts::wallet::{
    application::use_cases::{
        configure_wallet::ConfigureWalletUseCase,
        get_wallet::GetWalletUseCase,
        get_balance::GetBalanceUseCase,
        convert_balance::{ConvertBalanceUseCase, ConvertBalanceInput},
    },
    presentation::handlers::{
        configure_wallet::configure_wallet_handler,
        get_wallet::get_wallet_handler,
        get_balance::get_balance_handler,
        balance_ws::balance_ws_handler,
    },
};

#[derive(Deserialize)]
struct ConvertBalanceRequest {
    ratio: f64,
}

pub fn wallet_router(
    configure_use_case: Arc<ConfigureWalletUseCase>,
    get_use_case: Arc<GetWalletUseCase>,
    get_balance_use_case: Arc<GetBalanceUseCase>,
    convert_balance_use_case: Arc<ConvertBalanceUseCase>,
) -> Router {
    let uc = convert_balance_use_case;
    Router::new()
        .route("/api/v1/wallet/configure",
            post(configure_wallet_handler).with_state(configure_use_case))
        .route("/api/v1/wallet",
            get(get_wallet_handler).with_state(get_use_case))
        .route("/api/v1/wallet/balance",
            get(get_balance_handler).with_state(get_balance_use_case.clone()))
        .route("/ws/balance",
            get(balance_ws_handler).with_state(get_balance_use_case))
        .route("/api/v1/wallet/balance/convert",
            post(move |Json(body): Json<ConvertBalanceRequest>| {
                let uc = uc.clone();
                async move {
                    match uc.execute(ConvertBalanceInput { ratio: body.ratio }).await {
                        Ok(o) => (StatusCode::OK, Json(serde_json::json!({
                            "sats_converted": o.sats_converted,
                            "amount_xof": o.amount_xof,
                            "new_balance_sats": o.new_balance_sats,
                            "message": "Conversion successful"
                        }))).into_response(),
                        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
                    }
                }
            }))
}
