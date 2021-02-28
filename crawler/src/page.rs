use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str;
use url::Url;

#[async_recursion(?Send)]
pub async fn process_page(url: &Url, workdir: &Path, trail: &mut BTreeSet<String>) -> Result<()> {
    let bytes = fetch(url).await?;
    let url_path = url.path();

    persist_file(&bytes, &url_path, &workdir)?;
    trail.insert(url_path.to_string());

    // TODO: do nothing if "bytes" could not be converted to utf8 or is not html string.
    let text = str::from_utf8(&bytes).or(Err(anyhow!("Failed convert to string for {}", url)))?;
    let page = Html::parse_document(text);
    let links = Selector::parse("a").unwrap();

    for link in page.select(&links) {
        let should_process = |url: &Url| !trail.contains(no_trailing_slash(url.path()));

        match extract_url(&url, &link) {
            Some(url) if should_process(&url) => process_page(&url, workdir, trail).await?,
            _ => continue,
        }
    }

    Ok(())
}

fn extract_url(url: &Url, link: &ElementRef) -> Option<Url> {
    link.value()
        .attr("href")
        .filter(|href| href.starts_with("/"))
        .and_then(|href| url.join(&href).ok())
}

async fn fetch(url: &Url) -> Result<Vec<u8>> {
    surf::get(url)
        .await
        .or(Err(anyhow!("Url fetch error. Url {} is not correct", url)))?
        .body_bytes()
        .await
        .or(Err(anyhow!("Failed to parse {} url", url)))
}

fn no_leading_slash(input: &str) -> &str {
    if input == "/" {
        return input;
    }

    input.strip_prefix("/").unwrap_or(input)
}

fn no_trailing_slash(input: &str) -> &str {
    if input == "/" {
        return input;
    }

    input.strip_suffix("/").unwrap_or(input)
}

fn split_path(path: &str) -> (&str, &str) {
    let default_filename = "index.html";

    match path.rsplit_once("/") {
        Some((path, filename)) if filename.ends_with(".html") => (path, filename),
        _ => (path, default_filename),
    }
}

fn join(path: &PathBuf, part: &str) -> PathBuf {
    path.join(no_leading_slash(part))
}

fn persist_file(bytes: &Vec<u8>, url_path: &str, workdir: &Path) -> Result<()> {
    let (scope, filename) = split_path(url_path);
    let dir_path = join(&workdir.to_path_buf(), scope);
    let file_path = join(&dir_path, filename);

    let file_display = file_path.display();
    let file_create_err = Err(anyhow!("Can not create file {}", file_display));
    let file_write_err = Err(anyhow!("Can not write to {} file", file_display));
    let dir_create_err = Err(anyhow!("Can not create {} directory", dir_path.display()));

    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).or(dir_create_err)?;
    }

    fs::File::create(&file_path)
        .or(file_create_err)?
        .write(bytes)
        .or(file_write_err)?;

    Ok(())
}
