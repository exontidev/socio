use uuid::Uuid;

#[derive(Debug)]
pub struct Identifiable<T: std::fmt::Debug> {
    pub id: Uuid,
    pub data: T,
}

impl<T: std::fmt::Debug> Identifiable<T> {
    pub fn new_unique(data: T) -> Self {
        Self {
            id: Uuid::now_v7(),
            data,
        }
    }

    pub fn new_known(id: impl Into<Uuid>, data: T) -> Self {
        Self {
            id: id.into(),
            data,
        }
    }
}
