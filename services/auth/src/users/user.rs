#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy,
)]
pub struct WithoutPassword;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct WithPlainPassword {
    pub password: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct WithHashedPassword {
    pub hash: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct User<Secret> {
    pub username: String,

    #[serde(flatten)]
    pub secret: Secret,
}

impl From<User<WithHashedPassword>> for User<WithoutPassword> {
    fn from(u: User<WithHashedPassword>) -> Self {
        User {
            username: u.username,
            secret: WithoutPassword,
        }
    }
}
