mod collection;
mod db;
mod error;

use crate::error::Result;
use collection::Collection;
use common::{Identity, Job};
use db::Db;

pub use error::StoreError;

#[derive(Clone)]
pub struct Store {
    db: Db,
}

impl Store {
    pub async fn new(addr: impl Into<String>) -> Result<Self> {
        let db = Db::new(addr).await?;

        Ok(Self { db })
    }

    pub async fn insert_job(&self, job: &Job) -> Result<()> {
        let url = job.url.as_str();

        self.db
            .insert(
                "
                INSERT INTO jobs (url, source, title, description, tags, remote)
                VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING
                ",
                &[
                    &url,
                    &job.source,
                    &job.title,
                    &job.description,
                    &job.tags,
                    &job.remote,
                ],
            )
            .await
    }

    pub async fn list_jobs(&self, limit: i64, offset: i64) -> Result<Collection<Job>> {
        Collection::fetch(
            "
            SELECT count(id) AS total FROM jobs
            ",
            "
            SELECT id, url, source, title, description, tags, remote
            FROM jobs LIMIT $1 OFFSET $2
            ",
            &[&limit, &offset],
            &self.db,
        )
        .await
    }

    pub async fn create_user(&self, username: &str, hash: &str) -> Result<()> {
        self.db
            .insert(
                "
                INSERT INTO users (username, hash) VALUES ($1, $2)
                ",
                &[&username, &hash],
            )
            .await
    }

    pub async fn get_user_hash(&self, username: &str) -> Result<Option<Identity<String>>> {
        self.db
            .query_one(
                "
                SELECT id, hash FROM users WHERE username = $1
                ",
                &[&username],
            )
            .await
            .map(|res| res.map(|row| Identity::new(row.get("id"), row.get("hash"))))
    }
}
