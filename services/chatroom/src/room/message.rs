use serde::{Deserialize, Serialize};

use crate::{room::room::RoomId, user::user::UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub room: RoomId,

    pub user: UserId,
    pub content: String,
}
