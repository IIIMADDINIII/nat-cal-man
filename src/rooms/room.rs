use anyhow::{Context, Result};

#[derive(serde::Deserialize)]
pub struct SettingsCalenders {
    personal: String,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    username: String,
    password: String,
    calenders: SettingsCalenders,
}

pub struct Room {
    dav: crate::Dav,
    settings: Settings,
    personal: crate::dav::Calendar,
}
// 
impl Room {
    pub async fn new(settings: Settings, dav: &crate::Dav) -> Result<Self> {
        let mut dav = crate::Dav::from_dav(dav);
        dav.set_auth(&settings.username, &settings.password);
        let principal_info = dav.get_user_principal_info(&settings.username).await?;
        let base_path = principal_info.cal_home.with_context(|| format!{"There is no home for User {}", &settings.username})?;
        let this =
            Self {
                personal: crate::dav::Calendar::new(&dav, format!("{}{}/", &base_path, &settings.calenders.personal)).await?,
                dav,
                settings,
            };
        Ok(this)
    }

    pub async fn try_connection(&self) -> Result<()> {
        let result: Result<()> = Err(anyhow::Error::msg("test"));
        result.with_context(|| format!("Could not get data from User {}", &self.settings.username))
    }
}
