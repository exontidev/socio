use crate::api::ws::{
    RelaySender,
    requests::{RequestId, WebSocketMessage},
    status_codes::{NotifyCode, WebSocketError},
};

pub struct Notifier {
    pub id: Option<RequestId>,
    pub tx: RelaySender,
}

// FIX: duplicate
type Result = core::result::Result<NotifyCode, WebSocketError>;

impl Notifier {
    pub fn result(&self, result: Result) {
        let _ = match result {
            Ok(success) => self.code(success),
            Err(err) => self.error(err),
        };
    }

    pub fn code(&self, code: NotifyCode) {
        self.tx.send(WebSocketMessage::Notify {
            request_id: self.id,
            code: code,
        });
    }

    pub fn error(&self, err: WebSocketError) {
        self.tx.send(WebSocketMessage::Error {
            request_id: self.id,
            code: err,
        });
    }

    pub fn set_id(&mut self, id: RequestId) {
        self.id = Some(id);
    }
}
