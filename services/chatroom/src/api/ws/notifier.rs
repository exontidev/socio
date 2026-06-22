use crate::api::ws::{
    io::{RequestId, WebSocketMessage},
    status_codes::{NotifyCode, WebSocketError},
    types::{RelaySender, WebSocketResult},
};

pub struct Notifier {
    pub id: Option<RequestId>,
    pub tx: RelaySender,
}

impl Notifier {
    pub fn result(&self, result: WebSocketResult) {
        let _ = match result {
            Ok(success) => self.code(success),
            Err(err) => self.error(err),
        };
    }

    pub fn code(&self, code: NotifyCode) {
        let _ = self.tx.send(WebSocketMessage::Notify {
            request_id: self.id,
            code: code,
        });
    }

    pub fn error(&self, err: WebSocketError) {
        let _ = self.tx.send(WebSocketMessage::Error {
            request_id: self.id,
            code: err,
        });
    }

    pub fn set_id(&mut self, id: RequestId) {
        self.id = Some(id);
    }
}
