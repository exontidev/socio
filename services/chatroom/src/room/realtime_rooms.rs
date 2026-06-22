use std::sync::Arc;

use dashmap::DashMap;
use tokio::task::JoinHandle;

use crate::{
    api::ws::requests::WebSocketMessage,
    room::{message::Message, room::RoomId},
};

type Result<T> = core::result::Result<T, Error>;

type UserEventSender =
    tokio::sync::mpsc::UnboundedSender<WebSocketMessage>;
type RoomEventBroadcast =
    tokio::sync::broadcast::Sender<WebSocketMessage>;

#[derive(Debug, Clone)]
pub struct Rooms {
    pub rooms: DashMap<RoomId, RealtimeRoom>,
}

impl Rooms {
    pub fn get(&self, room: RoomId) -> Result<RealtimeRoom> {
        match self.rooms.get(&room) {
            Some(room) => return Ok(room.clone()),
            None => Err(Error::NotFound(room)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RealtimeRoom {
    pub event_broadcast: RoomEventBroadcast,
}

impl RealtimeRoom {
    pub fn join(&self, user_tx: UserEventSender) -> RoomHandle {
        let handle = tokio::spawn({
            let room = self.event_broadcast.clone();
            let mut stream = room.subscribe();

            async move {
                while let Ok(data) = stream.recv().await {
                    let _ = user_tx.send(data);
                }
            }
        });

        RoomHandle {
            event_broadcast: self.event_broadcast.clone(),
            handle,
        }
    }
}

pub struct RoomHandle {
    pub event_broadcast: RoomEventBroadcast,
    pub handle: JoinHandle<()>,
}

impl RoomHandle {
    pub fn message(&self, message: Message) -> Result<usize> {
        let message = Arc::new(message);

        self.event_broadcast
            .send(WebSocketMessage::NewMessage(message))
            .map_err(|_| Error::NoReceivers)
    }

    pub fn leave(self) {}
}

impl Drop for RoomHandle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Room not found: {0}")]
    NotFound(RoomId),
    #[error("No active receivers")]
    NoReceivers,
}
