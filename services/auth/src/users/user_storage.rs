use uuid::Uuid;

use crate::{
    helper::Identifiable,
    users::user::{
        User, WithHashedPassword, WithPlainPassword, WithoutPassword,
    },
};

pub enum Query {
    Name(String),
    Id(Uuid),
}

pub type Result<T> = core::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait UserStorage: Sync + Send {
    async fn add(
        &self,
        user: Identifiable<User<WithHashedPassword>>,
    ) -> Result<()>;

    async fn get<Visibility>(
        &self,
        query: Query,
    ) -> Result<Identifiable<User<Visibility>>>
    where
        User<Visibility>: From<User<WithHashedPassword>>;

    async fn remove(&self, user_id: Uuid) -> Result<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User {0} not found")]
    UserNotFoundById(Uuid),
    #[error("User {0} not found")]
    UserNotFoundByName(String),

    #[error("User with name {0} already exists")]
    UserAlreadyExistsWithName(String),
}
