use anyhow::{Result, Context};

#[derive(serde::Deserialize)]
pub struct Settings {
    username: String,
    password: String,
}

pub struct Room<'a> {
    dav: crate::Dav<'a>,
    settings: &'a Settings,
    displayname: String,
    cal_inbox: String,
    cal_outbox: String,
    cal_home: String,
}

impl<'a> Room<'a> {
    pub async fn new(settings: &'a Settings, dav: &'a crate::Dav<'_>) -> Result<Room<'a>> {
        let mut dav = crate::Dav::from_dav(dav);
        dav.set_auth(&settings.username, &settings.password);
        let mut this = Self {dav, settings, displayname: String::default(), cal_inbox: String::default(), cal_outbox: String::default(), cal_home: String::default()};
        this.init().await?;
        Ok(this)
    }

    async fn init(&mut self) -> Result<()> {
        let principal_info = self.dav.get_user_principal_info(&self.settings.username).await?;
        println!("{:?}", principal_info);
        self.displayname = principal_info.displayname.with_context(|| format!("There is no displayname for User {}", self.settings.username))?;
        self.cal_inbox = principal_info.cal_inbox.with_context(|| format!("There is no in_box for User {}", self.settings.username))?;
        self.cal_outbox = principal_info.cal_outbox.with_context(|| format!("There is no out_box for User {}", self.settings.username))?;
        self.cal_home = principal_info.cal_home.with_context(|| format!("There is no home for User {}", self.settings.username))?;
        Ok(())
    }

    pub async fn try_connection(&self) -> Result<()> {
        let result: Result<()> = Err(anyhow::Error::msg("test"));
        result.with_context(|| format!("Could not get data from User {}", &self.settings.username))
    }
}
