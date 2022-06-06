use anyhow::Result;

pub struct User<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

pub struct Dav<'a> {
    base_address: &'a str,
    authorization: Option<String>,
}

impl<'a> Dav<'a> {
    pub fn new(base_address: &'a str) -> Self {
        Self {
            base_address,
            authorization: None,
        }
    }

    pub fn from_dav(dav: &'a Dav) -> Self {
        Self {
            base_address: dav.base_address,
            authorization: dav.authorization.clone(),
        }
    }

    pub fn set_auth(&mut self, username: &str, password: &str) {
        self.authorization = Some(format!("Basic {}", base64::encode(format!("{username}:{password}"))));
    }

    pub async fn get_user_principal_info(&self, username: &str) -> Result<isahc::Response<isahc::AsyncBody>> {
        Ok(self.propfind(&format!("/principals/users/{username}/"), vec![("depth", "0")], "").await?)
    }

    pub async fn propfind(&self, url: &str, headers: Vec<(&str, &str)>, body: &str) -> Result<isahc::Response<isahc::AsyncBody>> {
        Ok(self.request(url, "PROPFIND", headers, body).await?)
    }

    async fn request(
        &self,
        url: &str,
        method: &str,
        headers: Vec<(&str, &str)>,
        body: &str,
    ) -> Result<isahc::Response<isahc::AsyncBody>> {
        let mut request = isahc::Request::builder();
        request = request.method(method).uri(format!("{}{}", self.base_address, url)).version(isahc::http::Version::HTTP_2);
        if let Some(authorization) = &self.authorization {
            request = request.header("authorization", authorization)
        }
        for header in headers.iter() {
            request = request.header(header.0, header.1)
        }
        Ok(isahc::send_async(request.body(())?).await?)
    }
}

