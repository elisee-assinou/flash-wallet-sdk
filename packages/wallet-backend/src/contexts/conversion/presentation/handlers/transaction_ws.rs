use std::sync::Arc;
use axum::{
    extract::{State, Path, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::IntoResponse,
};
use tokio::time::{sleep, Duration};
use crate::contexts::conversion::application::use_cases::get_transaction_status::{
    GetTransactionStatusUseCase,
    GetTransactionStatusInput,
};

pub async fn transaction_ws_handler(
    ws: WebSocketUpgrade,
    State(use_case): State<Arc<GetTransactionStatusUseCase>>,
    Path(transaction_id): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, use_case, transaction_id))
}

async fn handle_socket(
    mut socket: WebSocket,
    use_case: Arc<GetTransactionStatusUseCase>,
    transaction_id: String,
) {
    loop {
        let input = GetTransactionStatusInput {
            transaction_id: transaction_id.clone(),
        };

        match use_case.execute(input).await {
            Ok(output) => {
                let msg = serde_json::json!({
                    "transaction_id": output.transaction_id,
                    "status": output.status,
                    "is_completed": output.is_completed,
                });

                if socket.send(Message::Text(msg.to_string())).await.is_err() {
                    break;
                }

                if output.is_completed {
                    break;
                }
            }
            Err(e) => {
                let msg = serde_json::json!({
                    "error": e.to_string()
                });
                let _ = socket.send(Message::Text(msg.to_string())).await;
                break;
            }
        }

        // Poll toutes les 2 secondes
        sleep(Duration::from_secs(2)).await;
    }
}
