use argon2::{
    Algorithm::Argon2d, Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    Json, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::{
    state::Global,
    token::{
        append_access_token, append_tokens,
        claims::REFRESH_TOKEN_COOKIE,
    },
    users::{
        user::{User, WithHashedPassword, WithPlainPassword},
        user_storage::{Query, UserStorage},
    },
};

pub async fn handle_refresh(
    State(global): State<Global>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let token_verifier = global.token_verifier.clone();
    let token_durations = &global.config.tokens;

    let token_issuer = global.token_issuer.clone();

    let refresh_token_row = jar
        .get(REFRESH_TOKEN_COOKIE)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let refresh_token = refresh_token_row.value();
    let jwt = token_verifier.verify_refresh_token(refresh_token);

    let jar = match jwt {
        Ok(jwt) => append_tokens(
            token_issuer,
            jar,
            jwt.claims.refresh_token.user_id,
            token_durations,
        ),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    Ok((StatusCode::NO_CONTENT, jar))
}
