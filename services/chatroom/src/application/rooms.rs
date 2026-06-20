use anyhow::Result;

use crate::domain::room::Room;
use uuid::Uuid;

pub enum RoomQuery {
    Id(Uuid),
    Name(String),
}

pub enum RoomsQuery {
    Ids(Vec<Uuid>),
    User(Uuid),
}

#[async_trait::async_trait]
pub trait Rooms {
    fn get_room(query: RoomQuery) -> Result<Room>;
    fn get_rooms(query: RoomQuery) -> Result<Room>;
}
