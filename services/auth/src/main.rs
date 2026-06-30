use std::sync::Arc;

use auth::{
    api::{
        login::handle_login, refresh::handle_refresh,
        sign::handle_register,
    },
    config::Config,
    mocks::user_storage::MemoryUserStorage,
    state::Global,
    token::{
        claims::ACCESS_TOKEN_COOKIE, issuer::TokenIssuer,
        verifier::TokenVerifier,
    },
};
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config =
        Arc::new(Config::load("./Config.toml").await.unwrap());

    let token_issuer = Arc::new(TokenIssuer {
        algorithm: jsonwebtoken::Algorithm::EdDSA,
        private: EncodingKey::from_ed_pem(
            config.load_secret().await.unwrap().bytes(),
        )
        .unwrap(),
    });
    let token_verifier = Arc::new(TokenVerifier {
        algorithm: jsonwebtoken::Algorithm::EdDSA,
        public: DecodingKey::from_ed_pem(
            config.load_public().await.unwrap().bytes(),
        )
        .unwrap(),
    });
    let global = Global {
        users: Arc::new(MemoryUserStorage::empty()),
        config,
        token_issuer,
        token_verifier,
    };

    let config = Config::load("./Config.toml").await.unwrap();
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.network.ip.to_string(),
        config.network.port
    ))
    .await
    .unwrap();

    let router = Router::new()
        .route("/register", post(handle_register))
        .route("/login", post(handle_login))
        .route("/refresh", post(handle_refresh))
        .route(
            "/protected",
            get(
                async |State(global): State<Global>,
                       jar: CookieJar| {
                    if let Some(access) = jar.get(ACCESS_TOKEN_COOKIE)
                    {
                        let verifier = global.token_verifier;
                        let jwt = access.value().to_string();

                        dbg!(&verifier.verify_access_token(&jwt));

                        if verifier.verify_access_token(&jwt).is_ok()
                        {
                            println!("yay!");
                        }
                    }
                },
            ),
        )
        .with_state(global);

    axum::serve(listener, router).await.unwrap();
}
