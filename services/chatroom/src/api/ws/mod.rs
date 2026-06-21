pub mod error;

use std::sync::Arc;

use crate::{
    api::{
        state::AppState,
        ws::error::{NOT_A_UTF_8, ROOM_DOESNT_EXIST},
    },
    requests::{Request, WebSocketMessage},
    room::room::RoomId,
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{
            CloseCode, Message, WebSocket,
            close_code::INVALID,
        },
    },
    response::IntoResponse,
};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::{
    sync::{
        self,
        mpsc::{UnboundedReceiver, UnboundedSender},
        watch,
    },
    task::JoinHandle,
};

type WebSocketSender = SplitSink<WebSocket, Message>;
type WebSocketReceiver = SplitStream<WebSocket>;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| {
        println!("Error upgrading websocket: {}", error)
    })
    .on_upgrade(|ws| handle_socket(ws, state))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<AppState>,
) {
    let (relay_tx, relay_rx) =
        tokio::sync::mpsc::unbounded_channel::<
            WebSocketMessage,
        >();

    let (socket_tx, socket_rx) = socket.split();

    tokio::join!(
        handle_income(socket_rx, relay_tx, state.clone()),
        handle_outcome(socket_tx, relay_rx, state)
    );
}

async fn handle_income(
    mut receiver: WebSocketReceiver,
    relay_tx: UnboundedSender<WebSocketMessage>,
    state: Arc<AppState>,
) {
    let mut connections = vec![];

    while let Some(result) = receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        let text = match message {
            Message::Text(utf8_bytes) => {
                utf8_bytes.to_string()
            }

            Message::Close(_) => {
                let _ = relay_tx
                    .send(WebSocketMessage::Error(INVALID));
                continue;
            }

            Message::Ping(_) | Message::Pong(_) => continue,

            _ => {
                let _ = relay_tx.send(
                    WebSocketMessage::Error(NOT_A_UTF_8),
                );
                continue;
            }
        };

        let request =
            match serde_json::from_str::<Request>(&text) {
                Ok(request) => request,
                Err(_) => {
                    let _ = relay_tx.send(
                        WebSocketMessage::Error(INVALID),
                    );
                    continue;
                }
            };

        match request {
            Request::JoinRoom { room } => {
                match state
                    .subscribe(relay_tx.clone(), room)
                {
                    Ok(handle) => {
                        connections.push((room, handle));
                    }
                    Err(_) => {
                        let _ = relay_tx.send(
                            WebSocketMessage::Error(
                                ROOM_DOESNT_EXIST,
                            ),
                        );

                        continue;
                    }
                }
            }

            Request::LeaveRoom { room } => {
                connections.retain(|(id, handle)| {
                    if *id == room {
                        handle.abort();
                        false
                    } else {
                        true
                    }
                });
            }

            Request::LeaveAllRooms => {
                leave_rooms(&connections);
            }

            _ => (),
        }
    }

    leave_rooms(&connections);
}

fn leave_rooms(
    connections: &Vec<(RoomId, JoinHandle<()>)>,
) {
    connections.iter().for_each(|handle| {
        handle.1.abort();
    });
}

async fn handle_outcome(
    mut sender: WebSocketSender,
    mut relay_rx: UnboundedReceiver<WebSocketMessage>,
    _state: Arc<AppState>,
) {
    while let Some(event) = relay_rx.recv().await {
        let _ = sender
            .send(
                serde_json::to_string(&event)
                    .unwrap()
                    .into(),
            )
            .await;
    }
}
