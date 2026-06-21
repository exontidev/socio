use std::net::IpAddr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::{sync::broadcast, sync::mpsc, task::JoinHandle};

use crate::{
    requests::{Request, WebSocketMessage},
    room::{room::RoomId, rooms::Rooms},
    user::user::UserId,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub rooms: Rooms,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    room: RoomConfig,
    network: NetworkConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomConfig {
    max_rooms: u8,
}

impl AppState {
    pub fn subscribe(
        &self,
        user: mpsc::UnboundedSender<WebSocketMessage>,
        room: RoomId,
    ) -> anyhow::Result<JoinHandle<()>> {
        if let Ok(room) = self.rooms.get(&room) {
            return Ok(tokio::spawn({
                let room = room.clone();
                let mut stream = room.subscribe();

                async move {
                    while let Ok(data) = stream.recv().await {
                        let _ = user.send(data);
                    }
                }
            }));
        }

        Err(anyhow!(
            "Couldn't subscribe to the room {} not found",
            room.0
        ))
    }
}
