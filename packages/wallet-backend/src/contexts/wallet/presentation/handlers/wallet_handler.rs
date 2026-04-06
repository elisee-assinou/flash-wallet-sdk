
pub async fn get_balance_handler(
    State(state): State<crate::contexts::wallet::presentation::routes::wallet_routes::WalletState>,
) -> impl IntoResponse {
    let wallet = match state.get_wallet_use_case.execute().await {
        Ok(Some(w)) => w,
        Ok(None) => return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Wallet not configured"}))
        ).into_response(),
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    };

    let balance = match state.get_balance_use_case.execute(wallet.momo_number().value()).await {
        Ok(b) => b,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    };

    Json(serde_json::json!({
        "momo_number": wallet.momo_number().value(),
        "balance_sats": balance,
        "balance_btc": balance as f64 / 100_000_000.0,
    })).into_response()
}
