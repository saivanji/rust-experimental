use crate::Link;
use anyhow::{anyhow, Result};
use url::Url;

#[derive(Debug)]
pub struct Entrypoint {
    pub value: String,
}

impl Entrypoint {
    pub fn parse(input: &str) -> Result<Self> {
        let url = Url::parse(input).or(Err(anyhow!("Can not parse entrypoint url")))?;
        let scheme = url.scheme();

        if scheme != "http" && scheme != "https" {
            return Err(anyhow!("Invalid entrypoint scheme"));
        }

        let domain = url
            .domain()
            .ok_or(anyhow!("Invalid domain in entrypoint url"))?;

        let default_port = match scheme {
            "http" => 80,
            "https" => 443,
            _ => 0,
        };
        let port = url.port().unwrap_or(default_port).to_string();

        let mut value = String::from("");

        value.push_str(scheme);
        value.push_str("://");
        value.push_str(domain);
        value.push_str(":");
        value.push_str(&port);

        Ok(Self { value })
    }

    pub fn link<'a>(&'a self, path: &'a str) -> Link {
        Link::new(&self, path)
    }
}
