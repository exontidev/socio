use std::{str::FromStr, sync::Arc, time::Duration};

use axum::{Router, routing::get};
use chatroom::{
    api::{
        state::{AppState, Config},
        ws::websocket_handler,
    },
    requests::WebSocketMessage,
    room::{message::Message, room::RoomId, rooms::Rooms},
    user::user::UserId,
};
use dashmap::DashMap;
use tokio::{net::TcpListener, sync, time::sleep};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config("./Config.toml");

    let state = Arc::new(AppState {
        rooms: Rooms {
            rooms: DashMap::new(),
        },

        config,
    });

    init_room("butter", state.clone());
    init_room("obama", state.clone());
    init_room("lsd", state.clone());
    init_room("greedy capitasists", state.clone());
    init_room("videogames", state.clone());
    init_room("buffer overflow", state.clone());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");

    let router = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state);

    axum::serve(listener, router)
        .await
        .expect("Failed to serve backend");

    Ok(())
}

fn load_config(path: &str) -> Config {
    let contents = std::fs::read_to_string(path)
        .expect("Failed to read config file");
    toml::from_str(&contents).expect("Failed to parse config")
}

fn init_room(name: &'static str, state: Arc<AppState>) {
    let tx = sync::broadcast::Sender::new(512);
    let id = Uuid::now_v7();
    println!("id of room {} is {}", name, id);

    state.rooms.rooms.insert(RoomId(id), tx.clone());

    tokio::spawn(async move {
        loop {
            let _ = tx.send(WebSocketMessage::NewMessage(Message {
                user: UserId(Uuid::now_v7()),
                content: format!("i love {}...", name),
            }));

            sleep(Duration::from_secs(1)).await;
        }
    });
}
