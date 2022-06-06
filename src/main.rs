pub mod settings;
pub use settings::Settings;
pub mod rooms;
pub use rooms::Rooms;
pub mod dav;
pub use dav::Dav;

use anyhow::{Result};

#[async_std::main]
async fn main() -> Result<()> {
    let settings = Settings::read().await?;
    let request = Dav::new(&settings.cal_dav_address);
    Rooms::new(&settings.rooms, &request).await?;
    println!("test {}", &settings.cal_dav_address);
    Ok(())
}

