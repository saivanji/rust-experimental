use async_std::task;
use scraper::{Html, Selector};

// TODO: get WEBSITE_URL from params
const WEBSITE_URL: &str = "http://localhost:3001";

fn main() -> surf::Result<()> {
    task::block_on(fetch())
}

async fn fetch() -> surf::Result<()> {
    let html = surf::get(WEBSITE_URL).await?.body_string().await?;
    let document = Html::parse_document(&html);

    // Why need to use "parse" for selectors instead of passing raw strings to "select"?
    let links = Selector::parse("a").unwrap();

    for link in document.select(&links) {
        match link.value().attr("href") {
            // TODO: needed link should start eigher with "/" or with "WEBSITE_URL"
            Some(href) => println!("{}", href),
            None => println!("No href"),
        }
    }

    Ok(())
}
