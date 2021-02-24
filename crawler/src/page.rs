use crate::error::{CrawlError, Result};
use scraper::{Html, Selector};
use url::Url;

pub async fn process_page(url: &Url) -> Result<()> {
    let page = fetch_page(url).await?;

    // Why need to use "parse" for selectors instead of passing raw strings to "select"?
    let links = Selector::parse("a").unwrap();

    // TODO: unnest
    for link in page.select(&links) {
        match link.value().attr("href") {
            Some(href) => {
                if href.starts_with("/") {
                    let link_url = url.join(&href).or(Err(CrawlError::JoinError))?;

                    println!("{}", link_url)
                }
            }
            None => println!("No href"),
        }
    }

    Ok(())
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
