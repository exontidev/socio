use std::sync::Arc;

use crate::{
    config::TokenDuration,
    helper::Identifiable,
    state::Global,
    token::{
        claims::{AccessToken, RefreshToken},
        issuer::TokenIssuer,
    },
    users::{
        user::{User, WithHashedPassword, WithPlainPassword},
        user_storage::UserStorage,
    },
};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use uuid::Uuid;

const REFRESH_TOKEN_COOKIE: &'static str = "refresh_token";
const ACCESS_TOKEN_COOKIE: &'static str = "access_token";

pub async fn handle_sign(
    State(global): State<Global>,
    jar: CookieJar,
    Json(user): Json<User<WithPlainPassword>>,
) -> Result<impl IntoResponse, StatusCode> {
    let token_issuer = global.token_issuer.clone();
    let user_storage = global.users.clone();
    let token_durations = &global.config.tokens;

    let id = save_user(user_storage, user).await?;
    let cookies =
        issue_tokens(token_issuer, jar, id, token_durations).await;

    Ok((StatusCode::OK, cookies, "Sign-in success"))
}

async fn issue_tokens(
    token_issuer: Arc<TokenIssuer>,
    jar: CookieJar,
    id: Uuid,
    durations: &TokenDuration,
) -> CookieJar {
    let refresh_duration =
        chrono::Duration::from_std(durations.refresh_token_duration)
            .expect("refresh_token_duration out of chrono range");
    let access_duration =
        chrono::Duration::from_std(durations.access_token_duration)
            .expect("access_token_duration out of chrono range");

    let refresh_token =
        token_issuer.issue_refresh_token(RefreshToken {
            user_id: id,
            exp: (Utc::now() + refresh_duration).timestamp() as u64,
        });
    let access_token = token_issuer.issue_access_token(AccessToken {
        user_id: id,
        exp: (Utc::now() + access_duration).timestamp() as u64,
    });

    let updated_jar = jar
        .add(
            Cookie::build((REFRESH_TOKEN_COOKIE, refresh_token))
                .http_only(true),
        )
        .add(
            Cookie::build((ACCESS_TOKEN_COOKIE, access_token))
                .http_only(true),
        );

    updated_jar
}

async fn save_user(
    users: Arc<impl UserStorage>,
    user: User<WithPlainPassword>,
) -> Result<Uuid, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    let id = Uuid::now_v7();

    let hash = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(&user.secret.password.as_bytes(), &salt)
            .map(|h| h.to_string())
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = users
        .add(Identifiable {
            uuid: id,
            data: User {
                username: user.username,
                secret: WithHashedPassword {
                    hash: hash.to_string(),
                },
            },
        })
        .await;

    match result {
        Ok(_) => return Ok(id),
        Err(_) => return Err(StatusCode::CONFLICT),
    }
}
