use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use url::Url;

const FETCH_ERROR: &str = "Url fetch error. Please make sure the url is correct";

pub async fn process_page(url: &Url) -> Result<()> {
    let page = fetch_page(url).await?;

    // Why need to use "parse" for selectors instead of passing raw strings to "select"?
    let links = Selector::parse("a").unwrap();

    // TODO: unnest
    for link in page.select(&links) {
        match link.value().attr("href") {
            Some(href) => {
                if href.starts_with("/") {
                    let link_url = url.join(&href).or(Err(anyhow!(
                        "Join error. TODO: do not crash program and skip failed link instead"
                    )))?;

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
        .or(Err(anyhow!(FETCH_ERROR)))?
        .body_string()
        .await
        .or(Err(anyhow!(FETCH_ERROR)))?;

    Ok(Html::parse_document(&html_text))
}
