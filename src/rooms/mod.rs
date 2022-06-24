pub mod room;

pub use room::Room;

use anyhow::Result;

pub struct Rooms {
    pub rooms: Vec<Room>,
}

impl Rooms {
    pub async fn new(
        settings: Vec<room::Settings>,
        dav: &crate::Dav,
    ) -> Result<Rooms> {
        Ok(Self {
            rooms: futures::future::try_join_all(
                settings.into_iter().map(|setting| Room::new(setting, &dav)),
            )
            .await?,
        })
    }
}
