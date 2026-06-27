use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    helper::Identifiable,
    users::{
        user::{User, WithHashedPassword},
        user_storage::{
            Error::{UserNotFoundById, UserNotFoundByName},
            Query, Result, UserStorage,
        },
    },
};

pub struct MemoryUserStorage {
    pub map: Mutex<HashMap<Uuid, User<WithHashedPassword>>>,
}

impl MemoryUserStorage {
    pub fn empty() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl UserStorage for MemoryUserStorage {
    async fn add(
        &self,
        user: Identifiable<User<WithHashedPassword>>,
    ) -> Result<()> {
        let id = user.uuid;
        let user = user.data;

        self.map.lock().await.insert(id, user);
        Ok(())
    }

    async fn get<Visibility>(
        &self,
        query: Query,
    ) -> Result<User<Visibility>>
    where
        User<Visibility>: From<User<WithHashedPassword>>,
    {
        let user = match query {
            Query::Id(uuid) => self
                .map
                .lock()
                .await
                .get(&uuid)
                .cloned()
                .ok_or(UserNotFoundById(uuid))?,
            Query::Name(name) => self
                .map
                .lock()
                .await
                .iter()
                .find(|(_, u)| u.username == name)
                .map(|(_, u)| u.clone())
                .ok_or(UserNotFoundByName(name))?,
        };

        Ok(user.into())
    }

    async fn remove(&self, user_id: Uuid) -> Result<()> {
        self.map.lock().await.remove(&user_id);
        Ok(())
    }
}
