use std::sync::Arc;

use dashmap::DashMap;
use tokio::{sync, task::JoinHandle};

use crate::{
    requests::WebSocketMessage,
    room::{message::Message, room::RoomId},
};

type Result<T> = core::result::Result<T, Error>;

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
        room: RoomId,
    ) -> Result<tokio::sync::broadcast::Sender<WebSocketMessage>>
    {
        match self.rooms.get(&room) {
            Some(room) => return Ok(room.clone()),
            None => Err(Error::NotFound(room)),
        }
    }

    pub fn subscribe(
        &self,
        user: tokio::sync::mpsc::UnboundedSender<WebSocketMessage>,
        room: RoomId,
    ) -> Result<JoinHandle<()>> {
        if let Ok(room) = self.get(room) {
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

        Err(Error::NotFound(room))
    }

    pub fn send(&self, message: Message) -> Result<usize> {
        if let Ok(room) = self.get(message.room) {
            let message = Arc::new(message);

            return room
                .send(WebSocketMessage::NewMessage(message))
                .map_err(|_| Error::NoReceivers);
        }

        Err(Error::NotFound(message.room))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Room not found: {0}")]
    NotFound(RoomId),
    #[error("No active receivers")]
    NoReceivers,
}
