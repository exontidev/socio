use uuid::Uuid;

pub const REFRESH_TOKEN_COOKIE: &'static str = "refresh_token";
pub const ACCESS_TOKEN_COOKIE: &'static str = "access_token";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RefreshToken {
    pub refresh_token: RefreshTokenInternals,
    pub exp: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenInternals {
    pub user_id: Uuid,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessToken {
    pub access_token: AccessTokenInternals,
    pub exp: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenInternals {
    pub user_id: Uuid,
}
