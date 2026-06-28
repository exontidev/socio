use std::sync::Arc;

use crate::{
    helper::Identifiable,
    state::Global,
    token::claims::RefreshToken,
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
use chrono::{Duration, Utc};
use uuid::Uuid;

pub async fn handle_sign(
    State(global): State<Global>,
    Json(user): Json<User<WithPlainPassword>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_storage = global.users.clone();

    let id = save_user(user_storage, user).await?;

    let token =
        global.token_issuer.issue_refresh_token(RefreshToken {
            user_id: id,
            exp: (Utc::now() + Duration::days(7)).timestamp() as u64,
        });

    Ok(token)
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
