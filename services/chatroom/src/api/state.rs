use std::net::IpAddr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::{sync::broadcast, sync::mpsc, task::JoinHandle};

use crate::{
    requests::{UserAction, WebSocketMessage},
    room::{room::RoomId, rooms::Rooms},
    user::user::UserId,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub rooms: Rooms,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub room: RoomConfig,
    pub network: NetworkConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomConfig {
    pub max_rooms: u8,
}

impl AppState {}
