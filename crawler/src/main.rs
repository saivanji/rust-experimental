mod error;

use async_std::task;
use error::{CrawlError, Result};
use scraper::{Html, Selector};
use std::{env, process};
use url::Url;

fn main() {
    match task::block_on(proceed()) {
        Ok(_) => println!("Crawled successfully"),
        Err(CrawlError::ArgError) => exit("Please specify argument"),
        Err(CrawlError::ParseError) => exit("Url parse error. Please specify correct url"),
        Err(CrawlError::FetchError) => exit("Url fetch error. Please make sure the url is correct"),
    }
}

fn obtain_url() -> Result<Url> {
    match env::args().collect::<Vec<String>>().get(1) {
        Some(website_url) => Url::parse(website_url).or(Err(CrawlError::ParseError)),
        None => Err(CrawlError::ArgError),
    }
}

async fn fetch_page(url: &Url) -> Result<Html> {
    let html_text = surf::get(url)
        .await
        .or(Err(CrawlError::FetchError))?
        .body_string()
        .await
        .or(Err(CrawlError::FetchError))?;

    Ok(Html::parse_document(&html_text))
}

async fn proceed() -> Result<()> {
    let url = obtain_url()?;
    let page = fetch_page(&url).await?;

    // Why need to use "parse" for selectors instead of passing raw strings to "select"?
    let links = Selector::parse("a").unwrap();

    for link in page.select(&links) {
        match link.value().attr("href") {
            Some(href) => {
                // Filtering out external url's.
                if href.starts_with("/") {
                    println!("{}", href)
                }
            }
            None => println!("No href"),
        }
    }

    Ok(())
}

fn exit(msg: &str) {
    println!("{}", msg);
    process::exit(1)
}
