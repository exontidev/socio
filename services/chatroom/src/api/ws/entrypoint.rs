use std::sync::Arc;

use crate::api::{
    state::AppState,
    ws::{
        income::handle_income, io::WebSocketMessage,
        types::WebSocketSender,
    },
};
use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc::UnboundedReceiver;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| {
        println!("Error upgrading websocket: {}", error)
    })
    .on_upgrade(|ws| handle_socket(ws, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (relay_tx, relay_rx) =
        tokio::sync::mpsc::unbounded_channel::<WebSocketMessage>();

    let (socket_tx, socket_rx) = socket.split();

    tokio::join!(
        handle_income(socket_rx, relay_tx, state.clone()),
        handle_outcome(socket_tx, relay_rx, state)
    );
}

async fn handle_outcome(
    mut sender: WebSocketSender,
    mut relay_rx: UnboundedReceiver<WebSocketMessage>,
    _state: Arc<AppState>,
) {
    while let Some(event) = relay_rx.recv().await {
        let _ = sender
            .send(serde_json::to_string(&event).unwrap().into())
            .await;
    }
}
