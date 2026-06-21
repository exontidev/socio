use uuid::Uuid;

pub struct Identifiable<T: 'static + Send + Sync> {
    id: Uuid,
    data: T,
}
