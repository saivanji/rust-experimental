use crate::{File, Location};
use anyhow::{anyhow, Result};
use url::Url;

pub struct Link {
    url: Url,
}

impl Link {
    pub fn parse(input: &str) -> Option<Self> {
        Url::parse(input).ok().map(|url| Self { url })
    }

    pub async fn fetch<'a>(&'a self, workdir: &'a Location) -> Result<File<'a>> {
        let fetch_err = Err(anyhow!("Url fetch error. Url {} is not correct", self.url));
        let parse_err = Err(anyhow!("Failed to parse {} url", self.url));

        let location = workdir.concat(self.url.path());

        surf::get(&self.url)
            .await
            .or(fetch_err)?
            .body_bytes()
            .await
            .or(parse_err)
            .map(|bytes| File::from(bytes, &location))
    }
}
