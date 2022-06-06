pub mod room;

pub use room::Room;

use anyhow::Result;

pub struct Rooms<'a> {
    pub rooms: Vec<Room<'a>>,
}

impl<'a> Rooms<'a> {
    pub async fn new(
        settings: &'a Vec<room::Settings>,
        dav: &'a crate::Dav<'_>,
    ) -> Result<Rooms<'a>> {
        Ok(Self {
            rooms: futures::future::try_join_all(
                settings.iter().map(|setting| Room::new(setting, &dav)),
            )
            .await?,
        })
    }
}
