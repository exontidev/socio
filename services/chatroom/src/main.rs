use std::sync::Arc;

use axum::{Router, routing::get};
use chatroom::{
    api::{
        state::{AppState, Config},
        ws::websocket_handler,
    },
    room::{
        realtime_rooms::{RealtimeRoom, Rooms},
        room::RoomId,
    },
};
use dashmap::DashMap;
use tokio::{net::TcpListener, sync};
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

    state.rooms.rooms.insert(
        RoomId(id),
        RealtimeRoom {
            event_broadcast: tx.clone(),
        },
    );
}
