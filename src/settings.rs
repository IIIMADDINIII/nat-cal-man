use anyhow::{Context, Result};

const SETTINGS_FILE: &'static str = "./settings.json";

#[derive(serde::Deserialize)]
pub struct Settings {
    pub cal_dav_address: String,
    pub rooms: Vec<crate::rooms::room::Settings>,
}

impl Settings {
    pub async fn read() -> Result<Settings> {
        let json = async_std::fs::read_to_string(SETTINGS_FILE)
            .await
            .with_context(|| format!("Settings File {} does not exist!", SETTINGS_FILE))?;
        let settings: Settings = serde_json::from_str(&json).context("Settings File is not Formatted correctly")?;
        Ok(settings)
    }
}
