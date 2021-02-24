mod error;
mod input;
mod page;

use async_std::task;
use error::{CrawlError, DirectoryErrorReason, Result, WebsiteErrorReason};
use std::path::Path;
use std::process;
use CrawlError::{DirectoryError, FetchError, JoinError, WebsiteError};

fn main() {
    match task::block_on(start()) {
        Ok(_) => println!("Crawled successfully"),
        Err(WebsiteError(WebsiteErrorReason::NotProvided)) => {
            exit("Please specify website url to crawl")
        }
        Err(WebsiteError(WebsiteErrorReason::ParseFailure)) => {
            exit("Url parse error. Please specify correct url")
        }
        Err(WebsiteError(WebsiteErrorReason::InvalidScheme)) => exit("Invalid scheme"),
        Err(DirectoryError(DirectoryErrorReason::NotProvided)) => exit("Please specify directory"),
        Err(JoinError) => {
            exit("Join error. TODO: do not crash program and skip failed link instead")
        }
        Err(FetchError) => exit("Url fetch error. Please make sure the url is correct"),
    }
}

fn create_dirs(path: &Path) {
    println!("{:?}", path);
}

async fn start() -> Result<()> {
    let (website, path) = input::obtain()?;

    create_dirs(path);
    page::process_page(&website).await?;

    Ok(())
}

fn exit(msg: &str) {
    println!("{}", msg);
    process::exit(1)
}
