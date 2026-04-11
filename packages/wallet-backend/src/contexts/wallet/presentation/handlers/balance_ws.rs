use std::sync::Arc;
use axum::{
    extract::{State, WebSocketUpgrade, Query, ws::{WebSocket, Message}},
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use crate::contexts::wallet::application::use_cases::get_balance::GetBalanceUseCase;

#[derive(Deserialize)]
pub struct BalanceWsQuery {
    pub lightning_address: Option<String>,
}

pub async fn balance_ws_handler(
    ws: WebSocketUpgrade,
    State(use_case): State<Arc<GetBalanceUseCase>>,
    Query(query): Query<BalanceWsQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, use_case, query.lightning_address))
}

async fn handle_socket(
    mut socket: WebSocket,
    use_case: Arc<GetBalanceUseCase>,
    lightning_address: Option<String>,
) {
    loop {
        match use_case.execute_for(lightning_address.as_deref()).await {
            Ok(balance) => {
                if socket.send(Message::Text(balance.to_string())).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                let msg = serde_json::json!({"error": e.to_string()});
                let _ = socket.send(Message::Text(msg.to_string())).await;
                break;
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}
