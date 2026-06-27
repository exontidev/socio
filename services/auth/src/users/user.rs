#[derive(Debug, Clone, Copy)]
pub struct WithoutPassword;

#[derive(Debug, Clone)]
pub struct WithPlainPassword {
    pub plain: String,
}
#[derive(Debug, Clone)]
pub struct WithHashedPassword {
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct User<Secret> {
    pub username: String,
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
