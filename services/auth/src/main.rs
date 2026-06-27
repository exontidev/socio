use auth::{
    config::Config,
    helper::Identifiable,
    mocks::user_storage::MemoryUserStorage,
    users::{
        user::{User, WithHashedPassword},
        user_storage::{self, Query, UserStorage},
    },
};

use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    testimony: String,
    exp: usize,
}

#[tokio::main]
async fn main() {
    let memory_users = MemoryUserStorage::empty();

    let _ = memory_users.add(generate_user("larpslav")).await;
    let _ = memory_users.add(generate_user("sasha")).await;
    let _ = memory_users.add(generate_user("makSON")).await;

    let maksim = memory_users
        .get::<WithHashedPassword>(Query::Name("makSON".to_string()))
        .await
        .expect("maksim not found");

    dbg!(maksim);
}

pub fn generate_user(
    username: &'static str,
) -> Identifiable<User<WithHashedPassword>> {
    Identifiable {
        uuid: Uuid::now_v7(),
        data: User {
            secret: WithHashedPassword {
                hash: format!("0x{}666bullshit", &username),
            },
            username: String::from(username),
        },
    }
}

// jwt
//
// let config = Config::load("./Config.toml").await.unwrap();

// let public = config.load_public().await.unwrap();
// let secret = config.load_secret().await.unwrap();

// let header = Header::new(jsonwebtoken::Algorithm::EdDSA);

// let verifier = DecodingKey::from_ed_pem(public.bytes()).unwrap();
// let signer = EncodingKey::from_ed_pem(secret.bytes()).unwrap();

// let claims = Claims {
//     testimony: "yay".to_string(),
//     exp: 6767676767,
// };

// let jwt = encode(&header, &claims, &signer).unwrap();
// dbg!(&jwt);

// let validation = &Validation::new(jsonwebtoken::Algorithm::EdDSA);
// let decoded =
//     decode::<Claims>(&jwt, &verifier, validation).unwrap();

// dbg!(decoded);
