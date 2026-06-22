use derive_more::Display;
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
    Display,
)]
pub struct RoomId(pub Uuid);

pub struct Room {
    pub name: String,
    pub description: String,
}
