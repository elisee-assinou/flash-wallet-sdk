use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::contexts::wallet::domain::repositories::wallet_config_repository::WalletConfigRepository;
use crate::contexts::wallet::infrastructure::repositories::postgres_wallet_repo::PostgresWalletRepo;
use crate::contexts::lightning::infrastructure::lnd::LndClient;

#[derive(Serialize)]
pub struct LnurlpResponse {
    callback: String,
    #[serde(rename = "maxSendable")]
    max_sendable: u64,
    #[serde(rename = "minSendable")]
    min_sendable: u64,
    metadata: String,
    tag: String,
}

#[derive(Serialize)]
pub struct InvoiceResponse {
    pr: String,
    routes: Vec<String>,
}

#[derive(Deserialize)]
pub struct InvoiceQuery {
    amount: u64,
}

#[derive(Clone)]
pub struct LnurlpState {
    pub wallet_repo: Arc<PostgresWalletRepo>,
    pub lnd_client: Arc<Mutex<LndClient>>,
    pub base_url: String,
}

// GET /.well-known/lnurlp/:username
pub async fn lnurlp_handler(
    Path(username): Path<String>,
    State(state): State<LnurlpState>,
) -> impl IntoResponse {
    let wallet = match state.wallet_repo.find_by_username(&username).await {
        Ok(Some(w)) => w,
        Ok(None) => return (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": format!("User {} not found", username)}))
        ).into_response(),
        Err(e) => return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    };

    let callback = format!("{}/api/v1/lnurlp/{}/invoice", state.base_url, username);
    let metadata = format!(r#"[["text/plain","Flash Wallet - {}"]]"#, username);

    tracing::info!("LNURL-pay request for {} ({})", username, wallet.momo_number().value());

    Json(LnurlpResponse {
        callback,
        max_sendable: 100_000_000,
        min_sendable: 1_000,
        metadata,
        tag: "payRequest".to_string(),
    }).into_response()
}

// GET /api/v1/lnurlp/:username/invoice?amount=50000000
pub async fn lnurlp_invoice_handler(
    Path(username): Path<String>,
    Query(query): Query<InvoiceQuery>,
    State(state): State<LnurlpState>,
) -> impl IntoResponse {
    let wallet = match state.wallet_repo.find_by_username(&username).await {
        Ok(Some(w)) => w,
        Ok(None) => return (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": format!("User {} not found", username)}))
        ).into_response(),
        Err(e) => return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    };

    // Convertit millisatoshis → satoshis
    let amount_sats = query.amount / 1000;

    // Memo contient le numéro MoMo pour le matching
    let memo = format!("flash-wallet:{}", wallet.momo_number().value());

    let mut lnd = state.lnd_client.lock().await;
    let invoice = match lnd.add_invoice(amount_sats as i64, &memo).await {
        Ok(inv) => inv,
        Err(e) => return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ).into_response(),
    };

    tracing::info!(
        "Invoice generated for {} ({}): {} sats",
        username,
        wallet.momo_number().value(),
        amount_sats
    );

    Json(InvoiceResponse {
        pr: invoice,
        routes: vec![],
    }).into_response()
}
