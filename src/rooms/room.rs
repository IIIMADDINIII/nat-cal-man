use anyhow::{Result, Context};
use isahc::AsyncReadResponseExt;

#[derive(serde::Deserialize)]
pub struct Settings {
    username: String,
    password: String,
}

pub struct Room<'a> {
    dav: crate::Dav<'a>,
    settings: &'a Settings,
}

impl<'a> Room<'a> {
    pub async fn new(settings: &'a Settings, dav: &'a crate::Dav<'_>) -> Result<Room<'a>> {
        let mut dav = crate::Dav::from_dav(dav);
        dav.set_auth(&settings.username, &settings.password);
        let mut this = Self {dav, settings};
        this.init().await?;
        Ok(this)
    }

    async fn init(&mut self) -> Result<()> {
        println!("{}", self.dav.get_user_principal_info(&self.settings.username).await?.text().await?);
        Ok(())
    }

    pub async fn try_connection(&self) -> Result<()> {
        let result: Result<()> = Err(anyhow::Error::msg("test"));
        result.with_context(|| format!("Could not get data from User {}", &self.settings.username))
    }
}
