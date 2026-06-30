use std::sync::Arc;

use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use chrono::{TimeDelta, Utc};
use uuid::Uuid;

use crate::{
    config::TokenDuration,
    token::{
        claims::{
            ACCESS_TOKEN_COOKIE, AccessToken, AccessTokenInternals,
            REFRESH_TOKEN_COOKIE, RefreshToken,
            RefreshTokenInternals,
        },
        issuer::TokenIssuer,
    },
    users::user_storage,
};

pub mod claims;

pub mod issuer;
pub mod verifier;

pub fn append_tokens(
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

    let jar = append_access_token(
        token_issuer.clone(),
        access_duration,
        id,
        jar,
    );

    append_refresh_token(token_issuer, refresh_duration, id, jar)
}

pub fn append_access_token(
    token_issuer: Arc<TokenIssuer>,
    access_duration: TimeDelta,
    id: Uuid,
    jar: CookieJar,
) -> CookieJar {
    let access_token = token_issuer.issue_access_token(AccessToken {
        access_token: AccessTokenInternals { user_id: id },
        exp: (Utc::now() + access_duration).timestamp() as u64,
    });
    let updated_jar = jar.add(
        Cookie::build((ACCESS_TOKEN_COOKIE, access_token))
            .http_only(true)
            .same_site(SameSite::Strict)
            .max_age(time::Duration::seconds(
                access_duration.num_seconds(),
            ))
            .build(),
    );

    updated_jar
}

pub fn append_refresh_token(
    token_issuer: Arc<TokenIssuer>,
    refresh_duration: TimeDelta,
    id: Uuid,
    jar: CookieJar,
) -> CookieJar {
    let refresh_token =
        token_issuer.issue_refresh_token(RefreshToken {
            refresh_token: RefreshTokenInternals { user_id: id },
            exp: (Utc::now() + refresh_duration).timestamp() as u64,
        });

    let updated_jar = jar.add(
        Cookie::build((REFRESH_TOKEN_COOKIE, refresh_token))
            .http_only(true)
            .same_site(SameSite::Strict)
            .max_age(time::Duration::seconds(
                refresh_duration.num_seconds(),
            )),
    );

    updated_jar
}
