use crate::error::{CrawlError, DirectoryErrorReason, Result, WebsiteErrorReason};
use std::env;
use std::path::Path;
use url::Url;
use CrawlError::{DirectoryError, WebsiteError};

pub fn obtain<'a>() -> Result<(Url, &'a Path)> {
    let args: Vec<String> = env::args().collect();

    let website = match_website(&args)?;
    let path = match_path(&args)?;

    Ok((website, path))
}

fn match_website(args: &Vec<String>) -> Result<Url> {
    match args.get(1) {
        Some(url) => Url::parse(url)
            .or(Err(WebsiteError(WebsiteErrorReason::ParseFailure)))
            .and_then(filter_scheme),
        None => Err(WebsiteError(WebsiteErrorReason::NotProvided)),
    }
}

fn match_path(args: &Vec<String>) -> Result<&Path> {
    match args.get(2) {
        Some(dir) => Ok(Path::new(dir)),
        None => Err(DirectoryError(DirectoryErrorReason::NotProvided)),
    }
}

fn filter_scheme(url: Url) -> Result<Url> {
    let scheme = url.scheme();

    if scheme != "http" && scheme != "https" {
        return Err(WebsiteError(WebsiteErrorReason::InvalidScheme));
    }

    Ok(url)
}
