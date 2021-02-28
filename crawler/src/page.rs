use anyhow::{anyhow, Result};
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Path;
use url::Url;

const FETCH_ERROR: &str = "Url fetch error. Please make sure the url is correct";

pub async fn process_page(url: &Url, workdir: &Path, trail: &mut BTreeSet<String>) -> Result<()> {
    let url_path = url.path().to_string();

    if trail.contains(&url_path) {
        return Ok(());
    }

    let text = fetch_page(url).await?;
    let page = Html::parse_document(&text);
    let links = Selector::parse("a").unwrap();

    persist_page(&text, &url_path, &workdir)?;

    trail.insert(url_path);

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

fn crop_slash(input: &str) -> &str {
    match input.chars().nth(0) {
        Some('/') => &input[1..],
        _ => input,
    }
}

fn split_path(path: &str) -> (&str, &str) {
    let default_filename = "index.html";

    let (scope, filename) = match path.rsplit_once("/") {
        Some((path, filename)) if filename.ends_with(".html") => (path, filename),
        _ => (path, default_filename),
    };

    (scope, crop_slash(filename))
}

fn persist_page(text: &str, url_path: &str, workdir: &Path) -> Result<()> {
    let (scope, filename) = split_path(url_path);
    let dir_path = workdir.join(scope);
    let file_path = dir_path.join(filename);

    let file_display = file_path.display();
    let file_create_err = Err(anyhow!("Can not create file {}", file_display));
    let file_write_err = Err(anyhow!("Can not write to {} file", file_display));
    let dir_create_err = Err(anyhow!("Can not create {} directory", dir_path.display()));

    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).or(dir_create_err)?;
    }

    fs::File::create(&file_path)
        .or(file_create_err)?
        .write(text.as_bytes())
        .or(file_write_err)?;

    Ok(())
}
