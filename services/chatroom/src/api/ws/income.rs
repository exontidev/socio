use axum::extract::ws::Message as WsMessage;
use futures::StreamExt;
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};

use crate::{
    api::ws::{
        RelaySender, WebSocketReceiver,
        notifier::Notifier,
        staus_codes::{NotifyCode, WebSocketError},
    },
    helper::GlobalState,
    requests::{Request, RequestId, UserAction, WebSocketMessage},
    room::{message::Message, room::RoomId},
};

type Connections = Vec<(RoomId, JoinHandle<()>)>;
type Result = core::result::Result<NotifyCode, WebSocketError>;

pub struct ConnectionHandler {
    pub connections: Connections,
    pub max_rooms: u8,

    pub state: GlobalState,
    pub relay_tx: RelaySender,
}

impl ConnectionHandler {
    pub fn new(state: GlobalState, relay_tx: RelaySender) -> Self {
        Self {
            connections: vec![],
            max_rooms: state.config.room.max_rooms,
            state,
            relay_tx,
        }
    }

    pub fn handle(&mut self, action: UserAction) -> Result {
        match action {
            UserAction::JoinRoom { room } => self.join(room),
            UserAction::LeaveRoom { room } => self.leave(room),
            UserAction::LeaveAllRooms => self.leave_all(),
            UserAction::SendMessage(message) => self.send(message),

            _ => Err(WebSocketError::ActionDoesNotExist),
        }
    }

    fn join(&mut self, room: RoomId) -> Result {
        if self.connections.len() >= self.max_rooms as usize {
            return Err(WebSocketError::RoomLimitReached);
        }

        match self.state.rooms.subscribe(self.relay_tx.clone(), room)
        {
            Ok(handle) => {
                self.connections.push((room, handle));
                Ok(NotifyCode::RoomJoined)
            }

            Err(_) => Err(WebSocketError::NoRoomFound),
        }
    }
    fn leave(&mut self, room: RoomId) -> Result {
        if self.connections.is_empty() {
            return Err(WebSocketError::UserIsntInAnyRoom);
        }

        self.connections.retain(|(id, handle)| {
            if *id == room {
                handle.abort();
                false
            } else {
                true
            }
        });

        Ok(NotifyCode::RoomLeftGracefully)
    }
    fn leave_all(&mut self) -> Result {
        if self.connections.is_empty() {
            return Err(WebSocketError::UserIsntInAnyRoom);
        }

        self.connections.iter().for_each(|handle| {
            handle.1.abort();
        });

        self.connections.clear();

        Ok(NotifyCode::AllRoomsLeftGracefully)
    }
    fn send(&self, message: Message) -> Result {
        let result = self.state.rooms.send(message);

        match result {
            Ok(_) => Ok(NotifyCode::MessageRequestSent),
            Err(_) => Err(WebSocketError::RoomIsEmpty),
        }
    }
}

impl Drop for ConnectionHandler {
    fn drop(&mut self) {
        self.connections.iter().for_each(|(_, handle)| {
            handle.abort();
        });
    }
}

pub async fn handle_income(
    mut receiver: WebSocketReceiver,
    relay_tx: RelaySender,
    state: GlobalState,
) {
    let mut handler = ConnectionHandler::new(state, relay_tx.clone());
    let mut notifier = Notifier {
        id: None,
        tx: relay_tx.clone(),
    };

    while let Some(result) = receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        let text = match message {
            WsMessage::Text(utf8_bytes) => utf8_bytes.to_string(),

            WsMessage::Close(_) => {
                continue;
            }

            WsMessage::Ping(_) | WsMessage::Pong(_) => continue,

            _ => {
                notifier.error(WebSocketError::NonUTF8Request);
                continue;
            }
        };

        let (id, action) = match serde_json::from_str::<
            Request<UserAction>,
        >(&text)
        {
            Ok(Request { request, data }) => (request, data),
            Err(_) => {
                notifier.error(WebSocketError::InvalidRequest);
                continue;
            }
        };

        notifier.set_id(id);

        let result = handler.handle(action);
        notifier.result(result);
    }
}
