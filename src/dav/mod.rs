pub mod calendar;
pub mod common;
pub mod recv;
pub mod send;
use anyhow::Result;
pub use calendar::Calendar;
use isahc::AsyncReadResponseExt;

use self::{
    recv::{FromXML, MultiStatus, PropStat},
    send::ToXML,
};

pub struct User<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

pub struct Dav {
    base_address: String,
    authorization: Option<String>,
}

static GET_USER_PRINCIPAL_INFO_SET: send::PropFields =
    send::PropFields::from_bits_truncate(0b00000011);
static START_SYNC_SET: send::PropFields = send::PropFields::from_bits_truncate(0b00110000);
static ALL_CAL_ENTRYS_SET: send::PropFields = send::PropFields::from_bits_truncate(0b11000000);

impl Dav {
    pub fn new(base_address: String) -> Self {
        Self {
            base_address,
            authorization: None,
        }
    }

    pub fn from_dav(dav: &Dav) -> Self {
        Self {
            base_address: dav.base_address.clone(),
            authorization: dav.authorization.clone(),
        }
    }

    pub fn set_auth(&mut self, username: &str, password: &str) {
        self.authorization = Some(format!(
            "Basic {}",
            base64::encode(format!("{username}:{password}"))
        ));
    }

    pub async fn get_user_principal_info(&self, username: &str) -> Result<recv::PropStat> {
        Ok(self
            .propfind0(
                &format!("/remote.php/dav/principals/users/{username}/"),
                vec![],
                send::Propfind::new(GET_USER_PRINCIPAL_INFO_SET),
            )
            .await?
            .prop_stat)
    }

    pub async fn start_sync(&self, url: &str) -> Result<recv::PropStat> {
        Ok(self
            .propfind0(url, vec![], send::Propfind::new(START_SYNC_SET))
            .await?
            .prop_stat)
    }

    pub async fn propfind0(
        &self,
        url: &str,
        headers: Vec<(&str, &str)>,
        body: send::Propfind,
    ) -> Result<recv::Response> {
        let mut response = self
            .request(
                url,
                "PROPFIND",
                headers.into_iter().chain(vec![("depth", "0")]).collect(),
                &body.xml(),
            )
            .await?;
        let text = response.text().await?;
        let result = recv::MultiStatus::xml(&text)?.response.remove(0);
        if result.href != url {
            return Err(anyhow::Error::msg("Response has wrong URL"));
        }
        Ok(result)
    }

    pub async fn all_cal_entrys(&self, url: &str) -> Result<recv::MultiStatus> {
        Ok(self
            .report1(url, vec![], send::CalendarQuery::new(ALL_CAL_ENTRYS_SET))
            .await?)
    }

    pub async fn report1(
        &self,
        url: &str,
        headers: Vec<(&str, &str)>,
        body: send::CalendarQuery,
    ) -> Result<recv::MultiStatus> {
        let mut response = self
            .request(
                url,
                "REPORT",
                headers.into_iter().chain(vec![("depth", "1")]).collect(),
                &body.xml(),
            )
            .await?;
        let text = response.text().await?;
        let result = recv::MultiStatus::xml(&text)?;
        if result.response.iter().any(|e| !e.href.starts_with(url)) {
            return Err(anyhow::Error::msg("Response has wrong URL"));
        }
        Ok(result)
    }

    async fn request(
        &self,
        url: &str,
        method: &str,
        headers: Vec<(&str, &str)>,
        body: &str,
    ) -> Result<isahc::Response<isahc::AsyncBody>> {
        let mut request = isahc::Request::builder();
        request = request
            .method(method)
            .uri(format!("{}{}", self.base_address, url))
            .version(isahc::http::Version::HTTP_2);
        if let Some(authorization) = &self.authorization {
            request = request.header("authorization", authorization)
        }
        for header in headers.iter() {
            request = request.header(header.0, header.1)
        }
        Ok(isahc::send_async(request.body(body)?).await?)
    }
}
