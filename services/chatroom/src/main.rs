use std::collections::HashMap;

use chatroom::{
    application::{
        mocks::mock_rooms::MockRooms,
        rooms::{RoomFetchQuery, Rooms},
    },
    domain::{identifiable::Identifiable, room::Room},
};
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rooms = MockRooms {
        rooms: Mutex::new(HashMap::new()),
    };

    let id = Uuid::now_v7();

    let _ = rooms
        .add_room(Identifiable {
            id,
            data: Room {
                title: String::from("general"),
                description: String::from("data"),
            },
        })
        .await;

    let room = rooms
        .get_room(RoomFetchQuery::Name(
            "general".to_string(),
        ))
        .await;
    dbg!(&room);
    Ok(())
}
