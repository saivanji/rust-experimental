use crate::{utils, Entrypoint};
use anyhow::{anyhow, Result};

pub struct Link<'a> {
    origin: &'a Entrypoint,
    path: &'a str,
}

impl<'a> Link<'a> {
    pub fn new(origin: &'a Entrypoint, path: &'a str) -> Self {
        Self {
            origin,
            path: utils::no_leading_slash(path),
        }
    }

    pub async fn fetch(&self) -> Result<Vec<u8>> {
        let url = self.url();
        let fetch_err = Err(anyhow!("Can not fetch {} url", url));
        let parse_err = Err(anyhow!("Failed to parse {} url", url));

        surf::get(url)
            .await
            .or(fetch_err)?
            .body_bytes()
            .await
            .or(parse_err)
    }

    fn url(&self) -> String {
        let mut result = "".to_owned();
        result.push_str(&self.origin.value);
        result.push_str("/");
        result.push_str(self.path);

        result
    }
}
