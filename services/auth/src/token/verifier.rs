use crate::token::claims::{AccessToken, RefreshToken};
use jsonwebtoken::{
    Algorithm, DecodingKey, Header, TokenData, Validation, decode,
    encode, errors::Error,
};

pub type JWT = String;

pub struct TokenVerifier {
    pub algorithm: Algorithm,
    pub public: DecodingKey,
}

impl TokenVerifier {
    pub fn verify_access_token(
        &self,
        jwt: &str,
    ) -> Result<TokenData<AccessToken>, Error> {
        let validation = Validation::new(Algorithm::EdDSA);

        Ok(decode::<AccessToken>(
            jwt.as_bytes(),
            &self.public,
            &validation,
        )?)
    }

    pub fn verify_refresh_token(
        &self,
        jwt: &str,
    ) -> Result<TokenData<RefreshToken>, Error> {
        let validation = Validation::new(Algorithm::EdDSA);

        Ok(decode::<RefreshToken>(
            jwt.as_bytes(),
            &self.public,
            &validation,
        )?)
    }
}
