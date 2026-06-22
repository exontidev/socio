use axum::extract::ws::{Message, WebSocket};
use futures::stream::{SplitSink, SplitStream};
use tokio::sync::mpsc::UnboundedSender;

use crate::api::ws::{
    io::WebSocketMessage,
    status_codes::{NotifyCode, WebSocketError},
};

pub type UserRelaySender = UnboundedSender<WebSocketMessage>;

pub type RelaySender = UnboundedSender<WebSocketMessage>;

pub type WebSocketResult =
    core::result::Result<NotifyCode, WebSocketError>;

pub type WebSocketSender = SplitSink<WebSocket, Message>;
pub type WebSocketReceiver = SplitStream<WebSocket>;
