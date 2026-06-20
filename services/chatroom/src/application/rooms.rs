use anyhow::Result;

use crate::domain::{
    identifiable::Identifiable,
    room::{Room, RoomId},
};
use uuid::Uuid;

pub enum RoomFetchQuery {
    Id(Uuid),
    Name(String),
}

pub enum RoomsFetchQuery {
    Ids(Vec<Uuid>),
    User(Uuid),
    TopMessages { cursor: u32 },
}

#[async_trait::async_trait]
pub trait Rooms {
    async fn add_room(
        &self,
        room: Identifiable<Room>,
    ) -> Result<()>;

    async fn get_room(
        &self,
        query: RoomFetchQuery,
    ) -> Result<Identifiable<Room>>;
    async fn get_rooms(
        &self,
        query: RoomsFetchQuery,
    ) -> Result<Identifiable<Room>>;
}

pub enum Error {
    NotFound,
    RoomAlreadyExists,
}
