use uuid::Uuid;

pub struct RoomId(pub Uuid);

#[derive(Debug)]
pub struct Room {
    pub title: String,
    pub description: String,
}
