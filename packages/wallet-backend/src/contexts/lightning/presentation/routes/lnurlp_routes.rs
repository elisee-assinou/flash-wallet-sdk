use axum::{Router, routing::get};
use super::super::handlers::lnurlp_handler::{
    lnurlp_handler,
    lnurlp_invoice_handler,
    LnurlpState,
};

pub fn lnurlp_router(state: LnurlpState) -> Router {
    Router::new()
        .route("/.well-known/lnurlp/:username", get(lnurlp_handler))
        .route("/api/v1/lnurlp/:username/invoice", get(lnurlp_invoice_handler))
        .with_state(state)
}
