use axum::extract::ws::Message as WsMessage;
use futures::StreamExt;

use crate::{
    api::ws::{
        RelaySender, UserAction, WebSocketReceiver, WebSocketResult,
        notifier::Notifier, requests::Request,
        room_interface::RoomInterface, status_codes::WebSocketError,
    },
    helper::GlobalState,
};

pub async fn handle_income(
    mut receiver: WebSocketReceiver,
    relay_tx: RelaySender,
    state: GlobalState,
) {
    let mut rooms = RoomInterface::new(state, relay_tx.clone());
    let mut notifier = Notifier {
        id: None,
        tx: relay_tx.clone(),
    };

    while let Some(result) = receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        let text = match message {
            WsMessage::Text(utf8_bytes) => utf8_bytes.to_string(),
            WsMessage::Ping(_) | WsMessage::Pong(_) => continue,

            WsMessage::Close(_) => {
                break;
            }

            _ => {
                notifier.error(WebSocketError::NonUTF8Request);
                continue;
            }
        };

        let (id, action) = match serde_json::from_str::<
            Request<UserAction>,
        >(&text)
        {
            Ok(Request { request, data }) => (request, data),
            Err(_) => {
                notifier.error(WebSocketError::InvalidRequest);
                continue;
            }
        };

        notifier.set_id(id);

        let result = handle(&mut rooms, action);
        notifier.result(result);
    }
}

pub fn handle(
    rooms: &mut RoomInterface,
    action: UserAction,
) -> WebSocketResult {
    match action {
        UserAction::JoinRoom { room } => rooms.join(room),
        UserAction::LeaveRoom { room } => rooms.leave(room),
        UserAction::LeaveAllRooms => rooms.leave_all(),
        UserAction::SendMessage(message) => rooms.send(message),

        _ => Err(WebSocketError::ActionDoesNotExist),
    }
}
