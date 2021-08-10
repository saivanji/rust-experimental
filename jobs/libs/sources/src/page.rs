use anyhow::{anyhow, Result};
use log::info;
use scraper::{ElementRef, Html, Selector};
use url::Url;

pub struct Page {
    html: Html,
}

impl Page {
    pub fn fetch(url: &Url) -> Result<Self> {
        let text = reqwest::blocking::get(url.clone())?.text()?;
        let html = Html::parse_document(text.as_str());

        if html.errors.len() != 0 {
            return Err(anyhow!("Failed to parse {} page", url));
        }

        info!("{} was fetched successfully", url);

        Ok(Page { html })
    }

    pub fn node(&self) -> Node {
        Node {
            value: self.html.root_element(),
        }
    }
}

pub struct Node<'a> {
    value: ElementRef<'a>,
}

impl<'a> Node<'a> {
    pub fn select_first(&self, selector: &'a Selector) -> Option<Node> {
        self.select_all(selector).next()
    }

    pub fn select_all(&self, selector: &'a Selector) -> impl Iterator<Item = Node> + '_ {
        self.value
            .select(selector)
            .map(|element| Node { value: element })
    }

    pub fn attr(&self, value: &str) -> Option<String> {
        self.value.value().attr(value).map(String::from)
    }

    pub fn inner_html(&self) -> String {
        self.value.inner_html()
    }
}

pub fn path(path: &str) -> Selector {
    Selector::parse(path).unwrap()
}
