use std::sync::Arc;
use axum::{routing::post, Router};
use crate::contexts::merchant::{
    application::use_cases::{
        create_payment::CreatePaymentUseCase,
        configure_webhook::ConfigureWebhookUseCase,
    },
    presentation::handlers::{
        payment_handler::create_payment_handler,
        webhook_handler::configure_webhook_handler,
    },
};

pub fn merchant_router(
    create_payment: Arc<CreatePaymentUseCase>,
    configure_webhook: Arc<ConfigureWebhookUseCase>,
) -> Router {
    Router::new()
        .route("/api/v1/merchant/payment",
            post(create_payment_handler).with_state(create_payment))
        .route("/api/v1/merchant/webhook",
            post(configure_webhook_handler).with_state(configure_webhook))
}
