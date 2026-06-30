use uuid::Uuid;

#[derive(Debug)]
pub struct Identifiable<T: 'static> {
    pub uuid: Uuid,
    pub data: T,
}
