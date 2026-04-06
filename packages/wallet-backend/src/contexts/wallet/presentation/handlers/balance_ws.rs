use std::sync::Arc;
use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::IntoResponse,
};
use tokio::time::{sleep, Duration};
use crate::contexts::wallet::application::use_cases::get_balance::GetBalanceUseCase;

pub async fn balance_ws_handler(
    ws: WebSocketUpgrade,
    State(use_case): State<Arc<GetBalanceUseCase>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, use_case))
}

async fn handle_socket(
    mut socket: WebSocket,
    use_case: Arc<GetBalanceUseCase>,
) {
    loop {
        match use_case.execute().await {
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

        // Envoie la balance toutes les 3 secondes
        sleep(Duration::from_secs(10)).await;
    }
}
