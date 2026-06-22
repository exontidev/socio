use crate::{
    api::ws::{
        status_codes::{NotifyCode, WebSocketError},
        types::{RelaySender, WebSocketResult},
    },
    helper::GlobalState,
    room::{
        message::Message, realtime_rooms::RoomHandle, room::RoomId,
    },
};

type Connections = Vec<(RoomId, RoomHandle)>;

pub struct RoomInterface {
    pub active_rooms: Connections,
    pub max_rooms: u8,

    pub relay_tx: RelaySender,
    pub state: GlobalState,
}

impl RoomInterface {
    pub fn new(state: GlobalState, relay_tx: RelaySender) -> Self {
        Self {
            active_rooms: vec![],
            max_rooms: state.config.room.max_rooms,
            relay_tx,
            state,
        }
    }

    pub fn join(&mut self, room_id: RoomId) -> WebSocketResult {
        if self.active_rooms.len() >= self.max_rooms as usize {
            return Err(WebSocketError::RoomLimitReached);
        }

        match self.state.rooms.get(room_id) {
            Ok(room) => {
                let joined = room.join(self.relay_tx.clone());
                self.active_rooms.push((room_id, joined));

                Ok(NotifyCode::RoomJoined)
            }

            Err(_) => Err(WebSocketError::NoRoomFound),
        }
    }
    pub fn send(&self, message: Message) -> WebSocketResult {
        let room = self
            .active_rooms
            .iter()
            .find(|(id, _)| *id == message.room)
            .map(|(_, handle)| handle);

        let room = match room {
            Some(room) => room,
            None => {
                return Err(
                    WebSocketError::UserIsNotConnectedToGivenRoom,
                );
            }
        };

        let result = room.message(message);

        match result {
            Ok(_) => Ok(NotifyCode::MessageRequestSent),
            Err(_) => Err(WebSocketError::RoomIsEmpty),
        }
    }
    pub fn leave_all(&mut self) -> WebSocketResult {
        for (_, room) in self.active_rooms.drain(..) {
            room.leave();
        }

        Ok(NotifyCode::AllRoomsLeftGracefully)
    }
    pub fn leave(&mut self, room: RoomId) -> WebSocketResult {
        if self.active_rooms.is_empty() {
            return Err(WebSocketError::UserIsntInAnyRoom);
        }

        if let Some(pos) =
            self.active_rooms.iter().position(|(id, _)| *id == room)
        {
            let (_, handle) = self.active_rooms.remove(pos);
            handle.leave();
        }

        Ok(NotifyCode::RoomLeftGracefully)
    }
}
