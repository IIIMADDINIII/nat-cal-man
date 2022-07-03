use anyhow::{Context, Result};

pub trait FromXML {
    fn xml(xml: &str) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self::element(&xml.parse()?)?)
    }
    fn element(element: &minidom::Element) -> Result<Self>
    where
        Self: Sized;
}

#[derive(std::fmt::Debug)]
pub struct MultiStatus {
    pub response: Vec<Response>,
}

impl FromXML for MultiStatus {
    fn element(element: &minidom::Element) -> Result<Self>
    where
        Self: Sized,
    {
        if !element.is("multistatus", super::common::NAMESPACES.dav) {
            return Err(anyhow::Error::msg("Response is not a MultiStatus"));
        };
        let response = element
            .children()
            .into_iter()
            .map(|res| Response::element(&res))
            .collect::<Result<_, _>>()?;
        Ok(Self { response })
    }
}

#[derive(std::fmt::Debug)]
pub struct Response {
    pub href: String,
    pub prop_stat: PropStat,
}

impl FromXML for Response {
    fn element(element: &minidom::Element) -> Result<Self>
    where
        Self: Sized,
    {
        if !element.is("response", super::common::NAMESPACES.dav) {
            return Err(anyhow::Error::msg("Response is not a response"));
        };
        let href = element
            .get_child("href", super::common::NAMESPACES.dav)
            .context("response Element doesn't contain a href element")?
            .text();
        let prop_stat = PropStat::element(element)?;
        Ok(Self { href, prop_stat })
    }
}

#[derive(std::fmt::Debug)]
pub struct PropStat {
    pub displayname: Result<String>,
    pub cal_home: Result<String>,
    pub get_c_tag: Result<String>,
    pub sync_token: Result<String>,
    pub get_e_tag: Result<String>,
    pub calendar_data: Result<String>,
}

impl FromXML for PropStat {
    fn element(element: &minidom::Element) -> Result<Self> {
        let mut displayname = Err(anyhow::Error::msg("Displayname was not a result"));
        let mut cal_home = Err(anyhow::Error::msg("cal_home was not a result"));
        let mut get_c_tag = Err(anyhow::Error::msg("get_c_tag was not a result"));
        let mut sync_token = Err(anyhow::Error::msg("sync_token was not a result"));
        let mut get_e_tag = Err(anyhow::Error::msg("GET_E_TAG was not a result"));
        let mut calendar_data = Err(anyhow::Error::msg("CALENDAR_DATA was not a result"));
        for prop_stat in element.children() {
            if !prop_stat.is("propstat", super::common::NAMESPACES.dav) {
                continue;
            };
            let status = prop_stat
                .get_child("status", super::common::NAMESPACES.dav)
                .context("PropStat Element doesn't contain a Status element")?
                .text();
            let prop = prop_stat
                .get_child("prop", super::common::NAMESPACES.dav)
                .context("PropStat Element doesn't contain a Prop element")?;
            let error = match status.contains("200 OK") {
                true => None,
                false => Some(status),
            };
            for property in prop.children() {
                if property.is("displayname", super::common::NAMESPACES.dav) {
                    displayname = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property.text()),
                    };
                } else if property.is("calendar-home-set", super::common::NAMESPACES.c) {
                    cal_home = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property
                            .get_child("href", super::common::NAMESPACES.dav)
                            .context("href element not found")?
                            .text()),
                    };
                } else if property.is("getctag", super::common::NAMESPACES.cs) {
                    get_c_tag = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property.text()),
                    };
                } else if property.is("sync-token", super::common::NAMESPACES.dav) {
                    sync_token = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property.text()),
                    };
                } else if property.is("getetag", super::common::NAMESPACES.dav) {
                    get_e_tag = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property.text()),
                    };
                } else if property.is("calendar-data", super::common::NAMESPACES.c) {
                    calendar_data = match &error {
                        Some(err) => Err(anyhow::Error::msg(err.clone())),
                        None => Ok(property.text()),
                    };
                }
            }
        }
        Ok(Self {
            displayname,
            cal_home,
            get_c_tag,
            sync_token,
            get_e_tag,
            calendar_data,
        })
    }
}
