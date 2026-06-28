use crate::token::claims::{AccessToken, RefreshToken};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

pub type JWT = String;

pub struct TokenIssuer {
    pub algorithm: Algorithm,
    pub private: EncodingKey,
}

impl TokenIssuer {
    pub fn issue_access_token(&self, claims: AccessToken) -> JWT {
        let header = &Header::new(self.algorithm);
        encode(header, &claims, &self.private).unwrap()
    }

    pub fn issue_refresh_token(&self, claims: RefreshToken) -> JWT {
        let header = &Header::new(self.algorithm);
        encode(header, &claims, &self.private).unwrap()
    }
}
