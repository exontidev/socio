use chatroom::domain::{identifiable::Identifiable, room::Room};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let room = Identifiable::new_unique(Room {
        title: "global".to_string(),
        description: "chaos in the brain".to_string(),
    });

    dbg!(room);

    Ok(())
}
