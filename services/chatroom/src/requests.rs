use axum::extract::ws::CloseCode;
use serde::{Deserialize, Serialize};

use crate::{
    room::{message::Message, room::RoomId},
    user::user::UserId,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebSocketMessage {
    NewMessage(Message),
    NewParticipant(UserId),

    Error(CloseCode),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    JoinRoom { room: RoomId },
    LeaveRoom { room: RoomId },
    LeaveAllRooms,
    SendMessage(Message),
}
