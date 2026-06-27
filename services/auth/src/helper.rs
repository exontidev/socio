use uuid::Uuid;

pub struct Identifiable<T: 'static> {
    pub uuid: Uuid,
    pub data: T,
}
