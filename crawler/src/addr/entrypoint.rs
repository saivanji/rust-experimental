use crate::Link;
use anyhow::{anyhow, Result};
use url::Url;

pub struct Entrypoint {
    pub value: String,
}

impl Entrypoint {
    pub fn parse(input: &str) -> Result<Self> {
        let url = Url::parse(input).or(Err(anyhow!("Can not parse entrypoint url")))?;
        let domain = url
            .domain()
            .ok_or(anyhow!("Invalid domain in entrypoint url"))?;

        let mut value = url.scheme().to_owned();
        value.push_str("/");
        value.push_str(domain);

        Ok(Self { value })
    }

    pub fn link<'a>(&'a self, path: &'a str) -> Link {
        Link { origin: self, path }
    }
}
