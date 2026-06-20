use std::collections::HashMap;

use anyhow::Ok;
use tokio::sync::Mutex;

use crate::{
    application::rooms::{
        Error, RoomFetchQuery, Rooms, RoomsFetchQuery,
    },
    domain::{
        identifiable::Identifiable,
        room::{Room, RoomId},
    },
};

pub struct MockRooms {
    pub rooms: Mutex<HashMap<RoomId, Room>>,
}

#[async_trait::async_trait]
impl Rooms for MockRooms {
    async fn add_room(
        &self,
        payload: Identifiable<Room>,
    ) -> anyhow::Result<()> {
        self.rooms
            .lock()
            .await
            .insert(RoomId(payload.id), payload.data);

        Ok(())
    }

    async fn get_room(
        &self,
        query: RoomFetchQuery,
    ) -> anyhow::Result<Identifiable<Room>> {
        match query {
            RoomFetchQuery::Id(uuid) => {
                match self
                    .rooms
                    .lock()
                    .await
                    .get(&RoomId(uuid))
                    .cloned()
                {
                    Some(room) => {
                        return Ok(
                            Identifiable::new_known(
                                uuid, room,
                            ),
                        );
                    }
                    None => {
                        return Err(anyhow::anyhow!(
                            "No room found with ID {}",
                            uuid
                        ));
                    }
                };
            }
            RoomFetchQuery::Name(name) => match self
                .rooms
                .lock()
                .await
                .iter()
                .find(|(_, room)| room.title == name)
                .map(|(id, room)| {
                    Identifiable::new_known(
                        *id,
                        room.clone(),
                    )
                }) {
                Some(room) => return Ok(room),
                None => {
                    return Err(anyhow::anyhow!(
                        "No room found with name: {}",
                        name
                    ));
                }
            },
        }
    }

    async fn get_rooms(
        &self,
        query: RoomsFetchQuery,
    ) -> anyhow::Result<Identifiable<Room>> {
        todo!()
    }
}
