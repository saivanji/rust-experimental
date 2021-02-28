use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::iter;
use std::path::{Path, PathBuf};
use std::str;
use url::Url;

// TODO: rewrite using new structs.

// split files by structs - url, page, node

// struct Url {}

// struct Page {}

// impl Page {
//     fn select() {}
//     fn traverse() {}
// }

struct Node<'a> {
    kind: NodeKind,
    element: ElementRef<'a>,
}

impl<'a> Node<'a> {
    fn new(kind: NodeKind, element: ElementRef<'a>) -> Self {
        Self { kind, element }
    }
}

#[derive(Clone, Copy)]
enum NodeKind {
    Anchor,
    Link,
    Script,
}

#[async_recursion(?Send)]
pub async fn process_page(url: &Url, workdir: &Path, trail: &mut BTreeSet<String>) -> Result<()> {
    let bytes = fetch(url).await?;
    let url_path = url.path();

    persist_file(&bytes, &url_path, &workdir)?;
    trail.insert(no_trailing_slash(url_path).to_string());

    match convert_bytes(&bytes).and_then(parse_document) {
        Some(page) => {
            traverse(&page, url, workdir, trail).await?;
            Ok(())
        }
        None => Ok(()),
    }
}

async fn traverse(
    page: &Html,
    url: &Url,
    workdir: &Path,
    trail: &mut BTreeSet<String>,
) -> Result<()> {
    let mut all = Vec::new();

    let mut anchors = select(page, NodeKind::Anchor);
    let mut links = select(page, NodeKind::Link);
    let mut scripts = select(page, NodeKind::Script);

    all.append(&mut anchors);
    all.append(&mut links);
    all.append(&mut scripts);

    for link in page.select(&anchors) {
        let should_process = |url: &Url| !trail.contains(no_trailing_slash(url.path()));

        match extract_url(&url, &link) {
            Some(url) if should_process(&url) => process_page(&url, workdir, trail).await?,
            _ => continue,
        }
    }

    Ok(())
}

// TODO: will be moved to "Page" struct
fn select(page: &Html, kind: NodeKind) -> Vec<Node> {
    let selector = match kind {
        NodeKind::Anchor => "a",
        NodeKind::Link => "link",
        NodeKind::Script => "script",
    };

    page.select(&Selector::parse(selector).unwrap())
        .map(|element| Node::new(kind, element))
        .collect()
}

fn convert_bytes(bytes: &Vec<u8>) -> Option<&str> {
    match str::from_utf8(&bytes) {
        Ok(text) => Some(&text),
        Err(_) => None,
    }
}

fn parse_document(text: &str) -> Option<Html> {
    let page = Html::parse_document(text);

    if page.errors.len() != 0 {
        return None;
    }

    Some(page)
}

fn extract_url(url: &Url, link: &ElementRef) -> Option<Url> {
    link.value()
        .attr("href")
        // filtering out external links
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

    if path == "/" {
        return ("", default_filename);
    }

    match path.rsplit_once("/") {
        Some((path, filename)) if filename.contains(".") => (path, filename),
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
