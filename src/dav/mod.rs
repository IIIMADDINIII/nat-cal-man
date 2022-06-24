pub mod common;
pub mod send;
pub mod recv;
pub mod calendar;
pub use calendar::Calendar;
use anyhow::Result;
use isahc::AsyncReadResponseExt;

use self::{send::ToXML, recv::{FromXML, PropStat}};

pub struct User<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

pub struct Dav {
    base_address: String,
    authorization: Option<String>,
}

static GET_USER_PRINCIPAL_INFO_SET: send::PropFields =  send::PropFields::from_bits_truncate(0b00001111);

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

    pub async fn get_user_principal_info(
        &self,
        username: &str,
    ) -> Result<PropStat> {
        Ok(self
            .propfind(
                &format!("/remote.php/dav/principals/users/{username}/"),
                vec![("depth", "0")],
                send::Propfind::new(GET_USER_PRINCIPAL_INFO_SET),
            )
            .await?)
    }

    pub async fn get_c_tag(&self, url: &str) -> Result<PropStat> {
        Ok(self.propfind(url, vec![("depth", "0")], send::Propfind::new(send::PropFields::GET_C_TAG)).await?)
    }

    pub async fn propfind(
        &self,
        url: &str,
        headers: Vec<(&str, &str)>,
        body: send::Propfind,
    ) -> Result<PropStat> {
        let mut response = self
            .request(url, "PROPFIND", headers, &body.xml())
            .await?;
        let text = response.text().await?;
        let result = recv::PropStat::xml(&text)?;
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
