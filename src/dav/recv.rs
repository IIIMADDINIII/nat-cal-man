
use anyhow::{Context, Result};

pub trait FromXML {
    fn xml(xml: &str) -> Result<Self>
    where
        Self: Sized,
    {
        println!("{xml}");
        Ok(Self::element(&xml.parse()?)?)
    }
    fn element(element: &minidom::Element) -> Result<Self>
    where
        Self: Sized;
}


#[derive(std::fmt::Debug)]
pub struct PropStat {
    pub displayname: Result<String>,
    pub cal_home: Result<String>,
    pub cal_inbox: Result<String>,
    pub cal_outbox: Result<String>,
}

impl PropStat {
    pub fn new() -> Self {
        Self {
            displayname: Err(anyhow::Error::msg("Displayname was not a result")),
            cal_home: Err(anyhow::Error::msg("cal_home was not a result")),
            cal_inbox: Err(anyhow::Error::msg("cal_inbox was not a result")),
            cal_outbox: Err(anyhow::Error::msg("cal_outbox was not a result")),
        }
    }
}

impl FromXML for PropStat {
    fn element(element: &minidom::Element) -> Result<Self> {
        let mut result = PropStat::new();
        if !element.is("multistatus", super::common::NAMESPACES.dav) {
            return Err(anyhow::Error::msg("Response is not a MultiStatus"));
        };
        let response = element.get_child("response", super::common::NAMESPACES.dav)
                .context("MultiStatus Element doesn't contain a Response Element")?;
        for prop_stat in response.children() {
            if !prop_stat.is("propstat", super::common::NAMESPACES.dav) { continue; };
            let status = prop_stat
                .get_child("status", super::common::NAMESPACES.dav)
                .context("PropStat Element doesn't contain a Status element")?;
            let prop = prop_stat
                .get_child("prop", super::common::NAMESPACES.dav)
                .context("PropStat Element doesn't contain a Prop element")?;
            let error = match status.text().contains("200 OK") {
                true => None,
                false => Some(status.text()),
            };
            let get_value = |node: &minidom::Element| match &error {
                Some(error) => Err(anyhow::Error::msg(error.clone())),
                None => Ok(node.text()),
            };
            for property in prop.children() {
                if property.is("displayname", super::common::NAMESPACES.dav) {
                    result.displayname = get_value(property);
                } else if property.is("calendar-home-set", super::common::NAMESPACES.cal) {
                    result.cal_home = (|| get_value(property.get_child("href", super::common::NAMESPACES.dav).context("href element not found")?))();
                } else if property.is("schedule-inbox-URL", super::common::NAMESPACES.cal) {
                    result.cal_inbox = (|| get_value(property.get_child("href", super::common::NAMESPACES.dav).context("href element not found")?))();
                } else if property.is("schedule-outbox-URL", super::common::NAMESPACES.cal) {
                    result.cal_outbox = (|| get_value(property.get_child("href", super::common::NAMESPACES.dav).context("href element not found")?))();
                }
            }
        }
        Ok(result)
    }
}
