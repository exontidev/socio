use std::sync::Arc;

use uuid::Uuid;

use crate::api::state::AppState;

pub struct Identifiable<T: 'static + Send + Sync> {
    pub id: Uuid,
    pub data: T,
}

pub type GlobalState = Arc<AppState>;
