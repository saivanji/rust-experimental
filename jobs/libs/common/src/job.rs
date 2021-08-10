use crate::{Remote, SourceType};
use postgres::Row;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub url: Url,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub remote: Remote,
    pub source: SourceType,
}

impl Job {
    pub fn new(
        url: Url,
        title: String,
        description: String,
        tags: Vec<String>,
        remote: Remote,
        source: SourceType,
    ) -> Self {
        Self {
            url,
            title,
            description,
            tags,
            remote,
            source,
        }
    }
}

impl From<Row> for Job {
    fn from(row: Row) -> Self {
        let url = Url::parse(row.get("url")).unwrap();
        let source = row.get("source");
        let title = row.get("title");
        let description = row.get("description");
        let tags = row.get("tags");
        let remote = row.get("remote");

        Self::new(url, title, description, tags, remote, source)
    }
}
