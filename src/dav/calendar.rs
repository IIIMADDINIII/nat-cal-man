use anyhow::{Context, Result};

pub struct Calendar {
    dav: super::Dav,
    url: String,
    ctag: String,
    sync_token: String,
    events: Vec<Event>,
}

impl Calendar {
    pub async fn new(dav: &super::Dav, url: String) -> Result<Self> {
        let newDav = super::Dav::from_dav(&dav);
        let (sync, data) =
            futures::try_join!(newDav.start_sync(&url), newDav.all_cal_entrys(&url))?;
        let ctag = sync.get_c_tag.context("did not get c-Tag of calendar")?;
        let sync_token = sync
            .sync_token
            .context("did not get sync-token of calendar")?;
        let mut result = Self {
            dav: super::Dav::from_dav(&dav),
            url,
            ctag,
            sync_token,
            events: vec![],
        };
        result.apply_cal_data(data)?;
        Ok(result)
    }

    fn apply_cal_data(&mut self, data: super::recv::MultiStatus) -> Result<()> {
        for response in data.response {
            let event = Event::new(response)?;
        }
        Ok(())
    }
}

pub struct Event {
    href: String,
    etag: String,
}

impl Event {
    pub fn new(response: super::recv::Response) -> Result<Self> {
        let href = response.href;
        let etag = response.prop_stat.get_e_tag.context("Event with no eTag")?;
        let data = response
            .prop_stat
            .calendar_data
            .context("Event with no data")?;
        //ToDo: Parse Calendar object
        Ok(Self { href, etag })
    }
}
