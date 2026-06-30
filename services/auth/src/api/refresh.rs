use argon2::{
    Algorithm::Argon2d, Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    Json, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    state::Global,
    token::{
        append_access_token, claims::REFRESH_TOKEN_COOKIE,
        issue_tokens,
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

    let access_duration = chrono::Duration::from_std(
        token_durations.access_token_duration,
    )
    .expect("access_token_duration out of chrono range");

    let jar = match jwt {
        Ok(jwt) => append_access_token(
            token_issuer,
            access_duration,
            jwt.claims.refresh_token.user_id,
            jar,
        ),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    Ok((StatusCode::OK, jar, "New access token issued"))
}
