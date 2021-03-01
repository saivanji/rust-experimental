use crate::File;
use anyhow::{anyhow, Result};
use url::Url;

pub struct Link {
    url: Url,
}

impl Link {
    pub fn parse(input: &str) -> Option<Self> {
        Url::parse(input).ok().map(|url| Self { url })
    }

    pub async fn fetch(&self) -> Result<File> {
        let fetch_err = Err(anyhow!("Url fetch error. Url {} is not correct", self.url));
        let parse_err = Err(anyhow!("Failed to parse {} url", self.url));

        surf::get(self.url)
            .await
            .or(fetch_err)?
            .body_bytes()
            .await
            .or(parse_err)
            .map(|bytes| File::from(bytes, self.url.path()))
    }
}

// fn split_path(path: &str) -> (&str, &str) {
//     let default_filename = "index.html";

//     if path == "/" {
//         return ("", default_filename);
//     }

//     match path.strip_suffix("/").unwrap_or(input).rsplit_once("/") {
//         Some((path, filename)) if filename.contains(".") => (path, filename),
//         _ => (path, default_filename),
//     }
// }
