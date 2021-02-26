use anyhow::{anyhow, Result};
use std::path::Path;
use url::Url;

pub fn match_website(args: &Vec<String>) -> Result<Url> {
    match args.get(1) {
        Some(url) => Url::parse(url)
            .or(Err(anyhow!("Url parse error. Please specify correct url")))
            .and_then(filter_scheme),
        None => Err(anyhow!("Please specify website url to crawl")),
    }
}

pub fn match_path(args: &Vec<String>) -> Result<&Path> {
    match args.get(2) {
        Some(dir) => Ok(Path::new(dir)),
        None => Err(anyhow!("Please specify directory")),
    }
}

fn filter_scheme(url: Url) -> Result<Url> {
    let scheme = url.scheme();

    if scheme != "http" && scheme != "https" {
        return Err(anyhow!("Invalid website scheme"));
    }

    Ok(url)
}
