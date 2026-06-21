use std::sync::Arc;

use axum::extract::ws::{CloseCode, Message, close_code::INVALID};
use futures::{SinkExt, StreamExt};
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};

use crate::{
    api::{
        state::AppState,
        ws::{
            WebSocketReceiver,
            error::{NOT_A_UTF_8, PARSE_FAILED, ROOM_DOESNT_EXIST},
        },
    },
    requests::{Request, WebSocketMessage},
    room::room::RoomId,
};

type Connections = Vec<(RoomId, JoinHandle<()>)>;
type GlobalState = Arc<AppState>;

pub struct ConnectionHandler {
    pub connections: Connections,
    pub max_rooms: u8,

    pub state: GlobalState,
    pub relay_tx: UnboundedSender<WebSocketMessage>,
}

impl ConnectionHandler {
    pub fn new(
        state: GlobalState,
        relay_tx: UnboundedSender<WebSocketMessage>,
    ) -> Self {
        Self {
            connections: vec![],
            max_rooms: 5, // in config later
            state,
            relay_tx: relay_tx,
        }
    }

    pub fn join(&mut self, room: RoomId) {
        match self.state.subscribe(self.relay_tx.clone(), room) {
            Ok(handle) => {
                self.connections.push((room, handle));
            }

            Err(_) => {
                let _ = self
                    .relay_tx
                    .send(WebSocketMessage::Error(ROOM_DOESNT_EXIST));

                return;
            }
        }
    }

    pub fn leave(&mut self, room: RoomId) {
        self.connections.retain(|(id, handle)| {
            if *id == room {
                handle.abort();
                false
            } else {
                true
            }
        });
    }

    pub fn leave_all(&mut self) {
        self.connections.iter().for_each(|handle| {
            handle.1.abort();
        });
        dbg!("aborted all");
        self.connections.clear();
    }

    pub fn error(&self, code: CloseCode) {
        let _ = self.relay_tx.send(WebSocketMessage::Error(code));
    }
}

pub async fn handle_income(
    mut receiver: WebSocketReceiver,
    relay_tx: UnboundedSender<WebSocketMessage>,
    state: GlobalState,
) {
    let mut handler = ConnectionHandler::new(state, relay_tx.clone());

    while let Some(result) = receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        let text = match message {
            Message::Text(utf8_bytes) => utf8_bytes.to_string(),

            Message::Close(_) => {
                handler.error(INVALID);
                continue;
            }

            Message::Ping(_) | Message::Pong(_) => continue,

            _ => {
                handler.error(NOT_A_UTF_8);
                continue;
            }
        };

        let request = match serde_json::from_str::<Request>(&text) {
            Ok(request) => request,
            Err(_) => {
                handler.error(PARSE_FAILED);
                continue;
            }
        };

        match request {
            Request::JoinRoom { room } => handler.join(room),

            Request::LeaveRoom { room } => handler.leave(room),

            Request::LeaveAllRooms => {
                handler.leave_all();
            }

            _ => (),
        }
    }

    handler.leave_all();
}
