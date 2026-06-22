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
)]
pub struct UserId(pub Uuid);

#[derive(Debug)]
pub struct User {
    pub name: String,
}
