use std::sync::Arc;

use crate::{
    config::Config,
    mocks::user_storage::MemoryUserStorage,
    token::{issuer::TokenIssuer, verifier::TokenVerifier},
    users::user_storage::UserStorage,
};

pub type Global = State<MemoryUserStorage>;

pub struct State<Users: UserStorage + Send + Sync + 'static> {
    pub config: Arc<Config>,
    pub users: Arc<Users>,

    pub token_issuer: Arc<TokenIssuer>,
    pub token_verifier: Arc<TokenVerifier>,
}
impl<Users: UserStorage + Send + Sync + 'static> Clone
    for State<Users>
{
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
            config: self.config.clone(),
            token_issuer: self.token_issuer.clone(),
            token_verifier: self.token_verifier.clone(),
        }
    }
}
