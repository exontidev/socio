use serde::{Deserialize, Serialize};

use crate::user::user::UserId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub user: UserId,
    pub content: String,
}
