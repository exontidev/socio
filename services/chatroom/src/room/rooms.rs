use dashmap::DashMap;

use crate::{requests::WebSocketMessage, room::room::RoomId};

#[derive(Debug, Clone)]
pub struct Rooms {
    pub rooms: DashMap<
        RoomId,
        tokio::sync::broadcast::Sender<WebSocketMessage>,
    >,
}

impl Rooms {
    pub fn get(
        &self,
        room: &RoomId,
    ) -> anyhow::Result<tokio::sync::broadcast::Sender<WebSocketMessage>>
    {
        match self.rooms.get(&room) {
            Some(room) => return Ok(room.clone()),
            None => Err(anyhow::anyhow!(
                "Couldn't find a room {} to connect",
                room.0
            )),
        }
    }
}
