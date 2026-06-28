use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub exp: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessToken {
    pub user_id: Uuid,
    pub exp: u64,
}
