use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    api::ws::status_codes::{NotifyCode, WebSocketError},
    room::{message::Message, room::RoomId},
    user::user::UserId,
};

pub type RequestId = u32;
type AtomicMessage = Arc<Message>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum WebSocketMessage {
    NewMessage(AtomicMessage),
    NewParticipant(UserId),

    Notify {
        request_id: Option<RequestId>,
        code: NotifyCode,
    },
    Error {
        request_id: Option<RequestId>,
        code: WebSocketError,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum UserAction {
    JoinRoom { room: RoomId },
    LeaveRoom { room: RoomId },
    LeaveAllRooms,

    SendMessage(Message),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub request: RequestId,
    pub data: T,
}
