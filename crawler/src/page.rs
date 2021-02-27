use anyhow::{anyhow, Result};
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;

const FETCH_ERROR: &str = "Url fetch error. Please make sure the url is correct";

pub async fn process_page(url: &Url, context: &Path, trail: &mut BTreeSet<&str>) -> Result<()> {
    let url_path = url.path();

    if trail.contains(&url_path) {
        return Ok(());
    }

    let (path, filename) = split_path(&url_path);

    let dest = make_destination(path, context);

    let page_text = fetch_page(url).await?;
    let page = parse_page(&page_text);

    mkdir_if_absent(&dest)?;
    persist_page(&page_text, &dest, &filename)?;

    trail.insert(&url_path);

    // Why need to use "parse" for selectors instead of passing raw strings to "select"?
    let links = Selector::parse("a").unwrap();

    for link in page.select(&links) {
        process_link(&url, &link);
    }

    Ok(())
}

fn process_link(url: &Url, link: &ElementRef) {
    match link.value().attr("href") {
        Some(href) => {
            if href.starts_with("/") {
                match url.join(&href) {
                    Ok(link_url) => println!("{}", link_url),
                    Err(_) => {}
                }
            }
        }
        None => {}
    }
}

async fn fetch_page(url: &Url) -> Result<String> {
    surf::get(url)
        .await
        .or(Err(anyhow!(FETCH_ERROR)))?
        .body_string()
        .await
        .or(Err(anyhow!(FETCH_ERROR)))
}

fn parse_page(text: &str) -> Html {
    Html::parse_document(text)
}

fn crop_slash(input: &str) -> &str {
    match input.chars().nth(0) {
        Some('/') => &input[1..],
        _ => input,
    }
}

fn split_path(path: &str) -> (&str, &str) {
    let default_filename = "index.html";

    match path.rsplit_once("/") {
        Some((path, filename)) if filename.ends_with(".html") => (path, filename),
        _ => (path, default_filename),
    }
}

fn make_destination(path: &str, context: &Path) -> PathBuf {
    let path = crop_slash(path);

    context.join(path)
}

fn mkdir_if_absent(dest: &PathBuf) -> Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest).or(Err(anyhow!("Can not create {} directory", dest.display())))?;
    }

    Ok(())
}

fn persist_page(text: &str, dest: &PathBuf, filename: &str) -> Result<()> {
    let path = dest.join(filename);
    let mut file =
        fs::File::create(&path).or(Err(anyhow!("Can not create file {}", path.display())))?;

    file.write(text.as_bytes())
        .or(Err(anyhow!("Can not write to {} file", path.display())))?;

    Ok(())
}
