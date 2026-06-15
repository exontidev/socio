use std::sync::Arc;

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::{
    sync::{broadcast, mpsc},
    task::spawn_blocking,
};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(512);

    spawn_blocking({
        let tx = tx.clone();

        move || loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let _ = tx.send(buffer);
        }
    });

    let app = Router::new()
        .route("/test", get(websocket_handler))
        .with_state(tx.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    State(tx): State<broadcast::Sender<String>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| println!("Error upgrading websocket: {}", error))
        .on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, global: broadcast::Sender<String>) {
    let (sender, receiver) = socket.split();
    let (local_tx, local_rx) = tokio::sync::mpsc::channel::<Message>(512);

    tokio::join!(
        handle_sending(sender, local_rx),
        handle_global(global.clone(), local_tx.clone()),
        handle_incoming(receiver, global)
    );
}

async fn handle_global(global: broadcast::Sender<String>, distributor_tx: mpsc::Sender<Message>) {
    let mut stream = global.subscribe();

    while let Ok(text) = stream.recv().await {
        let _ = distributor_tx.send(text.into()).await;
    }
}

async fn handle_incoming(mut stream: SplitStream<WebSocket>, global: broadcast::Sender<String>) {
    while let Some(text) = stream.next().await {
        if let Ok(text) = text {
            let _ = global.send(text.into_text().unwrap().to_string());
        }
    }
}

async fn handle_sending(
    mut sender: SplitSink<WebSocket, Message>,
    mut rx: mpsc::Receiver<Message>,
) {
    while let Some(data) = rx.recv().await {
        let _ = sender.send(data).await;
    }
}
