use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
)]
pub struct RoomId(pub Uuid);

pub struct Room {
    name: String,
    description: String,
}
