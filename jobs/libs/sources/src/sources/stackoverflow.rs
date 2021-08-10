use crate::page::{path, Page};
use anyhow::Result;
use common::{Job, Remote, Source, SourceType};
use url::Url;

const BASE_URL: &str = "https://stackoverflow.com";
const INDEX_PATH: &str = "/jobs?sort=p";
const THRESHOLD: &str = "7h ago";

#[derive(Clone)]
pub struct StackOverflow {
    next_page: Option<String>,
}

impl StackOverflow {
    pub fn new() -> Self {
        Self {
            next_page: Some(String::from(INDEX_PATH)),
        }
    }

    fn fetch_description(&self, url: &Url) -> Result<Option<String>> {
        let page = Page::fetch(url)?;
        let root_node = page.node();

        Ok(root_node
            .select_first(&path("#overview-items section:last-of-type > div"))
            .map(|node| node.inner_html()))
    }

    fn fetch_page(&mut self) -> Result<Option<Vec<Job>>> {
        if let Some(next_page) = self.next_page.clone() {
            let base = Url::parse(BASE_URL)?;
            let page_url = base.join(&next_page)?;
            let page = Page::fetch(&page_url)?;
            let root_node = page.node();

            self.next_page = root_node
                .select_first(&path(".s-pagination a:last-child"))
                .and_then(|node| node.attr("href"));

            let jobs = root_node
                .select_all(&path("[data-jobid]"))
                .map(|node| {
                    let url = node
                        .attr("data-preview-url")
                        .and_then(|path| base.join(&path).ok())?;
                    let title = node
                        .select_first(&path("h2 > a"))
                        .and_then(|node| node.attr("title"))?;
                    let description = self.fetch_description(&url).ok().and_then(|desc| desc)?;

                    let tags = node
                        .select_all(&path("h3 + div a"))
                        .map(|node| node.inner_html())
                        .collect();

                    let remote = node
                        .select_all(&path("h3 + div + ul li"))
                        .map(|node| node.inner_html())
                        .find(|s| s.to_lowercase().contains("remote"));

                    let remote = match remote.as_deref() {
                        Some("Remote") => Remote::Yes,
                        Some("Limited remote") => Remote::Partial,
                        _ => Remote::No,
                    };

                    let should_skip = node
                        .select_first(&path("ul li:first-child span"))
                        .filter(|node| node.inner_html() == THRESHOLD)
                        .is_some();

                    if should_skip {
                        self.next_page = None;
                    }

                    Some(Job::new(url, title, description, tags, remote, self.kind()))
                })
                .filter(|node| node.is_some())
                .collect::<Option<Vec<Job>>>();

            Ok(jobs)
        } else {
            Ok(None)
        }
    }
}

impl Source for StackOverflow {
    fn kind(&self) -> SourceType {
        SourceType::StackOverflow
    }
}

impl Iterator for StackOverflow {
    type Item = Vec<Job>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(Some(jobs)) = self.fetch_page() {
            Some(jobs)
        } else {
            None
        }
    }
}
