use std::sync::Arc;

use auth::{
    api::sign::handle_sign, config::Config,
    mocks::user_storage::MemoryUserStorage, state::Global,
    token::issuer::TokenIssuer,
};
use axum::{Router, routing::post};
use jsonwebtoken::EncodingKey;
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

    let global = Global {
        users: Arc::new(MemoryUserStorage::empty()),
        config,
        token_issuer,
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
        .route("/sign", post(handle_sign))
        .with_state(global);

    axum::serve(listener, router).await.unwrap();
}
