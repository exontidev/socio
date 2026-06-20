use uuid::Uuid;

#[derive(
    Debug, Hash, PartialEq, Eq, PartialOrd, Clone, Copy,
)]
pub struct RoomId(pub Uuid);

impl From<RoomId> for Uuid {
    fn from(value: RoomId) -> Uuid {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub title: String,
    pub description: String,
}
