use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use crate::contexts::merchant::application::use_cases::configure_webhook::{
    ConfigureWebhookUseCase, ConfigureWebhookInput
};

#[derive(Deserialize)]
pub struct ConfigureWebhookRequest {
    pub lightning_address: String,
    pub webhook_url: String,
}

pub async fn configure_webhook_handler(
    State(use_case): State<Arc<ConfigureWebhookUseCase>>,
    Json(body): Json<ConfigureWebhookRequest>,
) -> impl IntoResponse {
    match use_case.execute(ConfigureWebhookInput {
        lightning_address: body.lightning_address,
        webhook_url: body.webhook_url,
    }).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Webhook configured successfully" }))
        ).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
