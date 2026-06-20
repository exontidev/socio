use uuid::Uuid;

pub struct MessageId(pub Uuid);

pub struct Message {
    pub user: Uuid,
    pub content: String,
}
