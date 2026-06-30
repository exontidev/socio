use argon2::{
    Algorithm::Argon2d, Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    Json, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    state::Global,
    token::append_tokens,
    users::{
        user::{User, WithHashedPassword, WithPlainPassword},
        user_storage::{Query, UserStorage},
    },
};

pub async fn handle_login(
    State(global): State<Global>,
    jar: CookieJar,
    Json(user): Json<User<WithPlainPassword>>,
) -> Result<impl IntoResponse, StatusCode> {
    let token_issuer = global.token_issuer.clone();
    let token_durations = &global.config.tokens;

    let matching_user = global
        .users
        .get::<WithHashedPassword>(Query::Name(user.username))
        .await;

    let matching_user = match matching_user {
        Ok(matching_user) => matching_user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let hash = matching_user.data.secret.hash;
    let parsed_hash = match PasswordHash::new(&hash) {
        Ok(parsed_hash) => parsed_hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let verified = Argon2::default()
        .verify_password(
            user.secret.password.as_bytes(),
            &parsed_hash,
        )
        .is_ok();

    let jar = append_tokens(
        token_issuer,
        jar,
        matching_user.uuid,
        token_durations,
    );

    match verified {
        true => Ok((StatusCode::NO_CONTENT, jar)),
        false => Err(StatusCode::UNAUTHORIZED),
    }
}
