use anyhow::{Result};

pub struct Calendar {
    //dav: super::Dav,
    //url: String,
}

impl Calendar {
    pub fn new(
      dav: &super::Dav,
      url: String,
    ) -> Result<Self> {
        let newDav = super::Dav::from_dav(&dav);
        Ok(Self {
            //dav: super::Dav::from_dav(&dav),
            //url,
        })
    }
}
