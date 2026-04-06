use std::sync::Arc;
use axum::{routing::post, Router};
use crate::contexts::merchant::{
    application::use_cases::create_payment::CreatePaymentUseCase,
    presentation::handlers::payment_handler::create_payment_handler,
};

pub fn merchant_router(create_payment: Arc<CreatePaymentUseCase>) -> Router {
    Router::new()
        .route("/api/v1/merchant/payment",
            post(create_payment_handler).with_state(create_payment))
}
